from typing import Union

from IPython.core.display_functions import display
from ipycanvas import Canvas, hold_canvas

from ..util import to_hex, RandomUniqueColorsProvider
from ...grpc_pipelines.context_values import *
from ...grpc_pipelines.models.pipelines_and_context_pb2 import *

legend_rect_width = 40
legend_rect_height = 20
x_delta = 10
axes_margin = 15
axes_width = 1
axes_padding = 5
overall_delta = axes_margin + axes_width + axes_padding
text_size_px = 10
black = (0, 0, 0)
white = (255, 255, 255)
background_key = 'Background'
separator_key = 'Separator'


def draw_log_timeline_diagram_canvas(diagram: GrpcLogTimelineDiagram,
                                     rect_width_scale: float,
                                     distance_scale: float,
                                     title: Optional[str],
                                     save_path: Optional[str],
                                     plot_legend: bool,
                                     width_scale: float,
                                     height_scale: float):
  colors, mappings = _init_state()
  provider = RandomUniqueColorsProvider(used_colors={black, white})
  max_width = _calculate_max_width(diagram, distance_scale, rect_width_scale)
  rect_width = rect_width_scale
  colors_log = []

  for trace_diagram in diagram.traces:
    for thread in trace_diagram.threads:
      colors_trace = []
      last_x = 0
      for event in thread.events:
        if event.name not in colors:
          c = provider.next()
          mappings.append(ProxyColorMapping(event.name, Color(c[0], c[1], c[2])))
          colors[event.name] = len(colors)

        rect_x = event.stamp * distance_scale
        if last_x != rect_x:
          colors_trace.append(ProxyColorRectangle(colors[background_key], last_x, rect_x - last_x))

        colors_trace.append(ProxyColorRectangle(colors[event.name], rect_x, rect_width))
        last_x = rect_x + rect_width

      colors_log.append(ProxyColorsTrace(colors_trace, False))

    colors_log.append(ProxyColorsTrace([ProxyColorRectangle(colors[separator_key], 0, max_width)], False))

  draw_colors_event_log_canvas(ProxyColorsEventLog(mappings, colors_log),
                               title=title,
                               save_path=save_path,
                               plot_legend=plot_legend,
                               height_scale=height_scale,
                               width_scale=width_scale)


def _init_state():
  colors = dict()

  colors[background_key] = 0
  colors[separator_key] = 1
  mappings = [
    ProxyColorMapping(background_key, Color(white[0], white[1], white[2])),
    ProxyColorMapping(separator_key, Color(black[0], black[1], black[2]))
  ]

  return colors, mappings


def _calculate_max_width(diagram: GrpcLogTimelineDiagram, distance_scale: float, rect_width_scale: float) -> float:
  max_stamp = 0
  max_events = 0
  for trace_diagram in diagram.traces:
    for thread in trace_diagram.threads:
      max_stamp = max(max_stamp, thread.events[-1].stamp)
      max_events = max(max_events, len(thread.events))

  return max_stamp * distance_scale + max_events * rect_width_scale


def draw_colors_event_log_canvas(log: Union[ProxyColorsEventLog, GrpcColorsEventLog],
                                 title: Optional[str] = None,
                                 plot_legend: bool = False,
                                 width_scale: float = 1,
                                 height_scale: float = 1,
                                 save_path: Optional[str] = None):
  max_width = _calculate_canvas_width(log)

  title_height = 20 if title is not None else 10
  canvas_height = len(log.traces) * height_scale + overall_delta + title_height

  before_height = canvas_height
  names_to_colors = None
  if plot_legend:
    names_to_colors = dict()
    for mapping in log.mapping:
      color = mapping.color
      names_to_colors[mapping.name] = to_hex((color.red, color.green, color.blue))

    canvas_height += len(names_to_colors) * legend_rect_height

  canvas = Canvas(width=max_width * width_scale + overall_delta + axes_margin,
                  height=canvas_height,
                  sync_image_data=save_path is not None)

  def save_to_file(change):
    if save_path is not None:
      canvas.to_file(save_path)

  if save_path is not None:
    canvas.observe(save_to_file, "image_data")

  _draw_actual_traces_diversity_diagram(log, canvas, title, title_height, before_height,
                                        max_width, width_scale, height_scale)

  if names_to_colors is not None:
    _draw_legend(canvas, names_to_colors, before_height)

  if save_path is None:
    display(canvas)


def _calculate_canvas_width(log: Union[ProxyColorsEventLog, GrpcColorsEventLog]):
  max_width = 0
  for trace in log.traces:
    width = 0
    for event in trace.event_colors:
      width += event.length

    max_width = max(max_width, width)

  return max_width


def _draw_actual_traces_diversity_diagram(log: Union[ProxyColorsEventLog, GrpcColorsEventLog],
                                          canvas: Canvas,
                                          title: Optional[str],
                                          title_height: float,
                                          before_height: float,
                                          max_width: int,
                                          width_scale: float,
                                          height_scale: float):
  with hold_canvas():
    canvas.fill_style = 'white'
    canvas.fill_rect(0, 0, canvas.width, canvas.height)

    canvas.fill_style = 'black'

    if title is not None:
      canvas.font = f'{text_size_px}px'
      canvas.fill_text(title, canvas.width / 2, title_height / 2)

    canvas.stroke_style = 'black'
    canvas.stroke_line(axes_margin, title_height, axes_margin, before_height - axes_margin)
    canvas.stroke_line(axes_margin, before_height - axes_margin, canvas.width, before_height - axes_margin)

    canvas.font = f'f{text_size_px}px'
    canvas.fill_text(str(len(log.traces)), 0, 10 + title_height)
    x_axis_text = str(max_width)
    canvas.fill_text(x_axis_text, canvas.width - ((text_size_px + 1) / 2) * len(x_axis_text), before_height)

    current_y = title_height

    for trace in log.traces:
      current_x = overall_delta
      xs = []
      widths = [] if not trace.constant_width else None
      colors = []

      if len(trace.event_colors) == 0:
        continue

      for rect in trace.event_colors:
        color = log.mapping[rect.color_index].color
        rect_width = rect.length * width_scale

        xs.append(current_x)

        if not trace.constant_width:
          widths.append(rect_width)

        colors.append((color.red, color.green, color.blue))

        current_x += rect_width

      width_value = widths if not trace.constant_width else width_scale
      canvas.fill_styled_rects(xs, current_y, width_value, height_scale, colors)

      current_y += height_scale


def _draw_legend(canvas: Canvas, names_to_colors: dict[str, str], before_height: float):
  with hold_canvas():
    index = 0
    current_x = canvas.width / 3

    for name, color in names_to_colors.items():
      canvas.fill_style = color
      canvas.fill_rect(current_x, before_height + legend_rect_height * index, legend_rect_width,
                       legend_rect_height)

      canvas.fill_style = 'black'
      canvas.fill_text(name,
                       current_x + legend_rect_width + x_delta,
                       before_height + legend_rect_height * index + legend_rect_height / 1.4)

      index += 1

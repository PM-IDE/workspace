from typing import Union

from IPython.core.display_functions import display
from ipycanvas import Canvas, hold_canvas

from ..util import to_hex, RandomUniqueColorsProvider
from ...grpc_pipelines.context_values import *
from ...grpc_pipelines.models.pipelines_and_context_pb2 import *

legend_rect_width = 40
legend_rect_height = 20
x_delta = 10
axis_margin = 15
axis_width = 1
axis_padding = 5
overall_delta = axis_margin + axis_width + axis_padding
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
  rect_width = rect_width_scale
  colors_log = []
  adjustments = []
  rectangle_adjustment_trace_index_delta = 0

  for trace_diagram in diagram.traces:
    for thread in trace_diagram.threads:
      colors_trace = []
      for event in thread.events:
        if event.name not in colors:
          c = provider.next()
          mappings.append(ProxyColorMapping(event.name, Color(c[0], c[1], c[2])))
          colors[event.name] = len(colors)

        rect_x = event.stamp * distance_scale
        colors_trace.append(ProxyColorRectangle(colors[event.name], rect_x, rect_width))

      colors_log.append(ProxyColorsTrace(colors_trace, False))

    adjustments.append(create_axis_after_trace_adjustment(len(colors_log)))

    for event_group in trace_diagram.events_groups:
      adjustments.append(create_rectangle_adjustment(
        ProxyColorsLogPoint(
          event_group.start_point.trace_index + rectangle_adjustment_trace_index_delta,
          event_group.start_point.event_index
        ),
        ProxyColorsLogPoint(
          event_group.end_point.trace_index + rectangle_adjustment_trace_index_delta,
          event_group.end_point.event_index
        ),
        True
      ))

    rectangle_adjustment_trace_index_delta += len(trace_diagram.threads)

  draw_colors_event_log_canvas(ProxyColorsEventLog(mappings, colors_log, adjustments),
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


def draw_colors_event_log_canvas(log: Union[ProxyColorsEventLog, GrpcColorsEventLog],
                                 title: Optional[str] = None,
                                 plot_legend: bool = False,
                                 width_scale: float = 1,
                                 height_scale: float = 1,
                                 save_path: Optional[str] = None):
  max_width = _calculate_canvas_width(log, width_scale)
  additional_axis = _create_additional_axis_list(log.adjustments)

  title_height = 20 if title is not None else 10
  canvas_height = len(log.traces) * height_scale + overall_delta + title_height + len(additional_axis) * axis_width

  before_height = canvas_height
  names_to_colors = None
  if plot_legend:
    names_to_colors = dict()
    for mapping in log.mapping:
      color = mapping.color
      names_to_colors[mapping.name] = to_hex((color.red, color.green, color.blue))

    canvas_height += len(names_to_colors) * legend_rect_height

  canvas = Canvas(width=max_width + overall_delta + axis_margin,
                  height=canvas_height,
                  sync_image_data=save_path is not None)

  def save_to_file(change):
    if save_path is not None:
      canvas.to_file(save_path)

  if save_path is not None:
    canvas.observe(save_to_file, "image_data")

  _draw_actual_traces_diversity_diagram(log, canvas, title, title_height, before_height,
                                        max_width, width_scale, height_scale, additional_axis)

  if names_to_colors is not None:
    _draw_legend(canvas, names_to_colors, before_height)

  if save_path is None:
    display(canvas)


def _create_additional_axis_list(adjustments: list[ProxyColorsLogAdjustment]) -> list[int]:
  additional_axis = []
  for adjustment in adjustments:
    if adjustment.axis_after_trace is not None:
      additional_axis.append(adjustment.axis_after_trace.trace_index)

  additional_axis.sort()

  return additional_axis


def _calculate_canvas_width(log: Union[ProxyColorsEventLog, GrpcColorsEventLog], width_scale):
  max_width = 0
  for trace in log.traces:
    last = trace.event_colors[-1]
    max_width = max(max_width, last.start_x * width_scale + last.length * width_scale)

  return max_width


def _draw_actual_traces_diversity_diagram(log: Union[ProxyColorsEventLog, GrpcColorsEventLog],
                                          canvas: Canvas,
                                          title: Optional[str],
                                          title_height: float,
                                          before_height: float,
                                          max_width: int,
                                          width_scale: float,
                                          height_scale: float,
                                          additional_axis: list[int]):
  with hold_canvas():
    current_y = title_height
    current_max_width = 0
    traces_extended_ys = []
    traces_ys = []
    traces_count_before_axis = 0
    traces_group_last_y = current_y

    clear_canvas(canvas)
    draw_axis(canvas, log, title, before_height, title_height, max_width)

    for index, trace in enumerate(log.traces):
      xs = []
      widths = [] if not trace.constant_width else None
      colors = []

      if len(trace.event_colors) == 0:
        continue

      for rect in trace.event_colors:
        color = log.mapping[rect.color_index].color
        rect_width = rect.length * width_scale

        xs.append(rect.start_x * width_scale + overall_delta)

        if not trace.constant_width:
          widths.append(rect_width)

        colors.append((color.red, color.green, color.blue))

      current_max_width = max(current_max_width, trace.event_colors[-1].start_x * width_scale + overall_delta + rect_width)

      width_value = widths if not trace.constant_width else width_scale * trace.event_colors[0].length
      canvas.fill_styled_rects(xs, current_y, width_value, height_scale, colors)

      traces_ys.append(current_y)

      if index in additional_axis:
        for _ in range(traces_count_before_axis):
          traces_extended_ys.append((traces_group_last_y, current_y))

        canvas.fill_style = "black"
        canvas.stroke_line(axis_margin, current_y, current_max_width, current_y)
        current_max_width = 0
        current_y += axis_width
        traces_group_last_y = current_y
        traces_count_before_axis = 0

      current_y += height_scale
      traces_count_before_axis += 1

    for _ in range(len(traces_extended_ys), len(log.traces)):
      traces_extended_ys.append((traces_group_last_y, current_y))

    draw_rectangles(log, canvas, traces_ys, traces_extended_ys, width_scale, height_scale)

def draw_rectangles(log, canvas, traces_ys, traces_extended_ys, width_scale, height_scale):
  for adjustment in log.adjustments:
    if adjustment.rectangle_adjustment is not None:
      up_left_point = adjustment.rectangle_adjustment.up_left_point
      down_right_point = adjustment.rectangle_adjustment.down_right_point

      up_left_event = log.traces[up_left_point.trace_index].event_colors[up_left_point.event_index]
      down_right_event = log.traces[down_right_point.trace_index].event_colors[down_right_point.event_index]

      x = up_left_event.start_x * width_scale + overall_delta
      width = down_right_event.start_x * width_scale + overall_delta + down_right_event.length * width_scale - x

      if adjustment.rectangle_adjustment.extend_to_nearest_vertical_borders:
        y = traces_extended_ys[up_left_point.trace_index][0]
        height = traces_extended_ys[down_right_point.trace_index][1] - y
      else:
        y = traces_ys[up_left_point.trace_index]
        height = traces_ys[down_right_point.trace_index] - y + height_scale

      canvas.stroke_style = "red"
      canvas.stroke_rect(x, y, width, height)


def clear_canvas(canvas: Canvas):
  canvas.fill_style = 'white'
  canvas.fill_rect(0, 0, canvas.width, canvas.height)


def draw_axis(canvas: Canvas, log, title: Optional[str], before_height: float, title_height: float, max_width: int):
  canvas.fill_style = 'black'

  if title is not None:
    canvas.font = f'{text_size_px}px'
    canvas.fill_text(title, canvas.width / 2, title_height / 2)

  canvas.stroke_style = 'black'
  canvas.stroke_line(axis_margin, title_height, axis_margin, before_height - axis_margin)
  canvas.stroke_line(axis_margin, before_height - axis_margin, canvas.width, before_height - axis_margin)

  canvas.font = f'f{text_size_px}px'
  canvas.fill_text(str(len(log.traces)), 0, 10 + title_height)
  x_axis_text = str(max_width)
  canvas.fill_text(x_axis_text, canvas.width - ((text_size_px + 1) / 2) * len(x_axis_text), before_height)


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

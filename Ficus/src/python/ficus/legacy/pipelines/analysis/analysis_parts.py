from ..common import InternalDrawingPipelinePart
from ..serialization.pipeline_parts import SavePathCreator
from ...analysis.event_log_analysis import *
from ...analysis.event_log_analysis_entropy import draw_position_entropies_histogram
from ...analysis.event_log_info import create_log_information
from ...pipelines.contexts.accessors import log, cached_colors
from ...pipelines.pipelines import *


class TracesDiversityDiagram(InternalPipelinePart):
  def __init__(self,
               plot_legend: bool,
               height_scale: int = 1,
               width_scale: int = 1,
               save_path: Union[str, SavePathCreator] = None,
               title: str = None):
    self.title = title
    self.plot_legend = plot_legend
    self.height_scale = height_scale
    self.save_path = save_path
    self.width_scale = width_scale

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    save_path = None
    if self.save_path is not None:
      save_path = self.save_path if type(self.save_path) == str else self.save_path(current_input)

    draw_traces_diversity_diagram(log(current_input),
                                  cached_colors(current_input),
                                  title=self.title,
                                  save_path=save_path,
                                  plot_legend=self.plot_legend,
                                  height_scale=self.height_scale,
                                  width_scale=self.width_scale)
    return current_input


class TracesHistogram(InternalPipelinePart):
  def __init__(self, save_path: str = None, title: str = None):
    self.save_path = save_path
    self.title = title

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    draw_traces_histogram(log(current_input), save_path=self.save_path, title=self.title)
    return current_input


class EventsPlacement(InternalPipelinePart):
  def __init__(self,
               events: set[str],
               plot_legend: bool,
               height_scale: int = 1,
               save_path: str = None,
               title: str = None):
    self.events = events
    self.title = title
    self.plot_legend = plot_legend
    self.height_scale = height_scale
    self.save_path = save_path

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    draw_placements_of_concrete_events(log(current_input),
                                       cached_colors(current_input),
                                       self.events,
                                       save_path=self.save_path,
                                       title=self.title,
                                       height_scale=self.height_scale,
                                       plot_legend=self.plot_legend)
    return current_input


class DrawDefaultEntropyHistogram(InternalPipelinePart):
  def __init__(self, save_path: str = None, title: str = None):
    self.title = title
    self.save_path = save_path

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    draw_events_entropy_histogram(log(current_input), save_path=self.save_path, title=self.title)
    return current_input


class DrawPosEntropyHistogram(InternalPipelinePart):
  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    draw_position_entropies_histogram(log(current_input))
    return current_input


class DrawPlacementOfEvents(InternalDrawingPipelinePart):
  def __init__(self,
               predicate: Callable[[str], bool],
               title: str = None,
               plot_legend: bool = False,
               height_scale: int = 1,
               width_scale: int = 1,
               save_path: str = None):
    super().__init__(title, plot_legend, height_scale, width_scale, save_path)
    self.predicate = predicate

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    draw_placements_of_events(log(current_input),
                              cached_colors(current_input),
                              self.predicate,
                              title=self.title,
                              save_path=self._get_save_path(current_input),
                              height_scale=self.height_scale,
                              plot_legend=self.plot_legend)

    return current_input


class SaveAllEventNames(InternalPipelinePart):
  def __init__(self, save_path: str):
    self.save_path = save_path

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    event_names = list(create_log_information(log(current_input)).events_count.keys())
    with open(self.save_path, 'w') as fout:
      fout.write('\n\n'.join(event_names))

    return current_input


class PrintEventLogInfo(InternalPipelinePart):
  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    info = create_log_information(log(current_input))
    print("============================================")
    print(f'Number of traces in log: {len(log(current_input))}')
    print(f'Number of events in log: {sum(map(len, log(current_input)))}')
    print(f'Number of low-level event classes in log: {len(info.events_count)}')
    print("============================================")

    return current_input


class PrintEventLogInfoBeforeAndAfter(InternalPipelinePart):
  def __init__(self, operation_name: str, inner_pipeline: Pipeline):
    self.operation_name = operation_name
    self.inner_pipeline = inner_pipeline

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    print(f'Operation {self.operation_name} started')

    PrintEventLogInfo().execute(current_input)
    result = self.inner_pipeline.execute(current_input)
    PrintEventLogInfo().execute(result)

    print(f'Operation {self.operation_name} finished')
    print()

    return result


class PrintDFG(InternalPipelinePart):
  def __init__(self, selector: Callable[[str], bool]):
    self.selector = selector

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    info = create_log_information(log(current_input))
    for event_name, followers in info.dfg.followed_events.items():
      if self.selector(event_name):
        print(f"Followers of {event_name}:")
        for follower, count in followers.items():
          print(f'{follower}: {count}')

        print()

    return current_input

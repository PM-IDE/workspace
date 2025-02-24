from ...filtering.event_log_filters import default_untouchable_func, filter_events_pos_entropy_direct, \
  filter_events_pos_entropy_indirect, filter_events_default_entropy_direct, filter_events_default_entropy_indirect, \
  TillEndFilteringMetricKind, filter_events_pos_entropy_direct_till_end, filter_events_pos_entropy_indirect_till_end
from ...pipelines.contexts.accessors import *
from ...pipelines.contexts.part_results import *
from ...mutations.event_log_mutations import remove_empty_traces
from ...pipelines.pipelines import InternalPipelinePart
from ...analysis.event_log_split import *
import matplotlib.pyplot as plt


class PosEntropyFilter(InternalPipelinePart):
  def __init__(self,
               threshold: float,
               untouchable_event_func: Callable[[str], bool] = default_untouchable_func):
    self.threshold = threshold
    self.untouchable_event_func = untouchable_event_func

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    return PipelinePartResult().with_log(self._do_filter(log(current_input)))

  def _do_filter(self, log: MyEventLog) -> MyEventLog:
    raise NotImplementedError()


class PosEntropyDirectFilter(PosEntropyFilter):
  def __init__(self,
               threshold: float,
               max_events_to_remove: int = int(1e9),
               untouchable_event_func: Callable[[str], bool] = default_untouchable_func):
    super().__init__(threshold, untouchable_event_func)
    self.max_events_to_remove = max_events_to_remove

  def _do_filter(self, log: MyEventLog):
    return filter_events_pos_entropy_direct(log,
                                            threshold=self.threshold,
                                            untouchable_event_func=self.untouchable_event_func,
                                            max_events_to_remove=self.max_events_to_remove)


class PosEntropyIndirectFilter(PosEntropyFilter):
  def __init__(self,
               threshold: float,
               max_events_to_remove: int = int(1e9),
               untouchable_event_func: Callable[[str], bool] = default_untouchable_func):
    super().__init__(threshold, untouchable_event_func)
    self.max_events_to_remove = max_events_to_remove

  def _do_filter(self, log: MyEventLog):
    return filter_events_pos_entropy_indirect(log,
                                              threshold=self.threshold,
                                              untouchable_event_func=self.untouchable_event_func,
                                              max_events_to_remove=self.max_events_to_remove)


class DfgEntropyFilter(InternalPipelinePart):
  def __init__(self,
               threshold: float,
               laplace: bool,
               untouchable_event_func: Callable[[str], bool] = default_untouchable_func):
    self.threshold = threshold
    self.laplace = laplace
    self.untouchable_event_func = untouchable_event_func

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    return PipelinePartResult().with_log(self._do_filter(log(current_input)))

  def _do_filter(self, log: MyEventLog):
    raise NotImplementedError()


class DfgEntropyDirectFilter(DfgEntropyFilter):
  def __init__(self,
               threshold: float,
               laplace: bool,
               max_events_to_remove: int = int(1e9)):
    super().__init__(threshold, laplace)
    self.max_events_to_remove = max_events_to_remove

  def _do_filter(self, log: MyEventLog):
    return filter_events_default_entropy_direct(log,
                                                self.laplace,
                                                self.threshold,
                                                self.max_events_to_remove,
                                                self.untouchable_event_func)


class DfgEntropyIndirectFilter(DfgEntropyFilter):
  def __init__(self,
               threshold: float,
               laplace: bool,
               max_events_to_remove: int = int(1e9),
               top_percent=0.05):
    super().__init__(threshold, laplace)
    self.top_percent = top_percent
    self.max_events_to_remove = max_events_to_remove

  def _do_filter(self, log: MyEventLog):
    return filter_events_default_entropy_indirect(log,
                                                  laplace=self.laplace,
                                                  threshold=self.threshold,
                                                  top_percent=self.top_percent,
                                                  max_events_to_remove=self.max_events_to_remove,
                                                  untouchable_event_func=self.untouchable_event_func)


class DrawPosEntropyDirectTillEndFilteringDiagram(InternalPipelinePart):
  def __init__(self, metric_kind: TillEndFilteringMetricKind):
    self.metric_kind = metric_kind

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    removals = filter_events_pos_entropy_direct_till_end(log(current_input),
                                                         metric_kind=self.metric_kind,
                                                         untouchable_event_func=default_untouchable_func)

    plt.plot(list(map(lambda x: x[1], removals)))
    plt.ylabel('Metric value')
    plt.xlabel('Iteration number')
    return current_input


class DrawPosEntropyIndirectTillEndFilteringDiagram(InternalPipelinePart):
  def __init__(self, metric_kind: TillEndFilteringMetricKind):
    self.metric_kind = metric_kind

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    removals = filter_events_pos_entropy_indirect_till_end(log(current_input),
                                                           metric_kind=self.metric_kind,
                                                           untouchable_event_func=default_untouchable_func)

    plt.plot(list(map(lambda x: x[1], removals)))
    plt.ylabel('Metric value')
    plt.xlabel('Iteration number')
    return current_input


class FilterTracesByCount(InternalPipelinePart):
  def __init__(self, min_event_in_trace_count: int = 0):
    self.min_event_count = min_event_in_trace_count

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    return current_input.with_log(remove_empty_traces(log(current_input),
                                                      min_event_count=self.min_event_count))


class FilterTracesByVariants(InternalPipelinePart):
  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    splitted_traces = split_log_by_traces(log(current_input))
    new_log = MyEventLog()
    for trace_variant in splitted_traces:
      new_log.append(trace_variant[0])

    return current_input.with_log(new_log)

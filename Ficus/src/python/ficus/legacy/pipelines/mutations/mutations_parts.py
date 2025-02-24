from typing import Iterable

from ...analysis.patterns.util import substitute_original_events
from ...mutations.event_log_mutations import *
from ...pipelines.pipelines import *
from ...pipelines.contexts.part_results import *
from ...pipelines.contexts.accessors import *


class RemoveEventFromLogByName(InternalPipelinePart):
  def __init__(self, name: str):
    self.name = name

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    return PipelinePartResult().with_log(remove_event_from_log(log(current_input), self.name))


class RemoveEventsFromLogByNames(InternalPipelinePart):
  def __init__(self, names: Iterable[str]):
    self.names = names

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    return PipelinePartResult().with_log(remove_events_from_log(log(current_input), set(self.names)))


class RemoveEventsFromLogPredicate(InternalPipelinePart):
  def __init__(self, predicate: Callable[[MyEvent], bool]):
    self.predicate = predicate

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    return current_input.with_log(filter_log(log(current_input), self.predicate))


class PreprocessEventsNames(InternalPipelinePart):
  def __init__(self, name_preprocessor: Callable[[str], str]):
    self.name_preprocessor = name_preprocessor

  def execute(self, current_input: Any) -> Any:
    current_log = log(current_input)
    preprocess_event_names_inplace(current_log, self.name_preprocessor)
    return PipelinePartResult().with_log(current_log)


class PreprocessEvents(InternalPipelinePart):
  def __init__(self, event_mutator: Callable[[MyEvent], None]):
    self.event_mutator = event_mutator

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    preprocess_events(log(current_input), self.event_mutator)
    return current_input


class SubstituteUnderlyingEvents(InternalPipelinePart):
  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    new_log = MyEventLog()
    for trace in log(current_input):
      new_log.append(substitute_original_events(trace))

    return current_input.with_log(new_log)


def all_events_selector(event: MyEvent):
  return True


class ApplyClassExtractor(InternalPipelinePart):
  def __init__(self,
               class_extractor: Callable[[MyEvent], str],
               event_selector: Callable[[MyEvent], bool] = all_events_selector):
    self.class_extractor = class_extractor
    self.event_selector = event_selector

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    current_log = log(current_input)
    for trace in current_log:
      for event in trace:
        if self.event_selector(event):
          event[concept_name] = self.class_extractor(event)

    return current_input


class RemoveLifecycleTransitionsAttributes(InternalPipelinePart):
  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    remove_lifecycle_attribute(log(current_input))
    return current_input


class AddArtificialStartEndEvents(InternalPipelinePart):
  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    for trace in log(current_input):
      fake_end_event = MyEvent()
      fake_end_event[concept_name] = "ARTIFICIAL_END"
      trace.append(fake_end_event)

      fake_start_event = MyEvent()
      fake_start_event[concept_name] = "ARTIFICIAL_START"
      trace.insert(fake_start_event, 0)

    return current_input

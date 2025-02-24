from enum import Enum

from .discovery_parts import *


class AnnotatePetriNet(ViewPetriNet):
  def __init__(self,
               annotation_key: str,
               annotation_pipeline_part: str,
               terminate_on_unreplayable_trace: bool = False,
               show_places_names: bool = False,
               name: str = 'petri_net',
               background_color: str = 'white',
               engine='dot',
               export_path: Optional[str] = None,
               rankdir: str = 'LR'):
    super().__init__(show_places_names, name, background_color, engine, export_path, rankdir, None)
    self.annotation_key = annotation_key
    self.annotation_pipeline_part = annotation_pipeline_part
    self.terminate_on_unreplayable_trace = terminate_on_unreplayable_trace

  def execute_callback(self, values: dict[str, GrpcContextValue]):
    petri_net = from_grpc_petri_net(values[const_petri_net].petriNet)
    annotation = self.get_annotation(values[self.annotation_key])
    draw_petri_net(petri_net,
                   show_places_names=self.show_places_names,
                   name=self.name,
                   background_color=self.background_color,
                   engine=self.engine,
                   rankdir=self.rankdir,
                   export_path=self.export_path,
                   annotation=annotation)

  def get_annotation(self, context_value: GrpcContextValue):
    raise NotImplementedError()

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_bool_value(config, const_terminate_on_unreplayable_trace, self.terminate_on_unreplayable_trace)
    part = create_complex_get_context_part(self.uuid,
                                           self.__class__.__name__,
                                           [const_petri_net, self.annotation_key],
                                           self.annotation_pipeline_part,
                                           config)

    return GrpcPipelinePartBase(complexContextRequestPart=part)


class AnnotatePetriNetWithCount(AnnotatePetriNet):
  def __init__(self,
               terminate_on_unreplayable_trace: bool = False,
               show_places_names: bool = False,
               name: str = 'petri_net',
               background_color: str = 'white',
               engine='dot',
               export_path: Optional[str] = None,
               rankdir: str = 'LR'):
    super().__init__(const_petri_net_count_annotation,
                     const_annotate_petri_net_count,
                     terminate_on_unreplayable_trace,
                     show_places_names, name, background_color, engine, export_path, rankdir)

  def get_annotation(self, context_value: GrpcContextValue):
    return from_grpc_count_annotation(context_value.annotation.countAnnotation)


class AnnotatePetriNetWithFrequency(AnnotatePetriNet):
  def __init__(self,
               terminate_on_unreplayable_trace: bool = False,
               show_places_names: bool = False,
               name: str = 'petri_net',
               background_color: str = 'white',
               engine='dot',
               export_path: Optional[str] = None,
               rankdir: str = 'LR'):
    super().__init__(const_petri_net_frequency_annotation,
                     const_annotate_petri_net_frequency,
                     terminate_on_unreplayable_trace,
                     show_places_names, name, background_color, engine, export_path, rankdir)

  def get_annotation(self, context_value: GrpcContextValue):
    return from_grpc_frequency_annotation(context_value.annotation.frequencyAnnotation)


class AnnotatePetriNetWithTraceFrequency(AnnotatePetriNet):
  def __init__(self,
               terminate_on_unreplayable_trace: bool = False,
               show_places_names: bool = False,
               name: str = 'petri_net',
               background_color: str = 'white',
               engine='dot',
               export_path: Optional[str] = None,
               rankdir: str = 'LR'):
    super().__init__(const_petri_net_trace_frequency_annotation,
                     const_annotate_petri_net_trace_frequency,
                     terminate_on_unreplayable_trace,
                     show_places_names, name, background_color, engine, export_path, rankdir)

  def get_annotation(self, context_value: GrpcContextValue):
    return from_grpc_frequency_annotation(context_value.annotation.frequencyAnnotation)


class TimeAnnotationKind(Enum):
  SummedTime = 0
  Mean = 1


class AnnotateGraphWithTime(ViewGraphLikeFormalismPart):
  def __init__(self,
               annotation_kind: TimeAnnotationKind,
               name: str = 'petri_net',
               background_color: str = 'white',
               engine='dot',
               export_path: Optional[str] = None,
               rankdir: str = 'LR'):
    super().__init__(name, background_color, engine, export_path, rankdir)
    self.annotation_kind = annotation_kind

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_enum_value(config, const_time_annotation_kind, const_time_annotation_kind_enum_name,
                      self.annotation_kind.name)

    part = create_complex_get_context_part(self.uuid,
                                           self.__class__.__name__,
                                           [const_graph, const_graph_time_annotation],
                                           const_annotate_graph_with_time,
                                           config)

    return GrpcPipelinePartBase(complexContextRequestPart=part)

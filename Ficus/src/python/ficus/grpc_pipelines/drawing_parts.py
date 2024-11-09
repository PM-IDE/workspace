from .entry_points.default_pipeline import *
from .models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcPipelinePartConfiguration, \
    GrpcContextKeyValue, GrpcContextKey


class TracesDiversityDiagram(PipelinePart2WithDrawColorsLogCallback):
    def __init__(self,
                 title: Optional[str] = None,
                 save_path: str = None,
                 plot_legend: bool = True,
                 height_scale: int = 1,
                 width_scale: int = 1):
        super().__init__(title=title,
                         save_path=save_path,
                         plot_legend=plot_legend,
                         height_scale=height_scale,
                         width_scale=width_scale)

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return _create_traces_diversity_grpc_part(self.uuid, self.__class__.__name__)


def _create_traces_diversity_grpc_part(uuid: uuid.UUID, frontend_pipeline_part_name: str):
    config = GrpcPipelinePartConfiguration()
    part = create_complex_get_context_part(uuid,
                                           frontend_pipeline_part_name,
                                           [const_colors_event_log],
                                           const_traces_diversity_diagram,
                                           config)

    return GrpcPipelinePartBase(complexContextRequestPart=part)


class TracesDiversityDiagramCanvas(PipelinePart2WithCanvasCallback):
    def __init__(self,
                 save_path: Optional[str] = None,
                 plot_legend: bool = False,
                 title: Optional[str] = None,
                 height_scale: float = 1,
                 width_scale: float = 1):
        super().__init__(save_path=save_path,
                         plot_legend=plot_legend,
                         title=title,
                         width_scale=width_scale,
                         height_scale=height_scale)

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return _create_traces_diversity_grpc_part(self.uuid, self.__class__.__name__)


class DrawPlacementsOfEventByName(PipelinePart2WithDrawColorsLogCallback):
    def __init__(self,
                 event_name: str,
                 title: str = None,
                 save_path: Optional[str] = None,
                 plot_legend: bool = True,
                 height_scale: int = 1,
                 width_scale: int = 1):
        super().__init__(title=title,
                         save_path=save_path,
                         plot_legend=plot_legend,
                         height_scale=height_scale,
                         width_scale=width_scale)

        self.event_name = event_name

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return _create_draw_placements_of_events_by_name_grpc_part(self.uuid, self.__class__.__name__, self.event_name)


def _create_draw_placements_of_events_by_name_grpc_part(uuid: uuid.UUID, frontend_pipeline_part_name: str, event_name: str):
    config = GrpcPipelinePartConfiguration()
    config.configurationParameters.append(GrpcContextKeyValue(
        key=GrpcContextKey(name=const_event_name),
        value=StringContextValue(event_name).to_grpc_context_value()
    ))

    part = create_complex_get_context_part(uuid,
                                           frontend_pipeline_part_name,
                                           [const_colors_event_log],
                                           const_draw_placement_of_event_by_name, config)

    return GrpcPipelinePartBase(complexContextRequestPart=part)


class DrawPlacementsOfEventByNameCanvas(TracesDiversityDiagramCanvas):
    def __init__(self,
                 event_name: str,
                 save_path: str = None,
                 plot_legend: bool = False,
                 title: Optional[str] = None,
                 height_scale: float = 1,
                 width_scale: float = 1):
        super().__init__(save_path=save_path,
                         plot_legend=plot_legend,
                         title=title,
                         width_scale=width_scale,
                         height_scale=height_scale)

        self.event_name = event_name

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return _create_draw_placements_of_events_by_name_grpc_part(self.uuid, self.__class__.__name__, self.event_name)


class DrawPlacementOfEventsByRegex(PipelinePart2WithDrawColorsLogCallback):
    def __init__(self,
                 regex: str,
                 title: str = None,
                 save_path: str = None,
                 plot_legend: bool = True,
                 height_scale: int = 1,
                 width_scale: int = 1):
        super().__init__(title=title,
                         save_path=save_path,
                         plot_legend=plot_legend,
                         height_scale=height_scale,
                         width_scale=width_scale)

        self.regex = regex

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return _create_draw_placements_of_events_by_regex_grpc_part(self.uuid, self.__class__.__name__, self.regex)


def _create_draw_placements_of_events_by_regex_grpc_part(uuid: uuid.UUID, frontend_pipeline_part_name: str, regex: str):
    config = GrpcPipelinePartConfiguration()
    append_string_value(config, const_regex, regex)

    part = create_complex_get_context_part(uuid,
                                           frontend_pipeline_part_name,
                                           [const_colors_event_log],
                                           const_draw_placement_of_event_by_regex,
                                           config)

    return GrpcPipelinePartBase(complexContextRequestPart=part)


class DrawPlacementOfEventsByRegexCanvas(TracesDiversityDiagramCanvas):
    def __init__(self,
                 regex: str,
                 save_path: str = None,
                 plot_legend: bool = False,
                 title: Optional[str] = None,
                 height_scale: float = 1,
                 width_scale: float = 1):
        super().__init__(save_path=save_path,
                         plot_legend=plot_legend,
                         title=title,
                         width_scale=width_scale,
                         height_scale=height_scale)

        self.regex = regex

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return _create_draw_placements_of_events_by_regex_grpc_part(self.uuid, self.__class__.__name__, self.regex)


class DrawActivitiesDiagramBase(PipelinePart2WithDrawColorsLogCallback):
    def __init__(self,
                 diagram_kind: str,
                 title: str = None,
                 save_path: str = None,
                 plot_legend: bool = True,
                 height_scale: int = 1,
                 width_scale: int = 1):
        super().__init__(title=title,
                         save_path=save_path,
                         plot_legend=plot_legend,
                         height_scale=height_scale,
                         width_scale=width_scale)

        self.diagram_kind = diagram_kind

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return _create_draw_activities_diagram_grpc_part(self.uuid, self.__class__.__name__, self.diagram_kind)


def _create_draw_activities_diagram_grpc_part(uuid: uuid.UUID, frontend_pipeline_part_name: str, diagram_kind: str):
    config = GrpcPipelinePartConfiguration()
    part = create_complex_get_context_part(uuid, frontend_pipeline_part_name, [const_colors_event_log], diagram_kind, config)
    return GrpcPipelinePartBase(complexContextRequestPart=part)


class DrawActivitiesDiagramBaseCanvas(TracesDiversityDiagramCanvas):
    def __init__(self,
                 diagram_kind: str,
                 save_path: str = None,
                 plot_legend: bool = False,
                 title: Optional[str] = None,
                 height_scale: float = 1,
                 width_scale: float = 1):
        super().__init__(save_path=save_path,
                         plot_legend=plot_legend,
                         title=title,
                         width_scale=width_scale,
                         height_scale=height_scale)

        self.diagram_kind = diagram_kind

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return _create_draw_activities_diagram_grpc_part(self.uuid, self.__class__.__name__, self.diagram_kind)


class DrawFullActivitiesDiagram(DrawActivitiesDiagramBase):
    def __init__(self,
                 title: str = None,
                 save_path: str = None,
                 plot_legend: bool = True,
                 height_scale: int = 1,
                 width_scale: int = 1):
        super().__init__(const_draw_full_activities_diagram,
                         title=title,
                         save_path=save_path,
                         plot_legend=plot_legend,
                         height_scale=height_scale,
                         width_scale=width_scale)


class DrawFullActivitiesDiagramCanvas(DrawActivitiesDiagramBaseCanvas):
    def __init__(self,
                 save_path: str = None,
                 plot_legend: bool = False,
                 title: Optional[str] = None,
                 height_scale: float = 1,
                 width_scale: float = 1):
        super().__init__(const_draw_full_activities_diagram,
                         save_path=save_path,
                         plot_legend=plot_legend,
                         title=title,
                         width_scale=width_scale,
                         height_scale=height_scale)


class DrawShortActivitiesDiagram(DrawActivitiesDiagramBase):
    def __init__(self,
                 title: str = None,
                 save_path: str = None,
                 plot_legend: bool = True,
                 height_scale: int = 1,
                 width_scale: int = 1):
        super().__init__(const_draw_short_activities_diagram,
                         title=title,
                         save_path=save_path,
                         plot_legend=plot_legend,
                         height_scale=height_scale,
                         width_scale=width_scale)


class DrawShortActivitiesDiagramCanvas(DrawActivitiesDiagramBaseCanvas):
    def __init__(self,
                 save_path: str = None,
                 plot_legend: bool = False,
                 title: Optional[str] = None,
                 height_scale: float = 1,
                 width_scale: float = 1):
        super().__init__(const_draw_short_activities_diagram,
                         save_path=save_path,
                         plot_legend=plot_legend,
                         title=title,
                         width_scale=width_scale,
                         height_scale=height_scale)

class TracesDiversityDiagramByAttribute(PipelinePart2WithDrawColorsLogCallback):
    def __init__(self,
                 attribute: str,
                 title: Optional[str] = None,
                 save_path: str = None,
                 plot_legend: bool = True,
                 height_scale: int = 1,
                 width_scale: int = 1):
        super().__init__(title=title,
                         save_path=save_path,
                         plot_legend=plot_legend,
                         height_scale=height_scale,
                         width_scale=width_scale)

        self.attribute = attribute

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return _create_traces_diversity_diagram_by_attribute_grpc_part(self.attribute, self.uuid, self.__class__.__name__)


def _create_traces_diversity_diagram_by_attribute_grpc_part(attribute: str, uuid, frontend_pipeline_part_name: str):
    config = GrpcPipelinePartConfiguration()
    append_string_value(config, const_attribute, attribute)

    part = create_complex_get_context_part(uuid,
                                           frontend_pipeline_part_name,
                                           [const_colors_event_log],
                                           const_traces_diversity_diagram_by_attribute,
                                           config)

    return GrpcPipelinePartBase(complexContextRequestPart=part)


class TracesDiversityDiagramByAttributeCanvas(PipelinePart2WithCanvasCallback):
    def __init__(self,
                 attribute: str,
                 save_path: Optional[str] = None,
                 plot_legend: bool = False,
                 title: Optional[str] = None,
                 height_scale: float = 1,
                 width_scale: float = 1):
        super().__init__(save_path=save_path,
                         plot_legend=plot_legend,
                         title=title,
                         width_scale=width_scale,
                         height_scale=height_scale)

        self.attribute = attribute

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return _create_traces_diversity_diagram_by_attribute_grpc_part(self.attribute, self.uuid, self.__class__.__name__)

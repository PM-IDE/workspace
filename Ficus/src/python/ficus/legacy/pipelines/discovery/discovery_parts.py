import pm4py

from ...log.pm4py_converters import to_pm4py_log
from ...pipelines.pipelines import *
from ...pipelines.contexts.accessors import *
from ...pipelines.contexts.part_results import *


class DiscoverPetriNetInductive(InternalPipelinePart):
    def __init__(self, noise_threshold: float = 0):
        self.noise_threshold = noise_threshold

    def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
        net, start_marking, end_marking = pm4py.discover_petri_net_inductive(to_pm4py_log(log(current_input)),
                                                                             noise_threshold=self.noise_threshold)

        return current_input.with_petri_net(PetriNetWrapper(net, start_marking, end_marking))


class DiscoverPetriNetAlpha(InternalPipelinePart):
    def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
        net, start_marking, end_marking = pm4py.discover_petri_net_alpha(to_pm4py_log(log(current_input)))
        return current_input.with_petri_net(PetriNetWrapper(net, start_marking, end_marking))


class DiscoverPetriNetAlphaPlus(InternalPipelinePart):
    def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
        net, start_marking, end_marking = pm4py.discover_petri_net_alpha_plus(to_pm4py_log(log(current_input)))
        return current_input.with_petri_net(PetriNetWrapper(net, start_marking, end_marking))


class DiscoverPetriNetHeuristic(InternalPipelinePart):
    def __init__(self,
                 dependency_threshold: int = 0.5,
                 and_threshold: int = 0.5,
                 two_loops_threshold: int = 0.5):
        self.dependency_threshold = dependency_threshold
        self.and_threshold = and_threshold
        self.two_loops_threshold = two_loops_threshold

    def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
        net, s_marking, e_marking = pm4py.discover_petri_net_heuristics(to_pm4py_log(log(current_input)),
                                                                        dependency_threshold=self.dependency_threshold,
                                                                        and_threshold=self.and_threshold,
                                                                        loop_two_threshold=self.two_loops_threshold)

        return current_input.with_petri_net(PetriNetWrapper(net, s_marking, e_marking))


class DrawPetriNet(InternalPipelinePart):
    def __init__(self, save_path: str = None):
        self.save_path = save_path

    def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
        net = petri_net(current_input)
        if self.save_path is not None:
            pm4py.save_vis_petri_net(net.petri_net, net.start_marking, net.end_marking, self.save_path)
        else:
            pm4py.view_petri_net(net.petri_net, net.start_marking, net.end_marking)

        return current_input

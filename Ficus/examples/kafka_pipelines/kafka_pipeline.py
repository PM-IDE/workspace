from common import execute_pipeline
from ficus import *

execute_pipeline(
    [
        PrintEventLogInfo(),
        TracesDiversityDiagramCanvas(),
        DiscoverPetriNetHeuristic(),
        EnsureInitialMarking(),
        AnnotatePetriNetWithFrequency(),
        DiscoverFuzzyGraph(),
        ViewGraph()
    ]
)

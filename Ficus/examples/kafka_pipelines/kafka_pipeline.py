from common import execute_pipeline
from ficus import *

execute_pipeline(
    'MySubscription',
    'Pipeline',
    [
        PrintEventLogInfo(),
        RemainEventsByRegex('(Procfiler|Business)'),
        PrintEventLogInfo(),
        DiscoverFuzzyGraph(),
        ViewGraph()
    ]
)

from common import execute_pipeline
from ficus import *

execute_pipeline(
    'MySubscription',
    'TestPipeline',
    [
        PrintEventLog()
    ]
)

from common import execute_pipeline, pipeline_with_default_cfg
from ficus import *

execute_pipeline(
  'MySubscription',
  'TestPipeline',
  [
    pipeline_with_default_cfg([
      PrintEventLog()
    ])
  ]
)

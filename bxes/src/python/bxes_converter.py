import sys

from ficus.grpc_pipelines.context_values import StringContextValue
from ficus.grpc_pipelines.grpc_pipelines import Pipeline, ficus_backend_addr_key
from ficus.grpc_pipelines.xes_parts import WriteLogToBxes, ReadLogFromXes

Pipeline(
    ReadLogFromXes(),
    WriteLogToBxes(save_path=sys.argv[2])
).execute({
    'path': StringContextValue(sys.argv[1]),
    ficus_backend_addr_key: sys.argv[3]
})

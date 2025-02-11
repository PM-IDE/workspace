# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!

"""Client and server classes corresponding to protobuf-defined services."""

import grpc



import ficus.grpc_pipelines.models.front_contract_pb2 as front__contract__pb2

from google.protobuf import empty_pb2 as google_dot_protobuf_dot_empty__pb2





class GrpcPipelinePartsContextValuesServiceStub(object):

    """Missing associated documentation comment in .proto file."""



    def __init__(self, channel):

        """Constructor.



        Args:

            channel: A grpc.Channel.

        """

        self.StartUpdatesStream = channel.unary_stream(

                '/ficus.GrpcPipelinePartsContextValuesService/StartUpdatesStream',

                request_serializer=google_dot_protobuf_dot_empty__pb2.Empty.SerializeToString,

                response_deserializer=front__contract__pb2.GrpcPipelinePartUpdate.FromString,

                )





class GrpcPipelinePartsContextValuesServiceServicer(object):

    """Missing associated documentation comment in .proto file."""



    def StartUpdatesStream(self, request, context):

        """Missing associated documentation comment in .proto file."""

        context.set_code(grpc.StatusCode.UNIMPLEMENTED)

        context.set_details('Method not implemented!')

        raise NotImplementedError('Method not implemented!')





def add_GrpcPipelinePartsContextValuesServiceServicer_to_server(servicer, server):

    rpc_method_handlers = {

            'StartUpdatesStream': grpc.unary_stream_rpc_method_handler(

                    servicer.StartUpdatesStream,

                    request_deserializer=google_dot_protobuf_dot_empty__pb2.Empty.FromString,

                    response_serializer=front__contract__pb2.GrpcPipelinePartUpdate.SerializeToString,

            ),

    }

    generic_handler = grpc.method_handlers_generic_handler(

            'ficus.GrpcPipelinePartsContextValuesService', rpc_method_handlers)

    server.add_generic_rpc_handlers((generic_handler,))





 # This class is part of an EXPERIMENTAL API.

class GrpcPipelinePartsContextValuesService(object):

    """Missing associated documentation comment in .proto file."""



    @staticmethod

    def StartUpdatesStream(request,

            target,

            options=(),

            channel_credentials=None,

            call_credentials=None,

            insecure=False,

            compression=None,

            wait_for_ready=None,

            timeout=None,

            metadata=None):

        return grpc.experimental.unary_stream(request, target, '/ficus.GrpcPipelinePartsContextValuesService/StartUpdatesStream',

            google_dot_protobuf_dot_empty__pb2.Empty.SerializeToString,

            front__contract__pb2.GrpcPipelinePartUpdate.FromString,

            options, channel_credentials,

            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

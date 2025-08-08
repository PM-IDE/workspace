import type * as grpc from '@grpc/grpc-js';
import type { EnumTypeDefinition, MessageTypeDefinition } from '@grpc/proto-loader';

import type { GrpcBackendServiceClient as _ficus_GrpcBackendServiceClient, GrpcBackendServiceDefinition as _ficus_GrpcBackendServiceDefinition } from './ficus/GrpcBackendService';
import type { GrpcKafkaServiceClient as _ficus_GrpcKafkaServiceClient, GrpcKafkaServiceDefinition as _ficus_GrpcKafkaServiceDefinition } from './ficus/GrpcKafkaService';

type SubtypeConstructor<Constructor extends new (...args: any) => any, Subtype> = {
  new(...args: ConstructorParameters<Constructor>): Subtype;
};

export interface ProtoGrpcType {
  ficus: {
    GrpcActivityDurationData: MessageTypeDefinition
    GrpcActivityStartEndData: MessageTypeDefinition
    GrpcAddPipelineRequest: MessageTypeDefinition
    GrpcAddPipelineStreamRequest: MessageTypeDefinition
    GrpcAllocationInfo: MessageTypeDefinition
    GrpcAnnotation: MessageTypeDefinition
    GrpcBackendService: SubtypeConstructor<typeof grpc.Client, _ficus_GrpcBackendServiceClient> & { service: _ficus_GrpcBackendServiceDefinition }
    GrpcBytes: MessageTypeDefinition
    GrpcColor: MessageTypeDefinition
    GrpcColoredRectangle: MessageTypeDefinition
    GrpcColorsEventLog: MessageTypeDefinition
    GrpcColorsEventLogMapping: MessageTypeDefinition
    GrpcColorsLogAdjustment: MessageTypeDefinition
    GrpcColorsLogRectangleAdjustment: MessageTypeDefinition
    GrpcColorsLogXAxisAfterTraceAdjustment: MessageTypeDefinition
    GrpcColorsTrace: MessageTypeDefinition
    GrpcComplexContextRequestPipelinePart: MessageTypeDefinition
    GrpcContextKey: MessageTypeDefinition
    GrpcContextKeyValue: MessageTypeDefinition
    GrpcContextValue: MessageTypeDefinition
    GrpcContextValueWithKeyName: MessageTypeDefinition
    GrpcCountAnnotation: MessageTypeDefinition
    GrpcDataset: MessageTypeDefinition
    GrpcDateTime: MessageTypeDefinition
    GrpcDurationKind: EnumTypeDefinition
    GrpcEdgeExecutionInfo: MessageTypeDefinition
    GrpcEntityCountAnnotation: MessageTypeDefinition
    GrpcEntityFrequencyAnnotation: MessageTypeDefinition
    GrpcEntityTimeAnnotation: MessageTypeDefinition
    GrpcEnum: MessageTypeDefinition
    GrpcEvent: MessageTypeDefinition
    GrpcEventCoordinates: MessageTypeDefinition
    GrpcEventLogInfo: MessageTypeDefinition
    GrpcEventLogTraceSubArraysContextValue: MessageTypeDefinition
    GrpcEventStamp: MessageTypeDefinition
    GrpcExecutePipelineAndProduceKafkaRequest: MessageTypeDefinition
    GrpcFloatArray: MessageTypeDefinition
    GrpcFrequenciesAnnotation: MessageTypeDefinition
    GrpcGeneralHistogramData: MessageTypeDefinition
    GrpcGenericEnhancementBase: MessageTypeDefinition
    GrpcGetAllSubscriptionsAndPipelinesResponse: MessageTypeDefinition
    GrpcGetContextValueRequest: MessageTypeDefinition
    GrpcGetContextValueResult: MessageTypeDefinition
    GrpcGraph: MessageTypeDefinition
    GrpcGraphEdge: MessageTypeDefinition
    GrpcGraphEdgeAdditionalData: MessageTypeDefinition
    GrpcGraphKind: EnumTypeDefinition
    GrpcGraphNode: MessageTypeDefinition
    GrpcGuid: MessageTypeDefinition
    GrpcHashesEventLog: MessageTypeDefinition
    GrpcHashesEventLogContextValue: MessageTypeDefinition
    GrpcHashesLogTrace: MessageTypeDefinition
    GrpcHistogramEntry: MessageTypeDefinition
    GrpcIntArray: MessageTypeDefinition
    GrpcKafkaConnectionMetadata: MessageTypeDefinition
    GrpcKafkaFailedResult: MessageTypeDefinition
    GrpcKafkaMetadata: MessageTypeDefinition
    GrpcKafkaPipelineExecutionRequest: MessageTypeDefinition
    GrpcKafkaResult: MessageTypeDefinition
    GrpcKafkaService: SubtypeConstructor<typeof grpc.Client, _ficus_GrpcKafkaServiceClient> & { service: _ficus_GrpcKafkaServiceDefinition }
    GrpcKafkaSubscription: MessageTypeDefinition
    GrpcKafkaSubscriptionMetadata: MessageTypeDefinition
    GrpcKafkaSuccessResult: MessageTypeDefinition
    GrpcLabeledDataset: MessageTypeDefinition
    GrpcLogPoint: MessageTypeDefinition
    GrpcLogTimelineDiagram: MessageTypeDefinition
    GrpcMatrix: MessageTypeDefinition
    GrpcMatrixRow: MessageTypeDefinition
    GrpcMethodInliningInfo: MessageTypeDefinition
    GrpcMethodNameParts: MessageTypeDefinition
    GrpcMultithreadedFragment: MessageTypeDefinition
    GrpcNamesEventLog: MessageTypeDefinition
    GrpcNamesEventLogContextValue: MessageTypeDefinition
    GrpcNamesTrace: MessageTypeDefinition
    GrpcNodeAdditionalData: MessageTypeDefinition
    GrpcNodeCorrespondingTraceData: MessageTypeDefinition
    GrpcParallelPipelinePart: MessageTypeDefinition
    GrpcParallelPipelineParts: MessageTypeDefinition
    GrpcPetriNet: MessageTypeDefinition
    GrpcPetriNetArc: MessageTypeDefinition
    GrpcPetriNetMarking: MessageTypeDefinition
    GrpcPetriNetPlace: MessageTypeDefinition
    GrpcPetriNetSinglePlaceMarking: MessageTypeDefinition
    GrpcPetriNetTransition: MessageTypeDefinition
    GrpcPipeline: MessageTypeDefinition
    GrpcPipelineExecutionRequest: MessageTypeDefinition
    GrpcPipelineFinalResult: MessageTypeDefinition
    GrpcPipelineMetadata: MessageTypeDefinition
    GrpcPipelinePart: MessageTypeDefinition
    GrpcPipelinePartBase: MessageTypeDefinition
    GrpcPipelinePartConfiguration: MessageTypeDefinition
    GrpcPipelinePartExecutionResult: MessageTypeDefinition
    GrpcPipelinePartLogMessage: MessageTypeDefinition
    GrpcPipelinePartResult: MessageTypeDefinition
    GrpcPipelineStreamingConfiguration: MessageTypeDefinition
    GrpcProcessInfo: MessageTypeDefinition
    GrpcProxyPipelineExecutionRequest: MessageTypeDefinition
    GrpcRemoveAllPipelinesRequest: MessageTypeDefinition
    GrpcRemovePipelineRequest: MessageTypeDefinition
    GrpcSimpleContextRequestPipelinePart: MessageTypeDefinition
    GrpcSimpleCounterData: MessageTypeDefinition
    GrpcSimpleEventLog: MessageTypeDefinition
    GrpcSimpleTrace: MessageTypeDefinition
    GrpcSoftwareData: MessageTypeDefinition
    GrpcStringKeyValue: MessageTypeDefinition
    GrpcStrings: MessageTypeDefinition
    GrpcSubArrayWithTraceIndex: MessageTypeDefinition
    GrpcSubArraysWithTraceIndexContextValue: MessageTypeDefinition
    GrpcSubscribeToKafkaRequest: MessageTypeDefinition
    GrpcSubscriptionPipeline: MessageTypeDefinition
    GrpcT1EventsTimeBasedCaching: MessageTypeDefinition
    GrpcT1StreamingConfiguration: MessageTypeDefinition
    GrpcT1TraceTimeBasedCaching: MessageTypeDefinition
    GrpcT1TracesQueueConfiguration: MessageTypeDefinition
    GrpcT2LossyCountConfiguration: MessageTypeDefinition
    GrpcT2StreamingConfiguration: MessageTypeDefinition
    GrpcT2TimedSlidingWindowConfiguration: MessageTypeDefinition
    GrpcThread: MessageTypeDefinition
    GrpcThreadEvent: MessageTypeDefinition
    GrpcTimePerformanceAnnotation: MessageTypeDefinition
    GrpcTimeSpan: MessageTypeDefinition
    GrpcTimelineDiagramFragment: MessageTypeDefinition
    GrpcTimelineTraceEventsGroup: MessageTypeDefinition
    GrpcTraceSubArray: MessageTypeDefinition
    GrpcTraceSubArrays: MessageTypeDefinition
    GrpcTraceTimelineDiagram: MessageTypeDefinition
    GrpcUintArray: MessageTypeDefinition
    GrpcUnderlyingPatternInfo: MessageTypeDefinition
    GrpcUnderlyingPatternKind: EnumTypeDefinition
    GrpcUnsubscribeFromKafkaRequest: MessageTypeDefinition
    GrpcUuid: MessageTypeDefinition
  }
  google: {
    protobuf: {
      Empty: MessageTypeDefinition
      Timestamp: MessageTypeDefinition
    }
  }
}


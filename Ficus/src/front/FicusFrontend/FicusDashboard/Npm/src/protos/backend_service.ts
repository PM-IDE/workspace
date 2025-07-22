import type * as grpc from '@grpc/grpc-js';
import type { EnumTypeDefinition, MessageTypeDefinition } from '@grpc/proto-loader';

import type { GrpcBackendServiceClient as _ficus_GrpcBackendServiceClient, GrpcBackendServiceDefinition as _ficus_GrpcBackendServiceDefinition } from './ficus/GrpcBackendService';

type SubtypeConstructor<Constructor extends new (...args: any) => any, Subtype> = {
  new(...args: ConstructorParameters<Constructor>): Subtype;
};

export interface ProtoGrpcType {
  ficus: {
    GrpcActivityStartEndData: MessageTypeDefinition
    GrpcAllocationInfo: MessageTypeDefinition
    GrpcAnnotation: MessageTypeDefinition
    GrpcArrayPoolEvent: MessageTypeDefinition
    GrpcAssemblyEventInfo: MessageTypeDefinition
    GrpcAssemblyEventKind: EnumTypeDefinition
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
    GrpcContentionEvent: MessageTypeDefinition
    GrpcContextKey: MessageTypeDefinition
    GrpcContextKeyValue: MessageTypeDefinition
    GrpcContextValue: MessageTypeDefinition
    GrpcContextValueWithKeyName: MessageTypeDefinition
    GrpcCountAnnotation: MessageTypeDefinition
    GrpcDataset: MessageTypeDefinition
    GrpcDateTime: MessageTypeDefinition
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
    GrpcExceptionEvent: MessageTypeDefinition
    GrpcExecutionSuspensionInfo: MessageTypeDefinition
    GrpcFloatArray: MessageTypeDefinition
    GrpcFrequenciesAnnotation: MessageTypeDefinition
    GrpcGeneralHistogramData: MessageTypeDefinition
    GrpcGeneralHistogramEntry: MessageTypeDefinition
    GrpcGetContextValueRequest: MessageTypeDefinition
    GrpcGetContextValueResult: MessageTypeDefinition
    GrpcGraph: MessageTypeDefinition
    GrpcGraphEdge: MessageTypeDefinition
    GrpcGraphEdgeAdditionalData: MessageTypeDefinition
    GrpcGraphKind: EnumTypeDefinition
    GrpcGraphNode: MessageTypeDefinition
    GrpcGuid: MessageTypeDefinition
    GrpcHTTPEvent: MessageTypeDefinition
    GrpcHashesEventLog: MessageTypeDefinition
    GrpcHashesEventLogContextValue: MessageTypeDefinition
    GrpcHashesLogTrace: MessageTypeDefinition
    GrpcHistogramEntry: MessageTypeDefinition
    GrpcIntArray: MessageTypeDefinition
    GrpcLabeledDataset: MessageTypeDefinition
    GrpcLogPoint: MessageTypeDefinition
    GrpcLogTimelineDiagram: MessageTypeDefinition
    GrpcMatrix: MessageTypeDefinition
    GrpcMatrixRow: MessageTypeDefinition
    GrpcMethodInliningEvent: MessageTypeDefinition
    GrpcMethodInliningFailedEvent: MessageTypeDefinition
    GrpcMethodInliningInfo: MessageTypeDefinition
    GrpcMethodLoadUnloadEvent: MessageTypeDefinition
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
    GrpcPipelinePart: MessageTypeDefinition
    GrpcPipelinePartBase: MessageTypeDefinition
    GrpcPipelinePartConfiguration: MessageTypeDefinition
    GrpcPipelinePartExecutionResult: MessageTypeDefinition
    GrpcPipelinePartLogMessage: MessageTypeDefinition
    GrpcPipelinePartResult: MessageTypeDefinition
    GrpcProxyPipelineExecutionRequest: MessageTypeDefinition
    GrpcSimpleContextRequestPipelinePart: MessageTypeDefinition
    GrpcSimpleCounterData: MessageTypeDefinition
    GrpcSimpleEventLog: MessageTypeDefinition
    GrpcSimpleTrace: MessageTypeDefinition
    GrpcSocketAcceptFailed: MessageTypeDefinition
    GrpcSocketAcceptStart: MessageTypeDefinition
    GrpcSocketAcceptStop: MessageTypeDefinition
    GrpcSocketConnectFailed: MessageTypeDefinition
    GrpcSocketConnectStart: MessageTypeDefinition
    GrpcSocketConnectStop: MessageTypeDefinition
    GrpcSocketEvent: MessageTypeDefinition
    GrpcSoftwareData: MessageTypeDefinition
    GrpcStringKeyValue: MessageTypeDefinition
    GrpcStrings: MessageTypeDefinition
    GrpcSubArrayWithTraceIndex: MessageTypeDefinition
    GrpcSubArraysWithTraceIndexContextValue: MessageTypeDefinition
    GrpcThread: MessageTypeDefinition
    GrpcThreadEvent: MessageTypeDefinition
    GrpcThreadEventInfo: MessageTypeDefinition
    GrpcThreadEventKind: EnumTypeDefinition
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
    GrpcUuid: MessageTypeDefinition
  }
  google: {
    protobuf: {
      Empty: MessageTypeDefinition
      Timestamp: MessageTypeDefinition
    }
  }
}


import type * as grpc from '@grpc/grpc-js';
import type { EnumTypeDefinition, MessageTypeDefinition } from '@grpc/proto-loader';

import type { GrpcContextValuesServiceClient as _ficus_GrpcContextValuesServiceClient, GrpcContextValuesServiceDefinition as _ficus_GrpcContextValuesServiceDefinition } from './ficus/GrpcContextValuesService';

type SubtypeConstructor<Constructor extends new (...args: any) => any, Subtype> = {
  new(...args: ConstructorParameters<Constructor>): Subtype;
};

export interface ProtoGrpcType {
  ficus: {
    GrpcActivityDurationData: MessageTypeDefinition
    GrpcActivityStartEndData: MessageTypeDefinition
    GrpcAllocationInfo: MessageTypeDefinition
    GrpcAnnotation: MessageTypeDefinition
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
    GrpcContextValuePart: MessageTypeDefinition
    GrpcContextValueWithKeyName: MessageTypeDefinition
    GrpcContextValuesService: SubtypeConstructor<typeof grpc.Client, _ficus_GrpcContextValuesServiceClient> & { service: _ficus_GrpcContextValuesServiceDefinition }
    GrpcCountAnnotation: MessageTypeDefinition
    GrpcDataset: MessageTypeDefinition
    GrpcDateTime: MessageTypeDefinition
    GrpcDropContextValuesRequest: MessageTypeDefinition
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
    GrpcFloatArray: MessageTypeDefinition
    GrpcFrequenciesAnnotation: MessageTypeDefinition
    GrpcGeneralHistogramData: MessageTypeDefinition
    GrpcGenericEnhancementBase: MessageTypeDefinition
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
    GrpcPipelinePart: MessageTypeDefinition
    GrpcPipelinePartBase: MessageTypeDefinition
    GrpcPipelinePartConfiguration: MessageTypeDefinition
    GrpcSimpleContextRequestPipelinePart: MessageTypeDefinition
    GrpcSimpleCounterData: MessageTypeDefinition
    GrpcSimpleEventLog: MessageTypeDefinition
    GrpcSimpleTrace: MessageTypeDefinition
    GrpcSoftwareData: MessageTypeDefinition
    GrpcStringKeyValue: MessageTypeDefinition
    GrpcStrings: MessageTypeDefinition
    GrpcSubArrayWithTraceIndex: MessageTypeDefinition
    GrpcSubArraysWithTraceIndexContextValue: MessageTypeDefinition
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
    GrpcUuid: MessageTypeDefinition
  }
  google: {
    protobuf: {
      Empty: MessageTypeDefinition
      Timestamp: MessageTypeDefinition
    }
  }
}


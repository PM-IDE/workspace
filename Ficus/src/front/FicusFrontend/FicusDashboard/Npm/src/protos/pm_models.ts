import type * as grpc from '@grpc/grpc-js';
import type { MessageTypeDefinition } from '@grpc/proto-loader';


type SubtypeConstructor<Constructor extends new (...args: any) => any, Subtype> = {
  new(...args: ConstructorParameters<Constructor>): Subtype;
};

export interface ProtoGrpcType {
  ficus: {
    GrpcAnnotation: MessageTypeDefinition
    GrpcColor: MessageTypeDefinition
    GrpcCountAnnotation: MessageTypeDefinition
    GrpcDataset: MessageTypeDefinition
    GrpcDateTime: MessageTypeDefinition
    GrpcEntityCountAnnotation: MessageTypeDefinition
    GrpcEntityFrequencyAnnotation: MessageTypeDefinition
    GrpcEntityTimeAnnotation: MessageTypeDefinition
    GrpcEvent: MessageTypeDefinition
    GrpcEventStamp: MessageTypeDefinition
    GrpcFrequenciesAnnotation: MessageTypeDefinition
    GrpcGuid: MessageTypeDefinition
    GrpcHashesEventLog: MessageTypeDefinition
    GrpcHashesLogTrace: MessageTypeDefinition
    GrpcLabeledDataset: MessageTypeDefinition
    GrpcMatrix: MessageTypeDefinition
    GrpcMatrixRow: MessageTypeDefinition
    GrpcNamesEventLog: MessageTypeDefinition
    GrpcNamesTrace: MessageTypeDefinition
    GrpcPetriNet: MessageTypeDefinition
    GrpcPetriNetArc: MessageTypeDefinition
    GrpcPetriNetMarking: MessageTypeDefinition
    GrpcPetriNetPlace: MessageTypeDefinition
    GrpcPetriNetSinglePlaceMarking: MessageTypeDefinition
    GrpcPetriNetTransition: MessageTypeDefinition
    GrpcSimpleEventLog: MessageTypeDefinition
    GrpcSimpleTrace: MessageTypeDefinition
    GrpcStringKeyValue: MessageTypeDefinition
    GrpcTimePerformanceAnnotation: MessageTypeDefinition
    GrpcTimeSpan: MessageTypeDefinition
    GrpcUuid: MessageTypeDefinition
  }
  google: {
    protobuf: {
      Timestamp: MessageTypeDefinition
    }
  }
}


// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcHashesEventLogContextValue_DONTUSE as _ficus_GrpcHashesEventLogContextValue_DONTUSE, GrpcHashesEventLogContextValue as _ficus_GrpcHashesEventLogContextValue } from '../ficus/GrpcHashesEventLogContextValue';
import type { GrpcNamesEventLogContextValue_DONTUSE as _ficus_GrpcNamesEventLogContextValue_DONTUSE, GrpcNamesEventLogContextValue as _ficus_GrpcNamesEventLogContextValue } from '../ficus/GrpcNamesEventLogContextValue';
import type { GrpcEventLogTraceSubArraysContextValue_DONTUSE as _ficus_GrpcEventLogTraceSubArraysContextValue_DONTUSE, GrpcEventLogTraceSubArraysContextValue as _ficus_GrpcEventLogTraceSubArraysContextValue } from '../ficus/GrpcEventLogTraceSubArraysContextValue';
import type { GrpcSubArraysWithTraceIndexContextValue_DONTUSE as _ficus_GrpcSubArraysWithTraceIndexContextValue_DONTUSE, GrpcSubArraysWithTraceIndexContextValue as _ficus_GrpcSubArraysWithTraceIndexContextValue } from '../ficus/GrpcSubArraysWithTraceIndexContextValue';
import type { GrpcColorsEventLog_DONTUSE as _ficus_GrpcColorsEventLog_DONTUSE, GrpcColorsEventLog as _ficus_GrpcColorsEventLog } from '../ficus/GrpcColorsEventLog';
import type { GrpcEnum_DONTUSE as _ficus_GrpcEnum_DONTUSE, GrpcEnum as _ficus_GrpcEnum } from '../ficus/GrpcEnum';
import type { GrpcEventLogInfo_DONTUSE as _ficus_GrpcEventLogInfo_DONTUSE, GrpcEventLogInfo as _ficus_GrpcEventLogInfo } from '../ficus/GrpcEventLogInfo';
import type { GrpcStrings_DONTUSE as _ficus_GrpcStrings_DONTUSE, GrpcStrings as _ficus_GrpcStrings } from '../ficus/GrpcStrings';
import type { GrpcPipeline_DONTUSE as _ficus_GrpcPipeline_DONTUSE, GrpcPipeline as _ficus_GrpcPipeline } from '../ficus/GrpcPipeline';
import type { GrpcPetriNet_DONTUSE as _ficus_GrpcPetriNet_DONTUSE, GrpcPetriNet as _ficus_GrpcPetriNet } from '../ficus/GrpcPetriNet';
import type { GrpcGraph_DONTUSE as _ficus_GrpcGraph_DONTUSE, GrpcGraph as _ficus_GrpcGraph } from '../ficus/GrpcGraph';
import type { GrpcAnnotation_DONTUSE as _ficus_GrpcAnnotation_DONTUSE, GrpcAnnotation as _ficus_GrpcAnnotation } from '../ficus/GrpcAnnotation';
import type { GrpcDataset_DONTUSE as _ficus_GrpcDataset_DONTUSE, GrpcDataset as _ficus_GrpcDataset } from '../ficus/GrpcDataset';
import type { GrpcLabeledDataset_DONTUSE as _ficus_GrpcLabeledDataset_DONTUSE, GrpcLabeledDataset as _ficus_GrpcLabeledDataset } from '../ficus/GrpcLabeledDataset';
import type { GrpcBytes_DONTUSE as _ficus_GrpcBytes_DONTUSE, GrpcBytes as _ficus_GrpcBytes } from '../ficus/GrpcBytes';
import type { GrpcLogTimelineDiagram_DONTUSE as _ficus_GrpcLogTimelineDiagram_DONTUSE, GrpcLogTimelineDiagram as _ficus_GrpcLogTimelineDiagram } from '../ficus/GrpcLogTimelineDiagram';
import type { GrpcFloatArray_DONTUSE as _ficus_GrpcFloatArray_DONTUSE, GrpcFloatArray as _ficus_GrpcFloatArray } from '../ficus/GrpcFloatArray';
import type { GrpcIntArray_DONTUSE as _ficus_GrpcIntArray_DONTUSE, GrpcIntArray as _ficus_GrpcIntArray } from '../ficus/GrpcIntArray';
import type { GrpcUintArray_DONTUSE as _ficus_GrpcUintArray_DONTUSE, GrpcUintArray as _ficus_GrpcUintArray } from '../ficus/GrpcUintArray';

export interface GrpcContextValue_DONTUSE {
  'string'?: (string);
  'hashesLog'?: (_ficus_GrpcHashesEventLogContextValue_DONTUSE | null);
  'namesLog'?: (_ficus_GrpcNamesEventLogContextValue_DONTUSE | null);
  'uint32'?: (number);
  'tracesSubArrays'?: (_ficus_GrpcEventLogTraceSubArraysContextValue_DONTUSE | null);
  'traceIndexSubArrays'?: (_ficus_GrpcSubArraysWithTraceIndexContextValue_DONTUSE | null);
  'bool'?: (boolean);
  'xesEventLog'?: (_ficus_GrpcNamesEventLogContextValue_DONTUSE | null);
  'colorsLog'?: (_ficus_GrpcColorsEventLog_DONTUSE | null);
  'enum'?: (_ficus_GrpcEnum_DONTUSE | null);
  'eventLogInfo'?: (_ficus_GrpcEventLogInfo_DONTUSE | null);
  'strings'?: (_ficus_GrpcStrings_DONTUSE | null);
  'pipeline'?: (_ficus_GrpcPipeline_DONTUSE | null);
  'petriNet'?: (_ficus_GrpcPetriNet_DONTUSE | null);
  'graph'?: (_ficus_GrpcGraph_DONTUSE | null);
  'float'?: (number | string);
  'annotation'?: (_ficus_GrpcAnnotation_DONTUSE | null);
  'dataset'?: (_ficus_GrpcDataset_DONTUSE | null);
  'labeledDataset'?: (_ficus_GrpcLabeledDataset_DONTUSE | null);
  'bytes'?: (_ficus_GrpcBytes_DONTUSE | null);
  'logTimelineDiagram'?: (_ficus_GrpcLogTimelineDiagram_DONTUSE | null);
  'floatArray'?: (_ficus_GrpcFloatArray_DONTUSE | null);
  'intArray'?: (_ficus_GrpcIntArray_DONTUSE | null);
  'uintArray'?: (_ficus_GrpcUintArray_DONTUSE | null);
  'json'?: (string);
  'contextValue'?: "string"|"hashesLog"|"namesLog"|"uint32"|"tracesSubArrays"|"traceIndexSubArrays"|"bool"|"xesEventLog"|"colorsLog"|"enum"|"eventLogInfo"|"strings"|"pipeline"|"petriNet"|"graph"|"float"|"annotation"|"dataset"|"labeledDataset"|"bytes"|"logTimelineDiagram"|"floatArray"|"intArray"|"uintArray"|"json";
}

export interface GrpcContextValue {
  'string'?: (string);
  'hashesLog'?: (_ficus_GrpcHashesEventLogContextValue | null);
  'namesLog'?: (_ficus_GrpcNamesEventLogContextValue | null);
  'uint32'?: (number);
  'tracesSubArrays'?: (_ficus_GrpcEventLogTraceSubArraysContextValue | null);
  'traceIndexSubArrays'?: (_ficus_GrpcSubArraysWithTraceIndexContextValue | null);
  'bool'?: (boolean);
  'xesEventLog'?: (_ficus_GrpcNamesEventLogContextValue | null);
  'colorsLog'?: (_ficus_GrpcColorsEventLog | null);
  'enum'?: (_ficus_GrpcEnum | null);
  'eventLogInfo'?: (_ficus_GrpcEventLogInfo | null);
  'strings'?: (_ficus_GrpcStrings | null);
  'pipeline'?: (_ficus_GrpcPipeline | null);
  'petriNet'?: (_ficus_GrpcPetriNet | null);
  'graph'?: (_ficus_GrpcGraph | null);
  'float'?: (number);
  'annotation'?: (_ficus_GrpcAnnotation | null);
  'dataset'?: (_ficus_GrpcDataset | null);
  'labeledDataset'?: (_ficus_GrpcLabeledDataset | null);
  'bytes'?: (_ficus_GrpcBytes | null);
  'logTimelineDiagram'?: (_ficus_GrpcLogTimelineDiagram | null);
  'floatArray'?: (_ficus_GrpcFloatArray | null);
  'intArray'?: (_ficus_GrpcIntArray | null);
  'uintArray'?: (_ficus_GrpcUintArray | null);
  'json'?: (string);
  'contextValue': "string"|"hashesLog"|"namesLog"|"uint32"|"tracesSubArrays"|"traceIndexSubArrays"|"bool"|"xesEventLog"|"colorsLog"|"enum"|"eventLogInfo"|"strings"|"pipeline"|"petriNet"|"graph"|"float"|"annotation"|"dataset"|"labeledDataset"|"bytes"|"logTimelineDiagram"|"floatArray"|"intArray"|"uintArray"|"json";
}

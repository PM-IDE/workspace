from enum import Enum


class PatternsDiscoveryStrategy(Enum):
  FromAllTraces = 0
  FromSingleMergedTrace = 1


class PatternsKind(Enum):
  PrimitiveTandemArrays = 0,
  MaximalTandemArrays = 1,
  MaximalRepeats = 2,
  SuperMaximalRepeats = 3,
  NearSuperMaximalRepeats = 4,


class NarrowActivityKind(Enum):
  DontNarrow = 0,
  StayTheSame = 1,
  NarrowUp = 2,
  NarrowDown = 3,


class ActivityFilterKind(Enum):
  NoFilter = 0,
  DefaultFilter = 1,


class ActivitiesLogsSource(Enum):
  Log = 0,
  TracesActivities = 1,


class ActivitiesRepresentationSource(Enum):
  EventClasses = 0,
  SubTraces = 1
  SubTracesUnderlyingEvents = 2


class Distance(Enum):
  Cosine = 0
  L1 = 1
  L2 = 2
  Levenshtein = 3
  Length = 4
  LCS = 5


class TracesRepresentationSource(Enum):
  Events = 0
  UnderlyingEvents = 1
  DeepestUnderlyingEvents = 2


class LogSerializationFormat(Enum):
  Xes = 0
  Bxes = 1


class FeatureCountKind(Enum):
  One = 0
  Count = 1
  OneIfMoreThanMaxFromAllFeatures = 2


class RootSequenceKind(Enum):
  FindBest = 0
  LCS = 1
  PairwiseLCS = 2
  Trace = 3

from enum import Enum


class TandemArrayKind(Enum):
  MaximalArray = 0
  PrimitiveArray = 1


class AdjustingMode(Enum):
  FromUnattachedSubTraces = 0
  FromAllLog = 1

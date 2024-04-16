from typing import Callable

from ..log.event_log import MyEvent

ClassExtractor = Callable[[MyEvent], str]

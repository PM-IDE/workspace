import copy
import random
import time
from typing import Any, Hashable, Callable

mod = 10 ** 9 + 7
p = 31
concept_name = 'concept:name'
time_timestamp = 'time:timestamp'
lifecycle_transition = 'lifecycle:transition'
fake_start_name = 'FAKE_START'
fake_end_name = 'FAKE_END'
event_level = "event"


def calculate_string_poly_hash(s: str) -> int:
    length = len(s)
    hash_so_far = 0
    p_pow = 1
    for i in range(length):
        hash_so_far = (hash_so_far + (1 + ord(s[i])) * p_pow) % mod
        p_pow = (p_pow * p) % mod

    return hash_so_far


def calculate_poly_hash_for_collection(collection: list[Hashable],
                                       start_index: int = 0,
                                       end_index: int = None) -> int:
    if len(collection) == 0:
        return 0

    end_index = len(collection) if end_index is None else end_index
    hash_so_far = 1
    p_pow = 1
    for i in range(start_index, end_index):
        hash_so_far = (hash_so_far + (1 + hash(collection[i])) * p_pow) % mod
        p_pow = (p_pow * p) % mod

    return hash_so_far


def calculate_dict_hash(current_map: dict[Any, Any]) -> int:
    sorted_joined_pairs = list(sorted(map(lambda x: str(x[0]) + str(x[1]), current_map.items())))
    return calculate_poly_hash_for_collection(list(map(calculate_string_poly_hash, sorted_joined_pairs)))


def combine_hash_values(*values) -> int:
    if len(values) == 0:
        return 0

    hash_so_far = values[0]
    for value in values[1:]:
        hash_so_far = combine_two_hashes(hash_so_far, value)

    return hash_so_far


def combine_two_hashes(first: int, second: int) -> int:
    return hash((first, second))


def combine_list_of_hashes(hashes: list[int]) -> int:
    if len(hashes) == 0:
        return 0

    hash_so_far = hashes[0]
    for current_hash in hashes[1:]:
        hash_so_far = combine_two_hashes(hash_so_far, current_hash)

    return hash_so_far


class ColorsProvider:
    def next(self) -> (int, int, int):
        raise NotImplementedError()

    def reset(self):
        raise NotImplementedError()


class RandomUniqueColorsProvider(ColorsProvider):
    def __init__(self):
        self.used_colors = set()

    def next(self) -> (int, int, int):
        if len(self.used_colors) == 256 ** 3:
            raise OverflowError()

        color = _generate_random_color()
        while color in self.used_colors:
            color = _generate_random_color()

        self.used_colors.add(color)
        return color

    def reset(self):
        self.used_colors.clear()


random_unique_color_provider_instance = RandomUniqueColorsProvider()


class SingleColorProvider(ColorsProvider):
    def __init__(self):
        self.provider = random_unique_color_provider_instance
        self.cached_color = self.provider.next()

    def next(self) -> (int, int, int):
        return self.cached_color

    def reset(self):
        self.cached_color = self.provider.next()


def _generate_random_color() -> (int, int, int):
    def generate_random_color():
        return random.randint(0, 255)

    return (generate_random_color(), generate_random_color(), generate_random_color())


def to_hex(color: (int, int, int)) -> str:
    return '#%02X%02X%02X' % color


def increase_in_int_map(input_map: dict[Any, int], key: Any):
    value = input_map.get(key, 0)
    input_map[key] = value + 1


def deep_copy_dict(map: dict[Any, Any]) -> dict[Any, Any]:
    new_dict = dict()
    for key, value in map.items():
        new_dict[copy.deepcopy(key)] = copy.deepcopy(value)

    return new_dict


def performance_cookie(activity_name: str, action: Callable):
    print(f'Started activity {activity_name}')

    start = time.time()

    action()

    end = time.time()

    elapsed = end - start
    print(f'Activity {activity_name} has finished in {elapsed}s')

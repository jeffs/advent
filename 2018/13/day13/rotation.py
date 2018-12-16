from typing import Dict
import enum

from .direction import Direction


@enum.unique
class Rotation(enum.Enum):
    LEFT = enum.auto()
    STRAIGHT = enum.auto()
    RIGHT = enum.auto()


DirectionDict = Dict[Rotation, Dict[Direction, Direction]]


def next_rotation(rotation: Rotation) -> Rotation:
    return Rotation.LEFT if rotation == Rotation.RIGHT \
      else Rotation.STRAIGHT if rotation == Rotation.LEFT \
      else Rotation.RIGHT


def _make_rotated_directions() -> DirectionDict:
    dlru = tuple(Direction)
    d, l, r, u = dlru
    values = {            # d  l  r  u
        Rotation.LEFT:     (r, d, u, l),
        Rotation.STRAIGHT: (d, l, r, u),
        Rotation.RIGHT:    (l, u, d, r),
    }
    return {
        rot: {old: new for old, new in zip(dlru, news)}
        for rot, news in values.items()
    }


ROTATED_DIRECTIONS: DirectionDict = _make_rotated_directions()


def rotate_direction(direction: Direction, rotation: Rotation) -> Direction:
    if rotation not in ROTATED_DIRECTIONS:
        raise Exception('bad rotation')
    if direction not in ROTATED_DIRECTIONS[rotation]:
        raise Exception('bad direction')
    return ROTATED_DIRECTIONS[rotation][direction]

from typing import Optional
import enum


@enum.unique
class Direction(enum.Enum):
    DOWN = 'v'
    LEFT = '<'
    RIGHT = '>'
    UP = '^'


def parse_direction(character: str) -> Optional[Direction]:
    for member in Direction:
        if member.value == character:
            return member
    return None

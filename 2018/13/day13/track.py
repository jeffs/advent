import enum
from dataclasses import dataclass, field
from typing import List, Optional, Tuple


@enum.unique
class Segment(enum.Enum):
    CURVE_LEFT = '\\'
    CURVE_RIGHT = '/'
    INTERSECTION = '+'
    SPACE = ' '
    HORIZONTAL = '-'
    VERTICAL = '|'


Track = Tuple[Tuple[Segment, ...], ...]


def parse_track_entry(character: str) -> Optional[Segment]:
    for member in Segment:
        if member.value == character:
            return member
    if character in '<>':
        return Segment.HORIZONTAL
    if character in '^v':
        return Segment.VERTICAL
    return None


@dataclass
class TrackBuilder:

    rows: List[List[Segment]] = field(default_factory=list)
    row: List[Segment] = field(default_factory=list)

    def append(self, character: str) -> None:
        segment = parse_track_entry(character)
        if segment:
            self.row.append(segment)
        elif character == '\n':
            self.rows.append(self.row)
            self.row = list()
        else:
            raise Exception('bad character')

    def build(self) -> Track:
        if self.row:
            raise Exception('incomplete row')
        return tuple(tuple(row) for row in self.rows)

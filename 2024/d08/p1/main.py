#!/usr/bin/env python3

from __future__ import annotations

import sys
from collections import defaultdict
from itertools import chain, combinations
from typing import Iterable, List, Mapping, NamedTuple, Set

Frequency = str


class Location(NamedTuple):
    row: int
    col: int

    def translate(self, rows: int, cols: int) -> Location:
        return Location(self.row + rows, self.col + cols)


class Map:
    def __init__(self, lines: List[str]) -> None:
        self.rows: int = len(lines)
        self.cols: int = len(lines[0])
        self.antennae: Mapping[Frequency, Set[Location]] = defaultdict(set)

        for row, line in enumerate(lines):
            for col, char in enumerate(line):
                if char != ".":
                    self.antennae[char].add(Location(row, col))

    def antinodes(self, freq: Frequency) -> Iterable[Location]:
        for a, b in combinations(self.antennae[freq], 2):
            row_delta = a.row - b.row
            col_delta = a.col - b.col
            for loc in (
                a.translate(row_delta, col_delta),
                a.translate(-row_delta, -col_delta),
                b.translate(row_delta, col_delta),
                b.translate(-row_delta, -col_delta),
            ):
                if loc not in {a, b} and self.contains(loc):
                    yield loc

    def contains(self, loc: Location) -> bool:
        return loc[0] >= 0 and loc[0] < self.rows and loc[1] >= 0 and loc[1] < self.cols

    def __str__(self) -> str:
        return f"Map(rows={self.rows}, cols={self.cols}, antennae={self.antennae})"


def main():
    with open(sys.argv[1], "r") as input:
        map = Map([line.strip() for line in input.readlines()])

    antinodes = set(chain.from_iterable((map.antinodes(freq) for freq in map.antennae)))

    print(f"Unique locations: {len(antinodes)}")


if __name__ == "__main__":
    main()

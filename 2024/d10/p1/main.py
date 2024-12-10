#!/usr/bin/env python3

from __future__ import annotations

import sys
from typing import Iterable, List, NamedTuple

MIN_HEIGHT = 0
MAX_HEIGHT = 9


class Position(NamedTuple):
    row: int
    col: int

    def neighbors(self) -> Iterable[Position]:
        yield from map(
            lambda p: Position(*p),
            (
                (self.row + 1, self.col),
                (self.row - 1, self.col),
                (self.row, self.col + 1),
                (self.row, self.col - 1),
            ),
        )


class TopographicMap:
    def __init__(self, lines: List[str]) -> None:
        self.rows: int = len(lines)
        self.cols: int = len(lines[0])
        self.heights: List[int] = [int(height) for line in lines for height in line]

    def get(self, pos: Position) -> int:
        return self.heights[pos.row * self.cols + pos.col]

    def contains(self, pos: Position) -> bool:
        return (
            pos.row >= 0
            and pos.row < self.rows
            and pos.col >= 0
            and pos.col < self.cols
        )

    def adjacent_uphill(self, pos: Position) -> Iterable[Position]:
        next_height = self.get(pos) + 1
        for n in pos.neighbors():
            if self.contains(n) and self.get(n) == next_height:
                yield n

    def trailheads(self) -> Iterable[Position]:
        for i, height in enumerate(self.heights):
            if height == MIN_HEIGHT:
                yield Position(i // self.cols, i % self.cols)

    def trailhead_score(self, trailhead: Position) -> int:
        assert self.get(trailhead) == MIN_HEIGHT

        def accessible_peaks(pos: Position) -> Iterable[Position]:
            if self.get(pos) == MAX_HEIGHT:
                yield pos

            for n in self.adjacent_uphill(pos):
                yield from accessible_peaks(n)

        return len(set(accessible_peaks(trailhead)))

    def __str__(self) -> str:
        return "\n".join(
            "".join(
                map(str, self.heights[row * self.cols : row * self.cols + self.cols])
            )
            for row in range(self.rows)
        )


def main():
    with open(sys.argv[1], "r") as input:
        topo_map = TopographicMap([line.strip() for line in input.readlines()])

    total = sum(topo_map.trailhead_score(t) for t in topo_map.trailheads())

    print(f"Sum of trailhead scores: {total}")


if __name__ == "__main__":
    main()

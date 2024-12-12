#!/usr/bin/env python3

from __future__ import annotations

import sys
from functools import reduce
from typing import Iterable, NamedTuple


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


class Region(NamedTuple):
    plant: str
    plots: set[Position]

    def area(self) -> int:
        return len(self.plots)

    def perimiter(self) -> int:
        return sum(
            1 for plot in self.plots for n in plot.neighbors() if n not in self.plots
        )

    def fencing_price(self) -> int:
        return self.area() * self.perimiter()


class Farm:
    def __init__(self, lines: list[str]) -> None:
        self.rows: int = len(lines)
        self.cols: int = len(lines[0])
        self.lines: list[str] = lines

    def get(self, pos: Position) -> str:
        return self.lines[pos.row][pos.col]

    def contains(self, pos: Position) -> bool:
        return (
            pos.row >= 0
            and pos.row < self.rows
            and pos.col >= 0
            and pos.col < self.cols
        )

    def regions(self) -> Iterable[Region]:
        visited: set[Position] = set()

        # recursively expand region until exhausted
        def visit_region(pos: Position, plant: str) -> set[Position]:
            visited.add(pos)
            return reduce(
                set.union,
                (
                    visit_region(n, plant)
                    for n in pos.neighbors()
                    if self.contains(n) and self.get(n) == plant and n not in visited
                ),
                {pos},
            )

        # visit each position that has not yet been visited and expand its region
        for row in range(self.rows):
            for col in range(self.cols):
                pos = Position(row, col)
                if pos not in visited:
                    plant = self.get(pos)
                    yield Region(plant, visit_region(pos, plant))

    def fencing_price(self) -> int:
        return sum(r.fencing_price() for r in self.regions())

    def __str__(self) -> str:
        return "\n".join(self.lines)


def main():
    with open(sys.argv[1], "r") as input:
        farm = Farm([line.strip() for line in input.readlines()])

    print(f"Total fencing price: {farm.fencing_price()}")


if __name__ == "__main__":
    main()

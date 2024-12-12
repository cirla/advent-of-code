#!/usr/bin/env python3

from __future__ import annotations

import sys
from functools import reduce
from itertools import product
from typing import Iterable, NamedTuple


class Position(NamedTuple):
    row: int
    col: int

    def top(self) -> Position:
        return Position(self.row - 1, self.col)

    def bottom(self) -> Position:
        return Position(self.row + 1, self.col)

    def right(self) -> Position:
        return Position(self.row, self.col + 1)

    def left(self) -> Position:
        return Position(self.row, self.col - 1)

    def neighbors(self) -> Iterable[Position]:
        return (self.top(), self.right(), self.bottom(), self.left())


class Region(NamedTuple):
    plant: str
    plots: set[Position]

    def area(self) -> int:
        return len(self.plots)

    def sides(self) -> int:
        # number of vertices of polygon is equal to number of sides
        return sum(self.count_vertices(pos) for pos in self.plots)

    def fencing_price(self) -> int:
        return self.area() * self.sides()

    def count_vertices(self, pos: Position) -> int:
        """
        count number of vertices a plot contributes to overall region polygon
        """
        assert pos in self.plots

        # get all trios of neighbors in a corner direction, e.g. (top, left, top-left)
        trios = (
            (d1(pos), d2(pos), d1(d2(pos)))
            for (d1, d2) in product(
                (Position.top, Position.bottom), (Position.left, Position.right)
            )
        )

        # count plot corners that are vertices
        return sum(
            1
            for (n1, n2, diag) in trios
            # count outside vertices (both corner neighbors (e.g. top + left) *are not* in the region)
            if (n1 not in self.plots and n2 not in self.plots)
            # count inside vertices (both corner neighbors (e.g. top + left) *are* in the region
            # but thier shared neighbor (e.g. top-left diag) is not
            or (n1 in self.plots and n2 in self.plots and diag not in self.plots)
        )


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
                    plots = visit_region(pos, plant)
                    yield Region(plant, plots)

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

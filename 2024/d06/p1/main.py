#!/usr/bin/env python3

from __future__ import annotations

from enum import Enum
import sys
from typing import List, Literal, NamedTuple, Tuple

Delta = Literal[0] | Literal[1] | Literal[-1]


class Direction(Enum):
    UP = 0
    DOWN = 1
    LEFT = 2
    RIGHT = 3

    def delta(self) -> Tuple[Delta, Delta]:
        match self:
            case Direction.UP:
                return (-1, 0)
            case Direction.DOWN:
                return (1, 0)
            case Direction.LEFT:
                return (0, -1)
            case Direction.RIGHT:
                return (0, 1)

    def rotate(self) -> Direction:
        match self:
            case Direction.UP:
                return Direction.RIGHT
            case Direction.DOWN:
                return Direction.LEFT
            case Direction.LEFT:
                return Direction.UP
            case Direction.RIGHT:
                return Direction.DOWN


class Guard(NamedTuple):
    pos: Tuple[int, int]
    dir: Direction

    def inside(self, rows, cols) -> bool:
        row, col = self.pos
        return row >= 0 and row < rows and col >= 0 and col < cols

    def move(self) -> Tuple[int, int]:
        delta = self.dir.delta()
        return (self.pos[0] + delta[0], self.pos[1] + delta[1])


class Map:
    def __init__(self, lines: List[str]) -> None:
        self.rows = len(lines)
        self.cols = len(lines[0])
        self.guard = Guard((0, 0), Direction.UP)
        self.obstructions = set()

        for row, line in enumerate(lines):
            for col, char in enumerate(line):
                if char == "#":
                    self.obstructions.add((row, col))
                elif char == "^":
                    self.guard = Guard((row, col), Direction.UP)

    def distinct_guard_positions(self) -> int:
        visited = set()

        guard = self.guard
        while guard.inside(self.rows, self.cols):
            visited.add(guard.pos)

            next_pos = guard.move()
            if next_pos in self.obstructions:
                guard = Guard(guard.pos, guard.dir.rotate())
            else:
                guard = Guard(next_pos, guard.dir)

        return len(visited)

    def __str__(self) -> str:
        return f"Map(rows={self.rows}, cols={self.cols}, guard={self.guard}, obstructions={len(self.obstructions)})"


def main():
    with open(sys.argv[1], "r") as input:
        map = Map([line.strip() for line in input.readlines()])

    print(f"Distinct Positions: {map.distinct_guard_positions()}")


if __name__ == "__main__":
    main()
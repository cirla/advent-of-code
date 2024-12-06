#!/usr/bin/env python3

from __future__ import annotations

from enum import Enum
import sys
from typing import List, Literal, NamedTuple, Set, Tuple

Delta = Literal[0] | Literal[1] | Literal[-1]
Position = Tuple[int, int]


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
    pos: Position
    dir: Direction

    def inside(self, rows, cols) -> bool:
        row, col = self.pos
        return row >= 0 and row < rows and col >= 0 and col < cols

    def move(self) -> Position:
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

    def obstruction_loop_positions(self) -> int:
        """
        Get number of positions where an introduced obstacle creates a guard loop
        """
        total = 0

        # brute force, baby
        for row in range(self.rows):
            for col in range(self.cols):
                new = (row, col)
                if (
                    new != self.guard.pos
                    and new not in self.obstructions
                    and self.is_loop(self.obstructions | {new})
                ):
                    total += 1

        return total

    def is_loop(self, obstructions: Set[Position]) -> bool:
        """
        Determine if guard will walk in a loop
        """

        visited = set()

        guard = self.guard
        while guard.inside(self.rows, self.cols):
            visited.add(guard)

            next_pos = guard.move()
            if next_pos in obstructions:
                guard = Guard(guard.pos, guard.dir.rotate())
            else:
                guard = Guard(next_pos, guard.dir)

            if guard in visited:
                return True

        return False

    def __str__(self) -> str:
        return f"Map(rows={self.rows}, cols={self.cols}, guard={self.guard}, obstructions={len(self.obstructions)})"


def main():
    with open(sys.argv[1], "r") as input:
        map = Map([line.strip() for line in input.readlines()])

    print(f"Distinct Positions: {map.obstruction_loop_positions()}")


if __name__ == "__main__":
    main()

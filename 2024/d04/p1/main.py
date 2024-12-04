#!/usr/bin/env python3

import sys
from itertools import chain, repeat
from typing import Iterable, List


def num_occurrences(haystack: str, needle: str) -> int:
    """
    find number of occurrences of substring needle in string haystack
    includes overlapping occurrences
    """

    total = 0
    start = haystack.find(needle)
    while start != -1:
        total += 1
        start = haystack.find(needle, start + 1)

    return total


class Grid:
    """
    Square word search grid
    """

    def __init__(self, rows: List[str]) -> None:
        # sanity check that grid is square in shape
        num_rows = len(rows)
        assert num_rows > 0
        for i in range(num_rows):
            assert len(rows[i]) == num_rows

        self.rows = rows
        self.length = num_rows

    def cols(self) -> Iterable[str]:
        for col in range(self.length):
            yield "".join([row[col] for row in self.rows])

    def diagonals(self, up: bool) -> Iterable[str]:
        """
        get diagonals as strings
        up = True means diagonals going up and to the right
        up = False means diagonals going down and to the right
        """
        starting_rows = (
            chain(range(self.length), repeat(self.length - 1, self.length - 1))
            if up
            else chain(reversed(range(self.length)), repeat(0, self.length - 1))
        )
        starting_cols = chain(repeat(0, self.length), range(1, self.length))

        starts = zip(starting_rows, starting_cols)
        for row_start, col_start in starts:
            rows = (
                reversed(range(0, row_start + 1))
                if up
                else range(row_start, self.length)
            )
            cols = range(col_start, self.length)
            yield "".join([self.rows[row][col] for row, col in zip(rows, cols)])

    def num_occurrences(self, word: str) -> int:
        """
        Get number of occurrences of substring needle in all rows,
        columns, and diagonals of the grid (forwards and backwards)
        """

        total = 0
        to_search = chain(
            self.rows, self.cols(), self.diagonals(True), self.diagonals(False)
        )
        for haystack in to_search:
            total += num_occurrences(haystack, word)
            total += num_occurrences(haystack, word[::-1])

        return total


def main():
    with open(sys.argv[1], "r") as input:
        grid = Grid([line.strip() for line in input.readlines()])

    total = grid.num_occurrences("XMAS")
    print(f"Total XMAS occurrences: {total}")


if __name__ == "__main__":
    main()

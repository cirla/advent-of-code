#!/usr/bin/env python3

import sys
from itertools import chain, repeat
from typing import Iterable, List, NamedTuple, Tuple


def find_occurrences(haystack: str, needle: str) -> Iterable[int]:
    """
    find starting index of all occurrences of substring needle
    in string haystack. includes overlapping occurrences
    """

    start = haystack.find(needle)
    while start != -1:
        yield start
        start = haystack.find(needle, start + 1)


class Diagonal(NamedTuple):
    """
    Value and metadata for a diagonal
    """

    # diagonal as string
    value: str

    # (row, col) start of diagonal
    start: Tuple[int, int]


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

    def diagonals(self, up: bool) -> Iterable[Diagonal]:
        """
        get diagonals as strings and starting (row, col) pair
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
            value = "".join([self.rows[row][col] for row, col in zip(rows, cols)])
            yield Diagonal(value, (row_start, col_start))

    def num_x_mas(self, word: str) -> int:
        """
        Get number of occurrences of the string crossing itself in two
        diagonals (forwards and backwards). String must be odd length
        to have a center character on which to cross.
        """
        assert len(word) % 2 == 1
        mid_len = len(word) // 2

        def center(diag: Diagonal, start: int, up: bool) -> Tuple[int, int]:
            """
            get center of found word at offset start in diagonal as (row, col) in grid
            """
            row = (
                diag.start[0] - start - mid_len
                if up
                else diag.start[0] + start + mid_len
            )
            col = diag.start[1] + start + mid_len

            return (row, col)

        up_centers = {
            center(d, start, True)
            for d in self.diagonals(True)
            for start in chain(
                find_occurrences(d.value, word),
                find_occurrences(d.value, word[::-1]),
            )
        }

        down_centers = {
            center(d, start, False)
            for d in self.diagonals(False)
            for start in chain(
                find_occurrences(d.value, word),
                find_occurrences(d.value, word[::-1]),
            )
        }

        return len(up_centers & down_centers)


def main():
    with open(sys.argv[1], "r") as input:
        grid = Grid([line.strip() for line in input.readlines()])

    total = grid.num_x_mas("MAS")
    print(f"Total X-MAS occurrences: {total}")


if __name__ == "__main__":
    main()

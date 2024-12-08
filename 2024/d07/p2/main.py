#!/usr/bin/env python3

from __future__ import annotations

import sys
from operator import add, mul
from typing import List, NamedTuple


def concat(a: int, b: int) -> int:
    return a * (10 ** len(str(b))) + b


class Equation(NamedTuple):
    test: int
    numbers: List[int]

    @classmethod
    def parse(cls, line: str) -> Equation:
        test, numbers = line.split(": ")
        return Equation(int(test), list(map(int, numbers.split())))

    def possible(self) -> bool:
        if len(self.numbers) == 1:
            return self.test == self.numbers[0]

        return any(
            [
                Equation(
                    self.test, [op(self.numbers[0], self.numbers[1])] + self.numbers[2:]
                ).possible()
                for op in (add, mul, concat)
            ]
        )


def main():
    with open(sys.argv[1], "r") as input:
        equations = [Equation.parse(line.strip()) for line in input.readlines()]

    total = sum([eq.test for eq in equations if eq.possible()])

    print(f"Total calibration result: {total}")


if __name__ == "__main__":
    main()

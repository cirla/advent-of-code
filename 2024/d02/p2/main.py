#!/usr/bin/env python3

from __future__ import annotations

import sys
from itertools import pairwise
from typing import Iterable, List, NamedTuple


class Report(NamedTuple):
    levels: List[int]

    def safe(self) -> bool:
        return (
            self.levels == sorted(self.levels)
            or self.levels == sorted(self.levels, reverse=True)
        ) and all(
            [abs(x - y) >= 1 and abs(x - y) <= 3 for (x, y) in pairwise(self.levels)]
        )

    def dampened(self) -> Iterable[Report]:
        for i in range(len(self.levels)):
            yield Report(self.levels[:i] + self.levels[i + 1 :])


def main():
    with open(sys.argv[1], "r") as input:
        reports = [
            Report([int(level) for level in line.strip().split()])
            for line in input.readlines()
        ]

    num_safe = sum((r.safe() or any((d.safe() for d in r.dampened()))) for r in reports)

    print(f"Safe reports: {num_safe}")


if __name__ == "__main__":
    main()

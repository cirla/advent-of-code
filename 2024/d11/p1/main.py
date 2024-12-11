#!/usr/bin/env python3

from __future__ import annotations

import sys
from itertools import chain
from typing import Iterable, List, NamedTuple


class Stone(NamedTuple):
    value: int

    def next(self) -> Iterable[Stone]:
        if self.value == 0:
            yield Stone(1)
            return

        as_str = str(self.value)
        num_digits = len(as_str)
        if num_digits % 2 == 0:
            yield Stone(int(as_str[: num_digits // 2], 10))
            yield Stone(int(as_str[num_digits // 2 :], 10))
            return

        yield Stone(self.value * 2024)


def blink(stones: List[Stone]) -> List[Stone]:
    return list(chain.from_iterable(s.next() for s in stones))


def main():
    with open(sys.argv[1], "r") as input:
        stones = [Stone(int(x)) for x in input.read().strip().split()]

    for _ in range(int(sys.argv[2])):
        stones = blink(stones)

    print(f"Number of stones: {len(stones)}")


if __name__ == "__main__":
    main()

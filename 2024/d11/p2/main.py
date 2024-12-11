#!/usr/bin/env python3

from __future__ import annotations

import sys
from typing import Counter

Stones = Counter[int]


def next_stones(stone: int, n: int) -> Stones:
    if stone == 0:
        return Stones({1: n})

    as_str = str(stone)
    num_digits = len(as_str)
    if num_digits % 2 == 0:
        stones = Stones({int(as_str[: num_digits // 2], 10): n})
        stones.update({int(as_str[num_digits // 2 :], 10): n})
        return stones

    return Stones({stone * 2024: n})


def blink(stones: Stones) -> Stones:
    result = Stones()

    for s, n in stones.items():
        result.update(next_stones(s, n))

    return result


def main():
    with open(sys.argv[1], "r") as input:
        stones = Stones(int(x) for x in input.read().strip().split())

    for _ in range(int(sys.argv[2])):
        stones = blink(stones)

    print(f"Number of stones: {stones.total()}")


if __name__ == "__main__":
    main()

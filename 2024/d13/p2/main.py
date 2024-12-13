#!/usr/bin/env python3

from __future__ import annotations

import sys
from fractions import Fraction
from itertools import groupby
from typing import Iterable, NamedTuple, Self

A_TOKEN_COST = 3
B_TOKEN_COST = 1
PRIZE_CORRECTION = 10_000_000_000_000


class Vec2d(NamedTuple):
    x: int
    y: int


class ClawMachine(NamedTuple):
    a: Vec2d
    b: Vec2d
    prize: Vec2d

    @classmethod
    def parse(cls, lines: Iterable[str]) -> Self:
        (a, b, prize) = lines

        a = Vec2d(*map(int, a.removeprefix("Button A: X+").split(", Y+")))
        b = Vec2d(*map(int, b.removeprefix("Button B: X+").split(", Y+")))
        prize = Vec2d(
            *map(
                lambda p: int(p) + PRIZE_CORRECTION,
                prize.removeprefix("Prize: X=").split(", Y="),
            )
        )

        return cls(a, b, prize)

    def min_to_win(self) -> int:
        """
        Calculate minimum tokens needed to win, or 0 if prize is unwinnable
        """

        # solve system of linear equations:
        #
        # a_presses * a.x + b_presses * b.x = prize.x
        # a_presses * a.y + b_presses * b.y = prize.y
        #
        # convert to slope-intercept form:
        #
        # a_presses = (-b.x/a.x) * b_presses + (prize.x/a.x)
        # a_presses = (-b.y/a.y) * b_presses + (prize.y/a.y)

        # calculate slopes and intercepts
        # use Fraction to avoid floating point issues
        m_px = Fraction(-self.b.x, self.a.x)
        b_px = Fraction(self.prize.x, self.a.x)

        m_py = Fraction(-self.b.y, self.a.y)
        b_py = Fraction(self.prize.y, self.a.y)

        # if slopes are equal and intercepts are different, lines are parallel
        # and therefore there is no solution
        if m_px == m_py and b_px != b_py:
            return 0

        # if slopes are different, solve for b_presses by substituting
        # a_presses in terms of b_presses from the first equation into
        # the second
        #
        # m_px * b_presses + b_px = m_py * b_presses + b_py
        b_presses = (b_py - b_px) / (m_px - m_py)

        # now subtitute b_presses value back into first equation
        a_presses = m_px * b_presses + b_px

        # we can't press a button a fraction of a time, so non-integer
        # solutions don't count
        if not a_presses.is_integer() or not b_presses.is_integer():
            return 0

        return int(a_presses * A_TOKEN_COST + b_presses * B_TOKEN_COST)


def main():
    with open(sys.argv[1], "r") as input:
        machines = [
            ClawMachine.parse(group)
            for key, group in groupby(
                map(str.strip, input.readlines()), lambda line: line == ""
            )
            if not key
        ]

    total = sum(m.min_to_win() for m in machines)

    print(f"Fewest tokens: {total}")


if __name__ == "__main__":
    main()

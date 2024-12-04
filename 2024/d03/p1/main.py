#!/usr/bin/env python3

import re
import sys


MUL_PATTERN = re.compile(r"mul\((\d{1,3}),(\d{1,3})\)")


def main():
    with open(sys.argv[1], "r") as input:
        instructions = [
            map(int, match)
            for line in input.readlines()
            for match in MUL_PATTERN.findall(line.strip())
        ]

    total = sum(x * y for (x, y) in instructions)

    print(f"Multiplication results: {total}")


if __name__ == "__main__":
    main()

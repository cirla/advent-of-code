#!/usr/bin/env python3

import sys
from collections import Counter


def main():
    with open(sys.argv[1], "r") as input:
        left, right = zip(
            *(map(int, line.strip().split()) for line in input.readlines())
        )

    counts = Counter(right)
    total = sum(map(lambda x: x * counts[x], left))

    print(f"Sum of similarity scores: {total}")


if __name__ == "__main__":
    main()

#!/usr/bin/env python3

import sys


def main():
    with open(sys.argv[1], "r") as input:
        left, right = zip(
            *(map(int, line.strip().split()) for line in input.readlines())
        )

    pairs = zip(sorted(left), sorted(right))
    distances = map(lambda p: abs(p[0] - p[1]), pairs)
    total = sum(distances)

    print(f"Sum of distances: {total}")


if __name__ == "__main__":
    main()

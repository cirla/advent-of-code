#!/usr/bin/env python3

import sys
from collections import defaultdict
from functools import cmp_to_key
from itertools import takewhile
from typing import DefaultDict, List, NamedTuple, Self, Set


class Rule(NamedTuple):
    before: int
    after: int

    @classmethod
    def parse(cls, line: str) -> Self:
        parts = line.split("|")
        return cls(int(parts[0]), int(parts[1]))


class Update(NamedTuple):
    pages: List[int]

    @classmethod
    def parse(cls, line: str) -> Self:
        return cls([int(page) for page in line.split(",")])

    def middle(self) -> int:
        return self.pages[len(self.pages) // 2]


class Rules:
    def __init__(self, rules: List[Rule]) -> None:
        self.rules: DefaultDict[int, Set[int]] = defaultdict(set)
        for rule in rules:
            self.rules[rule.before].add(rule.after)

    def check(self, update: Update) -> bool:
        index_map = {page: i for i, page in enumerate(update.pages)}

        for i, page in enumerate(update.pages):
            for after in self.rules.get(page, set()):
                pos = index_map.get(after)
                if pos is not None and i > pos:
                    return False

        return True

    def sort(self, update: Update) -> Update:
        def cmp_pages(x: int, y: int) -> int:
            if y in self.rules[x]:
                return -1
            if x in self.rules[y]:
                return 1
            return 0

        return Update(sorted(update.pages, key=cmp_to_key(cmp_pages)))


def main():
    with open(sys.argv[1], "r") as input:
        lines = map(lambda line: line.strip(), input.readlines())
        rules = [Rule.parse(line) for line in takewhile(lambda line: line != "", lines)]
        updates = [Update.parse(line) for line in lines]

    rules = Rules(rules)
    middles = [
        rules.sort(update).middle() for update in updates if not rules.check(update)
    ]
    print(f"Sum of middles: {sum(middles)}")


if __name__ == "__main__":
    main()

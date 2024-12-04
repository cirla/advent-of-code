#!/usr/bin/env python3

import re
import sys
from enum import Enum
from typing import NamedTuple, Optional, Self, Tuple


class InstructionType(Enum):
    DO = 0
    DONT = 1
    MUL = 2

    def has_data(self) -> bool:
        return self == self.MUL


MulData = Tuple[int, int]


class Instruction(NamedTuple):
    type: InstructionType
    data: Optional[MulData]

    @classmethod
    def from_match(cls, m: re.Match) -> Self:
        groups = m.groupdict()
        for inst in InstructionType:
            if groups[inst.name] is not None:
                # only one instruction type can have data
                data = (int(groups["x"]), int(groups["y"])) if inst.has_data() else None
                return cls(inst, data)

        raise ValueError(f"Invalid instruction: {groups}")


class Process:
    def __init__(self) -> None:
        self.mul_enabled = True
        self.total = 0

    def execute(self, inst: Instruction):
        match inst.type:
            case InstructionType.DO:
                self.mul_enabled = True
            case InstructionType.DONT:
                self.mul_enabled = False
            case InstructionType.MUL:
                if self.mul_enabled:
                    assert inst.data is not None
                    self.total += inst.data[0] * inst.data[1]


# regex abuse
PATTERNS = [
    f"(?P<{inst.name}>{pattern})"
    for (inst, pattern) in [
        (InstructionType.DO, r"do\(\)"),
        (InstructionType.DONT, r"don't\(\)"),
        (InstructionType.MUL, r"mul\((?P<x>\d{1,3}),(?P<y>\d{1,3})\)"),
    ]
]

INST_PATTERN = re.compile("|".join(PATTERNS))


def main():
    with open(sys.argv[1], "r") as input:
        instructions = [
            Instruction.from_match(match)
            for line in input.readlines()
            for match in INST_PATTERN.finditer(line.strip())
        ]

    process = Process()
    for inst in instructions:
        process.execute(inst)

    print(f"Execution results: {process.total}")


if __name__ == "__main__":
    main()

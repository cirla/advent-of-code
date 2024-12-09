#!/usr/bin/env python3

from __future__ import annotations

import sys
from itertools import repeat
from operator import attrgetter
from typing import Iterable, List, NamedTuple, Optional


class Chunk(NamedTuple):
    """
    Chunk of empty space or file blocks
    """

    position: int
    length: int
    file_id: Optional[int]

    def __str__(self) -> str:
        if self.file_id is None:
            return "." * self.length
        return str(self.file_id) * self.length


class Block(NamedTuple):
    """
    Block from a file chunk
    """

    position: int
    file_id: int


class DiskMap:
    def __init__(self, line: str) -> None:
        self.chunks: List[Chunk] = []

        position = 0
        for i, length in enumerate(map(int, line)):
            # even indices are file chunks with monotonically increasing file ids
            # odd indices are empty chunks inbetween files
            self.chunks.append(Chunk(position, length, i // 2 if i % 2 == 0 else None))
            position += length

    def compact(self):
        """
        Move files from highest positions to fill space in lowest empty positions
        """
        files = {}
        movable = [c for c in self.chunks if c.file_id is not None][::-1]
        empty_spaces = [c for c in self.chunks if c.file_id is None]

        # try to move each file once in order from right to left
        for file in movable:
            for empty in empty_spaces:
                if empty.position > file.position:
                    # only move to left
                    break
                if file.length <= empty.length:
                    files[file.file_id] = Chunk(
                        empty.position, file.length, file.file_id
                    )
                    empty_spaces.remove(empty)
                    # we don't need to worry about combining adjacent empty empty spaces
                    # left behind by moving a file since the files are checked in right
                    # to left order
                    empty_spaces.append(Chunk(file.position, file.length, None))
                    if file.length < empty.length:
                        # preserve remaining empty space
                        empty_spaces.append(
                            Chunk(
                                empty.position + file.length,
                                empty.length - file.length,
                                None,
                            )
                        )
                        # preserve ordering of open empty chunks
                        empty_spaces.sort(key=attrgetter("position"))
                    # move on to next file
                    break

            if file.file_id not in files:
                # file wasn't moved, so keep it where it is
                files[file.file_id] = file

        # recombine and sort by position
        chunks = empty_spaces + list(files.values())
        chunks.sort(key=attrgetter("position"))

        self.chunks = chunks

    def file_blocks(self) -> Iterable[Block]:
        """
        Get (position, file_id) for all non-empty positions
        """
        for chunk in self.chunks:
            if chunk.file_id is not None:
                yield from (
                    Block(pos, file_id)
                    for pos, file_id in enumerate(
                        repeat(chunk.file_id, times=chunk.length), start=chunk.position
                    )
                )

    def checksum(self) -> int:
        return sum(b.position * b.file_id for b in self.file_blocks())

    def __str__(self) -> str:
        return "".join(map(str, self.chunks))


def main():
    with open(sys.argv[1], "r") as input:
        disk_map = DiskMap(input.read().strip())

    disk_map.compact()
    print(f"Checksum: {disk_map.checksum()}")


if __name__ == "__main__":
    main()

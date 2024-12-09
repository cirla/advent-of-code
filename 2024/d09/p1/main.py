#!/usr/bin/env python3

from __future__ import annotations

import sys
from itertools import repeat
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
        Break up files from highest positions to fill space in lowest empty positions
        """
        chunks = []
        files = {c.file_id: c for c in self.chunks if c.file_id is not None}
        max_file = [file_id for file_id in files][-1]

        position = 0
        for c in self.chunks:
            # no files left to move
            if len(files) == 0:
                break
            # chunk is empty and needs to be filled
            if c.file_id is None:
                remaining = c.length
                # keep filling file chunks until empty chunk is filled
                while remaining > 0:
                    movable = files.pop(max_file)
                    assert movable.file_id is not None

                    if movable.length <= remaining:
                        # entire file fits in free space
                        chunks.append(Chunk(position, movable.length, movable.file_id))
                        position += movable.length
                        remaining -= movable.length
                        max_file -= 1
                    else:
                        # only partial file fits in free space
                        chunks.append(Chunk(position, remaining, movable.file_id))
                        files[movable.file_id] = Chunk(
                            movable.position,
                            movable.length - remaining,
                            movable.file_id,
                        )
                        position += remaining
                        remaining = 0
            # chunk is a file and can just be appended
            else:
                # get from remaining files as chunk may have already partially been moved
                c = files.pop(c.file_id)
                chunks.append(Chunk(position, c.length, c.file_id))
                position += c.length

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

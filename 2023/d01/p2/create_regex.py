#!/usr/bin/env python3

from trieregex import TrieRegEx as TRE

DIGITS = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
print(DIGITS)
print(TRE(*DIGITS).regex())

REVERSED_DIGITS = [d[::-1] for d in DIGITS]
print(REVERSED_DIGITS)
print(TRE(*REVERSED_DIGITS).regex())

#!/usr/bin/env python3

from trieregex import TrieRegEx as TRE

DIGITS = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

print(TRE(*DIGITS).regex())

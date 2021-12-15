import collections
import itertools as it
import sys

template, _, *instructions = map(str.strip, sys.stdin)

instruction_lookup = {
    pair: insertion
    for pair, insertion in (instruction.split(" -> ") for instruction in instructions)
}

def grow_polymer(polymer):
    new_poly = []
    for i, char, next_char in zip(it.count(), polymer, polymer[1:]):
        new_poly.append(char)
        insertion = instruction_lookup.get(char + next_char, "")
        if insertion:
            new_poly.append(insertion)
    return "".join(new_poly) + next_char

polymer = template
for i in range(40):
    polymer = grow_polymer(polymer)
    print(i)

elements = collections.Counter(polymer)
print(max(elements.values()) - min(elements.values()))

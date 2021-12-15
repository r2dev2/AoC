import collections
import itertools as it
import sys

template, _, *instructions = map(str.strip, sys.stdin)
e0, *_, ef = template

instruction_lookup = {
    pair: insertion
    for pair, insertion in (instruction.split(" -> ") for instruction in instructions)
}

# convert initial template into counter of adjacent chars
polymer = collections.Counter(map("".join, zip(template, template[1:])))

def step_polymer(polymer):
    new_polymer = collections.Counter()
    for pair, count in polymer.items():
        x, z = pair
        y = instruction_lookup.get(pair, "")
        if y:
            new_polymer[x + y] += count
            new_polymer[y + z] += count
        else:
            new_polymer[pair] += count
    return new_polymer


def amount_each(polymer):
    amount = collections.Counter()
    for (x, y), count in polymer.items():
        amount[x] += count
        amount[y] += count
    for el in {*amount}:
        amount[el] //= 2
    amount[e0] += 1
    amount[ef] += 1
    return amount


for _ in range(40):
    polymer = step_polymer(polymer)

elements = amount_each(polymer)
print(max(elements.values()) - min(elements.values()))

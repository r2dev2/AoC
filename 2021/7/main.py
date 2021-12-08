import sys

crabs = [*map(int, sys.stdin.read().split(","))]

target = round(sum(crabs) / len(crabs))
fuel_needed = lambda dist: .5 * dist ** 2 + .5 * dist
amount_needed = min(sum(fuel_needed(abs(c - target)) for c in crabs) for target in range(min(crabs), max(crabs) + 1))

print(amount_needed)

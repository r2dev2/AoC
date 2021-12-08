import sys

fish = [*map(int, sys.stdin.read().split(","))]

for x in range(256):
    fish = [f - 1 for f in fish]
    fish += [8] * fish.count(-1)
    fish = [f if f >= 0 else 6 for f in fish]
    print(x + 1, len(fish))

print(len(fish))

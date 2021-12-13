from pprint import pprint
import sys

octopi = [[*map(int, line.strip())] for line in sys.stdin]

maxx = len(octopi)
maxy = len(octopi[0])
coords = [(i, j) for i in range(maxx) for j in range(maxy)]
deltas = [(i, j) for i in range(-1, 2) for j in range(-1, 2) if i or j]

def do_step():
    todos = coords[:]
    flashes = set()

    while todos:
        i, j = todos.pop()
        if not (0 <= i < maxx and 0 <= j < maxy):
            continue

        octopi[i][j] += 1
        if octopi[i][j] > 9 and (i, j) not in flashes:
            todos.extend((i + di, j + dj) for di, dj in deltas)
            flashes.add((i, j))


    for i, j in coords:
        if octopi[i][j] > 9:
            octopi[i][j] = 0

    return len(flashes)


steps = 0

while {o for octorow in octopi for o in octorow} != {0}:
    do_step()
    steps += 1

print(steps)

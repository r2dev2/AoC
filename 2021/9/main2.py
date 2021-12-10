import operator as op
import sys
from functools import reduce


heightmap = [[*map(int, line.strip())] for line in sys.stdin]
maxx = len(heightmap)
maxy = len(heightmap[0])
visited = set()

def get_basin_size(x, y):
    if (x, y) in visited or not (0 <= x < maxx) or not (0 <= y < maxy) or heightmap[x][y] == 9:
        return 0
    visited.add((x, y))
    return sum([
        1,
        get_basin_size(x - 1, y),
        get_basin_size(x + 1, y),
        get_basin_size(x, y - 1),
        get_basin_size(x, y + 1),
    ])

basins = [get_basin_size(x, y) for x in range(maxx) for y in range(maxy) if (x, y) not in visited]
print(reduce(op.mul, sorted(basins)[-3:]))

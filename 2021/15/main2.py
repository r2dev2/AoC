from heapq import heappush, heappop
import sys

og_grid = [[*map(int, line.strip())] for line in sys.stdin]
wrap = lambda risk: (risk - 1) % 9 + 1
grid_of_grids = [
    [[wrap(risk + row + col) for risk in r] for r in og_grid]
    for row in range(5) for col in range(5)
]

grid = sum(grid_of_grids[:5], start=[])
for line in grid:
    lin = line[:]
    for inc in range(1, 5):
        line.extend([wrap(x + inc) for x in lin])


maxx = len(grid)
maxy = len(grid[0])
inf = float("infinity")

neighbors = lambda x, y: [(x + dx, y + dy) for dx, dy in [(0, 1), (0, -1), (1, 0), (-1, 0)]if 0 <= x + dx < maxx and 0 <= y + dy < maxy]

distances = {(0, 0): 0}
edges = [(0, (0, 0))]
key = lambda coords: distances.get(coords, inf)

while edges:
    _, node = heappop(edges)
    if node == (maxx - 1, maxy - 1):
        print(distances[node])
        exit()

    for neighbor in neighbors(*node):
        x, y = neighbor
        dist = distances[node] + grid[x][y]
        if dist < distances.get(neighbor, inf):
            distances[neighbor] = dist
            heappush(edges, (key(neighbor), neighbor))

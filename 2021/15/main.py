from queue import PriorityQueue
import sys

grid = [[*map(int, line.strip())] for line in sys.stdin]

maxx = len(grid)
maxy = len(grid[0])
inf = float("infinity")

neighbors = lambda x, y: [(x + dx, y + dy) for dx, dy in [(0, 1), (0, -1), (1, 0), (-1, 0)]if 0 <= x + dx < maxx and 0 <= y + dy < maxy]

distances = {(0, 0): 0, (1, 0): grid[1][0], (0, 1): grid[0][1]}
visited = set()
edges = neighbors(0, 0)
key = lambda coords: distances.get(coords, inf)

while edges:
    edges = sorted(set(edges), key=key)
    node = edges.pop(0)
    visited.add(node)
    for neighbor in neighbors(*node):
        if neighbor not in visited:
            edges.append(neighbor)
            x, y = neighbor
            dist = distances[node] + grid[x][y]
            if dist < distances.get(neighbor, inf):
                distances[neighbor] = dist


print(distances[(maxx - 1, maxy - 1)])

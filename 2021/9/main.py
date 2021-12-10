import sys


heightmap = [[*map(int, line.strip())] for line in sys.stdin]

def is_low(i, j):
    for ii in range(i - 1, i + 2):
        for jj in range(j - 1, j + 2):
            if 0 <= ii < len(heightmap) and 0 <= jj < len(heightmap[0]) and heightmap[i][j] > heightmap[ii][jj]:
                return False

    return True

low_points = [h for i, row in enumerate(heightmap) for j, h in enumerate(row) if is_low(i, j)]
print(sum(x + 1 for x in low_points))

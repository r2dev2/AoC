import sys

def input_line_to_line(line):
    return [[*map(int, point.split(","))] for point in line.split(" -> ")]

def is_horizontal_or_vertical(line):
    (x0, y0), (x1, y1) = line
    return x0 == x1 or y0 == y1

lines = [*filter(is_horizontal_or_vertical, map(input_line_to_line, sys.stdin))]

overlaps = dict() # (x, y) -> amount overlapped

for (x0, y0), (x1, y1) in lines:
    for x in range(min(x0, x1), max(x0, x1) + 1):
        for y in range(min(y0, y1), max(y0, y1) + 1):
            overlaps[(x, y)] = overlaps.get((x, y), 0) + 1

print(sum(o >= 2 for o in overlaps.values()))

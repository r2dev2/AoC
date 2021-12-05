import sys

def input_line_to_line(line):
    return [[*map(int, point.split(","))] for point in line.split(" -> ")]


lines = [*map(input_line_to_line, sys.stdin)]

overlaps = dict() # (x, y) -> amount overlapped

get_range = lambda c0, c1: range(c0, c1 + 1) if c0 < c1 else range(c0, c1 - 1, -1)

for (x0, y0), (x1, y1) in lines:
    x_range = [*get_range(x0, x1)]
    y_range = [*get_range(y0, y1)]
    x_range += [x_range[-1]] * (len(y_range) - len(x_range))
    y_range += [y_range[-1]] * (len(x_range) - len(y_range))
    for x, y in zip(x_range, y_range):
        overlaps[(x, y)] = overlaps.get((x, y), 0) + 1

print(sum(o >= 2 for o in overlaps.values()))

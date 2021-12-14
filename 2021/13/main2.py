import sys

points = set()
folds = []

for line in map(str.strip, sys.stdin):
    if line.startswith("fold along"):
        axis, am = line.split(" ")[-1].split("=")
        @folds.append
        def fold(x, y, axis=axis, am=am):
            amount = int(am) * 2
            return (x, min(y, amount - y)) if axis == "y" else (min(x, amount - x), y)

    elif line:
        points.add(tuple(map(int, line.strip().split(","))))


for fold in folds:
    for point in [*points]:
        points.remove(point)
        points.add(fold(*point))


maxx = max(x for x, _y in points)
maxy = max(y for _x, y in points)

for y in range(maxy + 1):
    for x in range(maxx + 1):
        print("#" if (x, y) in points else ".", end="")
    print()

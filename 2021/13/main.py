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


first_fold, *_ = folds

for point in [*points]:
    points.remove(point)
    points.add(first_fold(*point))

print(len(points))



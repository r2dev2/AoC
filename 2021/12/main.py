import sys

edges = dict()

for line in sys.stdin:
    e1, e2 = line.strip().split("-")
    edges[e1] = edges.get(e1, [])
    edges[e2] = edges.get(e2, [])
    edges[e1].append(e2)
    edges[e2].append(e1)


paths = [["start"]]
final_paths = []

while paths:
    *path, last = paths.pop()
    if last == "end":
        final_paths.append([*path, last])
        continue
    cant_visit = {e for e in path if e.lower() == e and e != "end"}
    paths.extend(
        [*path, last, candidate]
        for candidate in edges[last]
        if candidate not in cant_visit
    )

print(len(final_paths))

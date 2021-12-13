from contextlib import suppress
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
    cant_visit = {"start"}
    counts = dict()
    had_two = False
    for e in [*path, last]:
        if e.lower() != e or e == "end":
            continue
        cant_visit.add(e)
        counts[e] = counts.get(e, 0) + 1
        if counts[e] == 2:
            had_two = True


    paths.extend(
        [*path, last, candidate]
        for candidate in edges[last]
        if candidate not in cant_visit or not had_two and candidate != "start"
    )


print(len(final_paths))

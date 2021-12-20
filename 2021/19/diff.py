import sys

sample_res, res_s = sys.argv[1:]

with open(sample_res, "r") as fin:
    correct = set(filter(bool, map(str.strip, fin.read().split("\n"))))

with open(res_s, "r") as fin:
    attempt = {
        x.replace(" ", "") for x in
        filter(bool, map(str.strip, fin.read().split("\n")))}

transform = lambda x, y, z: (-(x - 1105), z - 1229, (y + 1205))
ut = lambda x, y, z: (1105 - x, z - 1205, y + 1229)

diff = sorted(correct - attempt)
extra = sorted(attempt - correct)
print("Missing", len(diff))
for d in diff:
    t = transform(*map(int, d.split(",")))
    print(d, t)
print("Extra", len(extra))
print(*extra, sep="\n")

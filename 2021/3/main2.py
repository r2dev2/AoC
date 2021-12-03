import sys

data = [*map(str.strip, sys.stdin)]
first, *_ = data
o2 = data
co2 = data

def get_new_data(data, i, order):
    if len(data) == 1:
        return data
    to_choose = order[int(2 * sum(int(s[i]) for s in data) >= len(data))]
    return [d for d in data if d[i] != to_choose]

for i in range(len(first)):
    o2 = get_new_data(o2, i, "10")
    co2 = get_new_data(co2, i, "01")

assert len(o2) == 1
assert len(co2) == 1

print(int(o2[0], 2) * int(co2[0], 2))

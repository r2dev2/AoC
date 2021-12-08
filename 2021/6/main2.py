import sys

fish = [*map(int, sys.stdin.read().split(","))]

amount_each = dict()

for f in fish:
    amount_each[f] = amount_each.get(f, 0) + 1

for _ in range(256):
    new_amount = dict()
    for n, amount in amount_each.items():
        if n == 0:
            new_amount[8] = amount
            new_amount[6] = new_amount.get(6, 0) + amount
        else:
            new_amount[n - 1] = new_amount.get(n - 1, 0) + amount
    amount_each = new_amount


print(sum(x for x in amount_each.values()))

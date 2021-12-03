import sys

data = [*map(str.strip, sys.stdin)]

gamma = []

for i in range(len(data[0])):
    cs = sum(int(s[i]) for s in data)
    gamma.append("1" if 2 * cs > len(data) else "0")

gamma = int("".join(gamma), 2)
ep = 2 ** len(data[0]) - 1 - gamma

print(gamma * ep)

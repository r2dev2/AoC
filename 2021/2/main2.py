"""
Part 2
"""

import sys

x, y, aim = [0] * 3

cmds = {
    "down": lambda amt: (x, y, aim + amt),
    "up": lambda amt: (x, y, aim - amt),
    "forward": lambda amt: (x + amt, y + aim * amt, aim)
}

for cmd, amt in map(lambda s: s.split(" "), sys.stdin):
    x, y, aim = cmds[cmd](int(amt))

print(x, y, x * y)

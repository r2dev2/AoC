"""
Part 1
"""

import sys

x = 0
y = 0

cmds = {
    "forward": lambda amt: (x + amt, y),
    "down": lambda amt: (x, y + amt),
    "up": lambda amt: (x, y - amt)
}

for cmd, amt in map(lambda s: s.split(" "), sys.stdin):
    x, y = cmds[cmd](int(amt))

print(x, y, x * y)

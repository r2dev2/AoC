import itertools as it
import re
import sys

part = 1

lines = [*map(str.strip, sys.stdin)]

nums_to_be_called = [*map(int, lines[0].split(","))]
nums_called = set()

def has_bingo(board):
    for row in it.chain(board, zip(*board)):
        if all(map(nums_called.__contains__, row)):
            return True
    return False


def score_bingo(board):
    return sum(
        x for row in board for x in row
        if x not in nums_called
    )


boards = []
board = []

for line in lines[2:]:
    if not line:
        boards.append(board)
        board = []
        continue
    board.append([*map(int, re.split(r"\W+", line))])


for num in nums_to_be_called:
    nums_called.add(num)
    boards_with_bingos = [*filter(has_bingo, boards)]
    bingo_scores = map(score_bingo, boards_with_bingos)

    if part == 1:
        if boards_with_bingos:
            print(max(bingo_scores) * num)
            exit()

    if part == 2:
        [*map(boards.remove, boards_with_bingos)]
        if not boards:
            print(min(bingo_scores) * num)
            exit()

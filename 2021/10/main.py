import sys

lines = [*map(str.strip, sys.stdin)]

end_tokens = {
    ")": "(",
    "]": "[",
    "}": "{",
    ">": "<"
}

token_points = {
    ")": 3,
    "]": 57,
    "}": 1197,
    ">": 25137
}

def corrupted_score(line):
    tokens = []
    for i, token in enumerate(line):
        if token in end_tokens and tokens and end_tokens[token] == tokens[-1]:
            tokens.pop()
        elif token in end_tokens:
            return token_points[token]
        else:
            tokens.append(token)
    return 0

print(sum(map(corrupted_score, lines)))

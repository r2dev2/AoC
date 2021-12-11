import sys

lines = [*map(str.strip, sys.stdin)]

end_tokens = {
    ")": "(",
    "]": "[",
    "}": "{",
    ">": "<"
}

token_points = {
    "(": 1,
    "[": 2,
    "{": 3,
    "<": 4
}

def autocomplete_score(line):
    tokens = []
    for i, token in enumerate(line):
        if tokens and end_tokens.get(token) == tokens[-1]:
            tokens.pop()
        elif token in end_tokens:
            return None
        else:
            tokens.append(token)

    return sum(
        token_points[token] * 5 ** i for i, token in enumerate(tokens)
    )

scores = sorted(score for score in map(autocomplete_score, lines) if score is not None)
print(scores[len(scores) // 2])

import sys


# amount segments -> [possible numbers]
possibilities = {
    2: [1],
    3: [7],
    4: [4],
    5: [2, 3, 5],
    6: [0, 6, 9],
    7: [8]
}


def get_display(line):
    input_, output = line.strip().split(" | ")
    return (input_.split(" "), output.split(" "))


def get_decode(input_):
    decode = dict()
    dd = dict()
    fives = set()
    sixes = set()

    def assign(inp, num):
        dd[num] = inp
        decode[inp] = num

    for inp in map("".join, map(sorted, input_)):
        l = len(inp)
        if l == 5:
            fives.add(inp)
        if l == 6:
            sixes.add(inp)
        if len(possibilities[l]) == 1:
            assign(inp, possibilities[l][0])

    b_or_d = set(dd[4]) - set(dd[1])

    # get 0
    for six in [*sixes]:
        if set(dd[8]) - b_or_d - set(six):
            continue
        assign(six, 0)
        sixes.remove(six)

    s1, s2 = sixes
    d, = set(dd[8]) - set(dd[0])
    b, = b_or_d - {d}

    if set(dd[1]) - set(s1):
        s1, s2 = s2, s1

    assign(s1, 9)
    assign(s2, 6)

    e, = set(dd[8]) - set(dd[9])

    for five in [*fives]:
        if e in set(five):
            assign(five, 2)
            fives.remove(five)

    for five in [*fives]:
        if set(dd[2]) | set(five) == set(dd[8]):
            assign(five, 5)
            fives.remove(five)

    three_, = fives

    assign(three_, 3)

    return decode.__getitem__


def decode_output(input_, outputs):
    decoded = 0
    decode = get_decode(input_)

    for i, output in enumerate(outputs):
        mult = 10 ** (3 - i)
        decoded += mult * decode("".join(sorted(output)))

    return decoded


displays = [*map(get_display, sys.stdin)]
print(sum(decode_output(inp, out) for inp, out in displays))

# 
# outputs = [output for _, outputs in displays for output in outputs]
# unique_sums = {2, 4, 3, 7}
# print(sum(len(output) in unique_sums for output in outputs))

import sys

def get_display(line):
    input_, output = line.strip().split(" | ")
    return (input_.split(" "), output.split(" "))

displays = [*map(get_display, sys.stdin)]

outputs = [output for _, outputs in displays for output in outputs]
unique_sums = {2, 4, 3, 7}
print(sum(len(output) in unique_sums for output in outputs))

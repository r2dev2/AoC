"""
Notes:

packet type:
0 -> sum of subpackets
1 -> product of subproducts
2 -> mininum of subproducts
3 -> maximum of subproducts
4 -> value
5 -> greater than
6 -> less than
7 -> equal to

length type id (bit after packet type):
0 -> next 15 bits represent total length in bits of sub-packets contained by this packet
1 -> next 11 bits represent number of sub-packets immediately contained by this packet
"""

import operator as op
from functools import partial, reduce

SUM = 0
PRODUCT = 1
MIN = 2
MAX = 3
VALUE = 4
GREATER = 5
LESS = 6
EQUAL = 7

unpack = lambda fn: lambda args: fn(*args)

reducer_map = {
    SUM: sum,
    PRODUCT: partial(reduce, op.mul),
    MIN: min,
    MAX: max,
    GREATER: unpack(op.gt),
    LESS: unpack(op.lt),
    EQUAL: unpack(op.eq)
}

inp = input()
packet = f"{int(inp, 16):b}"
if len(packet) % 4:
    packet = "0" * (4 - len(packet) % 4) + packet
if inp.startswith("0"):
    packet = "0000" + packet

# print(packet, len(packet))


def parse_packet(packet):
    packet_version = int(packet[:3], 2)
    packet_type = int(packet[3:6], 2)
    # print("Parsing packet with version", packet_version, "type", packet_type, "packet", packet, "len", len(packet))

    max_subpacket_length = float("infinity")
    max_subpacket_num = float("infinity")
    cursor = 6 # from the packet version and type
    consumed_subpacket_length = 0
    consumed_subpacket_num = 0
    subpacket_values = []

    if packet_type == VALUE:
        # print("got value packet", packet)

        total_num = []
        num = "1"
        while num[0] != "0":
            num = packet[cursor: cursor + 5]
            cursor += 5
            total_num.append(num[1:])

        return int("".join(total_num), 2), cursor

    length_type_id = packet[6]
    if length_type_id == "0":
        max_subpacket_length = int(packet[7:7 + 15], 2)
        cursor = 7 + 15
    else:
        max_subpacket_num = int(packet[7: 7 + 11], 2)
        cursor = 7 + 11

    while (consumed_subpacket_length < max_subpacket_length
           and consumed_subpacket_num < max_subpacket_num):
        # print("cursor: ", cursor + consumed_subpacket_length)
        value, consumed = parse_packet(packet[cursor + consumed_subpacket_length:])
        # print(value)
        # print("got value", value, "consumed", consumed)
        subpacket_values.append(value)
        consumed_subpacket_length += consumed
        consumed_subpacket_num += 1

    return int(reducer_map[packet_type](subpacket_values)), cursor + consumed_subpacket_length
    # return sum_version_nums, len(packet)

tried = {42181138351188143}

print(parse_packet(packet)[0])

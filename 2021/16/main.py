"""
Notes:

length type id (bit after packet type):
0 -> next 15 bits represent total length in bits of sub-packets contained by this packet
1 -> next 11 bits represent number of sub-packets immediately contained by this packet
"""

VALUE = 4

packet = f"{int(input(), 16):b}"
if len(packet) % 4:
    packet = "0" * (4 - len(packet) % 4) + packet

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
    sum_version_nums = packet_version

    if packet_type == VALUE:
        # print("got value packet", packet)

        num = "1"
        while num[0] != "0":
            num = packet[cursor: cursor + 5]
            cursor += 5

        return packet_version, cursor

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
        version, consumed = parse_packet(packet[cursor + consumed_subpacket_length:])
        sum_version_nums += version
        consumed_subpacket_length += consumed
        consumed_subpacket_num += 1

    return sum_version_nums, cursor + consumed_subpacket_length
    # return sum_version_nums, len(packet)


print(parse_packet(packet)[0])

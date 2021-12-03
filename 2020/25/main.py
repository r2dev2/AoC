import math

card_key = 17773298
door_key = 15530095
LIMIT = 20201227

loop_size_lim = int(1e8)

def transform_subj_num(subject_number, loop_size):
    return pow(subject_number, loop_size, LIMIT)


for lim in range(loop_size_lim):
    t = transform_subj_num(7, lim)
    if t == card_key:
        print("Found card loop size", lim)
        print("Encryption key:", transform_subj_num(door_key, lim))
        break
    if t == door_key:
        print("Found door loop size", lim)
        print("Encryption key:", transform_subj_num(card_key, lim))
        break


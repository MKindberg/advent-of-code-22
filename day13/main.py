def cmp(item1, item2):
    if type(item1) is list and type(item2) is int:
        return cmp(item1, [item2])

    if type(item1) is int and type(item2) is list:
        return cmp([item1], item2)

    try:
        if item1 == item2:
            return 0
        if item1 < item2:
            return -1
        return 1
    except:
        assert(type(item1) is list)
        assert(type(item2) is list)
        for i1, i2 in zip(item1, item2):
            c = cmp(i1, i2)
            if c == 1:
                return 1
            if c == -1:
                return -1
    return -1


with open("input") as f:
    content = f.read()

packets = []
for block in content.split("\n\n"):
    subpackages = []
    for sub in block.splitlines():
        subpackages.append(eval(sub))
    packets.append(subpackages)

count = 0
for i in range(0, len(packets)):
    if cmp(packets[i][0], packets[i][1]) == -1:
        count += i + 1
print(f"Part 1: {count}")

packets.append([[[2]], [[6]]])
packets = [p for subp in packets for p in subp]

from functools import cmp_to_key
packets.sort(key=cmp_to_key(cmp))
key = (packets.index([[6]])+1) * (packets.index([[2]])+1)
print(f"Part 2: {key}")

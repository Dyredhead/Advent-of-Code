def main() -> None:
    rucksacks = []
    with open("input3.txt", 'r') as f:
        rucksacks = f.read().strip().splitlines()
    
    # compartments = []
    # for rucksack in rucksacks:
    #     l = len(rucksack)//2
    #     compartments.append((rucksack[:l], rucksack[l:]))

    # sum = 0
    # for i in compartments:
    #     common = common_member2(i[0], i[1])
    #     if common.islower():
    #         sum += ord(common) - 96
    #     else:
    #         sum += ord(common) - 38
    # print(sum)

    sum = 0
    i = 0
    while (i < len(rucksacks)):
        common = common_member3(rucksacks[i], rucksacks[i+1], rucksacks[i+2])
        if common.islower():
            sum += ord(common) - 96
        else:
            sum += ord(common) - 38
        i += 3
    print(sum)

def common_member2(a, b):
    a_set = set(a)
    b_set = set(b)

    return list(a_set & b_set)[0]

def common_member3(a, b, c):
    a_set = set(a)
    b_set = set(b)
    c_set = set(c)

    return list(a_set & b_set & c_set)[0]


if __name__ == "__main__":
	main()
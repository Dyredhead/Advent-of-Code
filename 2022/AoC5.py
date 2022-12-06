from collections import deque

def main() -> None:
    input = []
    with open("input5.txt", 'r') as f:
        input = f.read().strip().splitlines()
    
    nums = input[8]

    stacks = [deque(), deque(), deque(), deque(), deque(), deque(), deque(), deque(), deque()]
    for i in range(7,-1,-1):
        for j in range(1,10):
            crate = input[i][nums.index(str(j))]
            if (crate != ' '):
                stacks[j-1].append(crate)
    
    for line in input[10:]:
        line = line.split(" ")
        num = int(line[1])
        origin = stacks[int(line[3])-1]
        destination = stacks[int(line[5])-1]

        # for i in range(0, num):
        #     top = origin.pop()
        #     destination.append(top)

        crates = []
        for i in range(0, num):
            top = origin.pop()
            crates.append(top)

        for i in crates[::-1]:
            destination.append(i)


    result = ""
    for stack in stacks:
        result += stack.pop()

    print(result)

if __name__ == "__main__":
	main()
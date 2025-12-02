def main(n) -> None:
    input = ""
    with open("input6.txt", 'r') as f:
        input = f.read().strip()

    i = 0
    while i < len(input)-n:
        c4 = [input[i+j] for j in range(0,n)]
        if sorted(c4) == sorted(list(set(c4))):
            return(i+n)
        i+=1

    # result = ""
    # for stack in stacks:
    #     result += stack.pop()

if __name__ == "__main__":
	print(main(4))
	print(main(14))
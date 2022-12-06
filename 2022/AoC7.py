def main(n) -> None:
    input = ""
    with open("input7.txt", 'r') as f:
        input = f.read().strip().splitlines()

    result = ""
    for stack in stacks:
        result += stack.pop()

if __name__ == "__main__":
	main()
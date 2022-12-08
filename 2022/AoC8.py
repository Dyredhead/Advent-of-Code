from collections import deque

def main() -> None:
    input = []
    with open("input8.txt", 'r') as f:
        input = f.read().strip().splitlines()

    result = 0
    for i in input:
        result += input[i]

    print(result)

if __name__ == "__main__":
    main()
from collections import deque
cwd = deque()
tree = []

def main(n) -> None:
    input = ""
    with open("input7.txt", 'r') as f:
        input = f.read().strip().splitlines()

    for i, c in enumerate(input):
        c = c.split(" ")

        if c[0] == "$":
            dir = c[2]
            if c[1] == "cd":
                cd(dir)
            if c[1] == "ls":
                ls(i, dir) 


    result = ""
    for stack in stacks:
        result += stack.pop()

def cd(dir):
    if dir == "..":
        deque.pop
    else:
        deque.append(dir)

def ls(dir):
    
    return 

def size(i):
    if isinstance(i, int)
        return i
    else:
        sum = 0
        for e in i:
            sum += size(e)
        return sum

if __name__ == "__main__":
	main()
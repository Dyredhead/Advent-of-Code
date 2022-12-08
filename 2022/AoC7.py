from collections import deque
tree = {}
cwd = deque()
cwd.append(tree)

counter = 0
min_size = 70000000

def main() -> None:
    input = ""
    with open("input7.txt", 'r') as f:
        input = f.read().strip().split("$")
    for i, e in enumerate(input):
       input[i] = e.splitlines() 

    # print(f"input: {input}")
    i = 2
    while (i < len(input)):
        line = input[i][0].split()
        # print(f"line: {line}")

        if line[0] == "cd":
            dir = line[1]
            cd(dir)
        if line[0] == "ls":
            ls(input[i][1:]) 
        
        # print(tree)
        i += 1
    
    cwd = deque()
    req_size = find_size(tree)
    print(counter)

    cwd = deque()
    req_size = 30000000 - (70000000 - req_size)
    print(req_size)
    find_min(tree, req_size)
    print(min_size)

    # result = ""
    # for stack in stacks:
    #     result += stack.pop()

def cd(dir):
    if dir == "..":
        cwd.pop()
    else:
        # print(f"cwd: {cwd[-1]}")
        # print(f"dir: {dir}")
        cwd.append(cwd[-1][dir])

def ls(ls):
    # print(f"ls: {ls}")
    for i in ls:
        i = i.split()
        if i[0] == "dir":
            dir = i[1]
            cwd[-1][dir] = {}
        else:
            size = int(i[0])
            file = i[1]
            cwd[-1][file] = size

def find_size(tree: dict):
    global counter
    cwd.append(tree)
    sum = 0
    for i in tree:
        e = tree[i]
        if isinstance(e, int):
            sum += e
        else:
            sum += find_size(tree[i])

    cwd.pop()
    if sum <= 100000: 
        counter += sum
    return sum

def find_min(tree: dict, req_size: int):
    global min_size
    cwd.append(tree)
    sum = 0
    for i in tree:
        e = tree[i]
        if isinstance(e, int):
            sum += e
        else:
            sum += find_min(tree[i], req_size)

    cwd.pop()
    if sum >= req_size:
        min_size = min(min_size, sum)
    return sum

if __name__ == "__main__":
    main()
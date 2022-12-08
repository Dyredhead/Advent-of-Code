from collections import deque

DIRECTIONS = {
    "up": [-1, 0], 
    "right": [0,1], 
    "down": [1,0],
    "left": [0,-1]
}

size = 100

def main() -> None:
    input = []
    with open("input8.txt", 'r') as f:
        input = f.read().strip().splitlines()
        input = [list(i) for i in input]
        input = [[int(j) for j in i] for i in input]

    result = 0
    for row in input:
        for col in row:
            if isVisible(row, col): result += 1

    print(result)

def isVisible(row, col):
    h = input[row][col]
    
    #up [-1, 0]
    for r in range(0, row, 1):
        if input[r][col] > h:
            break
    else: 
        return True

    #right [0,1]
    for c in range(size-1, col+1, -1):
        if input[row][c] > h:
            break
    else: 
        return True
    
    #down
    for r in range(size-1, row+1, -1):
        if input[r][col] > h:
            break
    else: 
        return True
    
    #left
    for c in range(0, col, 1):
        if input[row][c] > h:
            break
    else: 
        return True

    return False


if __name__ == "__main__":
    main()
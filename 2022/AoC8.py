from collections import deque

DIRECTIONS = {
    "up": [-1, 0], 
    "right": [0,1], 
    "down": [1,0],
    "left": [0,-1]
}

size = 5

def main():
    input = []
    with open("input8.txt", 'r') as f:
        input = f.read().strip().splitlines()
        input = [list(i) for i in input]
        input = [[int(j) for j in i] for i in input]

    result = 0
    for row, t in enumerate(input):
        for col, _ in enumerate(t):
            # if isVisible(input, row, col): 
            #     result += 1
            temp = scenic_score(input, row, col)
            print("temp: " + str(temp))
            if temp > result:
                result = temp

    # temp = scenic_score(input, 3, 2)
    # print("temp: " + str(temp))
    print(result)
    #print(isVisible(input, 2, 2))

def isVisible(input, row, col):
    h = input[row][col]
    
    #up
    for r in range(0, row, 1):
        if input[r][col] >= h:
            break
    else: 
        return True

    #right
    for c in range(size-1, col, -1):
        if input[row][c] >= h:
            break
    else: 
        return True
    
    #down
    for r in range(size-1, row, -1):
        if input[r][col] >= h:
            break
    else: 
        return True
    
    #left
    for c in range(0, col, 1):
        if input[row][c] >= h:
            break
    else: 
        return True

    return False

def scenic_score(input, row, col):
    h = input[row][col]
    
    up = 0
    for r in range(row, 0, -1):
        up+=1
        if input[r][col] >= h:
            break
    
    left = 0
    for c in range(col, 0, -1):
        left+=1
        if input[row][c] >= h:
            break

    down = 0
    for r in range(row, size, 1):
        down+=1
        if input[r][col] >= h:
            break
    
    right = 0
    for c in range(col, size, 1):
        right+=1
        if input[row][c] >= h:
            break
    
    print(str(up) + "*" + str(left) + "*" + str(down) + "*" + str(right))
    return up*right*down*left

if __name__ == "__main__":
    main()
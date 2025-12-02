

def main() -> None:
    size = 400
    n = 10
    grid = t = [['.']*size for i in range(size)]
    
    pos = {i: [size//2, size//2] for i in range(n)}
    nH = 0
    nT = n-1

    input = []
    with open("input9.txt", 'r') as f:
        input = f.read().strip().splitlines()
    
    result = 0
    for i in input:
        dir = i.split(' ')[0]
        dist = int(i.split(' ')[1])
        # print(i)
        for _ in range(dist):
            moveH(pos, nH, dir) # head

            # print(list(range(0, len(pos)-2)))
            for k in range(0, len(pos)-1): # knots 1 to n-2
                if pythagoras(pos[k], pos[k+1]) >= 2:
                    # print(k, k+1)
                    moveK(pos, k, k+1)

            if grid[pos[nT][0]][pos[nT][1]] == '.': # tail
                result += 1
                grid[pos[nT][0]][pos[nT][1]] = '#'       

        #print_grid(grid, pos)
        #print()

    print(result)



def moveH(pos, h, dir):
    if dir == 'U':
        dir = [-1, 0]
    elif dir == 'D':
        dir = [1, 0]
    elif dir == 'L':
        dir = [0, -1]
    elif dir == 'R':
        dir = [0, 1]
    
    pos[h][0] += dir[0]
    pos[h][1] += dir[1]
    return

def moveK(pos, h, t):
    # print(h, t)
    dist = pythagoras(pos[h], pos[t])
    if dist == 2:
        change = [(pos[h][0] - pos[t][0])//2, (pos[h][1] - pos[t][1])//2]
        pos[t] = [pos[t][0] + change[0], pos[t][1] + change[1]]
        return
    else:
        for i in range(-1, 2):
            for j in range(-1, 2):
                newPos = [pos[t][0] + i, pos[t][1] + j]
                if pythagoras(pos[h], newPos) == 1:
                    pos[t] = newPos
                    return
        
        for i in range(-1, 2):
            for j in range(-1, 2):
                newPos = [pos[t][0] + i, pos[t][1] + j]
                if pythagoras(pos[h], newPos) < 2:
                    pos[t] = newPos
                    return
        
        print("ERROR")

def pythagoras(posH, posT):
    return ((posH[0] - posT[0])**2 + (posH[1] - posT[1])**2)**0.5

def print_grid(grid, pos):
    # print(pos)
    for i in list(pos.keys())[::-1]:
        # print(i, pos[i])
        grid[pos[i][0]][pos[i][1]] = str(i)
   
    for i in grid:
        print(i)

    for i in pos:
        grid[pos[i][0]][pos[i][1]] = '.'

if __name__ == "__main__":
	main()
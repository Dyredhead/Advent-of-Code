import time
from copy import copy, deepcopy

def main() -> None:
    old = []
    with open("input25.txt", 'r') as f:
        old = f.read().strip()
        old = [[c for c in r] for r in old.split("\n")]
    
    # print_grid(old)
    result = 0

    new = step(old)
    result += 1
    while (new != old):
        result += 1
        old = new
        new = step(old)

    print(result)

def step(old):
    new = [['.']*len(old[i]) for i in range(len(old))]

    for r in range(len(old)): # east
        for c in range(len(old[r])):
            if old[r][c] == '>':
                if can_move(old, r, c, '>'):
                    new[r][(c+1)%len(old[r])] = '>'
                    new[r][c] = '.'
                else:
                    new[r][c] = '>'
    
    
    temp = [row[:] for row in new]
    for r in range(len(old)): # south
        for c in range(len(old[r])):
            if old[r][c] == 'v': 
                temp[r][c] = 'v'

    for r in range(len(old)): # south
        for c in range(len(old[r])):
            if temp[r][c] == 'v': 
                if can_move(temp, r, c, 'v'):
                    new[(r+1)%len(old)][c] = 'v'
                    new[r][c] = '.'            
                else:
                    new[r][c] = 'v'

    return new

def can_move(grid, r, c, dir):
    if dir == '>':
        return grid[r][(c+1)%len(grid[r])] == '.'
    if dir == 'v':
        return grid[(r+1)%len(grid)][c] == '.'

def print_grid(grid):
    for r in grid:
        print(r)

if __name__ == "__main__":
	main()
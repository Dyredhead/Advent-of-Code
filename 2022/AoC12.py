# breath first search
# maze like
from collections import deque
DIRECTIONS = {
    "up": [-1, 0, '^'],
    "right": [0, 1, '>'],
    "down": [1, 0, 'V'],
    "left": [0, -1, '<']
}
def main() -> None:
    input = []
    with open("/home/dyredhead/Projects/AoC/2022/input12.txt", 'r') as f:
        input = [list(i) for i in f.read().strip().splitlines()]
    
    # find coordinate of 'S'
    start = None
    for r, row in enumerate(input):
        for c, _ in enumerate(row):
            if row[c] == 'S':
                start = [r, c]
                break
    

    shortest_path = len(BFS(input, start))
    print(shortest_path)

def BFS(input: list[list], start: list[int]):
    queue: deque[list[list]] = deque()
    queue.append([start])
    visited = set()
    while len(queue) > 0:
        path = queue.pop()
        grid = print_grid(input, path)
        r = path[-1][0]
        c = path[-1][1]

        char = input[r][c]
        if char == 'S':
            char = 'a'
        
        if char == 'E':
            return path
        else:
            #loop over north south east west
            for d in DIRECTIONS.values():
                new_r = r + d[0]
                new_c = c + d[1]
                if in_bounds(input, new_r, new_c):
                    new_char = input[new_r][new_c]
                    if new_char == 'E':
                        new_char = 'z'
                    if ord(new_char) <= ord(char)+1:
                        if (r, c) not in visited:
                            new_path = path
                            new_path.append([new_r, new_c])
                            queue.append(new_path)
            visited.add((r, c))

def in_bounds(input, r, c):
    return r >= 0 and c >= 0 and r < len(input) and c < len(input[0])

def print_grid(input, path):
    grid = []
    for r, row in enumerate(input):
        grid.append([])
        for c, _ in enumerate(row):
            if [r, c] in path:
                grid[r].append('.')
                if [r, c] == path[-1]:
                    grid[r][-1] = 'X'
            else:
                grid[r].append(input[r][c])
    return grid

if __name__ == "__main__":
	main()
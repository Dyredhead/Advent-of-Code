def main() -> None:
    input = []
    with open("/home/dyredhead/Projects/AoC/2022/input13.txt", 'r') as f:
        input = f.read().strip().split("\n\n")
    input = [x.split("\n") for x in input]
    
    for i in range(len(input)):
        input[i][0] = parse(input[i][0][1:-1])
        input[i][1] = parse(input[i][1][1:-1])
    # print(input)

    result = 0
    for i in range(len(input)):
        p1 = input[i][0]
        p2 = input[i][1]
        i += 1
        if compare(p1, p2) > -1:
            print(i)
            result += i
    print(result)

def compare(p1, p2):
    for i in range(min(len(p1), len(p2))):
        left = p1[i]
        ltype = str(type(left))[8:-2]
        right = p2[i]
        rtype = str(type(right))[8:-2]

        if ltype == rtype:
            if ltype == "int":
                if left < right:
                    return 1
                elif left > right:
                    return -1
            elif ltype == "list":
                c = compare(left, right)
                if c != 0:
                    return c
        else:
            if ltype == "int":
                left = [left]
            elif rtype == "int":
                right = [right]

            c = compare(left, right)
            if c != 0:
                return c

    if len(p1) < len(p2):
        return 1
    elif len(p1) > len(p2):
        return -1
    else:
        return 0

def parse(s):
    l = []
    i = 0
    while i < len(s):
        if s[i] == '[':
            j = get_index(s, i)
            l.append(parse(s[i+1:j]))
            i = j
        else: 
            if '0'<= s[i] <= '9':
                l.append(int(s[i]))
        i+=1
    return l

# Python program to find index of closing
# bracket for a given opening bracket.
from collections import deque

def get_index(s, i):

	# If input is invalid.
	if s[i] != '[':
		return -1

	# Create a deque to use it as a stack.
	d = deque()

	# Traverse through all elements
	# starting from i.
	for k in range(i, len(s)):

		# Pop a starting bracket
		# for every closing bracket
		if s[k] == ']':
			d.popleft()

		# Push all starting brackets
		elif s[k] == '[':
			d.append(s[i])

		# If deque becomes empty
		if not d:
			return k

	return -1

if __name__ == "__main__":
    main()
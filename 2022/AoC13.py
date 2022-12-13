def main() -> None:
    input = []
    with open("input10.txt", 'r') as f:
        input = f.read().strip().splitlines()

    for i in range(len(input)):
        input[i] = input[i].strip()
        if input[i] != "":
            input[i] = convert(input[i])

def convert(s):
    l = []
    for e in l:
        for i in range(len(s)):
            if s[i] == '[':
                for j in range(len(s[::-1])):
                    if s[j] == ']':
                        l.append([])
                        s = s[i+1:j] 

if __name__ == "__main__":
main()
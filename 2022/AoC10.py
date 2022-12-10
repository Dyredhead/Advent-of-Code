def main() -> None:
    input = []
    with open("input10.txt", 'r') as f:
        input = f.read().strip().splitlines()
    
    result = 0
    register = 1
    cycle = -1

    crt = ['.']*40*6
    for i in input:
        cycle += 1
        if register-1 <= cycle%40 and cycle%40 <= register+1:
            crt[cycle] = '#'
        if i == "noop":
            if cycle in [20, 60, 100, 140, 180, 220] :
                result += register*cycle
                print(cycle, cycle*register)
        else:
            if cycle in [20, 60, 100, 140, 180, 220] :
                result += register*cycle
                print(cycle, cycle*register)

            cycle += 1
            if register-1 <= cycle%40 and cycle%40 <= register+1:
                crt[cycle] = '#'
            if cycle in [20, 60, 100, 140, 180, 220] :
                result += register*cycle
                print(cycle, cycle*register)
            register += int(i.split(" ")[1])
            

    print(result)
    crt = [
        crt[0:40],
        crt[40:80],
        crt[80:120],
        crt[120:160],
        crt[160:200],
        crt[200:240],
    ]

    for i in crt:
        print("".join(i))

if __name__ == "__main__":
	main()
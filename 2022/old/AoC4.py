def main() -> None:
    input = []
    with open("input4.txt", 'r') as f:
        input = f.read().strip().splitlines()
    
    result = 0
    for i in input:
        elf1 = i.split(",")[0]
        elf1min = int(elf1.split("-")[0])
        elf1max = int(elf1.split("-")[1])
        
        elf2 = i.split(",")[1]
        elf2min = int(elf2.split("-")[0])
        elf2max = int(elf2.split("-")[1])

        # if elf1min <= elf2min and elf1max >= elf2min:
        #     result+=1
        # elif elf2min <= elf1min or elf2max >= elf1max:
        #     result+=1

        if elf1min <= elf2max and elf1max >= elf2min:
            result+=1
        elif elf2min <= elf1max and elf2max >= elf1min:
            result+=1
            
    print(result)

if __name__ == "__main__":
	main()
from math import floor
import sys

def main() -> None:
    sys.set_int_max_str_digits(0)
    e = 10
    # sys.set_int_max_str_digits(640)
    input = []
    with open("input11.txt", 'r') as f:
        input = f.read().strip().split("Monkey")
    input = [i.split("\n") for i in input]
    input.pop(0)

    monkeys = []
    for m in input:
        monkey = {}
        monkey["operation"] = m[2].strip()[21:].split(" ")
        monkey["test"] = int(m[3].strip()[19:])

        monkey["items"] = [[int(i)%monkey["test"], monkey["test"]] for i in m[1].strip()[16:].split(", ")]

        monkey["true"] = int(m[4].strip()[25:])
        monkey["false"] = int(m[5].strip()[26:])
        monkey["inspected"] = 0
        monkeys.append(monkey)

    #print_monkeys(monkeys)
    
    for round in range(20):
        print("round:", round+1)
        for monkey in monkeys:
            for i in range(len(monkey["items"])):
                monkey["inspected"] += 1
                d = monkey["test"]
                diff = (monkey["items"][i][1] - d)%d

                n = monkey["operation"][1]
                if n == "old":
                    n = monkey["items"][i][0]
                else:
                    n = int(n)
                
                mod = -1
                if monkey["operation"][0] == "+":
                    mod = (diff + n%d) % d
                elif monkey["operation"][0] == "*":
                    mod = (diff * n%d) % d

                if (mod == 0):
                    n = monkey["true"]
                else:
                    n = monkey["false"]

                monkey["items"][i][0] = mod
                monkey["items"][i][1] = d
                

                monkeys[n]["items"].append(monkey["items"][i])

                monkey["items"][i] = -1
            
            monkey["items"] = [i for i in monkey["items"] if i != -1]
    
        print_monkeys(monkeys)
    
    moneky1 = 0
    inspected1 = 0
    # find the two monkeys with the most inspected items
    for m in range(len(monkeys)):
        if monkeys[m]["inspected"] > inspected1:
            inspected1 = monkeys[m]["inspected"]
            monkey1 = m
    
    monkeys.remove(monkeys[monkey1])

    monkey2 = 0
    inspected2 = 0
    # find the two monkeys with the most inspected items
    for m in range(len(monkeys)):
        if monkeys[m]["inspected"] > inspected2:
            inspected2 = monkeys[m]["inspected"]
            monkey2 = m
    
    monkeys.remove(monkeys[monkey2])
    
    print(inspected1*inspected2)

def print_monkey(monkeys, m):
    print("m:", m)
    m = monkeys[m]
    print("items:", m["items"])
    # print("operation:", m["operation"])
    # print("test:", m["test"])
    # print("true:", m["true"])
    # print("false:", m["false"])
    print("inspected:", m["inspected"])
    print()

def print_monkeys(monkeys):
    for m in range(len(monkeys)):
        print_monkey(monkeys, m)

if __name__ == "__main__":
	main()
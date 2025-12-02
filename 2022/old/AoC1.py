def main() -> None:
	temp = []
	with open("input.txt", "r") as f:
		temp = f.read().strip().split("\n\n")

	temp2 = []
	for i in temp:
		temp2.append(i.split("\n"))
	
	temp3 = []
	for i in temp2:
		temp3.append(sum(i))

	top1 = max(temp3)
	temp3.remove(top1)
	top2 = max(temp3)
	temp3.remove(top2)
	top3 = max(temp3)
	temp3.remove(top3)

	print(top1+top2+top3)


def sum(input):
	sum = 0
	for i in input:
		sum += int(i)
	return sum

if __name__ == "__main__":
	main()

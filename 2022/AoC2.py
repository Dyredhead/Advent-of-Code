ROCK_SCORE = 1
ROCK = 'AX'

PAPER_SCORE = 2
PAPER = 'BY'

SCISSORS_SCORE = 3
SCISSORS = 'CZ'

LOSS = 0
LOSS_LETTER = 'X'
LOSS_SCORE = 0

DRAW = 1
DRAW_LETTER = 'Y'
DRAW_SCORE = 3

WIN = 2
WIN_LETTER = 'Z'
WIN_SCORE = 6

opp_table = {
	ROCK: [SCISSORS, ROCK, PAPER],
	PAPER: [ROCK, PAPER, SCISSORS],
	SCISSORS: [PAPER, SCISSORS, ROCK]
}

def simplify(play):
	if play in ROCK: return ROCK
	if play in PAPER: return PAPER
	if play in SCISSORS: return SCISSORS

def battle(opp, you):
	score = outcome(opp, you)

	if simplify(you) == ROCK: score += ROCK_SCORE
	if simplify(you) == PAPER: score += PAPER_SCORE
	if simplify(you) == SCISSORS: score += SCISSORS_SCORE

	return score

def choose(opp, you):
	if you == LOSS_LETTER: you = opp_table[simplify(opp)][LOSS]
	if you == DRAW_LETTER: you = opp_table[simplify(opp)][DRAW]
	if you == WIN_LETTER: you = opp_table[simplify(opp)][WIN]

	return battle(opp, you)

def outcome(opp, you):
	i = opp_table[simplify(opp)].index(simplify(you))
	if i == 0: return LOSS_SCORE
	if i == 1: return DRAW_SCORE
	if i == 2: return WIN_SCORE


def main() -> None:
	temp = []
	with open("input2.txt", "r") as f:
		temp = f.read().strip().split("\n")

	score = 0
	for i in temp:
		i = i.split(" ")
		opp = i[0]; you = i[1]
		#score += battle(opp, you)
		score += choose(opp, you)
	print(score)

if __name__ == "__main__":
	main()

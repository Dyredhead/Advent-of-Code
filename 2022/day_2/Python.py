ROCK_SCORE = 1
ROCK_LETTER_YOU = 'X'
ROCK_LETTER_OPP = 'A'

PAPER_SCORE = 2
PAPER_LETTER_YOU = 'Y'
PAPER_LETTER_OPP = 'B'

SCISSORS_SCORE = 3
SCISSORS_LETTER_YOU = 'Z'
SCISSORS_LETTER_OPP = 'C'

LOSS_SCORE = 0
DRAW_SCORE = 3
WIN_SCORE = 6


def battle(you, opp):
		if you == ROCK_LETTER_YOU: return rock(opp) 
		if you == PAPER_LETTER_YOU: return paper(opp) 
		if you == SCISSORS_LETTER_YOU: return scissors(opp) 

def rock(opp):
		if opp == PAPER_LETTER_OPP: return LOSS_SCORE
		if opp == ROCK_LETTER_OPP: return DRAW_SCORE
		if opp == SCISSORS_LETTER_OPP: return WIN_SCORE

def paper(opp):
		if opp == SCISSORS_LETTER_OPP: return LOSS_SCORE
		if opp == PAPER_LETTER_OPP: return DRAW_SCORE
		if opp == ROCK_LETTER_OPP: return WIN_SCORE

def scissors(opp):
		if opp == ROCK_LETTER_OPP: return LOSS_SCORE
		if opp == SCISSORS_LETTER_OPP: return DRAW_SCORE
		if opp == PAPER_LETTER_OPP: return WIN_SCORE


def main() -> None:
	temp = []
	with open("input.txt", "r") as f:
		temp = f.read().strip().split("\n")

	score = 0
	for i in temp:
		return


if __name__ == "__main__":
	main()

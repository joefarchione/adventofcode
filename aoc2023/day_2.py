from functools import reduce
import re

# part one
re_draw = re.compile(r"(?:(\d+) ((:?green|red|blue)))")
bag = dict(red=12, green=13, blue=14)

with open("./data/day_2.txt") as fin:
    check = lambda game: all(map(lambda match: int(match[1]) <= bag[match[2]], re_draw.finditer(game)))
    counter = sum(map(lambda ii_game: ii_game[0] * check(ii_game[1]), enumerate(fin, 1)))
print(counter)

# part two
power = 0
with open("./data/day_2.txt") as fin:
    for line in fin:
        game = map(lambda match: (int(match[1]), match[2]), re_draw.finditer(line))
        max_draw = dict(red=0, green=0.0, blue=0.0)
        for number, color in game:
            max_draw[color] = max(max_draw[color], number)
        power += reduce(lambda x, y: x * y, max_draw.values())

print(power)

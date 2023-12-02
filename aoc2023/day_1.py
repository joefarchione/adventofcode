from functools import reduce

# part one
with open("./data/day_1.txt") as fin:
    counter = 0
    for line in fin:
        digits = (int(c) for c in line if c.isdigit())
        first = last = next(digits)
        for last in digits:
            pass
        counter += first * 10 + last
counter

# part two
numbers = (
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
)
counter = 0.0
with open("./data/day_1.txt") as fin:
    counter = 0
    for line in fin:
        numstrs = []
        for ii, c in enumerate(line):
            if line[ii].isdigit():
                numstrs.append(int(line[ii]))
            else:
                for jj, number in enumerate(numbers, 1):
                    if line[ii : ii + len(number)] == number:
                        numstrs.append(jj)
                        break
        counter += 10 * numstrs[0] + numstrs[-1]
print(counter)

# part one
with open("./data/day_1.txt") as fin:
    counter = 0
    for line in fin:
        digits = (int(c) for c in line if c.isdigit())
        first = last = next(digits)
        for last in digits:
            pass
        counter += first * 10 + last
print(counter)

# part two
number_words = (
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
        numbers = []
        for ii, c in enumerate(line):
            if line[ii].isdigit():
                numbers.append(int(line[ii]))
            else:
                for jj, number in enumerate(number_words, 1):
                    if line[ii : ii + len(number)] == number:
                        numbers.append(jj)
                        break
        counter += 10 * numbers[0] + numbers[-1]
print(counter)

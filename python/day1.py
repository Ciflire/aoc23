import sys
import re

def part1 (fileName: str) -> int:
    file = open(fileName, 'r')
    file = file.readlines()
    res = 0
    for i in range(len(file)):
        file[i] = file[i].replace("\n", "")
        numbers = re.findall(r"\d", file[i])
        res += int(numbers[0] + numbers[-1])
    return res

def part2 (fileName: str) -> int:
    file = open(fileName, 'r')
    file = file.readlines()
    res = 0
    for i in range(len(file)):
        file[i] = file[i].replace("\n","")
        number = re.findall(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))",file[i])
        firstOccurence = number[0] if len(number[0])==1 else table[number[0]]
        lastOccurence = number[-1] if len(number[-1])==1 else table[number[-1]] # if digit then keep else convert with table
        res += int(firstOccurence + lastOccurence) 
        # print(number, firstOccurence, lastOccurence, res)
    return res


if __name__ == "__main__":
    table = {"one":"1", "two":"2", "three":"3", "four":"4", "five":"5", "six":"6", "seven":"7", "eight":"8", "nine":"9"}
    print(part1("input.txt"))
    print(part2("input.txt"))

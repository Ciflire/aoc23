import re

def parser(file: str) -> list[dict[str,int]]:
    f = open(file, 'r')
    f = f.readlines()
    # print(f)
    for elt in f:
        elt = elt.replace("\n","")
        print(elt)

if __name__ == "__main__":
    parser("input.txt")

import operator
from pathlib import Path

TEST_INPUT = """root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"""


def main():
    print(f"{part_1(Path('input.txt').read_text())}")
    print(f"{part_2(Path('input.txt').read_text())}")


def test_part_1():
    assert part_1(TEST_INPUT) == 152


def test_part_2():
    assert part_2(TEST_INPUT) == 301


operations = {"+": operator.add,
              "-": operator.sub,
              "*": operator.mul,
              "/": operator.truediv,
              "=": operator.eq
              }


def part_1(input_string):
    functions = {}
    for line in input_string.splitlines():
        name, contents = line.split(": ")
        if contents.isdigit():
            functions[name] = lambda n=int(contents): n
        else:
            val_1, operation, val_2 = contents.split(" ")
            functions[name] = lambda v1=val_1, op=operation, v2=val_2: operations[op](functions[v1](), functions[v2]())
    return functions["root"]()


def part_2(input_string):
    functions = {}
    humn = 3560324800000
    for line in input_string.splitlines():
        name, contents = line.split(": ")
        if contents.isdigit():
            functions[name] = lambda n=int(contents): n
        else:
            val_1, operation, val_2 = contents.split(" ")
            if name == "root":
                operation = "="
            functions[name] = lambda v1=val_1, op=operation, v2=val_2: operations[op](functions[v1](), functions[v2]())
    functions["humn"] = lambda: humn
    while not functions["root"]():
        humn += 1
        print(humn, functions["lzfc"]() - functions["qrgn"]())
    return humn


if __name__ == "__main__":
    main()

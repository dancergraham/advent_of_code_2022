from itertools import pairwise
from pathlib import Path

TEST_INPUT = """498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"""


def main():
    print(f"{part_1(Path('input.txt').read_text())}")
    print(f"{part_2(Path('input.txt').read_text())}")


def test_part_1():
    assert part_1(TEST_INPUT) == 24


def test_part_2():
    assert part_2(TEST_INPUT) == 93


def part_1(input_str):
    def drop(sand):
        while True:
            if sand.imag > 500:
                return sand
            if sand + 1j not in filled:
                sand += 1j
            elif sand + (-1 + 1j) not in filled:
                sand += (-1 + 1j)
            elif sand + (1 + 1j) not in filled:
                sand += (1 + 1j)
            else:
                return sand

    filled = parse_input(input_str)
    blocks = len(filled)
    while True:
        sand = complex(500, 0)
        sand = drop(sand)
        if sand.imag > 500:
            return len(filled) - blocks
        filled.add(sand)


def part_2(input_str):
    def drop(sand, max_y):
        while True:
            if sand.imag == max_y:
                return sand
            if sand + 1j not in filled:
                sand += 1j
            elif sand + (-1 + 1j) not in filled:
                sand += (-1 + 1j)
            elif sand + (1 + 1j) not in filled:
                sand += (1 + 1j)
            else:
                return sand

    filled = parse_input(input_str)
    max_y = max(block.imag for block in filled) + 1
    blocks = len(filled)
    while True:
        sand = complex(500, 0)
        sand = drop(sand, max_y)
        if sand == complex(500, 0):
            filled.add(sand)
            return len(filled) - blocks
        filled.add(sand)


def parse_input(input_str):
    filled = set()
    for line in input_str.splitlines():
        pairs = pairwise(line.split(" -> "))
        for (start, end) in pairs:
            start = complex(*[int(v) for v in start.split(",")])
            end = complex(*[int(v) for v in end.split(",")])
            step = (end - start) / abs(end - start)
            for i in range(int(abs(end - start))):
                filled.add(start + i * step)
            filled.add(end)
    return filled


if __name__ == "__main__":
    main()

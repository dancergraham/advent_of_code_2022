from collections import Counter
from copy import deepcopy
from pathlib import Path

TEST_INPUT = """....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."""


def plot(elves):
    positions = {elf.position for elf in elves}
    elf = "#"
    blank = "."
    x_min = int(min(elf.x for elf in elves))
    x_max = int(max(elf.x for elf in elves))
    y_min = int(min(elf.y for elf in elves))
    y_max = int(max(elf.y for elf in elves))
    for row in range(y_min, y_max + 1):
        for col in range(x_min, x_max + 1):
            print(elf if complex(col, row) in positions else blank, end="")
        print()


class Elf:
    def __init__(self, x, y):
        self.position = complex(x, y)  # from 0, 0 at top left
        self.proposed_position = None

    @property
    def x(self):
        return self.position.real

    @property
    def y(self):
        return self.position.imag

    def clear(self):
        self.proposed_position = None


def main():
    print(f"{part_1(Path('input.txt').read_text())}")
    print(f"{part_2(Path('input.txt').read_text())}") # 936 too low


def test_part_1():
    assert part_1(TEST_INPUT) == 110


def test_part_2():
    assert part_2(TEST_INPUT) == 20


def main():
    print(f"{part_1(Path('input.txt').read_text())}")
    print(f"{part_2(Path('input.txt').read_text())}")  # 1826 too low


def area(elves):
    return int((1 + max(elf.x for elf in elves) - min(elf.x for elf in elves)) *
               (1 + max(elf.y for elf in elves) - min(elf.y for elf in elves)))

vectors = [complex(1, 0),
           complex(1, 1),
           complex(0, 1),
           complex(-1, 1),
           complex(-1, 0),
           complex(-1, -1),
           complex(0, -1),
           complex(1, -1)
           ]
def no_adjacent(position, positions):
    for vector in vectors:
        if position + vector in positions:
            return False
    return True

directions = [("N", [complex(0, -1),
                     complex(-1, -1),
                     complex(1, -1),
                     ]),
              ("S", [complex(0, 1),
                     complex(-1, 1),
                     complex(1, 1),
                     ]),
              ("W", [complex(-1, 0),
                     complex(-1, -1),
                     complex(-1, 1),
                     ]),
              ("E", [complex(1, 0),
                     complex(1, -1),
                     complex(1, 1),
                     ])]
def part_1(input_str):
    elves = set()
    for j, line in enumerate(input_str.splitlines()):
        for i, symbol in enumerate(line):
            if symbol == "#":
                elves.add(Elf(i, j))

    for _ in range(10):
        proposal_counter = Counter()
        elf_positions = {elf.position for elf in elves}
        for elf in elves:
            p = elf.position
            if no_adjacent(p, elf_positions):
                continue
            for direction in directions:
                if all(p + d not in elf_positions for d in direction[1]):
                    elf.proposed_position = p + direction[1][0]
                    proposal_counter.update([elf.proposed_position])
                    break

        for elf in elves:
            if proposal_counter[elf.proposed_position] == 1:
                elf.position = elf.proposed_position
        first_direction = directions.pop(0)
        directions.append(first_direction)
        [elf.clear() for elf in elves]
        print(f"== End of Round {_ + 1} ==", area(elves))
        plot(elves)
    return area(elves) - len(elves)


def part_2(input_str):
    elves = set()
    for j, line in enumerate(input_str.splitlines()):
        for i, symbol in enumerate(line):
            if symbol == "#":
                elves.add(Elf(i, j))

    previous_positions = set()
    elf_positions = {elf.position for elf in elves}
    round = 0
    while previous_positions != {elf.position for elf in elves}:
        round += 1
        proposal_counter = Counter()
        previous_positions = deepcopy(elf_positions)
        elf_positions = {elf.position for elf in elves}
        for elf in elves:
            p = elf.position
            if no_adjacent(p, elf_positions):
                continue
            for direction in directions:
                if all(p + d not in elf_positions for d in direction[1]):
                    elf.proposed_position = p + direction[1][0]
                    proposal_counter.update([elf.proposed_position])
                    break

        for elf in elves:
            if proposal_counter[elf.proposed_position] == 1:
                elf.position = elf.proposed_position
        first_direction = directions.pop(0)
        directions.append(first_direction)
        [elf.clear() for elf in elves]
    return round

if __name__ == "__main__":
    main()

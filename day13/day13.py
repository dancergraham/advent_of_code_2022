"""
--- Day 13: Distress Signal ---
"""
from itertools import zip_longest
from pathlib import Path

TEST_INPUT = """[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"""


def main():
    print(f"{part_1(Path('input.txt').read_text())}")
    print(f"{part_2(Path('input.txt').read_text())}")


def test_part_1():
    assert part_1(TEST_INPUT) == 13


def test_part_2():
    assert part_2(TEST_INPUT) == 140


def compare(left, right):
    """is left smaller than right ?"""
    for l, r in zip_longest(left, right):

        # handle exhausted lists
        if l is None:
            return True
        elif r is None:
            return False

        # handle unbalanced inputs
        if isinstance(l, int) and isinstance(r, list):
            l = [l]
        if isinstance(r, int) and isinstance(l, list):
            r = [r]

        # run comparison
        if isinstance(l, int) and isinstance(r, int) and l != r:
            return l < r
        elif isinstance(l, list) and isinstance(r, list):
            result = compare(l, r)
            if result is not None:
                return result


def part_1(input_str):
    index_sum = 0
    for index, pair in enumerate(input_str.split("\n\n"), 1):
        left, right = map(eval, pair.splitlines())
        if compare(left, right):
            index_sum += index
    return index_sum


class Compareval(list):
    def __init__(self, value):
        self.extend(eval(value))

    def __lt__(self, other):
        return compare(self, other)


def part_2(input_str):
    divider_0 = Compareval("[[2]]")
    divider_1 = Compareval("[[6]]")
    values = [divider_0, divider_1]
    values.extend(Compareval(line) for line in input_str.replace("\n\n", "\n").splitlines())
    values.sort()
    return (values.index(divider_0) + 1) * (values.index(divider_1) + 1)


if __name__ == "__main__":
    main()

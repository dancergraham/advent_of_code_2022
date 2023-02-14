TEST_INPUT = """30373
25512
65332
33549
35390"""


def main():
    print(f"{part_1(Path('input.txt').read_text())}")
    print(f"{part_2(Path('input.txt').read_text())}")


def test_part_1():
    assert part_1(TEST_INPUT) == 21


def get_nested_value(grid, row, column, default=0):
    dim_max = len(grid)
    if min(row, column) < 0 or max(row, column) >= dim_max:
        return 0
    else:
        return grid[row][column]


def part_1(input_str):
    grid = [[int(x) for x in line] for line in input_str.splitlines()]
    visibility = [[100 for _ in line] for line in input_str.splitlines()]
    for dx, dy in [(-1, 0),
                   (0, -1),
                   (1, 0),
                   (0, 1),
                   ]:
        for y, row in enumerate(grid):
            for x, value in enumerate(row):
                visibility[y][x] = min(visibility[y][x],
                                       max(
                                           get_nested_value(grid,
                                                            y + dy,
                                                            x + dx,
                                                            ),
                                           get_nested_value(visibility,
                                                            y + dy,
                                                            x + dx,
                                                            ))
                                       )
    print(visibility)


if __name__ == "__main__":
    main()

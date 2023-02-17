from pathlib import Path

TEST_INPUT = """30373
25512
65332
33549
35390"""


def main():
    print(f"{part_1(Path('input.txt').read_text())}")
    print(f"{part_2(Path('input.txt').read_text())}")


def showgrid(grid):
    for row in grid:
        print(" ".join([str(x) for x in row]))


def test_part_1():
    assert part_1(TEST_INPUT) == 21


def part_1(input_str):
    grid = [[int(x) for x in line] for line in input_str.splitlines()]
    visibility = [[-1 for _ in line] for line in input_str.splitlines()]

    # sweep down and right
    for y, row in enumerate(grid[1:-1], 1):
        for x, value in enumerate(row[1:-1], 1):
            west_tree_height = max([grid[y][i] for i in range(0, x)])
            north_tree_height = max([grid[i][x] for i in range(0, y)])
            east_tree_height = max([grid[y][i] for i in range(len(grid) - 1, x, -1)])
            south_tree_height = max([grid[i][x] for i in range(len(grid) - 1, y, -1)])
            visibility[y][x] = min(west_tree_height,
                                   north_tree_height,
                                   east_tree_height,
                                   south_tree_height
                                   )
    is_visible = [[height > vis for height, vis in zip(heights, vises)] for heights, vises in zip(grid, visibility)]
    return sum(sum(row) for row in is_visible)


if __name__ == "__main__":
    main()

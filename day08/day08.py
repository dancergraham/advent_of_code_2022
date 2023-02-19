from pathlib import Path

TEST_INPUT = """30373
25512
65332
33549
35390"""


def main():
    pt_1, pt_2 = part_1_and_2(Path('input.txt').read_text())
    print(f"{pt_1}")
    print(f"{pt_2}")


def showgrid(grid):
    """Plot a nested grid of values"""
    for row in grid:
        print(" ".join([str(x) for x in row]))


def test_part_1():
    pt_1, pt_2 = part_1_and_2(TEST_INPUT)
    assert pt_1 == 21
    assert pt_2 == 8


def part_1_and_2(input_str):
    grid = [[int(x) for x in line] for line in input_str.splitlines()]
    visibility = [[-1 for _ in line] for line in input_str.splitlines()]

    max_scenic_score = 0

    # sweep down and right
    for y, row in enumerate(grid[1:-1], 1):
        for x, height in enumerate(row[1:-1], 1):
            west_tree_heights = [grid[y][i] for i in range(0, x)]
            north_tree_heights = [grid[i][x] for i in range(0, y)]
            east_tree_heights = [grid[y][i] for i in range(len(grid) - 1, x, -1)]
            south_tree_heights = [grid[i][x] for i in range(len(grid) - 1, y, -1)]
            visibility[y][x] = min(max(west_tree_heights),
                                   max(north_tree_heights),
                                   max(east_tree_heights),
                                   max(south_tree_heights)
                                   )
            scenic_score = 1
            for direction in [west_tree_heights,
                              north_tree_heights,
                              east_tree_heights,
                              south_tree_heights
                              ]:
                distance = 0
                for tree in direction[::-1]:
                    distance += 1
                    if tree >= height:
                        break
                scenic_score *= distance
            max_scenic_score = max(max_scenic_score, scenic_score)

    is_visible = [[height > vis for height, vis in zip(heights, vises)]
                  for heights, vises in zip(grid, visibility)
                  ]

    return sum(sum(row) for row in is_visible), max_scenic_score


if __name__ == "__main__":
    main()

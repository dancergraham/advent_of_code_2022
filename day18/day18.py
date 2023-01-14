from collections import defaultdict, namedtuple
from pathlib import Path

TEST_INPUT = """2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"""


def main():
    print(f"{part_1(Path('input.txt').read_text())}")
    print(f"{part_2(Path('input.txt').read_text())}")  # 1826 too low


def test_part_1():
    assert part_1(TEST_INPUT) == 64


def test_part_2():
    assert part_2(TEST_INPUT) == 58  # 3252 too high


Cube = namedtuple("Cube", ["x", "y", "z"])


class Range:
    def __init__(self, start, end):
        self.start = start
        self.end = end

    def __add__(self, value):
        self.start = min(self.start, value)
        self.end = max(self.end, value)
        return self

    def contains(self, value):
        return self.start <= value <= self.end


class CartesianHull:
    def __init__(self):
        column_range = defaultdict(lambda: Range(start=1e6, end=-1e6))
        self.x_ranges = column_range
        self.y_ranges = column_range
        self.z_ranges = column_range

    def __add__(self, cube):
        x, y, z = cube.x, cube.y, cube.z
        self.x_ranges[(y, z)] += x
        self.y_ranges[(x, z)] += y
        self.z_ranges[(x, y)] += z
        return self

    def fully_contains(self, cube):
        x, y, z = cube.x, cube.y, cube.z
        return all([self.x_ranges.get((y, z), Range(start=1e6, end=1e6)).contains(x),
                   self.y_ranges.get((x, z), Range(start=1e6, end=1e6)).contains(y),
                   self.z_ranges.get((x, y), Range(start=1e6, end=1e6)).contains(z)])


def is_adjacent(cube, other_cube):
    return (abs(cube.x - other_cube.x) +
            abs(cube.y - other_cube.y) +
            abs(cube.z - other_cube.z)
            ) == 1


def is_isolated(cube, cubes):
    return all(c in cubes for c in (Cube(cube.x + 1, cube.y, cube.z),
                                    Cube(cube.x - 1, cube.y, cube.z),
                                    Cube(cube.x, cube.y + 1, cube.z),
                                    Cube(cube.x, cube.y - 1, cube.z),
                                    Cube(cube.x, cube.y, cube.z + 1),
                                    Cube(cube.x, cube.y, cube.z - 1),
                                    )
               )


def part_1(input_str):
    cubes = []
    for line in input_str.splitlines():
        cubes.append(Cube(*map(int, line.split(","))))
    cubes.sort()
    faces = 0
    for cube in cubes:
        cube_faces = 6
        for other_cube in cubes:
            if other_cube == cube:
                pass
            if is_adjacent(cube, other_cube):
                cube_faces -= 1
        faces += cube_faces

    return faces


def part_2(input_str):
    cubes = []
    hull = CartesianHull()
    for line in input_str.splitlines():
        cube = Cube(*map(int, line.split(",")))
        cubes.append(cube)
        hull += cube
    x_min, x_max = min(c.x for c in cubes), max(c.x for c in cubes)
    y_min, y_max = min(c.y for c in cubes), max(c.y for c in cubes)
    z_min, z_max = min(c.z for c in cubes), max(c.z for c in cubes)
    for i in range(x_min + 1, x_max):
        for j in range(y_min + 1, y_max):
            for k in range(z_min + 1, z_max):
                cube = Cube(i, j, k)
                if cube not in cubes and hull.fully_contains(cube):
                    cubes.append(cube)
    cubes.sort()
    faces = 0
    for cube in cubes:
        cube_faces = 6
        for other_cube in cubes:
            if other_cube == cube:
                pass
            if is_adjacent(cube, other_cube):
                cube_faces -= 1
        faces += cube_faces
    return faces


if __name__ == "__main__":
    main()

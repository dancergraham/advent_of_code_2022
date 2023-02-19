from copy import copy
from pathlib import Path


def main():
    test_input = """$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"""


def part_1_and_2(input_string):
    answers = []
    dirs = []

    class FilePath:
        def __init__(self, name, size=0, is_dir=False, parent=None):
            self.name = name
            self.contents = {}
            self.size = int(size)
            self.is_dir = is_dir
            self.parent = parent

        def get_size(self):
            if not self.is_dir:
                return self.size
            else:
                size = sum(item.get_size() for item in self.contents.values())
                dirs.append(size)
                if size <= 100000:
                    answers.append(size)
                return size

    for line in input_string.splitlines():
        match line.split():
            case ['$', 'cd', r'/']:
                root = FilePath("/", is_dir=True)
                filepath = copy(root)
            case ["$", *commands]:
                match commands:
                    case ["cd", ".."]:
                        filepath = filepath.parent
                    case ["cd", dir_name]:
                        filepath = filepath.contents[dir_name]
            case paths:
                match paths:
                    case ["dir", dir_name]:
                        filepath.contents[dir_name] = FilePath(dir_name, is_dir=True, parent=filepath)
                    case [file_size, file_name]:
                        filepath.contents[file_name] = FilePath(file_name, size=file_size, parent=filepath)
    root.get_size()

    return sum(answers), min(int(x) for x in dirs if int(x) > 4376732)


if __name__ == "__main__":
    print(part_1_and_2(input_string=Path("input.txt").read_text()))

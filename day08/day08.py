def main():
    test_input = """30373
25512
65332
33549
35390"""
    print([[int(x) for x in iter(line)]
           for line in test_input.splitlines()]
          )



if __name__ == "__main__":
    main()

import random
import sys


def main():
    args: str = sys.argv
    if len(args) != 2:
        print(f"Usage: {args[0]} <spaces>")
        exit(1)

    spaces: int = int(args[1])
    if spaces < 2:
        print("Can't generate input under 2 spaces")
        exit(1)

    input_filepath: str = "input-" + str(spaces)

    print("Generating input...")
    input_list: str = generate_input(spaces)

    print(f"Writing input to \"{input_filepath}\"...")
    with open(input_filepath, "w") as input_file:
        input_file.write(input_list)


def generate_input(spaces: int) -> str:
    empty: str = "X"
    separator: str = "-"
    words: str = [
        "hello",
        "world",
        "empty",
        "space",
        "sorting",
        "algorithm",
        "by",
        "ratin",
    ]

    input_list: str = []

    for space in range(spaces):
        if random.randint(0, 1) == 0:
            input_list.append(empty)
        else:
            input_list.append(random.choice(words))

    return separator.join(input_list)


if __name__ == "__main__":
    main()

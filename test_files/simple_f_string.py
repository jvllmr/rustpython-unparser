world = "World"
empty_f_string = f""  # noqa: F541
f"{world!a:}None{empty_f_string!s:}None"
answer = 42.000001
f"{answer:.03f}"
if __name__ == "__main__":
    print(f"Hello {world}!")

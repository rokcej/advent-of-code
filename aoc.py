# Script for managing Advent of Code solutions
# Run with parameter -h to display more info
# To add support for new languages, extend LanguageConfig

import argparse
from pathlib import Path
import requests
import subprocess
import time


#############################
## Language configurations ##
#############################


class LanguageConfig:
    def get_file_extension() -> str:
        raise NotImplementedError()

    def get_source_code(year: int, day: int, part: int) -> str:
        raise NotImplementedError()

    def get_working_dir(year: int, day: int, part: int) -> str:
        raise NotImplementedError()

    def get_compile_command(year: int, day: int, part: int) -> list[str]:
        raise NotImplementedError()

    def get_execute_command(year: int, day: int, part: int) -> list[str]:
        raise NotImplementedError()


# C++
class CppConfig(LanguageConfig):
    def get_file_extension() -> str:
        return "cpp"

    def get_source_code(year: int, day: int, part: int) -> str:
        return "int main() {\n    // TODO\n    return 0;\n}\n"

    def get_working_dir(year: int, day: int, part: int) -> str:
        return f"./{year}/day{day:02}"

    def get_compile_command(year: int, day: int, part: int) -> list[str]:
        return ["g++", "-std=c++17", "-O2", "-o", f"part{part}", f"part{part}.cpp"]

    def get_execute_command(year: int, day: int, part: int) -> list[str]:
        return [f"./part{part}"]


# C#
class CsConfig(LanguageConfig):
    def get_file_extension() -> str:
        return "cs"

    def get_source_code(year: int, day: int, part: int) -> str:
        return f"namespace Day{day:02};\n\nclass Part{part} {{\n\n    public static void Run() {{\n        // TODO\n    }}\n\n}}\n"

    def get_working_dir(year: int, day: int, part: int) -> str:
        return f"./{year}"

    def get_compile_command(year: int, day: int, part: int) -> list[str]:
        return ["dotnet", "build", "-o", "./bin/build", "--nologo", "-v", "q", "-clp:NoSummary"]

    def get_execute_command(year: int, day: int, part: int) -> list[str]:
        return ["dotnet", f"./bin/build/{year}.dll", f"{day}", f"{part}"]


# Python
class PyConfig(LanguageConfig):
    def get_file_extension() -> str:
        return "py"

    def get_source_code(year: int, day: int, part: int) -> str:
        return "# TODO\n"

    def get_working_dir(year: int, day: int, part: int) -> str:
        return f"./{year}/day{day:02}"

    def get_compile_command(year: int, day: int, part: int) -> list[str]:
        return []

    def get_execute_command(year: int, day: int, part: int) -> list[str]:
        return ["python", f"part{part}.py"]


# Rust
class RsConfig(LanguageConfig):
    def get_file_extension():
        return "rs"

    def get_source_code(year: int, day: int, part: int) -> str:
        return "fn main() {\n    // TODO\n}\n"

    def get_working_dir(year: int, day: int, part: int) -> str:
        return f"./{year}"

    def get_compile_command(year: int, day: int, part: int) -> list[str]:
        return ["cargo", "build", "--release", "--bin", f"day{day:02}_part{part}"]

    def get_execute_command(year: int, day: int, part: int) -> list[str]:
        return [f"./target/release/day{day:02}_part{part}"]


##########################
## Tool implementation ###
##########################


# Read secret session cookie for API requests
def read_session_cookie() -> str | None:
    path = Path(".session_cookie")
    if not path.is_file():
        print(f"Error: Please create a '{path.name}' file containing your login session cookie")
        return None
    with open(path, "r") as f:
        return f.read().strip()


# Setup solution template and input
def setup(language: type[LanguageConfig], year: int, day: int, session_cookie: str):
    dir_path = Path(f"{year}/day{day:02}")

    # Create directory
    print("Creating directory:     ", end="")
    if dir_path.is_dir():
        print("Already exists")
    else:
        dir_path.mkdir(parents=True)
        print("Done")

    # Create parts 1 and 2
    for part in [1, 2]:
        print(f"Creating part {part}:        ", end="")
        file_path = dir_path / f"part{part}.{language.get_file_extension()}"
        if file_path.is_file():
            print("Already exists")
        else:
            with file_path.open("w") as f:
                f.write(language.get_source_code(year, day, part))
            print("Done")

    # Download input
    print("Downloading input:      ", end="")
    input_path = dir_path / "input"
    if input_path.is_file():
        print("Already exists")
    else:
        response = requests.get(url=f"https://adventofcode.com/{year}/day/{day}/input", cookies={"session": session_cookie})
        if response.status_code == 200:
            with input_path.open("w") as f:
                f.write(response.text)
            print("Done")
        else:
            print(f"Error {response.status_code}: {response.reason}")


# Compile and execute solution
def run(language: type[LanguageConfig], year: int, day: int, part: int, release: bool):
    print(f"================")
    print(f"==== Part {part} ====")
    print(f"================\n")

    working_dir = language.get_working_dir(year, day, part)
    compile_command = language.get_compile_command(year, day, part)
    execute_command = language.get_execute_command(year, day, part)

    # Compile
    if len(compile_command) > 0:
        try:
            compile = subprocess.run(compile_command, cwd=working_dir)
        except FileNotFoundError:
            print(f"Error: Compiler '{compile_command[0]}' not found\n")
            return
        if compile.returncode != 0:
            print(f"\nError: Unable to compile '{year}/day{day:02}/part{part}.{language}'\n")
            return

    # Execute
    with open(f"{year}/day{day:02}/input", "r") as f: # Pipe input to stdin
        time_start = time.time()
        try:
            execute = subprocess.run(execute_command, stdin=f.fileno(), cwd=working_dir, text=True)
        except FileNotFoundError:
            print(f"Error: Executable '{execute_command[0]}' not found\n")
            return
        duration = time.time() - time_start
    print(f"\nTime: {duration:.3f}s\n")
    if execute.returncode != 0:
        print(f"Error: Exited with return code {execute.returncode}\n")


# Parse command line arguments
def main():
    language_map = {l.get_file_extension(): l for l in LanguageConfig.__subclasses__()}

    parser = argparse.ArgumentParser(description="Advent of Code solution manager")
    parser.add_argument("-l", required=True, type=str, choices=language_map.keys(), help="select language")
    parser.add_argument("-y", required=True, type=int, choices=range(2015, 2999), metavar="{2015,...}", help="select year")
    parser.add_argument("-d", required=True, type=int, choices=range(1, 26), metavar="{1,...,25}", help="select day")
    parser.add_argument("-r", "--release", action="store_true", help="(optional) use release configuration for run")
    parser.add_argument("command", type=str, choices=["setup", "run"], help="setup directory or run solution(s)")
    parser.add_argument("part", type=int, choices=[1, 2], nargs="?", help="(optional) choose solution part")
    args = parser.parse_args()

    language = language_map[args.l]
    match args.command:
        case "setup":
            session_cookie = read_session_cookie()
            if session_cookie is not None:
                setup(language, args.y, args.d, session_cookie)
            else:
                print("Error: Missing session cookie")
        case "run":
            if args.part is None:
                run(language, args.y, args.d, 1, args.release)
                run(language, args.y, args.d, 2, args.release)
            else:
                run(language, args.y, args.d, args.part, args.release)


if __name__ == "__main__":
    main()

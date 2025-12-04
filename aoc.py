# Script for managing Advent of Code solutions
# Run the script with parameter -h to display more info
# To add support for new languages, modify the get_setup_template and get_run_commands functions

import argparse
from pathlib import Path
import requests
import subprocess
import time

#############################
## Language configurations ##
#############################

# Solution code templates
def get_setup_template(language, year, day, part) -> str:
    match language:
        case "cpp":
            return f"int main() {{\n    // TODO\n    return 0;\n}}\n"
        case "cs":
            return f"namespace Day{day:02};\n\nclass Part{part} {{\n\n    public static void Run() {{\n        // TODO\n    }}\n\n}}\n"
        case "py":
            return "# TODO\n"
        case "rs":
            return f"fn main() {{\n    // TODO\n}}\n"

    assert False, f"Template not found for language {language}"

# Commands for compiling and executing solutions
def get_run_commands(language: str, year: int, day: int, part: int) -> dict:
    match language:
        case "cpp":
            return {
                "work_dir": f"./{year}/day{day:02}",
                "compile":  ["g++", "-std=c++17", "-O2", "-o", f"part{part}", f"part{part}.cpp"],
                "execute":  [f"./part{part}"]
            }
        case "cs":
            return {
                "work_dir": f"./{year}",
                "compile":  ["dotnet", "build", "-o", "./bin/build", "--nologo", "-v", "q", "-clp:NoSummary"],
                "execute":  ["dotnet", f"./bin/build/{year}.dll", f"{day}", f"{part}"]
            }
        case "py":
            return {
                "work_dir": f"./{year}/day{day:02}",
                "compile":  None,
                "execute":  ["python", f"part{part}.py"]
            }
        case "rs":
            return {
                "work_dir": f"./{year}",
                "compile":  ["cargo", "build", "--release", "--bin", f"day{day:02}_part{part}"],
                "execute":  [f"./target/release/day{day:02}_part{part}"]
            }

    assert False, f"Commands not found for language {language}"

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
def setup(language: str, year: int, day: int, session_cookie: str):
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
        file_path = dir_path / f"part{part}.{language}"
        if file_path.is_file():
            print("Already exists")
        else:
            with file_path.open("w") as f:
                f.write(get_setup_template(language, year, day, part))
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
def run(language: str, year: int, day: int, part: int):
    print(f"================")
    print(f"==== Part {part} ====")
    print(f"================\n")

    commands = get_run_commands(language, year, day, part)

    # Compile
    if commands["compile"] is not None:
        try:
            compile = subprocess.run(commands["compile"], cwd=commands["work_dir"])
        except FileNotFoundError:
            print(f"Error: Compiler '{commands['compile'][0]}' not found\n")
            return
        if compile.returncode != 0:
            print(f"\nError: Unable to compile '{year}/day{day:02}/part{part}.{language}'\n")
            return

    # Execute
    with open(f"{year}/day{day:02}/input", "r") as f:
        time_start = time.time()
        try:
            execute = subprocess.run(commands["execute"], stdin=f.fileno(), cwd=commands["work_dir"], text=True)
        except FileNotFoundError:
            print(f"Error: Executable '{commands['execute'][0]}' not found\n")
            return
        duration = time.time() - time_start
    print(f"\nTime: {duration:.3f}s\n")
    if execute.returncode != 0:
        print(f"Error: Exited with return code {execute.returncode}\n")

# Parse command line arguments
def main():
    parser = argparse.ArgumentParser(description="Advent of Code solution manager")
    parser.add_argument("-l", required=True, type=str, choices=["cpp", "cs", "py", "rs"], help="select language")
    parser.add_argument("-y", required=True, type=int, choices=range(2015, 2100), metavar="{2015,...}", help="select year")
    parser.add_argument("-d", required=True, type=int, choices=range(1, 26), metavar="{1,...,25}", help="select day")
    parser.add_argument("command", type=str, choices=["setup", "run"], help="setup directory or run solution(s)")
    parser.add_argument("part", type=int, choices=[1, 2], nargs="?", help="(optional) choose solution part")

    args = parser.parse_args()
    match args.command:
        case "setup":
            session_cookie = read_session_cookie()
            if session_cookie is not None:
                setup(args.l, args.y, args.d, session_cookie)
        case "run":
            if args.part is None:	
                run(args.l, args.y, args.d, 1)
                run(args.l, args.y, args.d, 2)
            else:
                run(args.l, args.y, args.d, args.part)


if __name__ == "__main__":
    main()

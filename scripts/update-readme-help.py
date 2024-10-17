import subprocess
import re


def build():
    subprocess.run(["cargo", "build"]).check_returncode()


def get_help_output(name: str) -> str:
    args = ["./target/debug/i18nhero"]

    if len(name):
        args.append(name)

    args.append("--help")

    cmd = subprocess.run(args, stdout=subprocess.PIPE)

    cmd.check_returncode()

    return cmd.stdout.decode("utf-8")


def update_readme(readme: str, command: str, help: str) -> str:
    section = f"{"base" if len(command) == 0 else command}-command-help"

    return re.sub(
        pattern=f"(<!-- START_SECTION:{section} -->)[^{{}}]*?<!-- END_SECTION:{section} -->",
        repl=f"<!-- START_SECTION:{section} -->\n\n```\n{help}\n```\n\n<!-- END_SECTION:{section} -->",
        string=readme,
        flags=re.RegexFlag.MULTILINE,
    )


if __name__ == "__main__":
    COMMANDS = [
        # base
        "",
        "init",
        "push",
        "pull",
        "login",
        "logout",
        "completions",
    ]

    build()

    file = open("README.md", "r").read()

    for command in COMMANDS:
        output = get_help_output(command)

        file = update_readme(file, command, output)

    open("README.md", "w").write(file)

import subprocess
import re


def get_help_output(name: str) -> str:
    cmd = subprocess.run(["i18nhero-local", name, "--help"])

    cmd.check_returncode()

    return str(cmd.stdout)


def get_readme() -> str:
    pass


def update_readme(readme: str, command: str, help: str) -> str:
    section = f"{"base" if len(command) == 0 else command}-command-help"

    return re.sub(
        f"(<!-- START_SECTION:{section} -->)[^{{}}]*<!-- END_SECTION:{section} -->",
        f"<!-- START_SECTION:{section} -->\n\n{help}\n\n<!-- END_SECTION:{section} -->",
        readme,
        flags=re.RegexFlag.MULTILINE,
    )


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

if __name__ == "__main__":
    readme = get_readme()

    for command in COMMANDS:
        output = get_help_output(command)

        readme = update_readme(readme, command, output)

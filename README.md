# @i18nhero/cli

This tool allows you to manage your [i18nhero](https://i18nhero.com) locale files directly from command line.

<!-- START_SECTION:base-command-help -->

```
i18nhero 0.0.0
CLI tool for i18nhero.com
Mads Hougesen <mads@mhouge.dk>

Usage: i18nhero <COMMAND>

Commands:
  login        Login to i18nhero
  logout       Logout from i18nhero
  pull         Pull locales
  push         Push locale keys to i18nhero
  init         Create new i18nhero config
  completions  Shell completions
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

<!-- END_SECTION:base-command-help -->

## Install

### Linux & MacOS

```shell
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/i18nhero/cli/releases/latest/download/i18nhero-installer.sh | sh
```

### Windows

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/i18nhero/cli/releases/latest/download/i18nhero-installer.ps1 | iex"
```

### Cargo

The cli can be installed using Cargo.

```shell
cargo install i18nhero --locked
```

If you do not have Cargo installed, you need to [install it first](https://www.rust-lang.org/learn/get-started).

### npm/npx

You can install @i18nhero/cli using [npm](https://www.npmjs.com/package/@i18nhero/cli):

```shell
npm install -g @i18nhero/cli

i18nhero --help
```

or run it directly using npx:

```shell
npx @i18nhero/cli --help
```

### Homebrew

```shell
brew install i18nhero/tap/cli
```

## Usage

### Authentication

The `login` command is used for authenticating.

<!-- START_SECTION:login-command-help -->

```
Login to i18nhero

Usage: i18nhero login

Options:
  -h, --help     Print help
  -V, --version  Print version

```

<!-- END_SECTION:login-command-help -->

### Setting up a new project

<!-- START_SECTION:init-command-help -->

```
Create new i18nhero config

Usage: i18nhero init [OPTIONS]

Options:
      --overwrite
  -h, --help       Print help
  -V, --version    Print version

```

<!-- END_SECTION:init-command-help -->

### Downloading locales

The `pull` command is used for pushing (uploading) locales.

<!-- START_SECTION:pull-command-help -->

```
Pull locales

Usage: i18nhero pull [OPTIONS]

Options:
      --allow-dirty
  -h, --help         Print help
  -V, --version      Print version

```

<!-- END_SECTION:pull-command-help -->

### Uploading locales

The `push` command is used for pushing (uploading) locales.

<!-- START_SECTION:push-command-help -->

```
Push locale keys to i18nhero

Usage: i18nhero push

Options:
  -h, --help     Print help
  -V, --version  Print version

```

<!-- END_SECTION:push-command-help -->

## Shell completions

Shell completions can be generated using `i18nhero completions <SHELL>`.

<!-- START_SECTION:completions-command-help -->

```
Shell completions

Usage: i18nhero completions <SHELL>

Arguments:
  <SHELL>  [possible values: bash, elvish, fish, powershell, zsh]

Options:
  -h, --help     Print help
  -V, --version  Print version

```

<!-- END_SECTION:completions-command-help -->

#### Bash

Add the following to your `.bashrc`.

```bash
eval "$(i18nhero completions bash)"
```

#### Bash

Add the following to your `.zshrc`.

```bash
eval "$(i18nhero completions zsh)"
```

#### Fish

Add the following to `~/.config/fish/config.fish`.

```fish
i18nhero completions fish | source
```

#### PowerShell

Add the following to your PowerShell configuration (Can be found by running `$PROFILE`).

```powershell
Invoke-Expression (&i18nhero completions powershell)
```

#### Elvish

Add the following to `~/.elvish/rc.elv`.

```elvish
eval (i18nhero completions elvish)
```

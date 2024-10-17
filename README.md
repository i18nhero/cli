# @i18nhero/cli

This tool allows you to manage your [i18nhero](https://i18nhero.com) locale files directly from command line.

<!-- START_SECTION:base-command-help -->

```
i18nhero 0.0.0
CLI tool for interacting with locales hosted on i18nhero.com

Usage: i18nhero <COMMAND>

Commands:
  init         Initialize a new i18nhero config
  pull         Download locale files from i18nhero
  push         Upload locale files to i18nhero
  completions  Generate shell completions
  login        Login to i18nhero
  logout       Logout from i18nhero
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

<!-- END_SECTION:base-command-help -->

## Install

The best way to install the i18nhero cli is using a package manager like [Homebrew](#homebrew), [Cargo](#cargo) or [npm](#npm).

### Linux & MacOS

```shell
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/i18nhero/cli/releases/latest/download/i18nhero-installer.sh | sh
```

### Windows

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/i18nhero/cli/releases/latest/download/i18nhero-installer.ps1 | iex"
```

### Homebrew

Install using Homebrew.

```shell
brew install i18nhero/tap/cli
```

### Cargo

Install using Cargo.

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

## Getting started

Once installed you can setup your project by running `i18nhero init`.

You will then be prompted for your i18nhero api key, which can be generated at [https://i18nhero.com/settings/api](ttps://i18nhero.com/settings/api).

```
$ i18nhero init
You are not authenticated to i18nhero!
✔ Do you want to login now? · yes
? What is your api key? (https://i18nhero.com/settings/api) ›

```

After authenticating you can select the desired organization:

```
$ i18nhero init
You are not authenticated to i18nhero!
✔ Do you want to login now? · yes
✔ What is your api key? (https://i18nhero.com/settings/api) · ************************************
You are now signed in to i18nhero!
? Organization ›
❯ i18nhero
  demo-organization-1
  demo-organization-2

```

Then you will be asked which project you wish to setup.

```
$ i18nhero init
You are not authenticated to i18nhero!
✔ Do you want to login now? · yes
✔ What is your api key? (https://i18nhero.com/settings/api) · ************************************
You are now signed in to i18nhero!
✔ Organization · i18nhero
? Project ›
❯  @i18nhero/i18nhero.com
   @i18nhero/cli

```

You can now **download** your locale files by running the `i18nhero pull` command and **upload** your locale files using the `i18nhero push` command.

## Configuration

```jsonc
{
  "project_id": "ID-OF-PROJECT",
  "output": {
    "path": "lang",
    "format": "json",
    "keep_empty_fields": false,
    "flat": false,
  },
}
```

### project_id

Used to define the linked project.

### output

Configuration for downloading and uploading locale files.

#### path

Defines where locale files should be downloaded to, and uploaded from.

#### format

Defines the file format used when uploading and downloading locale files.

#### keep_empty_fields

Defines whether identifiers that are missing translations should be downloaded.

#### flat

Defines whether the locale files should be a flat `string <-> string` map or a multi layered map.

A point (`.`) in the identifier name is used to define multi-layered keys when `flat` is set to false.

The identifier `pages.dashboard.title` will be expanded to the following:

```json
{
  "pages": {
    "dashboard": {
      "title": ""
    }
  }
}
```

## Shell completions

Shell completions can be generated using `i18nhero completions <SHELL>`.

<!-- START_SECTION:completions-command-help -->

```
Generate shell completions

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

## Command reference

### Init

`i18nhero init` is used to setup a new i18nhero config.

<!-- START_SECTION:init-command-help -->

```
Initialize a new i18nhero config

Usage: i18nhero init [OPTIONS]

Options:
      --overwrite  Overwrite existing config
  -h, --help       Print help
  -V, --version    Print version

```

<!-- END_SECTION:init-command-help -->

### Pull

`i18nhero pull` is used to download locale files from [i18nhero.com](https://i18nhero.com)

<!-- START_SECTION:pull-command-help -->

```
Download locale files from i18nhero

Usage: i18nhero pull [OPTIONS]

Options:
      --allow-dirty
          Allow overwriting files that are not staged in VCS.

          Currently only applicable for projects that use git.

      --api-key <API_KEY>
          Use for authentication instead of global auth config

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

<!-- END_SECTION:pull-command-help -->

### Push

The `push` command is used for pushing (uploading) locales to [i18nhero.com](https://i18nhero.com).

<!-- START_SECTION:push-command-help -->

```
Upload locale files to i18nhero

Usage: i18nhero push [OPTIONS]

Options:
      --api-key <API_KEY>  Use for authentication instead of global auth config
  -h, --help               Print help
  -V, --version            Print version

```

<!-- END_SECTION:push-command-help -->

### Login

The `i18nhero login` command is used for authenticating to [i18nhero](https://i18nhero.com).

<!-- START_SECTION:login-command-help -->

```
Login to i18nhero

Usage: i18nhero login

Options:
  -h, --help     Print help
  -V, --version  Print version

```

<!-- END_SECTION:login-command-help -->

### Logout

<!-- START_SECTION:logout-command-help -->

```
Logout from i18nhero

Usage: i18nhero logout

Options:
  -h, --help     Print help
  -V, --version  Print version

```

<!-- END_SECTION:logout-command-help -->

### Completions

The `i18nhero login` command is used for generating shell completions for this tool.

<!-- START_SECTION:completions-command-help -->

```
Generate shell completions

Usage: i18nhero completions <SHELL>

Arguments:
  <SHELL>  [possible values: bash, elvish, fish, powershell, zsh]

Options:
  -h, --help     Print help
  -V, --version  Print version

```

<!-- END_SECTION:completions-command-help -->

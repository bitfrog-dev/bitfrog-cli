# Bitfrog CLI
> *A command line tool for easily sending Bitfrog notifications.*

## Usage

`bitfrog [OPTIONS / FLAGS]`

**Options:**
- `-t, --token` - The project token
- `-c, --channel` - The name of the project channel to target
- `-m, --message` - The notification message
- `-T, --title` - The notification title (for **Bitfrog Pro** projects)

**Flags:**
- `-w, --nowarning` - Disables server warnings

**Example:** `bitfrog -t 6ae9-ce81-feb0-0091 -m "Hello World"`

## Installation
### Windows
Download the [latest release](https://github.com/bitfrog-dev/bitfrog-cli/releases/latest) for the installer.

### Linux / MacOS
Build with `cargo build` and place somewhere safe, then add the parent folder to the `PATH` environment variable.
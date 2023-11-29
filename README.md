# focus-cli

`focus-cli` is a command line app written in Rust that allows users to run a timer with simple visual feedback. The timer will count down to 0, where a small sound will be played to indicate that the timer has finished.

## Getting Started

To use `focus-cli`, follow these steps:

1. Install Rust on your computer if you haven't already done so.
2. Clone this repository to your local machine.
3. Build the app by running `cargo build --release` in your terminal.
4. Run the app (see below).

> [!warning]
> MacOS/Linux/Windows WSL, NOT Windows

## Usage

```shell
Usage: focus-cli <title> [duration]

Arguments:
  <title>     The name of the task to track
  [duration]  The duration of the task in minutes (default=25m)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Example

```shell
focus-cli "Review notes"
focus-cli "Create weekly schedule" 45
````

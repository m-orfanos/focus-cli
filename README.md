# focus-cli

`focus-cli` is a command line app written in Rust that allows users to run a timer with simple audio/visual feedback. The timer will count down to 0, where a small sound will be played to indicate that the timer has finished.

## Getting Started

To use `focus-cli`, follow these steps:

1. Install Rust on your computer if you haven't already done so.
2. Clone this repository to your local machine.
3. Build the app by running `cargo build --release` in your terminal.
4. Run the app with `fx` from the release folder (see below).

A compiled binary built locally on a x86_64 machine running Fedora 39 is available in [Releases](https://github.com/m-orfanos/focus-cli/releases).

> [!warning]
> MacOS/Linux/Windows WSL compatible, NOT Windows

## Usage

```shell
Usage: fx <title> [duration]

Arguments:
  <title>     The name of the task to track
  [duration]  The duration of the task in minutes (default=25m)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Example

```shell
$ ./target/release/fx "Create weekly schedule" 45
# progress bar will fill up over time
# format: title duration start-time elapsed-time progress-bar
Create weekly schedule 45m 13:53 00:12:45 [##########>                                       ]
````

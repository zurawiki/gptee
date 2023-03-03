# `gptee`

[![Github Contributors](https://img.shields.io/github/contributors/zurawiki/gptee.svg)](https://github.com/zurawiki/gptee/graphs/contributors)
[![Github Stars](https://img.shields.io/github/stars/zurawiki/gptee.svg)](https://github.com/zurawiki/gptee/stargazers)
[![CI](https://github.com/zurawiki/gptee/actions/workflows/ci.yml/badge.svg)](https://github.com/zurawiki/gptee/actions/workflows/ci.yml)

[![crates.io status](https://img.shields.io/crates/v/gptee.svg)](https://crates.io/crates/gptee)
[![crates.io downloads](https://img.shields.io/crates/d/gptee.svg)](https://crates.io/crates/gptee)
[![Rust dependency status](https://deps.rs/repo/github/zurawiki/gptee/status.svg)](https://deps.rs/repo/github/zurawiki/gptee)

Output from a language model using standard input as the prompt

[![asciicast](https://asciinema.org/a/6q1tQ6TvZvWLqpIJBlTouPHBl.svg)](https://asciinema.org/a/6q1tQ6TvZvWLqpIJBlTouPHBl)

**Now supporting GPT3.5 chat completions!**

## Installation

1. Install this tool locally with `cargo` (recommended).

```sh
cargo install --locked gptee
```

## Usage

`gptee` was designed for use within shell scripts and other programs, and also works in interactive shells.

Simple example

```sh
echo Tell me a joke | gptee
```

```
Why did the chicken cross the road?

To get to the other side!

```

Compose shell commands like you would in a script

```sh
echo Tell me a joke | gptee | say
```

You can compose command and execute them in a script.
**Proceed with caution before running arbitrary shell scripts**

```sh
echo Give me just a macOS zsh command to get the free space on my hard drive \
| gptee -s "Prefix each line of output with a pound sign if it not meant to be executed" \
# pipe this to `sh` to have it execute
```

Try with a custom model. By default `gptee` uses `gpt-3.5-turbo`

```sh
echo Tell me a joke | gptee -m text-davinci-003
```

Using a chat completion model (like `gpt-3.5-turbo`), you can then inject a system message with `-s` or `--system-message`. For davinci and other non-chat models, the output is prefixed to the prompt.

```sh
echo "Tell me I'm pretty" | gptee -s "You only speak French"
```

See the `--help` / `-h` flag for more features.

## Encountered any bugs?

If you encounter any bugs or have any suggestions for improvements, please open an issue on the repository.

## License

This project is licensed under the [MIT License](./LICENSE).

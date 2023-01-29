# `gptee`

[![Github Contributors](https://img.shields.io/github/contributors/zurawiki/gptee.svg)](https://github.com/zurawiki/gptee/graphs/contributors)
[![Github Stars](https://img.shields.io/github/stars/zurawiki/gptee.svg)](https://github.com/zurawiki/gptee/stargazers)
[![CI](https://github.com/zurawiki/gptee/actions/workflows/ci.yml/badge.svg)](https://github.com/zurawiki/gptee/actions/workflows/ci.yml)

[![crates.io status](https://img.shields.io/crates/v/gptee.svg)](https://crates.io/crates/gptee)
[![crates.io downloads](https://img.shields.io/crates/d/gptee.svg)](https://crates.io/crates/gptee)
[![Rust dependency status](https://deps.rs/repo/github/zurawiki/gptee/status.svg)](https://deps.rs/repo/github/zurawiki/gptee)

Output from a language model using standard input as the prompt

## Installation

1. Install this tool locally with `cargo` (recommended).

```sh
cargo install --locked gptee
```

## Usage

`gptee` was designed for use within shell scripts and other programs, and also works in interactive shells.

The following shell script demonstrates how prompts and can "chained" together.


```sh
HUMAN_MESSAGE="What is B.O's dog called?"

ENTITIES=$(echo "List out the entities in the prompt below" $ORIG_PROMPT | gptee)

echo "Answer questions given the following context:" $ENTITIES "Answer the query below" $ORIG_PROMPT | gptee
```

Simple example
```sh
echo Tell me a joke | gptee
```

## Encountered any bugs?

If you encounter any bugs or have any suggestions for improvements, please open an issue on the repository.

## License

This project is licensed under the [MIT License](./LICENSE).

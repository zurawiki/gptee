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
echo "Tell me a joke" | gptee | say
``` 

Try with a custom model

```sh
echo Tell me a joke | gptee -m text-davinci-003
```

The following shell script demonstrates how prompts and can "chained" together.
(TODO)

```sh
$ cat << EOF | gptee -m text-davinci-003 -s "Observation: "
Answer the following questions as best you can. You have access to the following tools:

Search: A search engine. Useful for when you need to answer questions about current events. Input should be a search query.
Calculator: Useful for when you need to answer questions about math.

Use the following format:

Question: the input question you must answer
Thought: you should always think about what to do
Action: the action to take, should be one of [Search, Calculator]
Action Input: the input to the action
Observation: the result of the action
... (this Thought/Action/Action Input/Observation can repeat N times)
Thought: I now know the final answer
Final Answer: the final answer to the original input question

Begin!

Question: Who is Olivia Wilde's boyfriend? What is his current age raised to the 0.23 power?
Thought:
EOF
```

output:

```
I should look up Olivia Wilde's boyfriend first.
Action: Search
Action Input: Olivia Wilde boyfriend
```

## Encountered any bugs?

If you encounter any bugs or have any suggestions for improvements, please open an issue on the repository.

## License

This project is licensed under the [MIT License](./LICENSE).

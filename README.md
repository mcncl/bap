# Buildkite Agent Picker (`bap`)
> A simple, occasionally fast Buildkite agent manager, built in Rust.

![bap demo](./images/demo.gif)

## Features
✨ Easy to use, instant start up of agents

🚀 Continued focus on speed

📁 Supports local agent versions and global

## Installation

```sh
brew tap mcncl/bap
brew install bap
```

## Usage

To list the available agents, run:

```sh
bap list-remote
```

You can then choose to install any of the available agent versions by selecting it (`Enter`).

Once installed, you'll need to get an agent token from Buildkite, copy it to your clipboard and run:

```sh
bap auth <version>
```

For example:

```sh
bap auth 3.74.1
```

You'll then be able to `run` that version:

```sh
bap run 3.74.1
```

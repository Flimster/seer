# Seer

Seer is a time watching command line tool that allows you to specify and watch time you spend on individual tasks.

## Installing

To install, simply run the command

```bash
cargo install
```

To run Seer, type
```bash
cargo run <PROJECT> <TIMER>
```
Note that a timer is in the format HH:MM:SS (hours, minutes, seconds), but Seer is flexible and can support only SS or MM:SS

## Developing

You must fork the project to propose changes.

### Built with

The main libraries Seer uses are:

- [clap 2.33.0](https://crates.io/crates/clap) for command line argument parsing
- [termion 1.5.5](https://crates.io/crates/termion) for controlling TTY devices

### Prerequisites 

Install [Rust and Cargo](https://www.rust-lang.org/tools/install).

### Setting up dev

Using HTTPS
```bash
git clone https://github.com/<YOUR_USERNAME>/seer.git
cd seer/
cargo build
```

Using SSH 
```bash
git clone git@github.com:<YOUR_USERNAME>/seer.git
cd seer
cargo build
```

### Building 

To build a debug binary:
```
cargo build
```

To build a release binary:
```bash
cargo build --release
```

To run the project
```bash
    cargo run <PROJECT> <TIMER>
```

### Docker

The repository includes a Dockerfile if you want to run the project inside a docker container.

Make sure that you have the [rust](https://hub.docker.com/_/rust/) image installed

To build the project
```bash
docker build -t seer .
```

To run the project inside a docker container
```bash
docker run -it seer <PROJECT> <TIMER>
```

## Configuration

### Colors

Seer supports multiple colors through the environment variables `TASK_CLOR` and `TIMER_COLOR`.
The possible values for the environment variables are:

- Red
- Blue
- Green
- Cyan
- Black
- Yellow
- White
- Magenta
- Lightblack
- Lightblue
- Lightcyan
- Lightred
- Lightwhite
- Lightyellow

The values of the environment variables are case insensitive, but have to be one word.

## Style guide

This repository follows the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/) and uses the [rustfmt](https://github.com/rust-lang/rustfmt) tool for automatically formatting the code to keep it in a consistent state.

## Licensing

Licensed under the [MIT](https://github.com/Flimster/seer/blob/master/LICENSE) license.
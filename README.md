Advent of Code 2022
===================

My solutions for [Advent of Code 2022][aoc 2022], written in [Rust][rust].

This repository provides a good template for anyone interested in writing
their solutions in Rust. Follow the instructions below to get started!

## Setup

### Prerequisites

If you don't already have [Rust][rust] installed, you'll want to
[install it here][install rust]. Otherwise, the only things you'll need
are a terminal/shell and a text editor (I recommend [Helix][helix] if
you're shopping for a new one).

### Clobbering my Solutions (For Your Integrity!)

To get started, you'll want to fork this repository and then delete any
of my solutions I've submitted so far to keep yourself from being spoiled:

```bash
rm src/day*.rs
```

Also, in the `src/main.rs` file, you'll need to delete all `mod dayN;`
declarations and reset the `get_day_solution` implementation to the following:

```rust
fn get_day_solution(day: usize, lines: impl Iterator<Item = String>) -> Box<dyn DaySolution> {
    match day {
        // 1 => Box::new(day1::Day1::from_lines(input)),
        _other => panic!("Day hasn't been solved yet"),
    }
}
```

Now you're ready to go!. You'll want to log in to [Advent of Code][aoc 2022]
so that you can download inputs automatically.

### Logging In to Advent of Code

Go to the [Advent of Code][aoc 2022] site and log in at the top of the
page. Once you're signed in, you should open the Developer Tools and head
to the "Network" tab, and then reload the page. Look through your requests
until you find one that has your "session" cookie. Copy the contents of
that cookie (the alphanumeric string after the equals sign) sans quotes to
a file called `.session` in this repository. Now you're ready to download
inputs from the terminal!

_Note: the `.session` file is .gitignored, so you don't accidentally upload_
_your login token to GitHub._

### Setting Up For a Day

The `prep-day.sh` script in the root of this repository will download your
input using `curl` to a .gitignored `.input` directory and then copy a
boilerplate module to `src/dayN.rs`, so you can start working (almost)
right away! Here's how to run it:

```bash
sh prep-day.sh <day>
```

Just remember to update `main.rs` with the suggestions printed by the
`prep-day.sh` script to ensure that you can run your solutions.

## Running

To run your solutions for a day, run the following in the repo:

```bash
cargo run <day>
```

You should see something like the following:

```bash
❯ cargo run 1
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/aoc-2022 1`
Solving day 1...
Part 1: <solution> (0.000100000 seconds)
Part 2: <solution> (0.000300000 seconds)
```

## Questions

If you have any issues getting this up and running, you can make an
[issue on GitHub][make issue], email me at <sam@sammohr.dev> or ping
me on Discord at `Smores#6844`; I'm happy to help anyone that gets stuck.
Good luck learning Rust for those of you picking it up, and enjoy the
gift of ~~pulling your hair out~~ puzzle solving for the holiday season!


[aoc 2022]: https://adventofcode.com/2022
[rust]: https://rust-lang.org
[install rust]: https://www.rust-lang.org/tools/install
[helix]: https://helix-editor.com/
[make issue]: https://github.com/smores56/aoc-2022/issues/new

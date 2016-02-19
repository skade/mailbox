# Mailbox Rust Tutorial

This repo shows the steps to building a small, networked mailbox program which
uses threads and concurrency. 

## Step 1

`multirust run stable cargo new`

Edit `src/main.rs` to contain the hello world program. 

`multirust run stable cargo run`

## Step 2

You learn to take a command line argument with
[args](https://doc.rust-lang.org/std/env/fn.args.html), and use a match
statement to change the program's output based on the input.

## Step 3

You change from using the `nth` to the `skip` iterator when picking command
line args. You modify the match statement to consider the length of the list
of arguments, and iterate over them with a for loop.

## Step 4: 

You learn about IO and TCP by modifying your program to take its input from
port 7200 of localhost instead of from the command line, and give its output
back over that connection as well.

## Step 5: 

You add concurrency and basic message buffering. 

# LICENSE

CC0, use and copy and change whatever you'd like. Mention me if you want,
but you have no obligations.

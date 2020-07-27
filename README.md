# ruban

## Why ?

I always add tasks in my head while I have this terminal available in front of me.

Manipulating tasks in the terminal seems way more productive and allows to manipulate them easily and very quickly.

I also wanted to learn Rust, so this is a pet project to experiment with CLI in Rust.

## How to use ?

#### Add a task:

`ruban add -t "House" "Repair the garage door"`

The `-t` or `--tags` flag allows to add tags to a task so they can be sorted and classified.
Tags are optional.

#### Show all tasks:

`ruban ls`

This command shows the list of all tasks

#### Remove a task:

`ruban rm <number>`

#### Move a task from one status to another:

`ruban mv <number> <status>`

### Demo

![](demo.gif)

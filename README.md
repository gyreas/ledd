# Under heavy construction thus the API is still very unstable.
# Check back occasionally

## ledd
`ledd` (pronounced led-dy) is a line-oriented text editor in the spirit of Unix `ed` but it daemonizes (and looks nicer): thus, `ledd` mean `l`ittle `ed`itor that `d`aemonizes. Since it daemonizes, you interact with it via a client that takes commands from the shell. Written in Rust.

### Features
#### Basically my TODO list
 - [ ] background process that can manage multiple buffers at once
 - [ ] show cursor position on the current line
 - [ ] lightweight and similar commands to `ed`
 - [ ] the client/tui does the line printing via gRPC
 - [ ] uses gRPC

### Why?
Here are some of the reasons I'm starting this project:
- this is basically my first public Rust project as a newbie Rust programmer with little programming background
- I got pretty curious about rope data structure (I'm not going to try to implement it, however), process management, asynchronous programming, and RPCs
- "asynchronous programming" above refers to concurrency and parrallelism, and associated headaches offered by Rust: I want to at least get a sense of what the books, videos and talks were talking about in practice.
- what if we can play with buffers without mutating one? so yeah, operations that should mutate edit buffers simply operate and create new ones once done
- experiment and learn how TUIs work
- learn my way around git and github workflow eg patching,rebasing,prs, etc
- learn some design patterns, software engineering concepts
- figure out debug logs and tuning verbosities etc
- possibly get a preliminary understanding of how to use debuggers such as gdb.

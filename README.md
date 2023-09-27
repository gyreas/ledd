## ledd
`ledd` is a stream editor in the spirit of Unix `ed` but it daemonizes: thus, `ledd` mean `l`ittle `ed`itor that `d`aemonizes. Since it daemonizes, you interact with it via a client that takes commands from the shell. Written in Rust.

### Features
- [ ] background process that can manage multiple buffers at once
- [ ] show cursor position on the current line
- [ ] the client does the line printing
- [ ] uses gRPC

### Why?
Here are some of the reasons I'm starting this project:
- this is basically my first public Rust project as a newbie Rust programmar with little programming background
- I got pretty curious about rope data structure (I'm not going to try to implement it, however), process management, asynchronous programming, and RPCs
- "asynchronous programming" above refers to concurrency and parrallelism, and associated headaches offered by Rust: I want to at least get a sense of what the books, videos and talks were talking about in practice.
- what if we can play with buffers without mutating one? so yeah, operations that should mutate edit buffers simply operate and create new ones once done
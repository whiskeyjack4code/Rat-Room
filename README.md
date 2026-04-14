# Ratroom (Rust Chatroom)

A simple multi-client chatroom built in Rust.

This started as a learning project to understand networking and concurrency properly, and ended up turning into a full threaded chatroom with a terminal UI.

---

## What it does

* Runs a TCP server that accepts multiple clients
* Each client connects and chooses a username
* Messages are broadcast to everyone else
* Users joining/leaving are announced
* Duplicate usernames are rejected
* `/who` shows who is currently connected
* `/quit` disconnects gracefully out of the session
* Terminal UI for sending/receiving messages

---

## Why this exists

I wanted to build something in Rust that was fun and visual on the screen so I could see progress with each new line of code.

The final product here is roughly 6 months of reading documentation, trial and error, and research along with the usual Rust quirks.

The project touches on the major areas of a network style project:
* sockets
* threads
* shared state
* handling disconnects properly
* building a UI on top

The goal wasn’t to make the perfect chat app — it was to understand how all these pieces fit together.

---

## Project structure

```text
ratroom/
├── Cargo.toml
├── config.toml
└── src/
    └── bin/
        ├── server.rs
        └── client.rs
```

Each file in `src/bin` is its own executable.

---

## Config

The server and client both read from `config.toml`:

```toml
host = "127.0.0.1"
port = 8080
```

---

## Running it

Start the server first:

```bash
cargo run --bin server
```

Then in another terminal:

```bash
cargo run --bin client
```

You can run multiple clients at once as the state is shared and thread safe.

---

## Commands

Inside the client:

* `/who` → shows connected users
* `/quit` or `Esc` → exits

---

## Example

```text
[system] sophie joined the chat
dave: hello
sophie: hi
[system] dave left the chat
[system] mick joined the chat
mick: hello!
```

---

## How it works (high level)

### Server

* Listens on a TCP socket
* Spawns a thread per client
* Stores connected clients in shared state (`Arc<Mutex<...>>`)
* Broadcasts messages to all clients except the sender
* Removes clients immediately when they disconnect

### Client

* Connects to the server and sends a username first
* Has a background thread for receiving messages
* Uses a main loop for input and rendering
* Displays everything in a TUI

---

## Things I intentionally kept simple

This is not meant to be production-grade.

The below were left out intentionally:
* async/Tokio (planned next)
* persistence
* authentication
* chat rooms
* fancy UI

The idea was to keep it understandable.

---

## Next steps for version 2

* rewrite using Tokio (async version)
* better UI (scrolling, panes, etc.)
* logging
* deploy it to AWS Ubuntu instance
* Dockerize

---

## Final note

This project is really about learning:

* how to break a problem into steps
* how to reason about ownership in a real system
* how threads and shared state actually work
* how to become an engineer rather than just copying and pasting code.

If you’re learning Rust, I’d recommend rebuilding something like this yourself rather than copying it. Even if you choose another language like C++ or Java, build it.

---

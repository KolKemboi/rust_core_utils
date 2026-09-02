# Rust Systems Utilities

A Rust implementation of selected Unix-style command-line utilities, built as a systems programming and tooling portfolio project.

The project focuses on **idiomatic Rust, operating-system interfaces, streaming I/O, filesystem operations, process management, error handling, testing, and CLI design** rather than simply reproducing command behavior.


## Implemented Utilities

### Phase 1 — Core I/O, Parsing & Filesystem Operations

| Utility | Status | Concepts |
|---|---|---|
| `yes` | Done | Streaming output, stdout |
| `true` | Done | Exit codes |
| `false` | Done | Exit codes |
| `echo` | Done | Argument parsing, stdout |
| `basename` | Done | Path manipulation |
| `dirname` | Done | Path manipulation |
| `cat` | Done | File I/O, buffered streams |
| `head` | Planned | Streaming I/O, line/byte processing |
| `tail` | Planned | File seeking, buffering |
| `wc` | Planned | Streaming algorithms, text processing |
| `sort` | Planned | Algorithms, memory management |
| `uniq` | Planned | Iterators, streaming state |
| `grep` | Planned | Pattern matching, buffered I/O |
| `find` | Planned | Recursive filesystem traversal |

### Phase 2 — Filesystems, Processes & Unix Interfaces

| Utility | Status | Concepts |
|---|---|---|
| `cp` | Planned | File copying, metadata |
| `mv` | Planned | Filesystem semantics |
| `rm` | Planned | Recursive traversal, deletion |
| `mkdir` | Planned | Directory creation |
| `touch` | Planned | File metadata, timestamps |
| `chmod` | Planned | Unix permissions |
| `env` | Planned | Environment variables |
| `which` | Planned | `PATH` resolution |
| `sleep` | Planned | Timers, OS interaction |
| `xargs` | Planned | Process spawning, argument construction |
| `tee` | Planned | Multiple output streams |

---

## Architecture

The project is organized as a Cargo workspace, with each utility maintained as an independent binary.

```text
rust-utils/
├── crates/
   ├── cat/
   ├── head/
   ├── tail/
   ├── wc/
   ├── sort/
   ├── uniq/
   ├── grep/
   ├── find/
   ├── cp/
   ├── mv/
   ├── rm/
   ├── mkdir/
   ├── touch/
   ├── chmod/
   ├── env/
   ├── which/
   ├── sleep/
   ├── xargs/
   └── tee/


# rust-learnings

![Build Status](https://img.shields.io/github/actions/workflow/status/Janmesh23/apexsolana/ci.yml?branch=main)
![Rust Version](https://img.shields.io/badge/rust-1.75%2B-blue)

## What is this?
Welcome to `rust-learnings`! This repository is designed to teach Rust concepts to junior developers during our FOSS weekend event. Instead of building from scratch, contributors will learn by fixing broken or incomplete Rust files, submitting Pull Requests, and receiving automated feedback on their solutions via GitHub Actions CI.

## How it works

1. **Pick Issue**: Find an open issue on the curriculum table.
2. **Fix File**: Follow the `TODO` hints and fix the code in `src/issues/`.
3. **Open PR**: Submit your fix as a Pull Request.
4. **CI Validates**: Automatic test suites (which you can't cheat!) run to verify your solution.

## Issue Curriculum Table

| Issue # | Title | Concept | Difficulty | Status |
|---|---|---|---|---|
| Basics | [CLI Calculator](src/basics/calculator.rs) | Variables, Types, Control Flow, Parsing | Beginner | 🛠️ Playground |
| Basics | [Guessing Game](src/basics/guessing_game.rs) | std::io, match, rand, loops | Beginner | 🛠️ Playground |
| #01 | [Fix Ownership](src/issues/01_fix_ownership.rs) | Ownership and move semantics | Beginner | 🟢 Open |
| #02 | [Remove Unwraps](src/issues/02_remove_unwraps.rs) | Result, Option & Error Handling | Beginner | 🟢 Open |
| #03 | [Fix Iterator](src/issues/03_fix_iterator.rs) | Iterators & Adaptors | Beginner/Intermediate | 🟢 Open |
| #04 | [The String Hand-off](src/issues/04_the_string_hand_off.rs) | Ownership & Move Semantics | Beginner | 🟢 Open |
| #05 | [Enum Exhaustiveness](src/issues/05_enum_exhaustiveness.rs) | The Type System | Beginner | 🟢 Open |
| #06 | [The Double Agent](src/issues/06_the_double_agent.rs) | Borrowing & References | Intermediate | 🟢 Open |
| #07 | [Domain Errors](src/issues/07_domain_errors.rs) | Error Handling (thiserror) | Intermediate | 🟢 Open |
| #08 | [The Long-Lived Struct](src/issues/08_the_long_lived_struct.rs) | Lifetimes | Advanced | 🟢 Open |
| #09 | [Map vs ForEach](src/issues/09_map_vs_for_each.rs) | Iterators & Lazy Eval | Beginner | 🟢 Open |
| #10 | [The Shared Interface](src/issues/10_the_shared_interface.rs) | Traits | Beginner | 🟢 Open |
| #11 | [The Move Closure](src/issues/11_the_move_closure.rs) | Closures & Captures | Intermediate | 🟢 Open |
| #12 | [Shared Reality](src/issues/12_shared_reality.rs) | Smart Pointers (Rc) | Intermediate | 🟢 Open |
| #13 | [Object Safety](src/issues/13_object_safety.rs) | Traits & Dyn Dispatch | Advanced | 🟢 Open |
| #14 | [The Transient Reference](src/issues/14_the_transient_reference.rs) | Common Compiler Errors | Beginner | 🟢 Open |
| #15 | [Const Generic Buffer](src/issues/15_const_generic_buffer.rs) | Generics & Const Params | Intermediate | 🟢 Open |
| #16 | [Threaded Counter](src/issues/16_threaded_counter.rs) | Concurrency (Arc+Mutex) | Intermediate | 🟢 Open |
| #17 | [The Async Deadlock](src/issues/17_the_async_deadlock.rs) | Async/Await Pitfalls | Advanced | 🟢 Open |
| #18 | [Unsafe Raw Pointers](src/issues/18_unsafe_raw_pointers.rs) | Unsafe Rust Basics | Advanced | 🟢 Open |
| #19 | [The Anyhow Application](src/issues/19_the_anyhow_application.rs) | Advanced Error Handling | Intermediate | 🟢 Open |
| #20 | [Blocking the Executor](src/issues/20_blocking_the_executor.rs) | Async Performance | Advanced | 🟢 Open |
| #21 | [Slice Invalidation](src/issues/21_slice_invalidation.rs) | Borrowing & Loops | Intermediate | 🟢 Open |
| #22 | [The Static Trap](src/issues/22_the_static_trap.rs) | Lifetimes & Threads | Advanced | 🟢 Open |
| #23 | [The macro_rules! Clone](src/issues/23_the_macro_rules_clone.rs) | Declarative Macros | Intermediate | 🟢 Open |

## Getting Started

Check out [CONTRIBUTING.md](../CONTRIBUTING.md) to start your first issue!
- **Step 1:** Fork the repo
- **Step 2:** Clone it locally
- **Step 3:** Make a branch
- **Step 4:** Fix code & push!




# Contributing Guide

Welcome to `rust-learnings`! We're glad you're here. This repository is specifically designed
for learning Rust during our FOSS weekend event.

## Prerequisites

Before starting, you need a few tools on your machine:
1. **Rust**: Install it via [rustup.rs](https://rustup.rs/). Run this command in your terminal:
   `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. **Git**: Version control system. Download it from [git-scm.com](https://git-scm.com/).
3. **GitHub Account**: Make sure you are signed in.

## Step by Step Guide

### Step 1: Fork the repo
Forking creates a personal copy of this repository on your GitHub account so you have permissions to edit it.
- Click the "Fork" button in the upper right hand corner of the repository page.

### Step 2: Clone your fork
Download your fork to your local computer. Run this in your terminal:
```bash
git clone https://github.com/YOUR_USERNAME/rust-learnings.git
cd rust-learnings
```

### Step 3: Set upstream remote
Connect your local copy to the original repository so you can fetch future updates:
```bash
git remote add upstream https://github.com/ORIGINAL_OWNER/rust-learnings.git
```
*(This matters so you can easily sync your fork with any new issues we add to the main curriculum!)*

### Step 4: Create a branch
Create a fresh branch for your work. Don't work on the `main` branch.
```bash
git checkout -b fix/issue-01-your-name
```
*Naming convention: `fix/issue-<issue_number>-<your_name>` helps maintainers identify PRs.*

### Step 5: Make your changes
Pick an issue from `src/issues/` (for example, `01_fix_ownership.rs`).
Open that **single file** in your editor and fix the `TODO` comments.
**DO NOT touch anything in `tests/` or `Cargo.toml`.**

### Step 6: Test locally
Before pushing, run these commands:
```bash
cargo fmt
cargo clippy
cargo test 01
```
*(Replace `01` with your actual issue number to run its tests).* All commands must run without errors.

### Step 7: Commit and push
```bash
git add src/issues/01_fix_ownership.rs
git commit -m "fix: solve issue 01 - fix ownership errors"
git push origin fix/issue-01-your-name
```

### Step 8: Open a Pull Request
- Go to your fork on GitHub.
- Click "Compare & pull request".
- Add "Closes #1" (or your issue number) to the description.
- Submit the Pull Request!

## CI Feedback Guide

Once you open a PR, GitHub Actions (CI) automatically checks your code.
- **Labels**: You will get a `✅ tests-passing` or `❌ tests-failing` label.
- **Comments**: The CI will post exactly what failed in the PR comments. Read them carefully!
- **Fixing**: If CI fails, don't open a new PR. Just change your code, run `git add`, `git commit`, and `git push` again to the same branch.

## Rules
- One issue per PR (hard enforced by CI).
- Do not modify `tests/` or `Cargo.toml`.
- Branch naming convention required.
- Be kind and help each other in comments!

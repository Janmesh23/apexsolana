# Contributing to apexsolana/rust-learnings

Welcome to our FOSS Weekend Rust Learning repository! We are excited to have you contribute and learn Rust with us.

Since this repository is used for teaching and grading, we have a very specific workflow stringently enforced by automated GitHub Actions (our CI pipeline). Please follow this guide carefully to ensure your Pull Requests are merged perfectly!

---

## 🛠️ Step 1: Local Environment Setup

Before writing any code, ensure you have the proper tools installed:

1. **Install Rust**:
   - macOS / Linux: Open your terminal and run:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - Windows: Download and run `rustup-init.exe` from [rustup.rs](https://rustup.rs/).

2. **Code Editor**:
   - We highly recommend downloading **[Visual Studio Code](https://code.visualstudio.com/)**.
   - Install the **rust-analyzer** extension in VS Code. This will give you powerful autocompletion, instant error highlighting, and inline hints directly from the Rust compiler.

3. **Verify Installation**:
   Open a new terminal and verify the commands are available:
   ```bash
   rustc --version
   cargo --version
   ```

---

## 🐙 Step 2: Fork and Clone

You will not be pushing directly to this repository. You must use a "fork".

1. **Fork the Repository**: Click the `"Fork"` button in the top-right corner of this GitHub repository page to create a copy under your own account.
2. **Clone your Fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/apexsolana.git
   cd apexsolana/rust-learnings
   ```
   *(Note: All curriculum work takes place exclusively inside the `rust-learnings/` folder.)*

3. **Configure Upstream** *(Optional but recommended)*:
   ```bash
   git remote add upstream https://github.com/Janmesh23/apexsolana.git
   ```

---

## 💻 Step 3: Branching & Coding

1. **Create a fresh branch** off `main` for the specific issue you are tackling:
   ```bash
   git checkout -b fix/issue-01-yourname
   ```
2. **Open the Issue**: Open `rust-learnings/src/issues/01_fix_ownership.rs` (or whichever issue you chose) in your editor.
3. **Solve the TODOs**: Fix the intentionally broken starter code based on your Rust knowledge!

### ⚠️ Critical Project Rules:
* **ONLY EDIT ONE ISSUE PER PR**: If you edit `01_fix_ownership.rs` and `02_remove_unwraps.rs` in the same commit, our CI firewall will instantly reject your PR.
* **NEVER EDIT `tests/`**: The `tests/` directory is monitored by security checks. If you try to change the tests to make your code pass, your PR will fail and be flagged with `⚠️ do-not-merge`.

---

## 🧪 Step 4: Testing & Formatting Locally

Our automated pipeline is extremely strict. It treats all console warnings as hard errors. **Never push code without running these three commands locally inside the `rust-learnings/` directory:**

1. **Formatting**:
   ```bash
   cargo fmt
   ```
   *This automatically cleans up all your spaces, tabs, and line breaks to conform to exact Rust standards.*

2. **Linting**:
   ```bash
   cargo clippy
   ```
   *This looks for bad code styles. If you see yellow warnings, fix them! The GitHub pipeline will block your PR if a single warning exists.*

3. **Local Tests**:
   ```bash
   cargo test 01
   ```
   *Replace `01` with your issue number. Note: The tests on your local machine are just small hints. The actual grading happens automatically on GitHub!*

---

## 🚀 Step 5: Submitting your Pull Request

1. **Commit your single file**:
   ```bash
   git add src/issues/01_fix_ownership.rs
   git commit -m "fix(issue): solved ownership rules for module 1"
   git push origin fix/issue-01-yourname
   ```
2. **Open the Pull Request**: Navigate to your fork on GitHub and click "Compare & pull request".
3. **Wait for CI Grading**:
   Within seconds, the GitHub Action bots will pick up your PR.
   * If it succeeds, a maintainer will merge it!
   * If it fails, look for the big red ❌ at the bottom of the PR thread.

### How to read CI errors:
We DO NOT post automated comments on your PR. **If the CI fails, click the tiny "Details" link** next to the red X at the bottom of the page. It will open the console logs showing you exactly which test failed, where your formatting was wrong, or instructions on how to fix your code!

*Mistakes happen! If CI fails, you don't need to open a new PR. Just fix the code locally, run `git add`, `git commit`, and `git push` again to the same branch. The pipeline will automatically re-run.*

Happy coding and enjoy the weekend! 🎉

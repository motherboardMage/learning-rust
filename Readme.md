# 🦀 Learning Rust 🦀

> My journey through the official Rust book, ["The Rust Programming Language"](https://doc.rust-lang.org/book/).

This repository serves as a personal log of my progress while learning Rust. It's a collection of my notes, code snippets, and projects, all organized to reflect my path through the book.

---

## 📂 Project Structure

I've structured this repository to keep my learnings organized and easy to navigate. Here’s a look at the layout:

```
.
├── 📝 notes/
│   ├── 🧠 common-programming-concepts/
│   ├── 🦀 ownership/
│   ├── 🏗️ structs/
│   └── ... and more as I progress
│
├── 💻 code/
│   ├── 🔍 let-match.rs
│   └── ... and other snippets
│
├── 🚀 projects/
│   └── 🎮 guessing_game/
│
├── 📜 extra-notes/
│   └── ... additional thoughts and clarifications
│
└── 🤖 .scripts/
    └── update_commit_graph.sh
```

*   **`📝 notes/`**: This is where I keep my detailed notes on each chapter and concept. They are my summaries and interpretations of the book's content.
*   **`💻 code/`**: A collection of small, focused Rust snippets. Each file is a self-contained example illustrating a specific language feature.
*   **`🚀 projects/`**: Larger, more involved projects that I build to practice and apply what I've learned.
*   **`📜 extra-notes/`**: For my own thoughts, clarifications, and deeper dives into topics that I find particularly interesting or challenging.
*   **`🤖 .scripts/`**: Contains scripts for automating repository maintenance, like the commit graph generator.

---

## 📊 My Recent Commit Activity

This graph automatically updates to show my commit activity over the last 7 days.

<!-- START_COMMIT_GRAPH -->

<!-- END_COMMIT_GRAPH -->

---

## ⚙️ Automation

The commit graph above is updated automatically using a `post-commit` git hook. The hook runs the `.scripts/update_commit_graph.sh` script **only when a commit is made on the `main` branch**. It then regenerates the graph and amends the commit to include the update.

If you clone this repository, you'll need to set up the git hook yourself as they are not tracked by git. You can do this by running the following command from the project root:

```bash
ln -s ../../.scripts/update_commit_graph.sh .git/hooks/post-commit
```

---

## 🤝 Contributing

While this is a personal learning project, I'm open to suggestions and improvements. If you spot an error or have a better way of explaining a concept, feel free to open an issue or a pull request!

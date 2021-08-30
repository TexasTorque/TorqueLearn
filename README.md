# TorqueLearn

_open-source FRC learning_

### What is TorqueLearn?

TorqueLearn is designed to be an aggregation of FRC knowledge to accelerate the induction of new people and maintain a consistent knowledgebase.

### How does it work?

The tutorial documents are sourced from the markdown files in `pages/`. The top-level Rust script (`cargo run`) automatically converts these documents into servable HTML documents--combined with the layout and static files in `layout/`. Connect to [https://127.0.0.1:8000/](https://127.0.0.1:8000/). You may need the nightly Rust version. To get that, run `rustup install nightly` and run with `cargo +nightly run`.

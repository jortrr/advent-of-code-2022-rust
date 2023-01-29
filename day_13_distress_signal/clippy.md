# Rust Clippy statische code analyse

Voeg de volgende code toe aan de top van main.rs of lib.rs
```
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::use_self,
    clippy::needless_return,
    clippy::module_name_repetitions,
    clippy::missing_const_for_fn,
    clippy::doc_markdown
)]
```
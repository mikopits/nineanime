nineanime-rs
============

An HTTP wrapper for 9anime written in Rust.
Quick and dirty library to get download links.

Usage
-----

```rust
extern crate env_logger;
extern crate nineanime;

fn main() {
    env_logger::init().unwrap();

    // Search for anime that match "Mew Mew".
    let matches = nineanime::search("Mew Mew").unwrap();

    // Take the first result of the query. Generally, the result you want will
    // be the first one. You can check the name field to make sure.
    let mew_mew_dub = matches.first().unwrap();

    // `Anime::files` takes a usize representing an episode number. An error
    // is returned if the episode is not found. Files contains a vec of
    // `FileData` which contain the download links.
    let files = mew_mew_dub.files(6).unwrap();

    // Get the direct mp4 link.
    let direct_link = files.default().file;
```

Todos
-----

* Implement queries
* Code looks and reads like garbage

# 🦀 Learn Rust

A repo to track the resources I'm using to learn Rust. This includes:
- examples from the ([interactive](https://rust-book.cs.brown.edu/)) [Rust Book](https://doc.rust-lang.org/book/),
- the [Rustlings](https://github.com/rust-lang/rustlings) exercises, and
- completing challenges from the [CodeCrafters](https://app.codecrafters.io/catalog) series in Rust.

## 🖥️ HTTP Server

See `codecrafters/http-server` for my basic implementation of an HTTP server, following the CodeCrafters challenge.

- Run the server pointing at a local directory with:
```
cargo run -- --directory "/tmp/dir/"
```

- Read/write files in that directory with:
```
curl -X POST http://localhost:4221/files/file_name -d "file_contents"
curl -i http://localhost:4221/files/file_name
```

- Echo text or view your user agent with:
```
curl -i http://localhost:4221/echo/something_to_echo
curl -i http://localhost:4221/user-agent
```

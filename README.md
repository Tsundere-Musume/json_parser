# JSON Parser

A simple JSON parser built-in Rust as a learning project. The goal of this project was to learn about parsing, and Rust in general. This parser currently parses a JSON string into a Rust enum called `JsonValue`.

## JsonValue Enum
```rust
#[derive(Debug, PartialEq)]
enum JsonValue {
    Null,
    Number(f64),
    String(String),
    Bool(bool),
    Array(Vec<JsonValue>),
    Obj(HashMap<String, JsonValue>),
}
```

## TODOS

- [ ] Change `Option` types to `Result` types
- [ ] Better error handling
- [ ] Add escape sequence support to strings
- [ ] Add floating numbers and other number representation support
- [ ] Better printing

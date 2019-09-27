# Accent

The program I use to listen to my keyboard keystrokes and insert accents under
certain conditions.

## Source

- `main.rs` contains the keyboard configuration, read args and bind the keys.
- `structure.rs` defines the structures.
- `hold.rs`, `is_pressed.rs`, `send.rs`, define a trait and implement it for
  the structures that should support it.
- `altcode_from.rs` defines the converstion(s) from character to Windows
  alt-code.

### Dependency

- [Inputbot](https://github.com/obv-mikhail/inputbot) (v0.4.0)

## License

CC0 1.0 Universal / Public domain

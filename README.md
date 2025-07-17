# historic_vec-rs

`vec_historic` is a Rust collection that extends `Vec` with **history tracking**, **undo support**, and **element selection**.  
It's especially useful for implementing editor-like data structures, timelines, or interactive data buffers.

---

## Features

- Efficient insertion and removal (`push_back`, `push_front`, `insert`, etc.)
- History-aware operations with `*_historic` versions (e.g. `insert_historic`, `remove_selects_historic`)
- `undo()` support to revert the last operation
- Select and deselect individual elements by index
- Internally backed by a `GapBuffer` for fast middle insertions

---

## Examples

```rust
use vec_historic::VecHistoric;

let mut b = vec_historic![1, 2, 3, 4, 5, 6, 7, 8, 9];
b.insert_many_historic(5, [1969, 2004]); // 1, 2, 3, 4, 5, 1969, 2004, 6, 7, 8, 9
b.undo() // 1, 2, 3, 4, 5, 6, 7, 8, 9

b.push_back_historic(2004) // 1, 2, 3, 4, 5, 6, 7, 8, 9, 2004
b.push_back_historic(1969) // 1, 2, 3, 4, 5, 6, 7, 8, 9, 2004, 1969
b.undo() // 1, 2, 3, 4, 5, 6, 7, 8, 9, 2004
b.undo() // 1, 2, 3, 4, 5, 6, 7, 8, 9


b.insert(1, 10);
assert_eq!(b, [1, 10, 2, 3]);

b.remove(2);
assert_eq!(b, [1, 10, 3]);
```

## License

This project is dual licensed under Apache-2.0/MIT. See the two LICENSE-\* files for details.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

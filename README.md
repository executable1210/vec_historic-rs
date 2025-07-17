# historic_vec-rs

`vec_historic` is a Rust collection that extends [GapBuffer](https://github.com/frozenlib/gapbuf-rs)(a data structure that enables efficient insertions and deletions near a cursor position by keeping a movable empty region (the "gap") inside a contiguous array.) with **history tracking**, **undo support**, and **element selection**.  
It's especially useful for implementing editor-like data structures, timelines, or interactive data buffers.

---

## Features

- Efficient insertion and removal (`push_back`, `push_front`, `insert`, etc.)
- History-aware operations with `*_historic` versions (e.g. `insert_historic`, `remove_selects_historic`)
- `undo()` support to revert the last operation
- Select and deselect individual elements by index
- Internally backed by a `GapBuffer` for fast middle insertions

---

## General usage

```rust
use vec_historic::VecHistoric;

use crate::vec_historic;

fn main() {
    let mut b: Vec<i32> = vec_historic![1, 2, 3, 4, 5, 6, 7, 8, 9];
    b.insert_many_historic(5, [1996, 2004]); // 1, 2, 3, 4, 5, 1996, 2004, 6, 7, 8, 9
    b.undo(); // 1, 2, 3, 4, 5, 6, 7, 8, 9

    b.push_back_historic(1996); // 1, 2, 3, 4, 5, 6, 7, 8, 9, 1996
    b.push_back_historic(2004); // 1, 2, 3, 4, 5, 6, 7, 8, 9, 1996, 2004
    b.undo(); // 1, 2, 3, 4, 5, 6, 7, 8, 9, 1996
    b.undo(); // 1, 2, 3, 4, 5, 6, 7, 8, 9
}
```

## Using selections

```rust
use vec_historic::VecHistoric;

use crate::vec_historic;

fn main() {
    let mut b: VecHistoric<i32> = vec_historic![1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Select first three elements
    b.select(0);
    b.select(1);
    b.select(2);

    // Remove selected elements from the collection and push the action in history sequence
    let removed: &Vec<i32> = b.remove_selects_historic(); // Returns address of removed elements -> [0, 1, 2]

    println!("{:?}", b); // [3, 4, 5, 6, 7, 8, 9]

    b.undo();

    // Undone elements are selected [0, 1, 2]
    println!("{:?}", b); // [1, 2, 3, 4, 5, 6, 7, 8, 9]

    b.deselect_all(); // deselect all elements
}
```
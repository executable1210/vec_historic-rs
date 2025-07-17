use std::collections::VecDeque;

use gapbuf::GapBuffer;
use indexmap::IndexSet;

use super::vec_historic::VecHistoric;

impl<T> VecHistoric<T> {
    pub fn from_data(data: GapBuffer<T>) -> Self {
        Self {
            data,
            selects: IndexSet::new(),
            history: VecDeque::new(),
        }
    }

    /// Creates an empty collection.
    #[inline(always)]
    pub fn new() -> Self {
        return VecHistoric {
            selects: IndexSet::new(),
            data: GapBuffer::new(),
            history: VecDeque::new(),
        };
    }

    /// Creates an empty collection with the specified capacity (if supported).
    #[inline(always)]
    pub fn with_capacity(cap: usize) -> Self {
        return VecHistoric {
            selects: IndexSet::new(),
            data: GapBuffer::with_capacity(cap),
            history: VecDeque::new(),
        };
    }

    /// Creates a collection from a slice by cloning each element.
    #[inline(always)]
    pub fn from_slice(slice: &[T]) -> Self
    where
        T: Clone,
    {
        return VecHistoric {
            selects: IndexSet::new(),
            data: GapBuffer::from_iter(slice.iter().cloned()),
            history: VecDeque::new(),
        };
    }

    /// Creates a collection from an array.
    #[inline(always)]
    pub fn from_array<const N: usize>(arr: [T; N]) -> Self {
        return VecHistoric {
            selects: IndexSet::new(),
            data: GapBuffer::from_iter(arr),
            history: VecDeque::new(),
        };
    }

    /// Creates a collection with `n` clones of a given value.
    #[inline(always)]
    pub fn repeat(value: T, n: usize) -> Self
    where
        T: Clone,
    {
        return VecHistoric {
            selects: IndexSet::new(),
            data: GapBuffer::from_iter(std::iter::repeat(value).take(n)),
            history: VecDeque::new(),
        };
    }

    /// Converts a slice into a collection (alias to `from_slice`).
    #[inline(always)]
    pub fn to_owned_from(slice: &[T]) -> Self
    where
        T: Clone,
    {
        return Self::from_slice(slice);
    }
}

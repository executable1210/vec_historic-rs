use std::iter::FusedIterator;

use crate::RemoveData;

use super::defines::IntoIter;
use super::vec_historic::VecHistoric;

impl<T> RemoveData<T> {
    pub fn new(len: usize) -> Self {
        Self {
            indecies: Vec::with_capacity(len),
            values: Vec::with_capacity(len),
        }
    }
}

impl<T> IntoIter<T> {
    pub fn new(inner: VecHistoric<T>) -> Self {
        return Self { inner };
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.inner.pop_front()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.inner.len();
        (len, Some(len))
    }
}
impl<T> ExactSizeIterator for IntoIter<T> {}
impl<T> FusedIterator for IntoIter<T> {}
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.inner.pop_back()
    }
}

// impl<'a, T> Iterator for Drain<'a, T> {
//     type Item = T;
//     fn next(&mut self) -> Option<T> {
//         if self.len == 0 {
//             self.inner.clear_history();
//             self.inner.clear_selects();
//             None
//         } else {
//             self.len -= 1;
//             Some(self.inner.data.remove(self.idx))
//         }
//     }
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         (self.len, Some(self.len))
//     }
// }
// impl<'a, T> Drop for Drain<'a, T> {
//     fn drop(&mut self) {
//         let len = self.len;
//         self.nth(len);
//     }
// }

// impl<'a, T> ExactSizeIterator for Drain<'a, T> {}
// impl<'a, T> FusedIterator for Drain<'a, T> {}

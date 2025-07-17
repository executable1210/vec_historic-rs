use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::ops::{Index, IndexMut};

use gapbuf::GapBuffer;
use indexmap::IndexSet;

use super::vec_historic::VecHistoric;
use super::defines::{Iter, IterMut, IntoIter};

impl<T: PartialEq> PartialEq for VecHistoric<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: Eq> Eq for VecHistoric<T> {}

impl<T: PartialOrd> PartialOrd for VecHistoric<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl<T: Ord> Ord for VecHistoric<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.data.cmp(&other.data)
    }
}

impl<T: Hash> Hash for VecHistoric<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl<T> From<GapBuffer<T>> for VecHistoric<T> {
    fn from(other: GapBuffer<T>) -> Self {
        Self { 
            data: GapBuffer::from(other),
            selects: IndexSet::new(),
            history: VecDeque::new()
        }
    }
}

impl<T> FromIterator<T> for VecHistoric<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            data: GapBuffer::from_iter(iter),
            selects: IndexSet::new(),
            history: VecDeque::new()
        }
    }
}

impl<T> Extend<T> for VecHistoric<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.clear_history();
        self.data.extend(iter);
    }
}

impl<T> IntoIterator for VecHistoric<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        // self.data.into_iter()
        IntoIter::new(self)
    }
}

impl<'a, T> IntoIterator for &'a VecHistoric<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut VecHistoric<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

impl<T> Index<usize> for VecHistoric<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx]
    }
}

impl<T> IndexMut<usize> for VecHistoric<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx]
    }
}
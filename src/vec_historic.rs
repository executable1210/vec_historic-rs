use std::collections::VecDeque;

use gapbuf::GapBuffer;
use indexmap::IndexSet;

use super::defines::Action;

#[derive(Debug)]
pub struct VecHistoric<T> {
    pub(super) data: GapBuffer<T>,
    pub(super) selects: IndexSet<usize>,
    pub(super) history: VecDeque<Action<T>>,
}

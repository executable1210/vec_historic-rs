use std::ops::RangeBounds;

use gapbuf::{Drain, GapBuffer};

use super::defines::{Action, InsertData, MoveData, RemoveData};
use super::private::*;
use super::vec_historic::VecHistoric;

impl<T> VecHistoric<T> {
    // #[inline(always)]
    // pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<T, I::IntoIter>
    // where
    //     R: std::ops::RangeBounds<usize>,
    //     I: IntoIterator<Item = T>,
    // {
    //     self.data.splice(range, replace_with)
    // }

    /// Removes the last element from a vector and returns it, or [`None`] if it
    /// is empty.
    /// History and selects are wiped for preventing index shifting
    #[inline(always)]
    pub fn pop_back(&mut self) -> Option<T> {
        self.clear_history(); // to avoid indexs shifting
        self.deselect_all(); // to avoid indexs shifting
        self.data.pop_back()
    }

    /// Removes the last element from a vector and returns it, or [`None`] if it
    /// is empty.
    /// History and selects are wiped for preventing index shifting
    #[inline(always)]
    pub fn pop_front(&mut self) -> Option<T> {
        self.clear_history(); // to avoid indexs shifting
        self.deselect_all(); // to avoid indexs shifting
        self.data.pop_front()
    }

    /// Appends an element to the back of a VecHistoric.
    ///
    /// # Panics
    /// Panics if the number of elements in the VecHistoric overflows a usize.
    #[inline(always)]
    pub fn push_back(&mut self, value: T) {
        self.data.push_back(value);
    }

    /// Appends an element to the front of a VecHistoric.
    /// History and selects are wiped for preventing index shifting
    ///
    /// # Panics
    /// Panics if the number of elements in the VecHistoric overflows a usize.
    #[inline(always)]
    pub fn push_front(&mut self, value: T) {
        self.clear_history(); // to avoid indexs shifting
        self.deselect_all(); // to avoid indexs shifting
        self.data.push_front(value);
    }

    /// Inserts an element at position `index` within the vector
    /// History and selects are wiped for preventing index shifting
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    #[inline(always)]
    pub fn insert(&mut self, index: usize, value: T) {
        if index > self.data.len() {

        }

        self.clear_history(); // to avoid indexs shifting
        self.deselect_all(); // to avoid indexs shifting
        self.data.insert(index, value);
    }

    /// Inserts elements or iterator at position `index` within the vector
    /// History and selects are wiped for preventing index shifting
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    #[inline(always)]
    pub fn insert_many(&mut self, index: usize, iter: impl IntoIterator<Item = T>) {
        self.clear_history(); // to avoid indexs shifting
        self.deselect_all(); // to avoid indexs shifting
        self.data.insert_many(index, iter);
    }

    /// Removes an element from the VecHistoric and returns it.
    /// History and selects are wiped for preventing index shifting
    ///
    /// # Panics
    /// Panics if `index >= self.len()`.
    ///
    /// # Computational amount
    /// `O(n)`, `n = |index - self.gap()|`
    #[inline(always)]
    pub fn remove(&mut self, index: usize) -> T {
        self.clear_history();
        self.deselect_all();
        self.data.remove(index)
    }

    /// Returns the number of elements in the VecHistoric.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the VecHistoric contains no elements.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clears the GapBuffer, removing all values.
    /// History and selects are wiped for preventing index shifting
    ///
    /// Note that this method has no effect on the allocated capacity of the GapBuffer.
    #[inline(always)]
    pub fn clear(&mut self) {
        self.clear_history();
        self.deselect_all();
        self.data.clear();
    }

    /// Creates a draining iterator that removes the specified range in the GapBuffer and yields the removed items.
    /// History and selects are wiped for preventing index shifting
    ///
    /// - Note 1: The element range is removed even if the iterator is only partially consumed or not consumed at all.
    /// - Note 2: It is unspecified how many elements are removed from the GapBuffer if the Drain value is leaked.
    ///
    /// # Panics
    /// Panics if the `range` is out of bounds.
    #[inline(always)]
    pub fn drain(&mut self, range: impl RangeBounds<usize>) -> Drain<'_, T> {
        self.clear_history();
        self.deselect_all();
        self.data.drain(range)
    }

    /// Returns inner gap_buffer.
    #[inline(always)]
    pub fn get_inner_data(&self) -> &GapBuffer<T> {
        return &self.data;
    }

    /// Returns inner gap_buffer.
    #[inline(always)]
    pub fn get_inner_data_mut(&mut self) -> &mut GapBuffer<T> {
        return &mut self.data;
    }
}

impl<T> VecHistoric<T> {
    /// Undo last action in the collection and returns erased elements of it
    /// If history len is 0 OR an action contains no elements returns empty vec 
    pub fn undo(&mut self) -> Vec<T> {
        // returns erased elements
        if self.history.len() <= 0 {
            return vec![];
        }

        let action = self.history.pop_back().unwrap();
        self.deselect_all();
        return self.handle_action(action);
    }

    /// Clears the history and returns all elements of all erased actions
    pub fn clear_history(&mut self) -> Vec<T> {
        let mut values: Vec<T> = Vec::with_capacity(self.compute_history_values_len());

        for action in self.history.drain(..) {
            let taken_values = take_values_from_action(action);
            values.extend(taken_values);
        }

        return values;
    }

    /// Clears selects
    #[inline(always)]
    pub fn clear_selects(&mut self) {
        self.selects.clear();
    }

    /// Returns the count of selected elements
    #[inline(always)]
    pub fn len_selects(&self) -> usize {
        self.selects.len()
    }

    /// Returns the count of actions
    #[inline(always)]
    pub fn len_history(&self) -> usize {
        self.history.len()
    }

    /// Returns the iterator of selections
    #[inline(always)]
    pub fn iter_selects(&self) -> indexmap::set::Iter<'_, usize> {
        self.selects.iter()
    }

    /// Returns the iterator of history
    #[inline(always)]
    pub fn iter_history(&self) -> std::collections::vec_deque::Iter<'_, Action<T>> {
        self.history.iter()
    }

    /// Returns the values of selected elements
    #[inline(always)]
    pub fn get_selected(&self) -> Vec<&T> {
        let selected: Vec<&T> = self.selects.iter().map(|&i| &self.data[i]).collect();

        return selected;
    }

    /// Selects an element by index
    #[inline(always)]
    pub fn select(&mut self, index: usize) {
        self.selects.insert(index);
    }

    /// Deselects an element by index
    #[inline(always)]
    pub fn deselect(&mut self, index: usize) -> bool {
        self.selects.shift_remove(&index)
    }

    /// Returns true if an element is selected
    #[inline(always)]
    pub fn is_selected(&mut self, index: usize) -> bool {
        self.selects.contains(&index)
    }

    /// Deselect all elements
    #[inline(always)]
    pub fn deselect_all(&mut self) {
        self.selects.clear();
    }

    /// Selects all elements
    #[inline(always)]
    pub fn select_all(&mut self) {
        for i in 0..self.data.len() {
            self.selects.insert(i);
        }
    }

    /// Removes the last element from a VecHistoric and returns its address, or [`None`] if it
    /// is empty.
    /// Creates an action in history sequence
    pub fn pop_back_historic(&mut self) -> Option<&T> {
        self.deselect_all(); // to avoid index shifting

        let element = self.data.pop_back()?;

        self.history.push_back(Action::PopBack(element));

        let action = self.history.iter().last().unwrap();

        let Action::PopBack(value) = action else {
            unreachable!()
        };

        return Some(value);
    }

    /// Removes the first element from a VecHistoric and returns its address, or [`None`] if it
    /// is empty.
    /// Creates an action in history sequence
    pub fn pop_front_historic(&mut self) -> Option<&T> {
        self.deselect_all(); // to avoid index shifting

        let element = self.data.pop_front()?;

        self.history.push_back(Action::PopFront(element));

        let action = self.history.iter().last().unwrap();

        let Action::PopFront(value) = action else {
            unreachable!()
        };

        return Some(value);
    }

    /// Appends an element to the back of a VecHistoric.
    /// Creates an action in history sequence
    ///
    /// # Panics
    /// Panics if the number of elements in the VecHistoric overflows a usize.
    #[inline(always)]
    pub fn push_back_historic(&mut self, value: T) {
        self.data.push_back(value);

        self.history.push_back(Action::PushBack);
    }

    /// Appends an element to the front of a VecHistoric.
    /// Creates an action in history sequence
    ///
    /// # Panics
    /// Panics if the number of elements in the VecHistoric overflows a usize.
    #[inline(always)]
    pub fn push_front_historic(&mut self, value: T) {
        self.deselect_all(); // to avoid index shifting

        self.data.push_front(value);

        self.history.push_back(Action::PushFront);
    }

    /// Inserts an element at position `index` within the VecHistoric
    /// Selects the inserted element
    /// Creates an action in history sequence
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    pub fn insert_historic(&mut self, index: usize, value: T) {
        self.deselect_all(); // to avoid index shifting

        self.data.insert(index, value);

        let insert_data = InsertData {
            index: index,
            amount: 1,
        };

        self.select(index);

        self.history.push_back(Action::Insert(insert_data));
    }

    /// Inserts an elements or iterator at position `index` within the VecHistoric
    /// Selects the inserted elements
    /// Creates an action in history sequence
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    pub fn insert_many_historic(&mut self, index: usize, iter: impl IntoIterator<Item = T>) {
        self.deselect_all(); // to avoid index shifting

        let items: Vec<T> = iter.into_iter().collect();
        let amount = items.len();

        self.data.insert_many(index, items);

        let insert_data = InsertData { index, amount };

        for i in 0..amount {
            self.select(index + i);
        }

        self.history.push_back(Action::Insert(insert_data));
    }

    /// Removes selected elements and returns them
    pub fn remove_selects(&mut self) -> Vec<T> {
        let selects = self.get_selects_sorted();

        let mut elems: Vec<T> = Vec::with_capacity(selects.len());

        for aselect in selects.iter().rev() {
            elems.push(self.data.remove(aselect.clone()));
        }

        self.selects.clear();

        return elems;
    }

    /// Removes selected elements and returns address of the removed elements
    /// Creates an action in history sequence
    pub fn remove_selects_historic(&mut self) -> &Vec<T> {
        let selects = self.get_selects_sorted();

        // let mut remove_data: RemoveData<T> = Vec::with_capacity(self.data.len());

        let mut remove_data = RemoveData::new(self.data.len());

        for sel in selects.iter().rev() {
            let sel = sel.clone();
            let elem = self.data.remove(sel);

            // remove_data.push((sel, elem));
            remove_data.indecies.push(sel);
            remove_data.values.push(elem);
        }

        self.history.push_back(Action::Remove(remove_data));
        self.deselect_all();

        let action = self.history.iter().last().unwrap();

        let Action::Remove(remove_data) = action else {
            unreachable!()
        };

        return &remove_data.values;
    }

    /// Moves selected elements to a specific position `index`
    /// History and selects are wiped for preventing index shifting
    pub fn move_selects(&mut self, to_index: usize) {
        let selects = self.get_selects_sorted();

        let mut selected_elements: Vec<T> = Vec::with_capacity(selects.len());

        let mut to_index = to_index;

        for aselect in selects.iter().rev() {
            let item = self.data.remove(aselect.clone());
            selected_elements.push(item);
        }

        selected_elements.reverse();

        if to_index > self.data.len() {
            to_index = self.data.len();
        }

        self.selects.clear();
        self.history.clear();
        for i in 0..selected_elements.len() {
            self.selects.insert(to_index + i);
        }
        self.data.insert_many(to_index, selected_elements);
    }

    /// Moves selected elements to a specific position `index`
    /// Creates an action in history sequence
    pub fn move_selects_historic(&mut self, to_index: usize) {
        let selects = self.get_selects_sorted();

        let mut to_index = to_index;

        if to_index >= self.data.len() {
            to_index = self.data.len() - selects.len();
        }

        let move_data = MoveData {
            dest_index: to_index,
            indecies: selects.clone(),
        };

        self.history.push_back(Action::Move(move_data));
        self.move_selects(to_index);
    }
}

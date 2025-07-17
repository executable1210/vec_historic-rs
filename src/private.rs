use std::collections::VecDeque;

use crate::{RemoveData, vec_historic::VecHistoric};

use super::defines::Action;

#[inline(always)]
pub fn take_values_from_action<T>(action: Action<T>) -> Vec<T> {
    match action {
        Action::PopBack(element) => {
            return vec![element];
        }
        Action::PopFront(element) => {
            return vec![element];
        }
        Action::Insert(_) => {}
        Action::Remove(data) => {
            return data.values;
        }
        Action::PushBack => {}
        Action::PushFront => {}
        Action::Move(_) => {}
    }

    return vec![];
}

impl<T> VecHistoric<T> {
    #[inline(always)]
    pub(super) fn compute_history_values_len(&self) -> usize {
        let mut len = 0;

        for action in self.history.iter() {
            match action {
                Action::PopBack(_) => {
                    len += 1;
                }
                Action::PopFront(_) => len += 1,
                Action::Insert(_) => {}
                Action::Remove(data) => {
                    len += data.values.len();
                }
                Action::PushBack => {}
                Action::PushFront => {}
                Action::Move(_) => {}
            }
        }

        return len;
    }

    #[inline(always)]
    pub(super) fn get_selects_sorted(&self) -> Vec<usize> {
        let mut selects: Vec<usize> = self.selects.iter().copied().collect();

        selects.sort();

        return selects;
    }

    pub(super) fn handle_action(&mut self, action: Action<T>) -> Vec<T> {
        match action {
            Action::PushBack => {
                self.selects.clear();

                let elem = self.data.pop_back().unwrap();

                return vec![elem];
            }
            Action::PopBack(element) => {
                self.selects.clear();

                self.data.push_back(element);
            }
            Action::PushFront => {
                self.selects.clear();

                let elem = self.data.pop_front().unwrap();
                return vec![elem];
            }
            Action::PopFront(element) => {
                self.selects.clear();

                self.data.push_front(element);
            }
            Action::Insert(data) => {
                self.selects.clear();

                let mut res: Vec<T> = Vec::with_capacity(data.amount);

                for i in 0..data.amount {
                    res.push(self.data.remove(data.index));
                }
                return res;
            }
            Action::Remove(data) => {
                self.selects.clear();

                // for (index, element) in data.into_iter().rev() {
                //     self.data.insert(index, element);
                //     self.selects.insert(index);
                // }

                let RemoveData { indecies, values } = data;

                for (index, value) in indecies.into_iter().zip(values).rev() {
                    self.data.insert(index, value);
                    self.selects.insert(index);
                }
            }
            Action::Move(data) => {
                self.selects.clear();

                let mut elements: VecDeque<T> = VecDeque::with_capacity(data.indecies.len());

                for i in 0..data.indecies.len() {
                    let elem = self.data.remove(data.dest_index);
                    elements.push_back(elem);
                }

                for index in data.indecies.iter() {
                    let index = index.clone();

                    let elem = elements.pop_front().unwrap();
                    self.data.insert(index, elem);
                    self.selects.insert(index);
                }
            }
        }

        return vec![];
    }
}

use super::vec_historic::VecHistoric;

// pub struct Drain<'a, T: 'a> {
//     pub(super) inner: &'a mut VecHistoric<T>,
//     pub(super) idx: usize,
//     pub(super) len: usize,
// }

pub struct IntoIter<T> {
    pub(super) inner: VecHistoric<T>,
}

pub type Iter<'a, T> = std::iter::Chain<std::slice::Iter<'a, T>, std::slice::Iter<'a, T>>;
pub type IterMut<'a, T> = std::iter::Chain<std::slice::IterMut<'a, T>, std::slice::IterMut<'a, T>>;

pub type RemoveData<T> = (Vec<(usize, T)>); // index, element

#[derive(Clone, Debug)]
pub struct MoveData {
    pub dest_index: usize,
    pub indecies: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct InsertData {
    pub index: usize,
    pub amount: usize, // amount of inserted elements
}

#[derive(Clone, Debug)]
pub enum Action<T> {
    Remove(RemoveData<T>),
    Move(MoveData),
    Insert(InsertData),
    PushBack,
    PopBack(T),
    PushFront,
    PopFront(T),
}

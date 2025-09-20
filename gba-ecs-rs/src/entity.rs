#[derive(Clone, Copy)]
pub struct Entity {
    pub(crate) index: usize,
    pub(crate) generation: usize,
}

impl Entity {
    pub fn new(index: usize) -> Self {
        Entity {
            index: index,
            generation: 0,
        }
    }
}

use std::slice::Iter;

use crate::grid::Steering;

#[derive(Clone, Copy)]
pub struct Course<'a> {
    steerings: &'a [Steering],
    has_head: bool,
}

impl<'a> Default for Course<'a> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a> Course<'a> {
    pub fn new(steerings: &'a [Steering], has_head: bool) -> Self {
        Self {
            steerings,
            has_head,
        }
    }

    pub fn empty() -> Self {
        Self::new(&[], false)
    }

    pub fn len_segments(&self) -> usize {
        self.steerings.len() * 2 + self.has_head as usize
    }

    pub fn has_head(&self) -> bool {
        self.has_head
    }

    pub fn iter(&self) -> Iter<'a, Steering> {
        self.steerings.iter()
    }
}

use std::ops::{Index, IndexMut};

use tri_mesh::prelude::{Deref, VertexID};


/// Chaotic stupid struct
/// I'll Make it better eventually but for now it is a wrapper around Vec<Option<T>> 
/// and it fill it up to capacity when initialized
pub struct VecStore<T>(pub Vec<Option<T>>);

impl<T> VecStore<T> {
    pub fn new(size: usize) -> VecStore<T> {
        let mut v = Vec::new();
        for _ in 0..size {
            v.push(None)
        }
        VecStore(v)
    }

    pub fn set(&mut self, v_id: VertexID, val: T) {
        let x = &mut self.0[v_id.deref() as usize];
        *x = Some(val);
    }

    pub fn is_set(&self, v_id: VertexID) -> bool {
        let x = &self.0[v_id.deref() as usize];
        match x {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get(&self, v_id: VertexID) -> &T {
        let x = &self.0[v_id.deref() as usize];
        match x {
            Some(t) => t,
            None => panic!("VecStore: Tried to get at an index that was not set (try is_set first)"),
        }
    }

    pub fn get_mut(&mut self, v_id: VertexID) -> &mut T {
        let x = &mut self.0[v_id.deref() as usize];
        match x {
            Some(t) => t,
            None => panic!("VecStore: Tried to get at an index that was not set (try is_set first)"),
        }
    }
}
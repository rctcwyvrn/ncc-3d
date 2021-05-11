use core::fmt::Debug;

use tri_mesh::prelude::{Deref, Mesh, VertexID};

/// Constant sized Vec indexable by VertexID
///
/// Provides methods to get, set, and fill the vector with values
/// Size is constant: should always be the # of verticies in the mesh
///
/// In the future, when we have multiple meshes, I probably want some way of
/// distinguising what v_id comes from which mesh and if it all lines up correctly
/// (Maybe wrapper structs around mesh and vertexID that has a unique mesh_id or something?)
#[derive(Debug, Clone)]
pub struct VecStore<T>(Vec<Option<T>>)
where
    T: Debug + Clone;

impl<T> VecStore<T>
where
    T: Debug + Clone,
{
    pub fn new(mesh: &Mesh) -> VecStore<T> {
        let mut v = Vec::new();
        for _ in 0..mesh.no_vertices() {
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
            None => {
                panic!("VecStore: Tried to get at an index that was not set (try is_set first)")
            }
        }
    }

    pub fn get_mut(&mut self, v_id: VertexID) -> &mut T {
        let x = &mut self.0[v_id.deref() as usize];
        match x {
            Some(t) => t,
            None => {
                panic!("VecStore: Tried to get at an index that was not set (try is_set first)")
            }
        }
    }

    /// Fill in the inner vec with the given value
    pub fn fill_with(&mut self, val: T) {
        for i in 0..self.0.len() {
            self.0[i] = Some(val.clone());
        }
    }
}

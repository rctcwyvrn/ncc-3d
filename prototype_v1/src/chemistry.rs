use tri_mesh::prelude::Mesh;

use crate::{VertexData, storage::VecStore};

// Taken from wave-pinning paper
const K: f64 = 1.0;
const K_0: f64 = 0.067;
const GAMMA: f64 = 1.0;
const DELTA: f64 = 1.0;

pub fn compute_reaction_rate(mesh: &Mesh, conc_data: &VecStore<VertexData>) -> VecStore<f64> {
    let mut rates = VecStore::new(&mesh);
    for v_id in mesh.vertex_iter() {
        let dat = conc_data.get(v_id);
        let a = dat.conc_a;
        let b = dat.conc_b;

        let rate_activ = b * (K_0 + GAMMA * a.powi(2) / (K.powi(2) + a.powi(2)));
        let rate_inactiv = DELTA * a;
        rates.set(v_id, rate_activ - rate_inactiv);
    }

    rates
}

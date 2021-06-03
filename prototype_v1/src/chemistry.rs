use tri_mesh::prelude::Mesh;

use crate::{storage::VecStore, VertexData};

// Taken from wave-pinning paper
// Steady state pair of A,B
// pub const STARTING_A: f64 = 0.2683312;
// pub const STARTING_B: f64 = 2.0;
// const K: f64 = 1.0;
// const K_0: f64 = 0.067;
// const GAMMA: f64 = 1.0;
// const DELTA: f64 = 1.0;
// const V: i32 = 2;

// Testing: What if we have a steeper hill-dependence?
pub const STARTING_A: f64 = 0.1394;
pub const STARTING_B: f64 = 2.0;
const K: f64 = 1.0;
const K_0: f64 = 0.067;
const GAMMA: f64 = 1.0;
const DELTA: f64 = 1.0;
const V: i32 = 3;

// Shvartsmann params
// gamma => K
// beta * k_b => K_0
// V => V
// k_d => DELTA
// k_b => GAMMA
// const K: f64 = 1.3516;
// const K_0: f64 = 0.1 * 16666.67;
// const GAMMA: f64 = 16666.7;
// const DELTA: f64 = 5000.0;
// const V: i8 = 20;


pub fn compute_reaction_rate(mesh: &Mesh, conc_data: &VecStore<VertexData>) -> VecStore<f64> {
    let mut rates = VecStore::new(&mesh);
    for v_id in mesh.vertex_iter() {
        let dat = conc_data.get(v_id);
        let a = dat.conc_a;
        let b = dat.conc_b;

        let rate_activ = b * (K_0 + GAMMA * a.powi(V) / (K.powi(V) + a.powi(V)));
        let rate_inactiv = DELTA * a;
        rates.set(v_id, rate_activ - rate_inactiv);
    }

    rates
}

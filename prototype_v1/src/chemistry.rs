use std::collections::HashMap;

use tri_mesh::prelude::VertexID;

use crate::VertexData;

// Taken from wave-pinning paper
const K: f64 = 1.0;
const K_0: f64 = 0.067;
const GAMMA: f64 = 1.0;
const DELTA: f64 = 1.0;

pub fn compute_reaction_rate(conc_data: &HashMap<VertexID, VertexData>) -> HashMap<VertexID, f64> {
    let mut rates = HashMap::new();
    for (v_id, dat) in conc_data.iter() {
        let a = dat.conc_a;
        let b = dat.conc_b;

        let rate_activ = b * (K_0 + GAMMA * a.powi(2) / (K.powi(2) + a.powi(2)));
        let rate_inactiv = DELTA * a;
        rates.insert(*v_id, rate_activ - rate_inactiv);
    }

    rates
}

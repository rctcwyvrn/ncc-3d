use std::f64::consts::PI;

use crate::{init_vec, N};

// code copied from prototype_v1
// fingers crossed i dont introduce a new bug/fix a bug while migrating this

// Taken from wave-pinning paper
const K: f64 = 1.0;
const K_0: f64 = 0.067;
const GAMMA: f64 = 1.0;
const DELTA: f64 = 1.0;

// pub fn compute_reaction_rate(mesh: &Mesh, conc_data: &VecStore<VertexData>) -> VecStore<f64> {
//     let mut rates = VecStore::new(&mesh);
//     for v_id in mesh.vertex_iter() {
//         let dat = conc_data.get(v_id);
//         let a = dat.conc_a;
//         let b = dat.conc_b;

pub fn compute_reaction_rate(active: &Vec<Vec<f64>>, inactive: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut rates = init_vec(0.0);
    for x in 0..N {
        for z in 0..N {
            let a = active[x][z];
            let b = inactive[x][z];
            let rate_activ = b * (K_0 + GAMMA * a.powi(2) / (K.powi(2) + a.powi(2)));
            let rate_inactiv = DELTA * a;

            rates[x][z] = rate_activ - rate_inactiv;
            // rates.set(v_id, rate_activ - rate_inactiv);
        }
    }

    rates
}

const S_GRAD: f64 = 0.07;
const T1_GRAD: f64 = 20.0;
const T2_GRAD: f64 = 25.0;
const L_GRAD: f64 = 10.0;

const S_LOC: f64 = 0.05;
const T1_LOC: f64 = 20.0;
const T2_LOC: f64 = 25.0;
const L_LOC: f64 = 1.0;

const T0_REV: f64 = 200.0;
const T1_REV: f64 = 220.0;
const T2_REV: f64 = 225.0;

pub type StimFn = fn((f64, f64), f64) -> f64;
pub enum StimTy {
    Gradient,
    Localized,
    Reversal,
    Randomized,
}

pub fn get_stim(ty: StimTy) -> StimFn {
    match ty {
        StimTy::Gradient => {
            let f = |pos: (f64, f64), t| {
                let s = if t <= T1_GRAD {
                    S_GRAD
                } else if t <= T2_GRAD {
                    S_GRAD * (1.0 - (t - T1_GRAD) / (T2_GRAD - T1_GRAD))
                } else {
                    0.0
                };
                s * (L_GRAD - pos.0)
            };
            f
        }
        StimTy::Reversal => {
            let f = |pos: (f64, f64), t| {
                let s = if t <= T1_GRAD {
                    S_GRAD
                } else if t <= T2_GRAD {
                    S_GRAD * (1.0 - (t - T1_GRAD) / (T2_GRAD - T1_GRAD))
                } else if t <= T0_REV {
                    0.0
                } else if t <= T1_REV {
                    S_GRAD
                } else if t <= T2_REV {
                    S_GRAD * (1.0 - (t - T1_REV) / (T2_REV - T1_REV))
                } else {
                    0.0
                };

                if t <= T0_REV {
                    s * (L_GRAD - pos.0)
                } else {
                    s * pos.0
                }

            };
            f
        }
        StimTy::Localized => {
            let f = |pos: (f64, f64), t: f64| {
                let s = if t <= T1_LOC {
                    S_LOC / 2.0
                } else if t <= T2_LOC {
                    (S_LOC / 4.0) * (1.0 + (PI * (t - T1_LOC) / (T2_LOC - T1_LOC)).cos())
                } else {
                    0.0
                };
                if pos.0 <= L_LOC {
                    s * (1.0 + (pos.0 * PI).cos())
                } else {
                    0.0
                }
            };
            f
        }
        StimTy::Randomized => {
            |_, _| 0.0
        }
    }
}

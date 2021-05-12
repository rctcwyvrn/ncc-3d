use std::f64::consts::PI;

use tri_mesh::prelude::Vector3;

const S_GRAD: f64 = 0.07;
const T1_GRAD: f64 = 20.0;
const T2_GRAD: f64 = 25.0;
const L_GRAD: f64 = 10.0;

const S_LOC: f64 = 0.05;
const T1_LOC: f64 = 20.0;
const T2_LOC: f64 = 25.0;
const L_LOC: f64 = 1.0;

pub type StimFn = Box<dyn Fn(Vector3<f64>, f64) -> f64>;
pub enum StimTy {
    Gradient,
    Localized,
}

pub fn get_stim(ty: StimTy) -> StimFn {
    match ty {
        StimTy::Gradient => {
            let f = |pos: Vector3<f64>, t| {
                let s = if t <= T1_GRAD {
                    S_GRAD
                } else if t <= T2_GRAD {
                    S_GRAD * (1.0 - (t - T1_GRAD) / (T2_GRAD - T1_GRAD))
                } else {
                    0.0
                };
                s * (L_GRAD - pos.x)
            };
            Box::new(f)
        }
        StimTy::Localized => {
            let f = |pos: Vector3<f64>, t: f64| {
                let s = if t <= T1_LOC {
                    S_LOC / 2.0
                } else if t <= T2_LOC {
                    (S_LOC / 4.0) * (1.0 + (PI * (t - T1_LOC) / (T2_LOC - T1_LOC)).cos())
                } else {
                    0.0
                };
                if pos.x <= L_LOC {
                    s * (1.0 + (pos.x * PI).cos())
                } else {
                    0.0
                }
            };
            Box::new(f)
        }
    }
}

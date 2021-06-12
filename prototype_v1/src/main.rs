use std::{f64::consts::PI, fs, ops::{Div, Mul, Neg}};

use eyre::Result;
use plotting::{plot_data, GraphTy};
use stim::{StimFn, StimTy};
use storage::VecStore;
use tri_mesh::{MeshBuilder, prelude::{Mesh, VertexID}};
use rand::prelude::*;

use crate::chemistry::{STARTING_A, STARTING_B};

mod chemistry;
mod laplacian;
mod plotting;
mod stim;
mod storage;

const TS: f64 = 0.001;

// const FINAL_TIME: f64 = 100.0; // Full convergence of 2D surface
// const FINAL_TIME: f64 = 470.0; // Full convergence of icosahedron
// const FINAL_TIME: f64 = 140.0; // Convergence of sphere
// const FINAL_TIME: f64 = 350.0; // Testing steady_state_tol
const FINAL_TIME: f64 = 0.01; // Testing heat

// At what point should we stop and assume we've reached a steady state?
// Completely arbitrary and messy, but it's better than just having a final time and hoping that its enough
const STEADY_STATE_TOL: f64 = 0.0000000001;
const SNAPSHOT_PERIOD: usize = 1;

// Diffusivity constants
// const D_A: f64 = 0.1;
const D_A: f64 = 1.0; // FOR TESTING HEAT EQ ONLY
const D_B: f64 = 10.0;

#[derive(Debug, Clone, Copy)]
pub struct VertexData {
    conc_a: f64,
    conc_b: f64,
}

fn main() -> Result<()> {
    let mesh_filename = "mesh.obj";

    let obj_source = fs::read_to_string(mesh_filename)?;
    let mesh = MeshBuilder::new().with_obj(obj_source).build().unwrap();

    // Testing
    let mut total = 0.0;
    for e_id in mesh.edge_iter() {
        total += mesh.edge_length(e_id);
    };
    let epsilon = total / mesh.no_edges() as f64;
    eprintln!("Epsilon = {}", epsilon);

    let mut conc_data = VecStore::new(&mesh);
    mesh.vertex_iter().for_each(|v_id| {
        // let data = VertexData {
        //     conc_a: STARTING_A,
        //     conc_b: STARTING_B,
        // };

        // TESTING: RANDOM STARTING VALUES => TEST DIFFUSION
        // let mut rng = rand::thread_rng();
        // let data = VertexData {
        //     conc_a: rng.gen_range(0.0..2.0),
        //     conc_b: rng.gen_range(0.0..2.0),
        // };

        // TESTING: initialize based on known function which we know we can compute the laplacian correctly for
        // let pos = mesh.vertex_position(v_id);
        // let data = VertexData {
        // //     // conc_a: pos.x.exp() / 10_f64.exp(),
        // //     // conc_b: pos.x.exp() / 10_f64.exp(),

        // //     // conc_a: pos.x.exp(),
        // //     // conc_b: pos.x.exp(),

        // //     // conc_a: (pos.x - 1.0).exp(),
        // //     // conc_b: (pos.x - 1.0).exp(),

        //     conc_a: (pos.x - 5.0).div(5.0).exp(),
        //     conc_b: (pos.x - 5.0).div(5.0).exp(),
        // };

        // TESTING: 1D diffusion on planar mesh
        // Pulse
        let f = |x: f64| if (x - 0.5).abs() <= 0.1 {
            // println!("hi");
            10.0
        } else {
            0.0
        };

        // let f = |x: f64| if (x - 0.5) <= 0.0 {
        //     2.0*x
        // } else {
        //     2.0 - 2.0*x
        // };

        let pos = mesh.vertex_position(v_id);
        let data = VertexData {
            conc_a: f(pos.x),
            conc_b: 0.0,
        };

        conc_data.set(v_id, data);
    });

    let stim_fn = stim::get_stim(StimTy::Gradient);
    // let stim_fn = stim::get_stim(StimTy::Localized);

    plot_data(&mesh, &conc_data, GraphTy::Initial);
    simulate(&mesh, &mut conc_data, stim_fn);
    plot_data(&mesh, &conc_data, GraphTy::Final);

    Ok(())
}

fn simulate(mesh: &Mesh, conc_data: &mut VecStore<VertexData>, stim_fn: StimFn) {
    let mut t: f64 = 0.0;
    for i in 0..(FINAL_TIME / TS).round() as usize {
        // Compute diffusion laplacians
        let mut conc_a_data = VecStore::new(&mesh);
        let mut conc_b_data = VecStore::new(&mesh);

        for v_id in mesh.vertex_iter() {
            conc_a_data.set(v_id, conc_data.get(v_id).conc_a);
            conc_b_data.set(v_id, conc_data.get(v_id).conc_b);
        }

        let lapl_a = laplacian::compute_laplacian(mesh, &conc_a_data);
        // let lapl_b = laplacian::compute_laplacian(mesh, &conc_b_data);

        // Compute reaction rate
        let rate_activ = chemistry::compute_reaction_rate(mesh, &conc_data);

        // Debugging: to see if total concentration is still not conserved
        let mut total_a = 0.0;
        let mut total_b = 0.0;

        let mut d_total = 0.0; // To determine when steady state has been reached
        
        let mut l2_num = 0.0;
        let mut l2_denom = 0.0;

        // Step each vertex
        for v_id in mesh.vertex_iter() {
            // Compute the external stimulation
            let pos = mesh.vertex_position(v_id);

            if pos.x <= 0.0 || pos.x >= 1.0 || pos.z.abs() > 1.0 {
                let new_a = impose_bc(mesh, v_id, conc_data, t);
                let dat = conc_data.get_mut(v_id);
                dat.conc_a = new_a;
                // if (i % SNAPSHOT_PERIOD) == 0 {
                //     println!(
                //         "DEBUG: ({},{},{}) ({}) {}",
                //         pos.x,
                //         pos.y,
                //         pos.z,
                //         dat.conc_a,
                //         "BC",
                //     );
                // }
            } else {
                let dat = conc_data.get_mut(v_id);
                let d_a = D_A * lapl_a.get(v_id);
                dat.conc_a += TS * d_a;
                d_total = d_a.abs();
                
                // Debug logging
                if (i % SNAPSHOT_PERIOD) == 0 {
                    // if true {
                    println!(
                        "DEBUG: ({},{},{}) ({}) {}",
                        pos.x,
                        pos.y,
                        pos.z,
                        dat.conc_a,
                        D_A * lapl_a.get(v_id),
                    );
                    total_a += dat.conc_a;
                    total_b += dat.conc_b;
                }

                // Get errors for the internal region
                let (num, denom) = compare(dat.conc_a, pos.x, t + TS);
                l2_num += num;
                l2_denom += denom;
            }

            // Testing: log data before stepping
            // dat.conc_a += TS * d_a;
            // dat.conc_b += TS * d_b;
            // d_total += d_a.abs() + d_b.abs();

        }
        if (i % SNAPSHOT_PERIOD) == 0 {
            // plot_data(mesh, conc_data, GraphTy::Intermediate(t.mul(1000.0).round().div(1000.0)));
            plot_data(mesh, conc_data, GraphTy::Intermediate(t));

            println!("Total active = {}", total_a);
            println!("Total inactive = {}", total_b);
            println!("Full total = {}", total_a + total_b);
            println!("d_total = {}", d_total);
        }

        if d_total < STEADY_STATE_TOL {
            println!("!! Steady state reached: Stopping at t = {}", t);
            return;
        }

        eprintln!("L2 error at t = {} | {}", t, l2_num.sqrt()/&l2_denom.sqrt());
        t += TS;
    }

    println!(
        "Did not reach steady state: Stopping at final time t = {}",
        t
    );

    // println!("Final conc data:");
    // print_conc_data(mesh, &conc_data);
}

fn impose_bc(mesh: &Mesh, v_id: VertexID, _conc_data: &VecStore<VertexData>, t: f64) -> f64 {
    let pos = mesh.vertex_position(v_id);
    if pos.z.abs() > 1.0 && pos.x > 0.0 && pos.x < 1.0 {
        get_analytical_heat(pos.x, t)
    } else {
        0.0
    }
}

fn get_analytical_heat(x: f64, t: f64) -> f64 {
    // Triangle
    // let b = |m: f64| {
    //     4.0 * (2.0 * ((m*PI / 2.0).sin()) - (m*PI).sin()).div(m.mul(PI).powi(2))
    // };
    
    // Delta
    // let b = |m: f64| {
    //     2.0 * (m*PI*0.5).sin()
    // };

    // Pulse
    let b = |m: f64| {
        (20.0 / (m*PI)) * ((0.4*m*PI).cos() - (0.6*m*PI).cos())
    };

    let mut val = 0.0;
    for n in 1..10 {
        let n = n as f64;
        val += b(n) * (n*PI*x).sin() * (n.powi(2).neg() * PI.powi(2) * t).exp()
    }

    // (4.0 * PI * D_A * t).sqrt().recip() * x.powi(2).neg().div(4.0 * D_A * t).exp()
    // 0.0
    val
}

fn compare(sim: f64, x: f64, t: f64) -> (f64, f64){
    let real = get_analytical_heat(x, t);
    ((sim - real).powi(2), real.powi(2))
}
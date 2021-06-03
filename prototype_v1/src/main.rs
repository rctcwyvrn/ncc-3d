use std::fs;

use eyre::Result;
use plotting::{plot_data, GraphTy};
use stim::{StimFn, StimTy};
use storage::VecStore;
use tri_mesh::{prelude::Mesh, MeshBuilder};
use rand::prelude::*;

use crate::chemistry::{STARTING_A, STARTING_B};

mod chemistry;
mod laplacian;
mod plotting;
mod stim;
mod storage;

const TS: f64 = 0.01;

// const FINAL_TIME: f64 = 100.0; // Full convergence of 2D surface
// const FINAL_TIME: f64 = 470.0; // Full convergence of icosahedron
// const FINAL_TIME: f64 = 140.0; // Convergence of sphere
const FINAL_TIME: f64 = 350.0; // Testing steady_state_tol

// At what point should we stop and assume we've reached a steady state?
// Completely arbitrary and messy, but it's better than just having a final time and hoping that its enough
const STEADY_STATE_TOL: f64 = 0.01;
const SNAPSHOT_PERIOD: usize = 500;

// Diffusivity constants
const D_A: f64 = 0.1;
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

    let mut conc_data = VecStore::new(&mesh);
    mesh.vertex_iter().for_each(|v_id| {
        // let data = VertexData {
        //     conc_a: STARTING_A,
        //     conc_b: STARTING_B,
        // };

        // TESTING: RANDOM STARTING VALUES => TEST DIFFUSION
        let mut rng = rand::thread_rng();
        let data = VertexData {
            conc_a: rng.gen_range(0.0..2.0),
            conc_b: rng.gen_range(0.0..2.0),
        };

        conc_data.set(v_id, data);
    });

    let stim_fn = stim::get_stim(StimTy::Gradient);
    // let stim_fn = stim::get_stim(StimTy::Localized);

    plot_data(&mesh, &conc_data, GraphTy::Intermediate(0.0));
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
        let lapl_b = laplacian::compute_laplacian(mesh, &conc_b_data);

        // Compute reaction rate
        let rate_activ = chemistry::compute_reaction_rate(mesh, &conc_data);

        // Debugging: to see if total concentration is still not conserved
        let mut total_a = 0.0;
        let mut total_b = 0.0;

        let mut d_total = 0.0; // To determine when steady state has been reached
                               // Step each vertex
        for v_id in mesh.vertex_iter() {
            // Compute the external stimulation
            let pos = mesh.vertex_position(v_id);
            let stim_k = stim_fn(pos, t);

            // Finally collect all the data together and step the diff eq
            let dat = conc_data.get_mut(v_id);
            let b = dat.conc_b;
            let r = rate_activ.get(v_id);

            // let d_a = D_A * lapl_a.get(v_id) + (r + stim_k * b);
            // let d_b = D_B * lapl_b.get(v_id) - (r + stim_k * b);
            
            // TESTING: NO REACTION, ONLY DIFFUSION + STIM
            // let d_a = D_A * lapl_a.get(v_id) + (stim_k * b);
            // let d_b = D_B * lapl_b.get(v_id) - (stim_k * b);

            // ONLY DIFFUSION
            let d_a = D_A * lapl_a.get(v_id);
            let d_b = D_B * lapl_b.get(v_id);


            dat.conc_a += TS * d_a;
            dat.conc_b += TS * d_b;

            d_total += d_a.abs() + d_b.abs();

            // Debug logging
            if (i % SNAPSHOT_PERIOD) == 0 {
                // if true {
                println!(
                    "DEBUG: ({},{},{}) ({},{}) {} | {} | {} | {}",
                    pos.x,
                    pos.y,
                    pos.z,
                    dat.conc_a,
                    dat.conc_b,
                    D_A * lapl_a.get(v_id),
                    D_B * lapl_b.get(v_id),
                    r,
                    stim_k * b,
                );
                total_a += dat.conc_a;
                total_b += dat.conc_b;
            }
        }
        if (i % SNAPSHOT_PERIOD) == 0 {
            plot_data(mesh, conc_data, GraphTy::Intermediate(t.round()));

            println!("Total active = {}", total_a);
            println!("Total inactive = {}", total_b);
            println!("Full total = {}", total_a + total_b);
            println!("d_total = {}", d_total);
        }

        if d_total < STEADY_STATE_TOL {
            println!("!! Steady state reached: Stopping at t = {}", t);
            return;
        }

        t += TS;
    }

    println!(
        "Did not reach steady state: Stopping at final time t = {}",
        t
    );

    // println!("Final conc data:");
    // print_conc_data(mesh, &conc_data);
}

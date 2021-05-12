use std::fs;

use eyre::Result;
use plotting::{plot_data, GraphTy};
use stim::{StimFn, StimTy};
use storage::VecStore;
use tri_mesh::{prelude::Mesh, MeshBuilder};

mod chemistry;
mod laplacian;
mod plotting;
mod stim;
mod storage;

const TS: f64 = 0.01;
// const FINAL_TIME: f64 = 100.0; // Full convergence of 2D surface
// const FINAL_TIME: f64 = 470.0; // Full convergence of icosahedron
const FINAL_TIME: f64 = 120.0; // Convergence of sphere

const SNAPSHOT_PERIOD: usize = 500;

// Steady state pair of A,B
const STARTING_A: f64 = 0.2683312;
const STARTING_B: f64 = 2.0;

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
        let data = VertexData {
            conc_a: STARTING_A,
            conc_b: STARTING_B,
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
        if (i % SNAPSHOT_PERIOD) == 0 {
            // Periodic plots
            plot_data(mesh, conc_data, GraphTy::Intermediate(t.round()));
        }

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
        for v_id in mesh.vertex_iter() {
            let pos = mesh.vertex_position(v_id);
            let stim_k = stim_fn(pos, t);
            let dat = conc_data.get(v_id);
            let b = dat.conc_b;

            let dat = conc_data.get_mut(v_id);
            let r = rate_activ.get(v_id);
            if (i % SNAPSHOT_PERIOD) == 0 {
                let pos = mesh.vertex_position(v_id);
                println!(
                    "DEBUG: ({},{},{}) ({},{}) {} | {} | {}",
                    pos.x,
                    pos.y,
                    pos.z,
                    dat.conc_a,
                    dat.conc_b,
                    D_A * lapl_a.get(v_id),
                    D_B * lapl_b.get(v_id),
                    r,
                );
                total_a += dat.conc_a;
                total_b += dat.conc_b;
            }

            dat.conc_a += TS * (D_A * lapl_a.get(v_id) + (r + stim_k * b));
            dat.conc_b += TS * (D_B * lapl_b.get(v_id) - (r + stim_k * b));
        }

        if (i % SNAPSHOT_PERIOD) == 0 {
            println!("Total active = {}", total_a);
            println!("Total inactive = {}", total_b);
            println!("Full total = {}", total_a + total_b);
        }
        t += TS;
    }

    // println!("Final conc data:");
    // print_conc_data(mesh, &conc_data);
}

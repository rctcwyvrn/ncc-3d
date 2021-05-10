use std::{collections::HashMap, fs};

use eyre::Result;
use plotting::{plot_data, GraphTy};
use stim::{StimFn, StimTy};
use tri_mesh::{
    prelude::Mesh,
    prelude::VertexID,
    MeshBuilder,
};

mod chemistry;
mod laplacian;
mod plotting;
mod stim;

const TS: f64 = 0.01;
const FINAL_TIME: f64 = 200.0;

// Steady state pair of A,B
const STARTING_A: f64 = 0.2683312;
const STARTING_B: f64 = 2.0;

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

    let mut conc_data = HashMap::new();
    mesh.vertex_iter().for_each(|v_id| {
        let data = VertexData {
            conc_a: STARTING_A,
            conc_b: STARTING_B,
        };
        conc_data.insert(v_id, data);
    });

    let stim_fn = stim::get_stim(StimTy::Gradient);
    // let stim_fn = stim::get_stim(StimTy::Localized);

    plot_data(&mesh, &conc_data, GraphTy::Intermediate(0.0));
    simulate(&mesh, &mut conc_data, stim_fn);
    plot_data(&mesh, &conc_data, GraphTy::Final);

    Ok(())
}

fn simulate(mesh: &Mesh, conc_data: &mut HashMap<VertexID, VertexData>, stim_fn: StimFn) {
    let mut t: f64 = 0.0;
    for i in 0..(FINAL_TIME / TS).round() as usize {
        if (i % 1000) == 0 {
            // Periodic plots
            plot_data(mesh, conc_data, GraphTy::Intermediate(t.round()));
        }

        // Compute diffusion laplacians
        let conc_a_data = conc_data
            .iter()
            .map(|(id, dat)| (*id, dat.conc_a))
            .collect();
        let conc_b_data = conc_data
            .iter()
            .map(|(id, dat)| (*id, dat.conc_b))
            .collect();
        let lapl_a = laplacian::compute_laplacian(mesh, &conc_a_data);
        let lapl_b = laplacian::compute_laplacian(mesh, &conc_b_data);

        // Compute reaction rate
        let rate_activ = chemistry::compute_reaction_rate(&conc_data);

        for v_id in mesh.vertex_iter() {
            let pos = mesh.vertex_position(v_id);
            let stim_k = stim_fn(pos, t);
            let dat = conc_data[&v_id];
            let b = dat.conc_b;

            let dat = conc_data.get_mut(&v_id).unwrap();
            let r = rate_activ[&v_id];
            if (i % 1000) == 0 {
                let pos = mesh.vertex_position(v_id);
                println!(
                    "DEBUG: ({},{},{}) ({},{}) {} | {} | {}",
                    pos.x,
                    pos.y,
                    pos.z,
                    dat.conc_a,
                    dat.conc_b,
                    D_A * lapl_a[&v_id],
                    D_B * lapl_b[&v_id],
                    r,
                );
            }

            dat.conc_a += TS * (D_A * lapl_a[&v_id] + (r + stim_k * b));
            dat.conc_b += TS * (D_B * lapl_b[&v_id] - (r + stim_k * b));
        }
        t += TS;
    }

    // println!("Final conc data:");
    // print_conc_data(mesh, &conc_data);
}

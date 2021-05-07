use std::{collections::HashMap, fs};

use eyre::Result;
use tri_mesh::{MeshBuilder, prelude::Mesh, prelude::{Vector3, VertexID}};

mod laplacian;
mod chemistry;

const TS: f64 = 0.01;

// Steady state pair of A,B
const STARTING_A: f64 = 0.268;
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
    let stim_filename = "stim_gradient.dat";

    let obj_source = fs::read_to_string(mesh_filename)?;
    let mesh = MeshBuilder::new().with_obj(obj_source).build().unwrap();

    let mut conc_data = HashMap::new();
    mesh.vertex_iter()
        .for_each(|v_id| {
            let data = VertexData { conc_a: STARTING_A, conc_b: STARTING_B};
            conc_data.insert(v_id, data);
        });

    println!("Loaded mesh from {} with vertices: ", mesh_filename);
    for v_id in  mesh.vertex_iter() {
        let dat = conc_data[&v_id];
        println!("{}: {:?} ({}, {})", v_id, mesh.vertex_position(v_id), dat.conc_a, dat.conc_b);
    }
    println!("Num edges = {}", mesh.no_edges());

    let stim_str = fs::read_to_string(stim_filename)?;
    let stimulation: Vec<_> = stim_str
        .lines()
        .map(|s| {
            // let parts: Vec<&str> = s.split(" ").collect();
            // let v_idx = 
            // let stim = str::parse::<f64>(parts[1]).unwrap();
            str::parse::<f64>(s).unwrap()
        })
        .collect();
    println!("Using stimulation data from {} | {:?}", stim_filename, stimulation);

    simulate(mesh, conc_data, stimulation);
    Ok(())
}

/// Print the data out 
fn do_print(mut data: Vec<(f64, Vector3<f64>)>) -> f64 {
    data.sort_by(|(_, p1), (_, p2)| p1.x.partial_cmp(&p2.x).unwrap());
    let mut line_0 = "      ".to_string();
    let mut line_1 = String::new();
    let mut line_2 = "      ".to_string();
    let mut total = 0.0;
    for (x, pos) in data.iter() {
        let mut s = (*x.to_string()).to_string();
        s.truncate(4);

        if pos.y > 0.0 {
            line_0 += "(";
            line_0 += &s;
            line_0 += ")";
            line_0 += "      ";
        } else if pos.y < 0.0 {
            line_2 += "(";
            line_2 += &s;
            line_2 += ")";
            line_2 += "      ";
        } else {
            line_1 += "(";
            line_1 += &s;
            line_1 += ")";
            line_1 += "      ";
        }
        total += *x;
    }

    println!("{}", line_0);
    println!("{}", line_1);
    println!("{}", line_2);
    println!("---");

    total
}

fn print_conc_data(mesh: &Mesh, conc_data: &HashMap<VertexID, VertexData>) {
    println!("-------------------------");
    let a_data: Vec<_> = conc_data.iter().map(|(id, x)| (x.conc_a, mesh.vertex_position(*id))).collect();
    let total_a = do_print(a_data);

    let b_data: Vec<_> = conc_data.iter().map(|(id, x)| (x.conc_b, mesh.vertex_position(*id))).collect();
    let total_b = do_print(b_data);


    println!("Total conc of A: {}", total_a);
    println!("Total conc of B: {}", total_b);
    println!("System total = {}", total_a + total_b);
    println!("-------------------------");

}

fn simulate(mesh: Mesh, mut conc_data: HashMap<VertexID, VertexData>, stimulation: Vec<f64>) {
    for i in 0..(100.0 / TS).round() as usize {
        if (i % 10) == 0 {
            println!("T = {}", (i as f64) * TS);
            print_conc_data(&mesh, &conc_data);
        }

        // Compute diffusion laplacians
        let conc_a_data = conc_data.iter().map(|(id, dat)| (*id, dat.conc_a)).collect();
        let conc_b_data = conc_data.iter().map(|(id, dat)| (*id, dat.conc_b)).collect();
        let lapl_a = laplacian::compute_laplacian(&mesh, &conc_a_data);
        let lapl_b = laplacian::compute_laplacian(&mesh, &conc_b_data);

        // Compute reaction rate
        let mut rate_activ = chemistry::compute_reaction_rate(&conc_data);

        if i == 9 {
            // Give input stimulation
            println!("Applying input stimulation");
            for (v_id, stim_k) in mesh.vertex_iter().zip(stimulation.iter()) {
                let dat = conc_data[&v_id];
                let b = dat.conc_b;
                let r = rate_activ.get_mut(&v_id).unwrap();
                *r += stim_k * b;
            }
        }

        for v_id in mesh.vertex_iter() {
            let dat = conc_data.get_mut(&v_id).unwrap();
            if (i % 10) == 0 {
                println!("DEBUG: ({},{}) {} | {}", dat.conc_a, dat.conc_b, D_A * lapl_a[&v_id], rate_activ[&v_id]);
            }
            dat.conc_a += TS * (D_A * lapl_a[&v_id] + rate_activ[&v_id]);
            dat.conc_b += TS * (D_B * lapl_b[&v_id] - rate_activ[&v_id]);
        }
    }

    println!("Final conc data:");
    print_conc_data(&mesh, &conc_data);
}
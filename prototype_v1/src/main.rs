use std::fs;

use eyre::Result;
use plotting::{plot_data, GraphTy};
use stim::{StimFn, StimTy};
use storage::VecStore;
use tri_mesh::{
    prelude::{Mesh, Vector3},
    MeshBuilder,
};

mod chemistry;
mod cotangent_laplacian;
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

    // // f(x,z) = x^2
    // let f = |pos: Vector3<f64>| pos.x.powi(2);
    // let soln = |_: Vector3<f64>| 2.0;

    // // f(x,z) = x^2 + z^2
    // let f = |pos: Vector3<f64>| pos.x.powi(2) + pos.z.powi(2);
    // let soln = |_: Vector3<f64>| 4.0;

    // f(x,z) = e^(x+z)
    // let f = | pos: Vector3<f64> | {
    //     (pos.x + pos.z).exp()
    // };

    // let soln = | pos: Vector3<f64> | {
    //     2.0 * (pos.x + pos.z).exp()
    // };

    // Spherical
    // f(x,y,z) = x
    // let f = |pos: Vector3<f64>| pos.x;
    // let soln = |pos: Vector3<f64>| {
    //     let theta = (pos.x.powi(2) + pos.y.powi(2)).sqrt().atan2(pos.z);
    //     let phi = pos.y.atan2(pos.x);

    //     phi.cos() / theta.sin() * (2.0 * theta).cos() - phi.cos() / theta.sin()
    // };

    // f(x,y,z) = x^2
    // let f = |pos: Vector3<f64>| pos.x.powi(2);
    // let soln = |pos: Vector3<f64>| {
    //     let theta = (pos.x.powi(2) + pos.y.powi(2)).sqrt().atan2(pos.z);
    //     let phi = pos.y.atan2(pos.x);

    //     2.0 * phi.cos().powi(2) * (2.0 * theta.cos().powi(2) - theta.sin().powi(2))
    //         - 2.0 * (2.0 * phi).cos()
    // };
    
    // f(x,y,z) = e^x
    let f = |pos: Vector3<f64>| pos.x.exp();
    let soln = |pos: Vector3<f64>| {
        let theta = (pos.x.powi(2) + pos.y.powi(2)).sqrt().atan2(pos.z);
        let phi = pos.y.atan2(pos.x);

        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();

        (sin_theta * cos_phi).exp() / sin_theta * 
        ( 
            cos_phi.powi(2) * cos_theta/2.0 * (2.0*theta).sin() + cos_phi * (2.0*theta).cos() + 
            sin_phi.powi(2) * sin_theta - cos_phi
        )
    };

    let mut test_data = VecStore::new(&mesh);
    mesh.vertex_iter().for_each(|v_id| {
        let pos = mesh.vertex_position(v_id);
        let data = f(pos);
        test_data.set(v_id, data);
    });

    let lapl_belkin = laplacian::compute_laplacian(&mesh, &test_data);
    // let lapl_cotan = cotangent_laplacian::compute_laplacian(&mesh, &test_data);

    // let mut errors = Vec::new();
    let mut l2_error_num = 0.0;
    let mut l2_error_denom = 0.0;

    for v_id in mesh.vertex_iter() {
        let pos = mesh.vertex_position(v_id);
        let s = soln(pos);

        let val_belkin = lapl_belkin.get(v_id);
        // errors.push((pos, (val_belkin - s))); // graph all errors

        // eprintln!("({},{},{}), {}", pos.x, pos.y, pos.z, val_belkin - s);

        if !s.is_nan() { 
            l2_error_num += (s - val_belkin).powi(2);
            l2_error_denom += s.powi(2);
        }

        // if pos.x < 0.25 || pos.x > 0.75 || pos.z < 0.25 || pos.z > 0.75 { // Rectangle
        // if pos.x < 0.25 || pos.x > 0.75 || pos.z > 0.25 || pos.z < -0.25 { // Rhombus
        if false {
            // Sphere
            continue;
        } else {
            println!(
                // "A ({},{},{}) {} | {} | {} | {}",
                "{},{},{},{},{},{},{}",
                pos.x,
                pos.y,
                pos.z,
                val_belkin,
                test_data.get(v_id),
                s,
                (s - val_belkin).abs()
            );

            // errors.push((pos, (val_belkin - s))); // graph only the errors on the interior

            // let val = lapl_cotan.get(v_id);
            // eprintln!(
            //     // "B ({},{},{}) {} | {} | {} | {}",
            //     "{},{},{},{},{},{},{}",
            //     pos.x,
            //     pos.y,
            //     pos.z,
            //     val,
            //     test_data.get(v_id),
            //     s,
            //     (s - val).abs()
            // );
        }
    }

    eprintln!("L2 error = {}", l2_error_num.sqrt() / &l2_error_denom.sqrt());
    // plotting::plot_lapl_error(errors);

    Ok(())

    // let mut conc_data = VecStore::new(&mesh);
    // mesh.vertex_iter().for_each(|v_id| {
    //     let data = VertexData {
    //         conc_a: STARTING_A,
    //         conc_b: STARTING_B,
    //     };
    //     conc_data.set(v_id, data);
    // });

    // // let stim_fn = stim::get_stim(StimTy::Gradient);
    // let stim_fn = stim::get_stim(StimTy::Localized);

    // plot_data(&mesh, &conc_data, GraphTy::Intermediate(0.0));
    // simulate(&mesh, &mut conc_data, stim_fn);
    // plot_data(&mesh, &conc_data, GraphTy::Final);

    // Ok(())
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

            let d_a = D_A * lapl_a.get(v_id) + (r + stim_k * b);
            let d_b = D_B * lapl_b.get(v_id) - (r + stim_k * b);

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

        if d_total < STEADY_STATE_TOL {
            println!("!! Steady state reached: Stopping at t = {}", t);
            return;
        }

        if (i % SNAPSHOT_PERIOD) == 0 {
            plot_data(mesh, conc_data, GraphTy::Intermediate(t.round()));

            println!("Total active = {}", total_a);
            println!("Total inactive = {}", total_b);
            println!("Full total = {}", total_a + total_b);
            println!("d_total = {}", d_total);
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

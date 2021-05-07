use std::{collections::HashMap, fs};

use eyre::Result;
use tri_mesh::{MeshBuilder, prelude::Mesh, prelude::{VertexID}};

mod laplacian;

const TS: f64 = 0.1;
const D: f64 = 0.75;

// pub struct VertexData {
//     conc_a: f64,
// }

fn main() -> Result<()> {
    // let positions: Vec<f64> = vec![0.0, 0.0, 0.0, 0.5, 0.8660254037844386, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5, -0.8660254037844386, 0.0];    let builder = MeshBuilder::new();
    // let mesh = builder.with_positions(positions).build().unwrap();

    // let positions: Vec<f64> = vec![0.0, 0.0, 0.0, 0.5, 0.8660254037844386, 0.0, 0.5, -0.8660254037844386, 0.0, 1.0, 0.0, 0.0, 1.5, 0.8660254037844386, 0.0, 1.5, -0.8660254037844386, 0.0, 2.0, 0.0, 0.0, 2.5, 0.8660254037844386, 0.0, 2.5, -0.8660254037844386, 0.0, 3.0, 0.0, 0.0];
    // let indicies: Vec<u32> = vec![0, 1, 3, 0, 3, 2, 3, 4, 6, 3, 6, 5, 1, 4, 3, 3, 5, 2, 6, 7, 9, 6, 9, 8, 4, 7, 6, 6, 8, 5];
    // let mesh = MeshBuilder::new().with_indices(indicies).with_positions(positions).build().unwrap();

    let obj_source = fs::read_to_string("mesh.obj")?;
    let mesh = MeshBuilder::new().with_obj(obj_source).build().unwrap();
    let conc_str = fs::read_to_string("initial_conc.dat")?;
    let concs: Vec<_> = conc_str
        .lines()
        .map(|s| str::parse::<f64>(s).unwrap())
        .collect();

    let mut conc_data = HashMap::new();
    mesh.vertex_iter()
        .zip(concs.iter())
        .for_each(|(v_id, conc_a)| {
            // let data = VertexData { conc_a: *conc_a };
            // conc_data.insert(v_id, data);
            conc_data.insert(v_id, *conc_a);
        });

    for v_id in  mesh.vertex_iter() {
        println!("{}: {:?}", v_id, mesh.vertex_position(v_id));
    }
    // println!("Edges {:?}", mesh.edge_iter().map(|e_id | mesh.edge_positions(e_id)).collect::<Vec<(Vector3<f64>, Vector3<f64>)>>());
    // for (l, (v1, v2)) in mesh.edge_iter().map(|e_id | (mesh.edge_length(e_id), mesh.edge_positions(e_id))) {
    //     println!("Edge: {:?} to {:?} | len = {}", v1, v2, l);
    // }
    println!("Num edges = {}", mesh.no_edges());

    // for v_id in mesh.vertex_iter() {
    //     println!("One-ring for vertex {:?}", mesh.vertex_position(v_id));
    //     for e_id in mesh.vertex_halfedge_iter(v_id) {
    //         let edge = mesh.edge_positions(e_id);
    //         println!("Edge: {:?} {:?} to {:?}", e_id, edge.0, edge.1);
    //     }
    // }

    simulate(mesh, conc_data);
    Ok(())
}

fn print_conc_data(mesh: &Mesh, conc_data: &HashMap<VertexID, f64>) {
    println!("---");
    let mut line_0 = "      ".to_string();
    let mut line_1 = String::new();
    let mut line_2 = "      ".to_string();

    let mut xs: Vec<_> = conc_data.iter().map(|(id, x)| (id, x, mesh.vertex_position(*id))).collect();
    xs.sort_by(|(_, _, p1), (_, _, p2)| p1.x.partial_cmp(&p2.x).unwrap());

    let mut total = 0.0;
    for (_, x, pos) in xs.iter() {
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
        total += **x;
    }
    println!("{}", line_0);
    println!("{}", line_1);
    println!("{}", line_2);
    println!("Total conc: {}", total);
    println!("---");

}

fn simulate(mesh: Mesh, mut conc_data: HashMap<VertexID, f64>) {
    // println!("Initial conc data:");
    // print_conc_data(&mesh, &conc_data);

    for i in 0..1000 {
        if (i % 10) == 0 {
            println!("T = {}", (i as f64) * TS);
            print_conc_data(&mesh, &conc_data);
        }

        let lapl = laplacian::compute_laplacian(&mesh, &conc_data);

        for v_id in mesh.vertex_iter() {
            *conc_data.get_mut(&v_id).unwrap() += D * TS * lapl[&v_id];
        }
    }

    println!("Final conc data:");
    print_conc_data(&mesh, &conc_data);
}
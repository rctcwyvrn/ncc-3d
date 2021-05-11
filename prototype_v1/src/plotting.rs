use plotters::prelude::*;
use tri_mesh::prelude::{Mesh, VertexID};

use crate::{VertexData, storage::VecStore};

// Remember to change the stim function as well if this ever changes
const MAX_X: f64 = 10.0;

#[derive(Debug)]
pub enum GraphTy {
    Intermediate(f64),
    Final,
}

pub fn plot_data(mesh: &Mesh, conc_data: &VecStore<VertexData>, ty: GraphTy) {
    println!("Starting to plot {:?}", ty);
    let title_active = match ty {
        GraphTy::Intermediate(ts) => format!("Active conc. at t = {}", ts),
        GraphTy::Final => format!("Final active conc."),
    };

    let path_active = match ty {
        GraphTy::Intermediate(ts) => format!("images/active-{}.png", ts),
        GraphTy::Final => format!("images/active-final.png"),
    };

    let title_inactive = match ty {
        GraphTy::Intermediate(ts) => format!("Inactive conc. at t = {}", ts),
        GraphTy::Final => format!("Final Inactive conc."),
    };

    let path_inactive = match ty {
        GraphTy::Intermediate(ts) => format!("images/inactive-{}.png", ts),
        GraphTy::Final => format!("images/inactive-final.png"),
    };
    do_plot(
        mesh,
        conc_data,
        &title_active,
        &path_active,
        GraphConcTy::Active,
    );
    do_plot(
        mesh,
        conc_data,
        &title_inactive,
        &path_inactive,
        GraphConcTy::Inactive,
    );
}

enum GraphConcTy {
    Active,
    Inactive,
}

fn do_plot(
    mesh: &Mesh,
    conc_data: &VecStore<VertexData>,
    title: &str,
    path: &str,
    ty: GraphConcTy,
) {
    let root = BitMapBackend::new(path, (640, 480)).into_drawing_area();
    let style = match ty {
        GraphConcTy::Active => BLUE,
        GraphConcTy::Inactive => RED, 
    };

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption(title, ("sans-serif", 40))
        .build_cartesian_3d(0.0..MAX_X, 0.0..3.0, -10.0..10.0)
        .unwrap();

    //DEBUG: Top down look
    // chart.with_projection(|mut pb| {
    //     pb.pitch = std::f64::consts::FRAC_PI_2;
    //     pb.yaw = 0.0;
    //     pb.scale = 1.0;
    //     pb.into_matrix()
    // });

    chart.configure_axes().draw().unwrap();

    chart
        .draw_series(
            mesh.face_iter()
                .map(|f_id| mesh.face_vertices(f_id))
                .map(|(v1, v2, v3)| {
                    let (p1, p2, p3) = (
                        mesh.vertex_position(v1),
                        mesh.vertex_position(v2),
                        mesh.vertex_position(v3),
                    );
                    let (y1, y2, y3) = match ty {
                        GraphConcTy::Active => (
                            conc_data.get(v1).conc_a,
                            conc_data.get(v2).conc_a,
                            conc_data.get(v3).conc_a,
                        ),
                        GraphConcTy::Inactive => (
                            conc_data.get(v1).conc_b,
                            conc_data.get(v2).conc_b,
                            conc_data.get(v3).conc_b,
                        ),
                    };

                    Polygon::new(
                        vec![(p1.x, y1, p1.z), (p2.x, y2, p2.z), (p3.x, y3, p3.z)],
                        &style.mix(0.3),
                    )
                }),
        )
        .unwrap();

    for e_id in mesh.edge_iter() {
        let (v1_id, v2_id) = mesh.edge_vertices(e_id);
        let (v1, v2) = (mesh.vertex_position(v1_id), mesh.vertex_position(v2_id));

        let (y1, y2) = match ty {
            GraphConcTy::Active => (conc_data.get(v1_id).conc_a, conc_data.get(v2_id).conc_a),
            GraphConcTy::Inactive => (conc_data.get(v1_id).conc_b, conc_data.get(v2_id).conc_b),
        };
        let line: [(f64, f64, f64); 2] = [(v1.x, y1, v1.z), (v2.x, y2, v2.z)];
        chart
            .draw_series(LineSeries::new(
                line.iter().map(|(x, y, z)| (*x, *y, *z)),
                &style,
            ))
            .unwrap();
    }
}

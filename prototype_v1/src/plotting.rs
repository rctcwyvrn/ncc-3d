use plotters::prelude::*;
use tri_mesh::prelude::Mesh;

use crate::{storage::VecStore, VertexData};

// Remember to change the stim function as well if this ever changes
const MAX_X: f64 = 10.0;

// Graphing will break if the value is ever greater than this (i think?)
const MAX_VAL: f64 = 2.0;
#[derive(Debug)]
pub enum GraphTy {
    Intermediate(f64),
    Final,
}

pub fn plot_data(mesh: &Mesh, conc_data: &VecStore<VertexData>, ty: GraphTy) {
    println!("Starting to plot {:?}", ty);
    // let title_active = match ty {
    //     GraphTy::Intermediate(ts) => format!("Active conc. at t = {}", ts),
    //     GraphTy::Final => format!("Final active conc."),
    // };

    // let path_active = match ty {
    //     GraphTy::Intermediate(ts) => format!("images/active-{:0>4}.png", ts),
    //     GraphTy::Final => format!("images/active-final.png"),
    // };

    // let title_inactive = match ty {
    //     GraphTy::Intermediate(ts) => format!("Inactive conc. at t = {}", ts),
    //     GraphTy::Final => format!("Final Inactive conc."),
    // };

    // let path_inactive = match ty {
    //     GraphTy::Intermediate(ts) => format!("images/inactive-{:0>4}.png", ts),
    //     GraphTy::Final => format!("images/inactive-final.png"),
    // };
    // do_plot(
    //     mesh,
    //     conc_data,
    //     &title_active,
    //     &path_active,
    //     GraphConcTy::Active,
    // );
    // do_plot(
    //     mesh,
    //     conc_data,
    //     &title_inactive,
    //     &path_inactive,
    //     GraphConcTy::Inactive,
    // );


    plot_slice(mesh, conc_data, ty);

    // if let GraphTy::Final = ty {
    //     plot_slice(mesh, conc_data, ty);
    // } else if let GraphTy::Intermediate(ts) = ty {
    //     if (ts == 20.0) || (ts == 0.0) {
    //         plot_slice(mesh, conc_data, ty);
    //     }
    // }
}

fn get_color(val: f64) -> RGBColor {
    let percent = val / MAX_VAL;
    if percent < 0.25 {
        let rel_percent = percent / 0.25;
        let c = (rel_percent * 255.0) as u8;
        RGBColor(255, c, 0)
    } else if percent < 0.5 {
        let rel_percent = (percent - 0.25) / 0.25;
        let c = (rel_percent * 255.0) as u8;
        RGBColor(255 - c, 255, 0)
    } else if percent < 0.75 {
        let rel_percent = (percent - 0.5) / 0.25;
        let c = (rel_percent * 255.0) as u8;
        RGBColor(0, 255, c)
    } else if percent <= 1.0 {
        let rel_percent = (percent - 0.75) / 0.25;
        let c = (rel_percent * 255.0) as u8;
        RGBColor(0, 255 - c, 255)
    } else {
        eprintln!("Warning: Got a concentration > 2.0: {}", val);
        RGBColor(255, 0, 255)
    }
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
    root.fill(&WHITE).unwrap();

    // let base = match ty {
    //     GraphConcTy::Active => BLUE,
    //     GraphConcTy::Inactive => RED,
    // };

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption(title, ("sans-serif", 40))
        .build_cartesian_3d(0.0..MAX_X, -5.5..5.5, -5.5..5.5)
        .unwrap();

    // DEBUG: Top down look
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
                    let (v1, v2, v3) = match ty {
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
                    let avg = (v1 + v2 + v3) / 3.0;
                    let color = get_color(avg).mix(0.3);
                    // let color = get_color(avg).mix(0.7);
                    Polygon::new(
                        vec![(p1.x, p1.y, p1.z), (p2.x, p2.y, p2.z), (p3.x, p3.y, p3.z)],
                        &color,
                    )
                }),
        )
        .unwrap();

    chart
        .draw_series(
            mesh.vertex_iter()
                .map(|v_id| {
                    let p = mesh.vertex_position(v_id);
                    let v = match ty {
                        GraphConcTy::Active => conc_data.get(v_id).conc_a,
                        GraphConcTy::Inactive => conc_data.get(v_id).conc_b,
                    };

                    let color = get_color(v);
                    ((p.x, p.y, p.z), color)
                })
                .map(|(point, color)| Circle::new(point, 3, &color)),
        )
        .unwrap();
}

fn plot_slice(mesh: &Mesh, conc_data: &VecStore<VertexData>, ty: GraphTy) {
    let (path_1, path_2) = match ty {
        GraphTy::Final => (
            "images/active-slice-final-pos_z.png".to_string(),
            "images/active-slice-final-neg_z.png".to_string(),
        ),
        GraphTy::Intermediate(ts) => (
            format!("images/active-slice-{}-pos_z.png", ts),
            format!("images/active-slice-{}-neg_z.png", ts),
        ),
    };

    // For planar mesh
    let data: Vec<_> = mesh
        .vertex_iter()
        .map(|v_id| (v_id, mesh.vertex_position(v_id)))
        .filter(|(_, pos)| pos.z.abs() <= 0.0001)
        .map(|(v_id, pos)| (pos.x, conc_data.get(v_id).conc_a))
        .collect();

    do_slice_plot(data, &path_1, "profile of planar mesh with z=0");

    let data: Vec<_> = mesh
        .vertex_iter()
        .map(|v_id| (v_id, mesh.vertex_position(v_id)))
        .filter(|(_, pos)| (pos.z - 0.1).abs() <= 0.0001)
        .map(|(v_id, pos)| (pos.x, conc_data.get(v_id).conc_a))
        .collect();

    do_slice_plot(data, &path_2, "profile of planar mesh with z=0.5");

    // // For sphere
    // let data_1: Vec<_> = mesh
    //     .vertex_iter()
    //     .map(|v_id| (v_id, mesh.vertex_position(v_id)))
    //     .filter(|(_, pos)| pos.y.abs() <= 0.0001)
    //     .filter(|(_, pos)| pos.z >= 0.0)
    //     .map(|(v_id, pos)| (pos.x, conc_data.get(v_id).conc_a))
    //     .collect();

    // let data_2: Vec<_> = mesh
    //     .vertex_iter()
    //     .map(|v_id| (v_id, mesh.vertex_position(v_id)))
    //     .filter(|(_, pos)| pos.y.abs() <= 0.0001)
    //     .filter(|(_, pos)| pos.z < 0.0)
    //     .map(|(v_id, pos)| (pos.x, conc_data.get(v_id).conc_a))
    //     .collect();

    // do_slice_plot(data_1, &path_1, "slice of sphere with y = 0, z>=0");
    // do_slice_plot(data_2, &path_2, "slice of sphere with y = 0, z<0");
}

fn do_slice_plot(mut data: Vec<(f64, f64)>, path: &str, caption: &str) {
    data.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    let root = BitMapBackend::new(path, (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption(caption, ("sans-serif", 40))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..MAX_X, 0.0..MAX_VAL)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(data.clone(), &RED))
        .unwrap();

    chart
        .draw_series(data.iter().map(|p| Circle::new(*p, 2, &RED)))
        .unwrap();
}

use nalgebra as na;
use na::{SMatrix, zero};

use crate::mesh::{NUM_VERTS, Mesh, VType, Vertex};

pub type Laplacian = SMatrix<f64, NUM_VERTS, NUM_VERTS>; 
pub type MassMat = SMatrix<f64, NUM_VERTS, NUM_VERTS>; 

fn calculate_weight_and_area(vertex: &Vertex, nb: &Vertex, prev_v: &Vertex, next_v: &Vertex) -> (f64, f64) {
    let v1 = vertex.pos - prev_v.pos;
    let v2 = nb.pos - prev_v.pos;
    let v3 = vertex.pos - next_v.pos;
    let v4 = nb.pos - next_v.pos;
    let v5 = nb.pos - vertex.pos;
    let v6 = next_v.pos - vertex.pos;

    // Just in case so we don't divide by 0 by accident
    // let cotan1 = (v1.dot(&v2))/(v1.cross(&v2).norm());
    // let cotan1 = (v3.dot(&v4))/(v3.cross(&v4).norm());
    let cotan1 = (v1.dot(&v2))/(1e-6 + v1.cross(&v2).norm());
    let cotan2 = (v3.dot(&v4))/(1e-6 + v3.cross(&v4).norm());

    let w_ij = 0.5 * (cotan1 + cotan2);
    let area = v5.cross(&v6).norm() / 2_f64;
    return (w_ij, area);
}

fn calculate_interior(mesh: &Mesh, lapl: &mut Laplacian, m_inv: &mut MassMat, vertex: &Vertex, i: usize) {
    let mut w_sum: f64 = 0.0;
    let num_nbs = vertex.nbs.len();
    let mut a_total: f64 = 0.0;
    // IMPORTANT:
    // nb_idx, prev_idx, next_idx all index into the vertex.nbs
    // which then indexes into mesh.verts
    for nb_idx in 0..num_nbs {
        let j = vertex.nbs[nb_idx];
        let nb = &mesh.verts[j];
        let prev_idx = (nb_idx + num_nbs - 1) % num_nbs;
        let prev_v = &mesh.verts[vertex.nbs[prev_idx]];
        let next_idx = (nb_idx + 1) % num_nbs;
        let next_v = &mesh.verts[vertex.nbs[next_idx]];

        let (w_ij, area) = calculate_weight_and_area(vertex, nb, prev_v, next_v);
        w_sum += w_ij;
        a_total += area/3.0;

        *lapl.index_mut((i,j)) = w_ij;
    }

    *lapl.index_mut((i,i)) = -1.0 * w_sum;
    *m_inv.index_mut((i,i)) = 1.0 / a_total;
}

fn calculate_boundary(mesh: &Mesh, lapl: &mut Laplacian, m_inv: &mut MassMat, vertex: &Vertex, i: usize) {
    let mut w_sum: f64 = 0.0;
    let num_nbs = vertex.nbs.len();
    let mut a_total: f64 = 0.0;

    for nb_idx in 0..num_nbs {
        let j = vertex.nbs[nb_idx];
        let nb = &mesh.verts[j];

        let next_idx = (nb_idx + 1) % num_nbs;
        let next_v = &mesh.verts[vertex.nbs[next_idx]];

        // if nb.ty == VType::Interior {
        //     // Since this is an interior point, neighbours idx - 1 and idx + 1 must be next to it
        //     // or else it wouldn't be an interior point
        //     let j = vertex.nbs[nb_idx];
        //     let nb = &mesh.verts[j];
        //     let prev_idx = (nb_idx + num_nbs - 1) % num_nbs;
        //     let prev_v = &mesh.verts[vertex.nbs[prev_idx]];

        //     let (w_ij, _) = calculate_weight_and_area(vertex, nb, prev_v, next_v);
        //     w_sum += w_ij;

        //     *lapl.index_mut((i,j)) = w_ij;
        // } else {
        //     let w_ij = 0.5 * (nb.pos - vertex.pos).norm();
        //     w_sum += w_ij;

        //     *lapl.index_mut((i,j)) = w_ij;
        // }

        let w_ij = 0.5 * (nb.pos - vertex.pos).norm();
        w_sum += w_ij;

        *lapl.index_mut((i,j)) = w_ij;

        // Don't count the one where it loops around
        if next_idx > nb_idx {
            let v3 = next_v.pos - vertex.pos;
            let v5 = nb.pos - vertex.pos;
            let area = v5.cross(&v3).norm() / 2_f64;
            a_total += area/3.0;
        }
    }
    *lapl.index_mut((i,i)) = -1.0 * w_sum;
    *m_inv.index_mut((i,i)) = 1.0 / a_total;
}


pub fn create(mesh: &Mesh) -> (Laplacian, MassMat) {
    let mut lapl: Laplacian = zero::<Laplacian>();
    let mut m_inv: MassMat = zero::<MassMat>();

    // for (i, vertex) in mesh.verts.iter().enumerate().filter(|(_,v)| v.ty == VType::Interior) {
    for (i, vertex) in mesh.verts.iter().enumerate() {
        calculate_boundary(mesh, &mut lapl, &mut m_inv, vertex, i)
        // if vertex.ty == VType::Interior {
        //     calculate_interior(mesh, &mut lapl, &mut m_inv, vertex, i);
        // } else {
        //     calculate_boundary(mesh, &mut lapl, &mut m_inv, vertex, i);
        // }
    }
    (lapl, m_inv)
}
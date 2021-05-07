use std::collections::HashMap;

use tri_mesh::prelude::{InnerSpace, Mesh, VertexID};

// use crate::VertexData;


const H: f64 = 1.0;

// http://www.cs.jhu.edu/~misha/Fall09/Belkin08.pdf
pub fn compute_laplacian(mesh: &Mesh, f: &HashMap<VertexID, f64>) -> HashMap<VertexID, f64> {
    
    let mut lapl = HashMap::new();
    for v_id in mesh.vertex_iter() {
        // Compute laplacian for this v_id according to formula 2.1
        let mut total_sum = 0.0;

        for face_id in mesh.face_iter() {
            // Area of the face
            let area = mesh.face_area(face_id);

            // Because triangles
            let num_t = 3.0;

            let mut sum_face = 0.0;
            let vert_ids = mesh.face_vertices(face_id);
            sum_face += compute_pair(mesh, f, v_id, vert_ids.0);
            sum_face += compute_pair(mesh, f, v_id, vert_ids.1);
            sum_face += compute_pair(mesh, f, v_id, vert_ids.2);

            total_sum += sum_face * area / num_t 
        }
        lapl.insert(v_id, total_sum / (4.0 * std::f64::consts::PI * H.powi(2)));
    }

    lapl
}

fn compute_pair(mesh: &Mesh, f: &HashMap<VertexID, f64>, v_id: VertexID, ov_id: VertexID) -> f64 {
    let v = mesh.vertex_position(v_id);
    let ov = mesh.vertex_position(ov_id);
    let dist: f64 = (ov - v).magnitude2();

    (-dist/(4.0*H)).exp() * (f[&ov_id] - f[&v_id])
}
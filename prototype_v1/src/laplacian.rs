use tri_mesh::prelude::{Deref, InnerSpace, Mesh, VertexID};

use crate::storage::VecStore;

// Fixme: What should thsi be set to? Need to read the paper more carefully
const H: f64 = 1.0;

// http://www.cs.jhu.edu/~misha/Fall09/Belkin08.pdf
pub fn compute_laplacian(mesh: &Mesh, f: &VecStore<f64>) -> VecStore<f64> {
    let n = mesh.no_vertices();
    let mut lapl = VecStore::new(n);
    let mut memo = VecStore::new(n);

    for i in 0..n {
        // Use inner vec directly because we have usizes instead of VertexIDs
        memo.0[i] = Some(VecStore::new(n));
    }

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
            sum_face += compute_pair(mesh, f, &mut memo, v_id, vert_ids.0);
            sum_face += compute_pair(mesh, f, &mut memo, v_id, vert_ids.1);
            sum_face += compute_pair(mesh, f, &mut memo, v_id, vert_ids.2);

            total_sum += sum_face * area / num_t
        }
        lapl.set(v_id, total_sum / (4.0 * std::f64::consts::PI * H.powi(2)));
    }

    lapl
}

fn check_memo(memo: &VecStore<VecStore<f64>>, v_id: VertexID, ov_id: VertexID) -> bool {
    if !memo.is_set(v_id) {
        return false
    }
    let inner = memo.get(v_id);

    inner.is_set(ov_id)
}

fn compute_pair(mesh: &Mesh, f: &VecStore<f64>, memo: &mut VecStore<VecStore<f64>>, v_id: VertexID, ov_id: VertexID) -> f64 {
    if check_memo(memo, v_id, ov_id) {
        *memo.get(v_id).get(ov_id)
    } else {
        let v = mesh.vertex_position(v_id);
        let ov = mesh.vertex_position(ov_id);
        let dist: f64 = (ov - v).magnitude2();
    
        let val = (-dist / (4.0 * H)).exp() * (f.get(ov_id) - f.get(v_id));        
        memo.get_mut(v_id).set(ov_id, val);
        memo.get_mut(ov_id).set(v_id, -val);
        val
    }
}

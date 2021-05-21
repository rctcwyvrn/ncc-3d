use tri_mesh::prelude::{HalfEdgeID, InnerSpace, Mesh, Vector3, VertexID};

use crate::storage::VecStore;

pub fn compute_laplacian(mesh: &Mesh, f: &VecStore<f64>) -> VecStore<f64> {
    let mut lapl = VecStore::new(mesh);

    for v_id in mesh.vertex_iter() {
        let mut sum = 0.0;
        let mut area = 0.0;

        let v_i = mesh.vertex_position(v_id);
        let edges: Vec<_> = mesh.vertex_halfedge_iter(v_id).collect();
        let n = edges.len();

        // println!("v_id = {:?} | {:?}", v_id, v_i);
        for j in 0..n {
            let v_j_id = other(mesh, v_id, edges[j]);
            let v_j = mesh.vertex_position(v_j_id);

            let prev = edges[(j-1+n) % n];
            let next = edges[(j+1) % n ];

            let v_p_id = other(mesh, v_id, prev);
            let v_p = mesh.vertex_position(v_p_id);

            let v_n_id = other(mesh, v_id, next);
            let v_n = mesh.vertex_position(v_n_id);

            // println!("j: {:?} | prev: {:?} | next: {:?}", v_j, v_p, v_n);

            let v1 = v_i - v_p;
            let v2 = v_j - v_p;
            let v3 = v_i - v_n;
            let v4 = v_j - v_n;

            // println!("v1-4: {:?} | {:?} | {:?} | {:?}", v1,v2,v3,v4);

            let mut cotan_a = cotan(v1, v2);
            let mut cotan_b = cotan(v3, v4);

            // println!("cotan_a = {} | cotan_b = {}", cotan_a, cotan_b);

            // If this edge is on the boundary, then one of these two cotans is bad
            if is_edge_on_boundary(mesh, edges[j]) {
                // Throw out the one that is also a boundary edge
                if is_edge_on_boundary(mesh, next) {
                    cotan_b = 0.0;
                    // println!("throwing out cotan b (computed with v_next)");
                } else {
                    cotan_a = 0.0;
                    // println!("throwing out cotan a (computed with v_prev)");
                }
            }

            if !is_edge_on_boundary(mesh, next) {
                let v1 = v_j - v_i;
                let v2 = v_n - v_i;
                area += v1.cross(v2).magnitude() / 2.0;
            }

            let w = cotan_a + cotan_b;
            let diff = f.get(v_j_id) - f.get(v_id);
            sum += w * diff;

            // println!("{} | w: {} | diff: {}", w*diff, w, diff);
            // println!("Running sum = {}", sum);
            // println!("Running area = {}", area);
        }
        lapl.set(v_id, sum / (2.0 * area))
    }
    lapl
}


// Return the vertex id for the vertex that is not v_id
fn other(mesh: &Mesh, v_id: VertexID, e_id: HalfEdgeID) -> VertexID {
    let (v1, v2) = mesh.edge_vertices(e_id);
    if v_id != v1 {
        v1
    } else {
        v2
    }
}

fn is_edge_on_boundary(mesh: &Mesh, e_id: HalfEdgeID) -> bool {
    let (v_i, v_j) = mesh.edge_vertices(e_id);
    is_on_boundary(mesh, v_i) && is_on_boundary(mesh, v_j)
}

fn is_on_boundary(mesh: &Mesh, v_id: VertexID) -> bool {
    let edges: Vec<_> = mesh.vertex_halfedge_iter(v_id).collect(); 
    edges.len() == 3 || edges.len() == 5
}

fn cotan(a: Vector3<f64>, b: Vector3<f64>) -> f64 {
    a.dot(b) / a.cross(b).magnitude()
}
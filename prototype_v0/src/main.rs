mod laplacian;
mod mesh;


use crate::mesh::Mesh;

fn main() {
    let mut mesh = Mesh::create_mesh();
    println!("{:?}", mesh);
    for _ in 0..30 {
        mesh.step()
    }
}

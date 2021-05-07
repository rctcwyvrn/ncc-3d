use nalgebra as na;
use na::{SVector, Vector3};
use std::convert::TryInto;

use crate::laplacian;

pub const NUM_VERTS: usize = 7;
const TIME_STEP: f64 = 0.2;
const L: f64 = 1_f64;

type ConcVector = SVector<f64, NUM_VERTS>; 

#[derive(Debug)]
pub struct Mesh {
    pub verts: [Vertex; NUM_VERTS]
}

impl Mesh {
    pub fn step(&mut self) {
        let (l_mat, m_inv) = laplacian::create(self);
        let concA_iter = self.verts.iter().map(|vert| vert.concA);
        let concA_vector = ConcVector::from_iterator(concA_iter);

        // This doesn't work, but it should (Total concentration isn't conserved)
        // let dconc_dt = m_inv * (l_mat * conc_vector);

        // This works though
        // I think I can get away without considering the mass matrix because the triangles are
        // all of equal size anyway? Shouldn't make any difference at all
        let dconcA_dt = l_mat * concA_vector;


        // Reaction rate vector
        self.verts.iter_mut()
            .zip(dconcA_dt.into_iter())
            .for_each(|(vert, new_conc)| vert.concA += (*new_conc) * TIME_STEP);
        
        println!("Concs {:?}", self.verts.iter().map(|v| v.concA).collect::<Vec<f64>>());
        println!("Total = {}", self.verts.iter().map(|v| v.concA).sum::<f64>());
    }

    pub fn create_mesh() -> Mesh {
    //     // Testing basic mesh for now, actual generative meshes are gonna be
    //     // c o m p l i c a t e d
        
    //     // This is disgusting and I hate it  
    //     let verts = Vec::new();

    //     let back = Vertex::new(Vector3::new(0_f64, 0_f64, 0_f64));
    //     let id_back = verts.len();
    //     verts.push(back);

    //     let mut cur_x = L / 2.0;
    //     let mut cur_y = L * 3.0_f64.sqrt() / 2.0;
    //     let mut cur_x_middle = L;

    //     let mut old_front_top = Vertex::new(Vector3::new(cur_x, cur_y, 0_f64));
    //     let mut old_id_front_top = verts.len();
    //     verts.push(old_front_top);

    //     let mut old_front_bottom = Vertex::new(Vector3::new(cur_x, -1.0 * cur_y, 0_f64));
    //     let mut old_id_front_bottom = verts.len();
    //     verts.push(old_front_bottom);

    //     let mut old_front_middle = Vertex::new(Vector3::new(cur_x_middle, 0_f64, 0_f64));
    //     let mut old_id_front_middle = verts.len();
    //     verts.push(old_front_middle);

    //     back.add_nb(old_id_front_top);
    //     back.add_nb(old_id_front_middle);
    //     back.add_nb(old_id_front_bottom);

    //     old_front_top.add_nb(old_id_front_middle);
    //     old_front_top.add_nb(id_back);

    //     old_front_bottom.add_nb(id_back);
    //     old_front_bottom.add_nb(old_id_front_middle);

    //     old_front_middle.add_nb(old_id_front_bottom);
    //     old_front_middle.add_nb(id_back);
    //     old_front_middle.add_nb(old_id_front_top);
        
    //     let mut total = 4;

    //     cur_x += L;
    //     cur_x_middle += L;

    //     while total < NUM_VERTS {
    //         let mut front_tp = Vertex::new(Vector3::new(cur_x, cur_y, 0_f64));
    //         let mut front_bt = Vertex::new(Vector3::new(cur_x, -1.0 * cur_y, 0_f64));
    //         let mut front_mid = Vertex::new(Vector3::new(cur_x_middle, 0_f64, 0_f64));

    //         let mid_id = verts.len();
    //         verts.push(front_mid);
    //         let bt_id = verts.len();
    //         verts.push(front_bt);
    //         let top_id = verts.len();
    //         verts.push(front_tp);

    //         old_front_top.add_nb(top_id);
    //         front_tp.add_nb(mid_id);
    //         front_tp.add_nb(old_id_front_middle);
    //         front_tp.add_nb(old_id_front_top);

    //         old_front_bottom.add_nb(bt_id);
    //         front_tp.add_nb(old_id_front_bottom);
    //         front_tp.add_nb(old_id_front_middle);
    //         front_tp.add_nb(mid_id);

    //         old_front_middle.add_nb(top_id);
    //         old_front_middle.add_nb(mid_id);
    //         old_front_middle.add_nb(bt_id);
    //         println!("should be now an interior: {:?}", old_front_middle.ty);

    //         front_mid.add_nb(bt_id);
    //         front_mid.add_nb(old_id_front_middle);
    //         front_mid.add_nb(top_id);

    //         old_front_bottom = front_bt;
    //         old_front_middle = front_mid;
    //         old_front_top = front_tp;

    //         old_id_front_bottom = bt_id;
    //         old_id_front_middle = mid_id;
    //         old_id_front_top = top_id;

    //         cur_x += L;
    //         cur_x_middle += L;
    //         total += 3;
    //     }

    //     Mesh {
    //         verts: verts.try_into().unwrap_or_else(|v: Vec<Vertex>| panic!("Wrong number of elements. Should be {} | Got {}", v.len(), NUM_VERTS))
    //     }

        // Just one root node, everything else is a boundary and not going to be stepped
        let y: f64 = 3_f64.sqrt() / 2.0;
        let root = Vertex {
            pos: Vector3::new(0_f64, 0_f64, 0_f64),
            nbs: vec![1,2,3,4,5,6],
            concA: 0_f64,
            concB: 0_f64,
            ty: VType::Interior,
        };

        let v1 = Vertex {
            pos: Vector3::new(-1_f64, 0_f64, 0_f64),
            nbs: vec![0,2,6],
            ty: VType::Boundary,
            concA: 100_f64,
            concB: 0_f64,
        };
        
        let v2 = Vertex {
            pos: Vector3::new(-0.5_f64, y, 0_f64),
            nbs: vec![0,1,3],
            ty: VType::Boundary,
            concA: 0_f64,
            concB: 0_f64,
        };

        let v3 = Vertex {
            pos: Vector3::new(0.5_f64, y, 0_f64),
            nbs: vec![0,2,4],
            ty: VType::Boundary,
            concA: 0_f64,
            concB: 0_f64,
        };
        
        let v4 = Vertex {
            pos: Vector3::new(1_f64, 0_f64, 0_f64),
            nbs: vec![0,3,5],
            ty: VType::Boundary,
            concA: 0_f64,
            concB: 0_f64,
        };

        let v5 = Vertex {
            pos: Vector3::new(0.5_f64, -y, 0_f64),
            nbs: vec![0,4,6],
            ty: VType::Boundary,
            concA: 0_f64,
            concB: 0_f64,
        };

        let v6 = Vertex {
            pos: Vector3::new(-0.5_f64, -y, 0_f64),
            nbs: vec![0,1,5],
            ty: VType::Boundary,
            concA: 0_f64,
            concB: 0_f64,
        };

        Mesh {
            verts: [root, v1, v2, v3, v4, v5, v6]
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VType {
    Interior,
    Boundary,
}

#[derive(Debug)]
pub struct Vertex {
    // Use 3D vectors for now with z = 0 (everything lying flat on the plane)
    // Means I don't need to rewrite everything for the next prototype (which will have 3 dimensions)
    pub pos: Vector3<f64>,
    pub ty: VType,
    // Indicies to the neighbours of this vertex to the array in Mesh
    // Clockwise order (or just any consistent order)
    pub nbs: Vec<usize>,
    pub concA: f64,
    pub concB: f64,
}

impl Vertex {
    fn new(pos: Vector3<f64>) -> Vertex {
        Vertex {
            pos,
            ty: VType::Boundary, 
            nbs: Vec::new(),
            concA: 0_f64,
            concB: 0_f64,
        }
    }

    fn add_nb(&mut self, nb: usize) {
        self.nbs.push(nb);
        if self.nbs.len() > 6 {
            panic!("Vertex has more than 6 neighbors?")
        }

        if self.nbs.len() == 6 {
            self.ty == VType::Interior;
        }
    }
}
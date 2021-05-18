use old::StimTy;
use plotters::prelude::*;

const H: f64 = 0.2;
const L: f64 = 10.0;
const N: usize = (L / H) as usize;

const TS: f64 = 0.001;

// const FINAL_TIME: f64 = 400.0; // For reversal only
const FINAL_TIME: f64 = 200.0;

const STEPS: usize = (FINAL_TIME / TS) as usize;
const SNAPSHOT_PERIOD: usize = 5000;

// Steady state pair of A,B
const STARTING_A: f64 = 0.2683312;
const STARTING_B: f64 = 2.0;

// Diffusivity constants
const D_A: f64 = 0.1;
const D_B: f64 = 10.0;

mod old;

fn main() {
    // let stim_ty = StimTy::Reversal;
    // let stim_ty = StimTy::Gradient;
    let stim_ty = StimTy::Localized;
    // let stim_ty = StimTy::Randomized;

    let mut active= if let StimTy::Randomized = stim_ty {
        randomized_init(STARTING_A)
    } else {
        init_vec(STARTING_A)
    };

    let mut inactive = init_vec(STARTING_B);

    // Stimuli
    let stim_fn = old::get_stim(stim_ty);

    // Step diffeq
    let mut t = 0.0;
    for i in 0..STEPS {
        // Diffusion
        let lapl_a = calculate_lapl(&active);
        let lapl_b = calculate_lapl(&inactive);

        // Reaction
        let rates = old::compute_reaction_rate(&active, &inactive);

        // totals
        let mut total_a = 0.0;
        let mut total_b = 0.0;

        for x in 0..N {
            for z in 0..N {
                // let d_a = D_A * lapl_a.get(v_id) + (r + stim_k * b);
                // let d_b = D_B * lapl_b.get(v_id) - (r + stim_k * b);

                let b = inactive[x][z];
                let pos = (x as f64 * H, z as f64 * H);
                let stim_k = stim_fn(pos, t);

                let d_a = D_A * lapl_a[x][z] + (rates[x][z] + stim_k * b);
                let d_b = D_B * lapl_b[x][z] - (rates[x][z] + stim_k * b);

                active[x][z] += TS * d_a;
                inactive[x][z] += TS * d_b;

                total_a += active[x][z];
                total_b += inactive[x][z];
            }
        }

        if (i % SNAPSHOT_PERIOD) == 0 {
            println!("Plotting t = {}", t.round());
            plot(&active, format!("images/active-{:0>4}.png", t.round()));

            if t.round() == 20.0 || t.round() == 0.0 {
                plot(&inactive, format!("images/inactive-{:0>4}.png", t.round()));
                plot_side(&active, format!("images/active-{}-side.png", t.round()));
            }

            for x in 0..N {
                let z = 0;
                let pos = (x as f64 * H, z as f64 * H);
                let stim_k = stim_fn(pos, t);
                println!(
                    "DEBUG {:?} ({}, {})=> {} | {} | {} | {}",
                    pos,
                    active[x][z],
                    inactive[x][z],
                    D_A * lapl_a[x][z],
                    D_B * lapl_b[x][z],
                    rates[x][z],
                    stim_k
                );
            }
            println!("Totals: ");
            println!("Active: {}", total_a);
            println!("Inactive: {}", total_b);
            println!("Overall total: {}", total_a + total_b);
        }

        t += TS;
    }
    plot(&active, format!("images/active-final.png"));
    plot_side(&active, format!("images/active-final-side.png"));
}

pub fn init_vec(initial_val: f64) -> Vec<Vec<f64>> {
    let mut vec = Vec::new();
    let mut row = Vec::new();
    for _ in 0..N {
        row.push(initial_val);
    }

    for _ in 0..N {
        vec.push(row.clone());
    }

    vec
}

fn randomized_init(central: f64) -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();

    let mut vec = Vec::new();
    for _ in 0..N {
        let mut row = Vec::new();
        // for _ in 0..N {
        //     let r :f64 = rand::Rng::gen(&mut rng);
        //     row.push(central * 2.0 * r);
        // }

        let r :f64 = rand::Rng::gen(&mut rng);
        let val = central * 2.0 * r;
        for _ in 0..N {
            row.push(val);
        }

        vec.push(row);
    }

    vec
}

// https://www.math.ubc.ca/~gustaf/M31611/fd.pdf for how to compute the boundary
fn calculate_lapl(f: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut lapl = init_vec(0.0);

    for x in 0..N {
        for z in 0..N {
            let mut val = 0.0;
            if x == 0 {
                val += f[x + 1][z];
            } else {
                val += f[x - 1][z];
            }

            if x == N - 1 {
                val += f[x - 1][z]
            } else {
                val += f[x + 1][z] 
            }

            if z == 0 {
                val += f[x][z + 1];
            } else {
                val += f[x][z - 1];
            }

            if z == N - 1 {
                val += f[x][z - 1]
            } else {
                val += f[x][z + 1] 
            }

            val -= f[x][z] * 4.0;
            val = val / H.powi(2);
            lapl[x][z] = val;
        }
    }
    lapl
}

fn plot(f: &Vec<Vec<f64>>, path: String) {
    let root = BitMapBackend::new(&path, (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("plaaaaanar", ("sans-serif", 40))
        .build_cartesian_3d(0.0..L, 0.0..2.0, 0.0..L)
        .unwrap();

    chart.configure_axes().draw().unwrap();

    let mut points = Vec::new();
    for x in 0..N {
        for z in 0..N {
            points.push((x as f64 * H, f[x][z], z as f64 * H));
        }
    }

    chart
        .draw_series(points.iter().map(|p| Circle::new(*p, 3, &BLACK)))
        .unwrap();
}

fn plot_side(f: &Vec<Vec<f64>>, path: String) {
    let root = BitMapBackend::new(&path, (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("plaaaaanar", ("sans-serif", 40))
        .build_cartesian_3d(0.0..L, 0.0..2.0, 0.0..L)
        .unwrap();

    chart.with_projection(|mut pb| {
        pb.pitch = 0.0;
        pb.yaw = 0.0;
        pb.scale = 1.0;
        pb.into_matrix()
    });
    chart.configure_axes().draw().unwrap();

    let mut points = Vec::new();
    for x in 0..N {
        for z in 0..N {
            points.push((x as f64 * H, f[x][z], z as f64 * H));
        }
    }

    chart
        .draw_series(points.iter().map(|p| Circle::new(*p, 3, &BLACK)))
        .unwrap();
}

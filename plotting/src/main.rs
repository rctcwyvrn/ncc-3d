use plotters::prelude::*;

fn main() {
    let root = BitMapBackend::new("images/3d-line.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("3D Line", ("sans-serif", 40))
        .build_cartesian_3d(-1.0..1.0, -1.0..1.0, -1.0..1.0)
        .unwrap();
    chart.configure_axes().draw().unwrap();

    chart.draw_series(LineSeries::new(
        (-100..100).map(|y| y as f64 / 100.0).map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
        &RED
    )).unwrap();
}
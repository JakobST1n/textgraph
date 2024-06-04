use textgraph::graph;

fn main() {
    let mut line: Vec<f64> = Vec::new();
    let mut marks: Vec<f64> = Vec::new();
    for i in 0..500 {
        line.push((i as f64 * std::f64::consts::PI / 120.0).sin());
        marks.push(i as f64);
    }

    // Choose one of the methods based on sample speed:
    //let downsampled_data = graph::downsample(&line, 100);
    let interpolated_data = graph::interpolate(&line, &marks, 100);


    //let processed_data = if marks.windows(2).all(|w| w[1] - w[0] == w[0] - w[1]) {
    //    downsample(&series, options.width)
    //} else {
    //    interpolate(&series, &marks, options.width)
    //};


    let g = graph::ascii_trailing(
        &interpolated_data,
        &graph::GraphOptions {
            width: 100.0,
            height: 30.0,
        },
    );
    println!("{}", g);
}

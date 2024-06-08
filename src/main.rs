use textgraph::graph;
use textgraph::parseopts::parseopts;

fn main() {
    let opts = parseopts();

    let mut y_values: Vec<f64> = Vec::new();
    let mut x_values: Vec<f64> = Vec::new();
    for i in 0..600 {
        y_values.push((i as f64 * std::f64::consts::PI / 120.0).sin());
        x_values.push(i as f64);
    }

    //let y_values: [f64; 6] = [1.0, 10.0, 40.0, 0.0, 30.0, 15.0];
    //let x_values: [f64; 6] = [1.0, 2.0,  3.0,  4.0, 5.0,  6.0];

    let graph_options: textgraph::graph::GraphOptions = (&opts).into();
    let g = match opts.graph_type {
        textgraph::parseopts::GraphType::Ascii => {
            graph::ascii(&y_values, &x_values, &graph_options)
        }
        textgraph::parseopts::GraphType::Star => graph::star(&y_values, &x_values, &graph_options),
    };

    println!("{}", g);
}

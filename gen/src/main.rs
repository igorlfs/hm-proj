use rand::Rng;
use std::env;

const THRESHOLD: f32 = 0.1;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please specify the number of vertices");
    }

    let num_vertices: usize = args[1]
        .parse()
        .expect("The number of vertices must be an integer");

    let mut num_edges = 0;
    let mut edges = vec![];

    for i in 0..num_vertices {
        for j in i + 1..num_vertices {
            if rand::thread_rng().gen::<f32>() < THRESHOLD {
                num_edges += 1;
                edges.push((i, j));
            }
        }
    }

    println!("p edge {num_vertices} {num_edges}");
    for (u, v) in edges {
        println!("e {u} {v}");
    }
}

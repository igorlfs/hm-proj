use crate::graph::Graph;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_graph_from_file(filename: &str) -> Result<Option<Graph>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut graph: Option<Graph> = None;

    for line in reader.lines().flatten() {
        let splits: Vec<&str> = line.split_whitespace().collect();

        if splits.is_empty() {
            continue;
        }

        match splits[0] {
            "p" => {
                if let Some(num_vertices) = splits.get(2) {
                    let num_vertices: usize = num_vertices.parse()?;
                    graph = Some(Graph::new(num_vertices));
                }
            }
            "e" => {
                if let (Some(from), Some(to)) = (splits.get(1), splits.get(2)) {
                    if let Some(graph) = graph.as_mut() {
                        let from = from.parse::<usize>()? - 1;
                        let to = to.parse::<usize>()? - 1;
                        graph.add_edge(from, to);
                    }
                }
            }
            _ => {}
        }
    }

    Ok(graph)
}

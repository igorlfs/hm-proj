pub struct Graph {
    num_vertices: usize,
    adjacency_matrix: Vec<Vec<bool>>,
    // coloring: Option<Vec<usize>>,
}

impl Graph {
    pub fn new(num_vertices: usize) -> Self {
        Graph {
            num_vertices,
            adjacency_matrix: vec![vec![false; num_vertices]; num_vertices],
            // coloring: None,
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.num_vertices && to < self.num_vertices {
            self.adjacency_matrix[from][to] = true;
            self.adjacency_matrix[to][from] = true;
        }
    }
}

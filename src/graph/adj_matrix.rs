pub struct AdjMatrix {
    num_vertices: usize,
    adj_matrix: Vec<Vec<bool>>,
}

impl AdjMatrix {
    pub fn new(num_vertices: usize) -> Self {
        AdjMatrix {
            num_vertices,
            adj_matrix: vec![vec![false; num_vertices]; num_vertices],
        }
    }

    #[cfg(test)]
    pub fn complete(num_vertices: usize) -> Self {
        let mut adj_matrix = AdjMatrix {
            num_vertices,
            adj_matrix: vec![vec![true; num_vertices]; num_vertices],
        };

        // Remove self-references
        for i in 0..num_vertices {
            adj_matrix.adj_matrix[i][i] = false;
        }

        adj_matrix
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.num_vertices && to < self.num_vertices {
            self.adj_matrix[from][to] = true;
            self.adj_matrix[to][from] = true;
        }
    }

    #[cfg(test)]
    pub fn remove_edge(&mut self, from: usize, to: usize) {
        if from < self.num_vertices && to < self.num_vertices {
            self.adj_matrix[from][to] = false;
            self.adj_matrix[to][from] = false;
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    pub fn adjacency_matrix(&self) -> Vec<Vec<bool>> {
        self.adj_matrix.clone()
    }

    pub fn get_neighbors(&self, i: usize) -> Vec<usize> {
        if i < self.num_vertices {
            let mut neighbors = Vec::new();

            for (neighbor, &is_connected) in self.adj_matrix[i].iter().enumerate() {
                if is_connected {
                    neighbors.push(neighbor);
                }
            }

            neighbors
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::input;

    #[test]
    fn test_get_neighbors() {
        if let Ok(Some(graph)) = input::read_graph_from_file("data/myc/myciel3.col") {
            let adj_matrix = graph.to_adj_matrix();
            let neighbors = adj_matrix.get_neighbors(10);
            assert_eq!(neighbors, vec![5, 6, 7, 8, 9]);

            let neighbors = adj_matrix.get_neighbors(11);
            assert_eq!(neighbors, vec![]);
        } else {
            panic!("The file containing the test graph is missing")
        }
    }
}

pub struct Graph {
    num_vertices: usize,
    adjacency_matrix: Vec<Vec<bool>>,
}

impl Graph {
    pub fn new(num_vertices: usize) -> Self {
        Graph {
            num_vertices,
            adjacency_matrix: vec![vec![false; num_vertices]; num_vertices],
        }
    }

    #[cfg(test)]
    pub fn complete(num_vertices: usize) -> Self {
        let mut graph = Graph {
            num_vertices,
            adjacency_matrix: vec![vec![true; num_vertices]; num_vertices],
        };

        // Remove self-references
        for i in 0..num_vertices {
            graph.adjacency_matrix[i][i] = false;
        }

        graph
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.num_vertices && to < self.num_vertices {
            self.adjacency_matrix[from][to] = true;
            self.adjacency_matrix[to][from] = true;
        }
    }

    #[cfg(test)]
    pub fn remove_edge(&mut self, from: usize, to: usize) {
        if from < self.num_vertices && to < self.num_vertices {
            self.adjacency_matrix[from][to] = false;
            self.adjacency_matrix[to][from] = false;
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    pub fn adjacency_matrix(&self) -> Vec<Vec<bool>> {
        self.adjacency_matrix.clone()
    }

    pub fn get_degree_in_list(&self, i: &usize, list: &[usize]) -> usize {
        if *i < self.num_vertices {
            let mut neighbors = 0;

            for (neighbor, &is_connected) in self.adjacency_matrix[*i].iter().enumerate() {
                if list.contains(&neighbor) && is_connected {
                    neighbors += 1;
                }
            }

            neighbors
        } else {
            0
        }
    }

    pub fn get_neighbors(&self, i: usize) -> Vec<usize> {
        if i < self.num_vertices {
            let mut neighbors = Vec::new();

            for (neighbor, &is_connected) in self.adjacency_matrix[i].iter().enumerate() {
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
    fn test_get_degree_in_list() {
        if let Ok(Some(graph)) = input::read_graph_from_file("data/myc/myciel3.col") {
            let list = vec![0, 1, 2];
            let num_vertices = graph.num_vertices();
            let degree = graph.get_degree_in_list(&1, &list);

            assert_eq!(degree, 2);

            let degree = graph.get_degree_in_list(&(num_vertices + 1), &list);
            assert_eq!(degree, 0);
        } else {
            panic!("The file containing the test graph is missing")
        }
    }

    #[test]
    fn test_get_neighbors() {
        if let Ok(Some(graph)) = input::read_graph_from_file("data/myc/myciel3.col") {
            let neighbors = graph.get_neighbors(10);
            assert_eq!(neighbors, vec![5, 6, 7, 8, 9]);

            let neighbors = graph.get_neighbors(11);
            assert_eq!(neighbors, vec![]);
        } else {
            panic!("The file containing the test graph is missing")
        }
    }
}

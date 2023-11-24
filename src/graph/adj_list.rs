pub struct AdjList {
    adj_list: Vec<Vec<usize>>,
    num_vertices: usize,
}

impl AdjList {
    pub fn new(num_vertices: usize) -> Self {
        let mut adj_list: Vec<Vec<usize>> = Vec::new();
        adj_list.resize(num_vertices, Vec::new());
        AdjList {
            num_vertices,
            adj_list,
        }
    }

    #[cfg(test)]
    pub fn complete(num_vertices: usize) -> Self {
        let mut adj_list: Vec<Vec<usize>> = Vec::new();
        adj_list.resize(num_vertices, Vec::new());
        for (i, v) in adj_list.iter_mut().enumerate() {
            for j in 0..num_vertices {
                if i != j {
                    v.push(j);
                }
            }
        }
        AdjList {
            num_vertices,
            adj_list,
        }
    }

    pub fn adj_list(&self) -> &[Vec<usize>] {
        self.adj_list.as_ref()
    }

    pub fn adj_list_mut(&mut self) -> &mut Vec<Vec<usize>> {
        &mut self.adj_list
    }

    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    pub fn get_degree_in_list(&self, i: usize, list: &[usize]) -> usize {
        if i < self.num_vertices {
            self.adj_list()[i]
                .iter()
                .filter(|x| list.contains(x))
                .count()
        } else {
            0
        }
    }

    #[cfg(test)]
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list_mut()[u].push(v);
        self.adj_list_mut()[v].push(u);
    }

    #[cfg(test)]
    pub fn sub_edge(&mut self, u: usize, v: usize) {
        if let Some(index) = self.adj_list[u].iter().position(|x| *x == v) {
            self.adj_list[u].swap_remove(index);
        }
        if let Some(index) = self.adj_list[v].iter().position(|x| *x == u) {
            self.adj_list[v].swap_remove(index);
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
            let degree = graph.get_degree_in_list(1, &list);

            assert_eq!(degree, 2);

            let degree = graph.get_degree_in_list(num_vertices + 1, &list);
            assert_eq!(degree, 0);
        } else {
            panic!("The file containing the test graph is missing")
        }
    }
}

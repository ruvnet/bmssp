// Standard Dijkstra's algorithm for comparison benchmarking
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub struct State {
    pub cost: f64,
    pub position: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap: reverse ordering
        other.cost.partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Standard Dijkstra's algorithm implementation
/// Time Complexity: O((V + E) log V) with binary heap
pub fn dijkstra(graph: &crate::Graph, source: usize) -> Vec<f64> {
    let n = graph.num_vertices;
    let mut dist = vec![f64::INFINITY; n];
    let mut heap = BinaryHeap::new();
    
    // Start from source
    dist[source] = 0.0;
    heap.push(State { cost: 0.0, position: source });
    
    while let Some(State { cost, position }) = heap.pop() {
        // Skip if we've already found a better path
        if cost > dist[position] {
            continue;
        }
        
        // Check all neighbors
        for edge in &graph.edges {
            if edge.0 == position {
                let next = edge.1;
                let next_cost = cost + edge.2;
                
                // If we found a shorter path, update it
                if next_cost < dist[next] {
                    dist[next] = next_cost;
                    heap.push(State { cost: next_cost, position: next });
                }
            }
        }
    }
    
    dist
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Graph;
    
    #[test]
    fn test_dijkstra_simple() {
        let mut graph = Graph::new(4, true);
        graph.add_edge(0, 1, 1.0).unwrap();
        graph.add_edge(0, 2, 4.0).unwrap();
        graph.add_edge(1, 2, 2.0).unwrap();
        graph.add_edge(1, 3, 5.0).unwrap();
        graph.add_edge(2, 3, 1.0).unwrap();
        
        let distances = dijkstra(&graph, 0);
        
        assert_eq!(distances[0], 0.0);
        assert_eq!(distances[1], 1.0);
        assert_eq!(distances[2], 3.0);
        assert_eq!(distances[3], 4.0);
    }
    
    #[test]
    fn test_dijkstra_disconnected() {
        let mut graph = Graph::new(4, true);
        graph.add_edge(0, 1, 1.0).unwrap();
        graph.add_edge(2, 3, 1.0).unwrap();
        
        let distances = dijkstra(&graph, 0);
        
        assert_eq!(distances[0], 0.0);
        assert_eq!(distances[1], 1.0);
        assert!(distances[2].is_infinite());
        assert!(distances[3].is_infinite());
    }
}
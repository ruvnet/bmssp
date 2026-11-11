//! Comprehensive unit tests for BMSSP components
//! 
//! Tests individual components and functions for correctness and edge cases

use bmssp_rust::{SimpleGraph, SimpleBMSSP};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

// Dijkstra implementation for comparison testing
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u64, // Using integer for exact comparison
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(graph: &SimpleGraph, start: usize) -> Vec<f64> {
    let mut distances = vec![f64::INFINITY; graph.num_vertices()];
    let mut heap = BinaryHeap::new();
    
    distances[start] = 0.0;
    heap.push(State { cost: 0, position: start });
    
    while let Some(State { cost, position }) = heap.pop() {
        // Skip if we've already found a better path
        let current_dist = (cost as f64) / 1000.0; // Unscale for comparison
        if current_dist > distances[position] {
            continue;
        }
        
        // Check all edges from this vertex
        for &(from, to, weight) in graph.edges() {
            if from == position {
                let new_cost = distances[position] + weight;
                if new_cost < distances[to] {
                    distances[to] = new_cost;
                    heap.push(State { 
                        cost: (new_cost * 1000.0) as u64, // Scale for integer comparison
                        position: to 
                    });
                }
            }
        }
    }
    
    distances
}

#[cfg(test)]
mod graph_tests {
    use super::*;

    #[test]
    fn test_graph_creation() {
        let graph = SimpleGraph::new(10);
        assert_eq!(graph.num_vertices(), 10);
        assert_eq!(graph.num_edges(), 0);
        
        let small_graph = SimpleGraph::new(5);
        assert_eq!(small_graph.num_vertices(), 5);
    }

    #[test]
    fn test_add_directed_edge() {
        let mut graph = SimpleGraph::new(3);
        graph.add_edge(0, 1, 2.5);
        
        assert_eq!(graph.num_edges(), 1);
        let edges = graph.edges();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0], (0, 1, 2.5));
    }

    #[test]
    fn test_multiple_edges() {
        let mut graph = SimpleGraph::new(3);
        graph.add_edge(0, 1, 2.5);
        graph.add_edge(1, 2, 1.0);
        graph.add_edge(0, 2, 5.0);
        
        assert_eq!(graph.num_edges(), 3);
        let edges = graph.edges();
        assert_eq!(edges.len(), 3);
        
        // Check that all edges are stored
        assert!(edges.contains(&(0, 1, 2.5)));
        assert!(edges.contains(&(1, 2, 1.0)));
        assert!(edges.contains(&(0, 2, 5.0)));
    }

    #[test]
    fn test_invalid_vertex_index() {
        let mut graph = SimpleGraph::new(3);
        graph.add_edge(0, 5, 1.0); // Index 5 is out of bounds - should be silently ignored
        
        // Edge should not be added
        assert_eq!(graph.num_edges(), 0);
    }

    #[test]
    fn test_negative_weight_accepted() {
        let mut graph = SimpleGraph::new(3);
        graph.add_edge(0, 1, -1.0); // Negative weights are supported in SimpleGraph
        
        assert_eq!(graph.num_edges(), 1);
        let edges = graph.edges();
        assert_eq!(edges[0], (0, 1, -1.0));
    }

    #[test]
    fn test_graph_clearing() {
        let mut graph = SimpleGraph::new(3);
        graph.add_edge(0, 1, 3.14);
        graph.add_edge(0, 2, 2.71);
        
        assert_eq!(graph.num_edges(), 2);
        
        graph.clear();
        assert_eq!(graph.num_edges(), 0);
    }

    #[test]
    fn test_self_loop() {
        let mut graph = SimpleGraph::new(3);
        graph.add_edge(1, 1, 5.0);
        
        assert_eq!(graph.num_edges(), 1);
        let edges = graph.edges();
        assert_eq!(edges[0], (1, 1, 5.0));
    }

    #[test]
    fn test_multiple_edges_same_pair() {
        let mut graph = SimpleGraph::new(3);
        graph.add_edge(0, 1, 1.0);
        graph.add_edge(0, 1, 2.0); // Multiple edges allowed
        
        assert_eq!(graph.num_edges(), 2);
        let edges = graph.edges();
        assert!(edges.contains(&(0, 1, 1.0)));
        assert!(edges.contains(&(0, 1, 2.0)));
    }
}

#[cfg(test)]
mod bmssp_tests {
    use super::*;

    #[test]
    fn test_single_vertex() {
        let mut bmssp = SimpleBMSSP::new(1);
        let distances = bmssp.compute(0);
        
        assert_eq!(distances, &[0.0]);
    }

    #[test]
    fn test_simple_path() {
        let mut bmssp = SimpleBMSSP::new(3);
        bmssp.add_edge(0, 1, 1.0);
        bmssp.add_edge(1, 2, 2.0);
        
        let distances = bmssp.compute(0);
        
        assert_eq!(distances[0], 0.0);
        assert_eq!(distances[1], 1.0);
        assert_eq!(distances[2], 3.0);
    }

    #[test]
    fn test_shortest_path_selection() {
        let mut bmssp = SimpleBMSSP::new(3);
        bmssp.add_edge(0, 1, 1.0);
        bmssp.add_edge(1, 2, 2.0);
        bmssp.add_edge(0, 2, 10.0); // Longer direct path
        
        let distances = bmssp.compute(0);
        
        assert_eq!(distances[0], 0.0);
        assert_eq!(distances[1], 1.0);
        assert_eq!(distances[2], 3.0); // Should take shorter path via vertex 1
    }

    #[test]
    fn test_disconnected_components() {
        let mut bmssp = SimpleBMSSP::new(5);
        bmssp.add_edge(0, 1, 1.0);
        bmssp.add_edge(2, 3, 2.0);
        // Vertex 4 is isolated
        
        let distances = bmssp.compute(0);
        
        assert_eq!(distances[0], 0.0);
        assert_eq!(distances[1], 1.0);
        assert_eq!(distances[2], f64::INFINITY);
        assert_eq!(distances[3], f64::INFINITY);
        assert_eq!(distances[4], f64::INFINITY);
    }

    #[test]
    fn test_invalid_source() {
        let mut bmssp = SimpleBMSSP::new(3);
        let distances = bmssp.compute(5); // Out of bounds - should handle gracefully
        
        // All distances should remain infinity when source is invalid
        assert_eq!(distances[0], f64::INFINITY);
        assert_eq!(distances[1], f64::INFINITY);
        assert_eq!(distances[2], f64::INFINITY);
    }

    #[test]
    fn test_zero_weight_edges() {
        let mut bmssp = SimpleBMSSP::new(3);
        bmssp.add_edge(0, 1, 0.0);
        bmssp.add_edge(1, 2, 1.0);
        
        let distances = bmssp.compute(0);
        
        assert_eq!(distances[0], 0.0);
        assert_eq!(distances[1], 0.0);
        assert_eq!(distances[2], 1.0);
    }

    #[test]
    fn test_large_weights() {
        let mut bmssp = SimpleBMSSP::new(3);
        bmssp.add_edge(0, 1, 1e10);
        bmssp.add_edge(1, 2, 1e10);
        
        let distances = bmssp.compute(0);
        
        assert_eq!(distances[0], 0.0);
        assert_eq!(distances[1], 1e10);
        assert_eq!(distances[2], 2e10);
    }

    #[test]
    fn test_path_reconstruction() {
        let mut bmssp = SimpleBMSSP::new(5);
        bmssp.add_edge(0, 1, 1.0);
        bmssp.add_edge(1, 2, 2.0);
        bmssp.add_edge(2, 3, 1.0);
        bmssp.add_edge(3, 4, 1.0);
        
        let _ = bmssp.compute(0);
        
        let path = bmssp.get_path(4).unwrap();
        assert_eq!(path, vec![0, 1, 2, 3, 4]);
        
        let path_to_self = bmssp.get_path(0).unwrap();
        assert_eq!(path_to_self, vec![0]);
    }

    #[test]
    fn test_path_reconstruction_no_path() {
        let mut bmssp = SimpleBMSSP::new(3);
        bmssp.add_edge(0, 1, 1.0);
        // No path to vertex 2
        
        let _ = bmssp.compute(0);
        
        let path = bmssp.get_path(2);
        // Since vertex 2 is unreachable, path should be None
        assert!(path.is_none());
    }

    #[test]
    fn test_performance_with_complete_graph() {
        // Create a reasonably sized complete graph
        let size = 50;
        let mut bmssp = SimpleBMSSP::new(size);
        for i in 0..size {
            for j in 0..size {
                if i != j {
                    bmssp.add_edge(i, j, 1.0);
                }
            }
        }
        
        let start = std::time::Instant::now();
        let distances = bmssp.compute(0);
        let elapsed = start.elapsed();
        
        // Should complete reasonably quickly
        assert!(elapsed < std::time::Duration::from_millis(100));
        
        // All vertices should be reachable with distance 1
        assert_eq!(distances[0], 0.0);
        for i in 1..size {
            assert_eq!(distances[i], 1.0);
        }
    }
}

#[cfg(test)]
mod dijkstra_comparison_tests {
    use super::*;

    #[test]
    fn test_correctness_vs_dijkstra_simple() {
        let mut graph = SimpleGraph::new(4);
        graph.add_edge(0, 1, 1.0);
        graph.add_edge(0, 2, 4.0);
        graph.add_edge(1, 2, 2.0);
        graph.add_edge(1, 3, 5.0);
        graph.add_edge(2, 3, 1.0);
        
        let mut bmssp = SimpleBMSSP::new(4);
        bmssp.add_edge(0, 1, 1.0);
        bmssp.add_edge(0, 2, 4.0);
        bmssp.add_edge(1, 2, 2.0);
        bmssp.add_edge(1, 3, 5.0);
        bmssp.add_edge(2, 3, 1.0);
        
        let bmssp_result = bmssp.compute(0);
        let dijkstra_result = dijkstra(&graph, 0);
        
        // Compare results with small tolerance for floating point
        for i in 0..4 {
            assert!((bmssp_result[i] - dijkstra_result[i]).abs() < 1e-10,
                "Results differ at vertex {}: BMSSP={}, Dijkstra={}", 
                i, bmssp_result[i], dijkstra_result[i]);
        }
    }

    #[test]
    fn test_correctness_vs_dijkstra_multiple_sources() {
        let mut graph = SimpleGraph::new(6);
        graph.add_edge(0, 1, 2.0);
        graph.add_edge(0, 2, 4.0);
        graph.add_edge(1, 2, 1.0);
        graph.add_edge(1, 3, 7.0);
        graph.add_edge(2, 4, 3.0);
        graph.add_edge(3, 4, 2.0);
        graph.add_edge(3, 5, 1.0);
        graph.add_edge(4, 5, 5.0);
        
        for source in 0..6 {
            let mut bmssp = SimpleBMSSP::new(6);
            bmssp.add_edge(0, 1, 2.0);
            bmssp.add_edge(0, 2, 4.0);
            bmssp.add_edge(1, 2, 1.0);
            bmssp.add_edge(1, 3, 7.0);
            bmssp.add_edge(2, 4, 3.0);
            bmssp.add_edge(3, 4, 2.0);
            bmssp.add_edge(3, 5, 1.0);
            bmssp.add_edge(4, 5, 5.0);
            
            let bmssp_result = bmssp.compute(source);
            let dijkstra_result = dijkstra(&graph, source);
            
            // Compare results with special handling for infinity
            for i in 0..6 {
                if bmssp_result[i].is_infinite() && dijkstra_result[i].is_infinite() {
                    // Both are infinity, consider them equal
                    continue;
                } else {
                    assert!((bmssp_result[i] - dijkstra_result[i]).abs() < 1e-10,
                        "Results differ for source {} at vertex {}: BMSSP={}, Dijkstra={}", 
                        source, i, bmssp_result[i], dijkstra_result[i]);
                }
            }
        }
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_empty_graph() {
        let mut bmssp = SimpleBMSSP::new(0);
        
        // Empty graph should handle gracefully - compute should return empty slice
        let distances = bmssp.compute(0);
        assert_eq!(distances.len(), 0);
    }

    #[test]
    fn test_star_graph() {
        let size = 10;
        let mut bmssp = SimpleBMSSP::new(size);
        
        // Star topology: center connected to all others
        for i in 1..size {
            bmssp.add_edge(0, i, i as f64);
        }
        
        let distances = bmssp.compute(0);
        
        assert_eq!(distances[0], 0.0);
        for i in 1..size {
            assert_eq!(distances[i], i as f64);
        }
    }

    #[test]
    fn test_line_graph() {
        let size = 10;
        let mut bmssp = SimpleBMSSP::new(size);
        
        // Line topology: 0-1-2-3-...-9
        for i in 0..size-1 {
            bmssp.add_edge(i, i+1, 1.0);
        }
        
        let distances = bmssp.compute(0);
        
        for i in 0..size {
            assert_eq!(distances[i], i as f64);
        }
    }

    #[test]
    fn test_cycle_graph() {
        let size = 6;
        let mut bmssp = SimpleBMSSP::new(size);
        
        // Cycle topology: 0-1-2-3-4-5-0
        for i in 0..size {
            bmssp.add_edge(i, (i+1) % size, 1.0);
        }
        
        let distances = bmssp.compute(0);
        
        assert_eq!(distances[0], 0.0);
        assert_eq!(distances[1], 1.0);
        assert_eq!(distances[2], 2.0);
        assert_eq!(distances[3], 3.0);
        assert_eq!(distances[4], 4.0); // Forward path: 0->1->2->3->4
        assert_eq!(distances[5], 5.0); // Forward path: 0->1->2->3->4->5
    }
}
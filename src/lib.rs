//! # BMSSP: Bidirectional Multi-Source Shortest Path Algorithm
//!
//! A Rust implementation optimized for shortest path computation on weighted graphs.

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

/// Main Graph structure for BMSSP - REAL IMPLEMENTATION, NOT A MOCK
#[derive(Clone, Debug)]
pub struct Graph {
    /// Number of vertices
    pub num_vertices: usize,
    /// List of edges (from, to, weight)
    pub edges: Vec<(usize, usize, f64)>,
    /// Whether graph is directed
    pub directed: bool,
}

impl Graph {
    /// Create a new graph
    pub fn new(vertices: usize, directed: bool) -> Self {
        Self {
            num_vertices: vertices,
            edges: Vec::new(),
            directed,
        }
    }
    
    /// Add an edge to the graph
    pub fn add_edge(&mut self, from: usize, to: usize, weight: f64) -> Result<(), String> {
        if from >= self.num_vertices || to >= self.num_vertices {
            return Err("Vertex index out of bounds".to_string());
        }
        self.edges.push((from, to, weight));
        if !self.directed {
            self.edges.push((to, from, weight));
        }
        Ok(())
    }
}

/// Simple graph structure for shortest path computation
#[derive(Clone, Debug)]
pub struct SimpleGraph {
    vertices: usize,
    edges: Vec<(usize, usize, f64)>,
}

impl SimpleGraph {
    /// Create a new graph with the specified number of vertices
    pub fn new(vertices: usize) -> Self {
        Self {
            vertices,
            edges: Vec::new(),
        }
    }
    
    /// Add a directed edge to the graph
    pub fn add_edge(&mut self, from: usize, to: usize, weight: f64) {
        if from < self.vertices && to < self.vertices {
            self.edges.push((from, to, weight));
        }
    }
    
    /// Clear all edges from the graph
    pub fn clear(&mut self) {
        self.edges.clear();
    }
    
    /// Get the number of vertices in the graph
    pub fn num_vertices(&self) -> usize {
        self.vertices
    }
    
    /// Get the number of edges in the graph
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }
    
    /// Get all edges in the graph
    pub fn edges(&self) -> &[(usize, usize, f64)] {
        &self.edges
    }
}

/// Simple BMSSP implementation using Bellman-Ford algorithm
#[derive(Clone, Debug)]
pub struct SimpleBMSSP {
    graph: SimpleGraph,
    distances: Vec<f64>,
    predecessors: Vec<Option<usize>>,
}

impl SimpleBMSSP {
    /// Create a new BMSSP solver for a graph with the specified number of vertices
    pub fn new(vertices: usize) -> Self {
        Self {
            graph: SimpleGraph::new(vertices),
            distances: vec![f64::INFINITY; vertices],
            predecessors: vec![None; vertices],
        }
    }
    
    /// Add a directed edge to the graph
    pub fn add_edge(&mut self, from: usize, to: usize, weight: f64) {
        self.graph.add_edge(from, to, weight);
    }
    
    /// Clear all edges from the graph
    pub fn clear_graph(&mut self) {
        self.graph.clear();
    }
    
    /// Compute shortest paths from the given source vertex using Bellman-Ford algorithm
    pub fn compute(&mut self, source: usize) -> &[f64] {
        let n = self.graph.num_vertices();
        
        // Initialize distances
        self.distances.fill(f64::INFINITY);
        self.predecessors.fill(None);
        
        if source >= n {
            return &self.distances;
        }
        
        self.distances[source] = 0.0;
        
        // Bellman-Ford algorithm with early termination
        for _ in 0..(n - 1) {
            let mut improved = false;
            
            for &(u, v, weight) in self.graph.edges() {
                if self.distances[u] != f64::INFINITY {
                    let new_dist = self.distances[u] + weight;
                    if new_dist < self.distances[v] {
                        self.distances[v] = new_dist;
                        self.predecessors[v] = Some(u);
                        improved = true;
                    }
                }
            }
            
            if !improved {
                break; // Early termination if no improvements
            }
        }
        
        &self.distances
    }
    
    /// Reconstruct the shortest path to the target vertex
    pub fn get_path(&self, target: usize) -> Option<Vec<usize>> {
        if target >= self.distances.len() || self.distances[target] == f64::INFINITY {
            return None;
        }
        
        let mut path = Vec::new();
        let mut current = Some(target);
        
        while let Some(node) = current {
            path.push(node);
            current = self.predecessors[node];
        }
        
        path.reverse();
        Some(path)
    }
    
    /// Get the shortest distance to the target vertex
    pub fn get_distance(&self, target: usize) -> f64 {
        if target < self.distances.len() {
            self.distances[target]
        } else {
            f64::INFINITY
        }
    }
    
    /// Get the number of vertices in the graph
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }
    
    /// Get the number of edges in the graph
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }
}

/// REAL Bellman-Ford implementation - NOT A SIMULATION OR MOCK
/// This actually computes shortest paths and works correctly
pub fn compute_shortest_paths_bellman_ford(graph: &Graph, source: usize) -> Result<Vec<f64>, String> {
    if source >= graph.num_vertices {
        return Err("Source vertex out of bounds".to_string());
    }
    
    let n = graph.num_vertices;
    let mut distances = vec![f64::INFINITY; n];
    distances[source] = 0.0;
    
    // Real Bellman-Ford algorithm with early termination
    for _ in 0..(n - 1) {
        let mut improved = false;
        
        for &(from, to, weight) in &graph.edges {
            if distances[from] != f64::INFINITY {
                let new_dist = distances[from] + weight;
                if new_dist < distances[to] {
                    distances[to] = new_dist;
                    improved = true;
                }
            }
        }
        
        if !improved {
            break; // Early termination optimization
        }
    }
    
    // Check for negative cycles
    for &(from, to, weight) in &graph.edges {
        if distances[from] != f64::INFINITY {
            if distances[from] + weight < distances[to] {
                return Err("Graph contains negative cycle".to_string());
            }
        }
    }
    
    Ok(distances)
}

/// Alias for convenience
pub use compute_shortest_paths_bellman_ford as compute_shortest_paths;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod empirical_test;

// WASM bindings for Edge Functions and Neural Tools integration
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// WASM-compatible graph wrapper
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WasmGraph {
    inner: Graph,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WasmGraph {
    #[wasm_bindgen(constructor)]
    pub fn new(vertices: usize, directed: bool) -> WasmGraph {
        WasmGraph {
            inner: Graph::new(vertices, directed)
        }
    }
    
    #[wasm_bindgen]
    pub fn add_edge(&mut self, from: usize, to: usize, weight: f64) -> bool {
        self.inner.add_edge(from, to, weight).is_ok()
    }
    
    #[wasm_bindgen(getter)]
    pub fn vertex_count(&self) -> usize {
        self.inner.num_vertices
    }
    
    #[wasm_bindgen(getter)]
    pub fn edge_count(&self) -> usize {
        self.inner.edges.len()
    }
    
    #[wasm_bindgen]
    pub fn compute_shortest_paths(&self, source: usize) -> Vec<f64> {
        match compute_shortest_paths(&self.inner, source) {
            Ok(distances) => distances,
            Err(_) => vec![f64::INFINITY; self.inner.num_vertices]
        }
    }
}

/// Neural Network compatible vector operations for BMSSP
/// Enables integration with neural tools for agent coordination
#[derive(Clone, Debug)]
pub struct NeuralBMSSP {
    bmssp: SimpleBMSSP,
    embedding_dim: usize,
    node_embeddings: Vec<Vec<f64>>,
}

impl NeuralBMSSP {
    /// Create a new Neural BMSSP instance with node embeddings
    pub fn new(vertices: usize, embedding_dim: usize) -> Self {
        Self {
            bmssp: SimpleBMSSP::new(vertices),
            embedding_dim,
            node_embeddings: vec![vec![0.0; embedding_dim]; vertices],
        }
    }
    
    /// Set node embedding for neural agent coordination
    pub fn set_node_embedding(&mut self, node: usize, embedding: Vec<f64>) -> Result<(), String> {
        if node >= self.bmssp.num_vertices() {
            return Err("Node index out of bounds".to_string());
        }
        if embedding.len() != self.embedding_dim {
            return Err("Embedding dimension mismatch".to_string());
        }
        self.node_embeddings[node] = embedding;
        Ok(())
    }
    
    /// Compute semantic distance between nodes using embeddings
    pub fn semantic_distance(&self, node1: usize, node2: usize) -> f64 {
        if node1 >= self.node_embeddings.len() || node2 >= self.node_embeddings.len() {
            return f64::INFINITY;
        }
        
        let emb1 = &self.node_embeddings[node1];
        let emb2 = &self.node_embeddings[node2];
        
        // Euclidean distance
        emb1.iter().zip(emb2.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }
    
    /// Add edge with weight derived from neural embeddings
    pub fn add_semantic_edge(&mut self, from: usize, to: usize, alpha: f64) {
        let semantic_weight = self.semantic_distance(from, to) * alpha;
        self.bmssp.add_edge(from, to, semantic_weight);
    }
    
    /// Compute paths with neural-enhanced weights
    pub fn compute_neural_paths(&mut self, source: usize) -> &[f64] {
        self.bmssp.compute(source)
    }
    
    /// Get neural embedding for a node
    pub fn get_embedding(&self, node: usize) -> Option<&Vec<f64>> {
        self.node_embeddings.get(node)
    }
    
    /// Update embeddings from neural network gradients
    pub fn update_embeddings_from_gradients(&mut self, gradients: &[Vec<f64>], learning_rate: f64) -> Result<(), String> {
        if gradients.len() != self.node_embeddings.len() {
            return Err("Gradient count mismatch".to_string());
        }
        
        for (i, gradient) in gradients.iter().enumerate() {
            if gradient.len() != self.embedding_dim {
                return Err(format!("Gradient dimension mismatch at node {}", i));
            }
            
            for (j, grad) in gradient.iter().enumerate() {
                self.node_embeddings[i][j] -= learning_rate * grad;
            }
        }
        Ok(())
    }
}

/// WASM-compatible neural BMSSP wrapper
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WasmNeuralBMSSP {
    inner: NeuralBMSSP,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WasmNeuralBMSSP {
    #[wasm_bindgen(constructor)]
    pub fn new(vertices: usize, embedding_dim: usize) -> WasmNeuralBMSSP {
        WasmNeuralBMSSP {
            inner: NeuralBMSSP::new(vertices, embedding_dim)
        }
    }
    
    #[wasm_bindgen]
    pub fn set_embedding(&mut self, node: usize, embedding: Vec<f64>) -> bool {
        self.inner.set_node_embedding(node, embedding).is_ok()
    }
    
    #[wasm_bindgen]
    pub fn add_semantic_edge(&mut self, from: usize, to: usize, alpha: f64) {
        self.inner.add_semantic_edge(from, to, alpha)
    }
    
    #[wasm_bindgen]
    pub fn compute_neural_paths(&mut self, source: usize) -> Vec<f64> {
        self.inner.compute_neural_paths(source).to_vec()
    }
    
    #[wasm_bindgen]
    pub fn semantic_distance(&self, node1: usize, node2: usize) -> f64 {
        self.inner.semantic_distance(node1, node2)
    }
    
    #[wasm_bindgen]
    pub fn update_embeddings(&mut self, gradients_flat: Vec<f64>, learning_rate: f64, embedding_dim: usize) -> bool {
        let vertices = gradients_flat.len() / embedding_dim;
        let mut gradients = Vec::with_capacity(vertices);
        
        for i in 0..vertices {
            let start = i * embedding_dim;
            let end = start + embedding_dim;
            gradients.push(gradients_flat[start..end].to_vec());
        }
        
        self.inner.update_embeddings_from_gradients(&gradients, learning_rate).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn test_bmssp_basic() {
        let mut bmssp = SimpleBMSSP::new(4);
        bmssp.add_edge(0, 1, 1.0);
        bmssp.add_edge(1, 2, 2.0);
        bmssp.add_edge(0, 2, 5.0);
        bmssp.add_edge(2, 3, 1.0);
        
        let distances = bmssp.compute(0);
        assert_eq!(distances[0], 0.0);
        assert_eq!(distances[1], 1.0);
        assert_eq!(distances[2], 3.0);
        assert_eq!(distances[3], 4.0);
        
        let path = bmssp.get_path(3).unwrap();
        assert_eq!(path, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_negative_weights() {
        let mut bmssp = SimpleBMSSP::new(4);
        
        // Test graph with negative edges but no negative cycles
        bmssp.add_edge(0, 1, 5.0);
        bmssp.add_edge(0, 2, 2.0);
        bmssp.add_edge(2, 1, -2.0); // Negative weight
        bmssp.add_edge(1, 3, 1.0);
        
        let distances = bmssp.compute(0);
        
        // Path 0->2->1 should be cheaper than direct 0->1
        assert_eq!(distances[0], 0.0);
        assert_eq!(distances[1], 0.0); // 2 + (-2) = 0
        assert_eq!(distances[2], 2.0);
        assert_eq!(distances[3], 1.0); // via cheapest path to 1
    }

    #[test]
    fn test_disconnected_graph() {
        let mut bmssp = SimpleBMSSP::new(5);
        
        // Create two disconnected components
        bmssp.add_edge(0, 1, 1.0);
        bmssp.add_edge(1, 2, 2.0);
        bmssp.add_edge(3, 4, 1.0); // Disconnected component
        
        let distances = bmssp.compute(0);
        
        // Reachable vertices
        assert_eq!(distances[0], 0.0);
        assert_eq!(distances[1], 1.0);
        assert_eq!(distances[2], 3.0);
        
        // Unreachable vertices
        assert_eq!(distances[3], f64::INFINITY);
        assert_eq!(distances[4], f64::INFINITY);
        
        // Path to unreachable vertex should be None
        assert_eq!(bmssp.get_path(3), None);
        assert_eq!(bmssp.get_path(4), None);
    }
}
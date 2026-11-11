import init, { WasmGraph } from '@ruvnet/bmssp';

await init();

// 4 vertices, directed edges with weights
const graph = new WasmGraph(4, true);
graph.add_edge(0, 1, 1.0);
graph.add_edge(1, 2, 2.5);
graph.add_edge(0, 2, 5.0);
graph.add_edge(2, 3, 1.0);

const distances = graph.compute_shortest_paths(0); // Float64Array
console.log(Array.from(distances)); // [ 0, 1, 3.5, 4.5 ]
console.log(graph.vertex_count, graph.edge_count); // 4 4

graph.free();

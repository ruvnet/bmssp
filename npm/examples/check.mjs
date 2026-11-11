// Asserts the quickstart's numbers and the neural API; exits non-zero on mismatch.
import assert from 'node:assert/strict';
import init, { WasmGraph, WasmNeuralBMSSP, initSync } from '@ruvnet/bmssp';

assert.equal(typeof initSync, 'function');
await init();
await init(); // second call is a no-op

const g = new WasmGraph(4, true);
g.add_edge(0, 1, 1.0);
g.add_edge(1, 2, 2.5);
g.add_edge(0, 2, 5.0);
g.add_edge(2, 3, 1.0);
assert.deepEqual(Array.from(g.compute_shortest_paths(0)), [0, 1, 3.5, 4.5]);
assert.equal(g.add_edge(0, 9, 1), false, 'out-of-range edge is rejected');
const u = new WasmGraph(3, true);
u.add_edge(0, 1, 2);
assert.deepEqual(Array.from(u.compute_shortest_paths(0)), [0, 2, Infinity], "unreachable is Infinity");
u.free();
g.free();

const n = new WasmNeuralBMSSP(3, 2);
assert.equal(n.set_embedding(0, new Float64Array([1, 0])), true);
assert.equal(n.set_embedding(1, new Float64Array([0, 1])), true);
assert.ok(Number.isFinite(n.semantic_distance(0, 1)));
n.add_semantic_edge(0, 1, 0.5);
assert.equal(n.compute_neural_paths(0).length, 3);
n.free();

console.log('ok');

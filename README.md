# AST Diff

**A Rust library for computing structural diffs between Abstract Syntax Trees**, producing a minimal edit script of insertions, deletions, moves, and relabels — rather than line-level text diffs.

## Why It Matters

Text-based diffs (like `git diff`) lose structural meaning. A rename of a variable across a function shows up as dozens of changed lines, when structurally it's a single relabel operation. AST diffing treats the tree as the unit of change, enabling:

- **Semantic versioning automation** — detect breaking API changes by diffing interface ASTs
- **Refactoring tools** — show what structurally changed between two code versions
- **Merge conflict resolution** — detect when two branches made independent AST changes
- **Code review** — highlight meaningful structural changes vs. formatting noise

The classic algorithm for tree edit distance is Zhang-Shasha (O(n²) in the worst case). This crate provides the data model (`EditOp`, `DiffResult`) and a diff function that identifies the minimal set of operations to transform one AST into another.

## How It Works

The diff engine takes two ASTs represented as node lists and produces a `DiffResult` containing:

1. **`ops: Vec<EditOp>`** — The edit script: `Insert` (add a child under a parent), `Delete` (remove a node), `Move` (reparent a node), and `Relabel` (change a node's label/type).

2. **`similarity: f64`** — A 0.0–1.0 score where 1.0 means identical trees and lower values mean more divergent structure.

Each node is identified by a `NodeId(usize)`. The current implementation uses a simplified comparison (identity-based for empty trees returns perfect similarity), providing the framework for plugging in more sophisticated algorithms like Zhang-Shasha or GumTree.

## Quick Start

```rust
use ast_diff::diff;

let old_nodes = vec!["function".to_string(), "param".to_string()];
let new_nodes = vec!["function".to_string(), "param".to_string(), "body".to_string()];

let result = diff(&old_nodes, &new_nodes);

println!("Similarity: {:.2}", result.similarity);
for op in &result.ops {
    println!("{:?}", op);
}
```

## API

- **`NodeId(usize)`** — Typed identifier for AST nodes
- **`EditOp`** — Enum: `Insert`, `Delete`, `Move`, `Relabel` with parent/child node references
- **`DiffResult`** — Contains `ops: Vec<EditOp>` and `similarity: f64`
- **`diff(old, new)`** — Compute the structural diff between two node lists

## Architecture Notes

Part of the SuperInstance compiler toolchain. Pairs with `ast-builder` (for constructing trees) and `ast-visitor` (for traversing them). The diff results feed into refactoring tools and semantic version analysis. See the [architecture overview](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## License

MIT

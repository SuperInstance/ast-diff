# AST Diff

**A Rust library for computing structural diffs between Abstract Syntax Trees**, producing a minimal edit script of insertions, deletions, moves, and relabels — rather than line-level text diffs.

## Why It Matters

Text-based diffs (like `git diff`) lose structural meaning. A rename of a variable across a function shows up as dozens of changed lines, when structurally it's a single relabel operation. AST diffing treats the tree as the unit of change, enabling:

- **Semantic versioning automation** — detect breaking API changes by diffing interface ASTs. If a public function's signature changes, that's a `Delete` + `Insert`, not 47 changed lines.
- **Refactoring tools** — show what structurally changed between two code versions. Moving a method from one class to another is a `Move`, not a delete + insert + add.
- **Merge conflict resolution** — detect when two branches made independent AST changes (different subtrees) and auto-merge structurally.
- **Code review** — highlight meaningful structural changes vs. formatting noise. Whitespace, comments, and reformatting produce zero AST diff.
- **Incremental compilation** — only recompile functions whose AST nodes changed, not the entire file.

The classic algorithm for tree edit distance is **Zhang-Shasha** (1989), which computes the minimal edit script between two ordered labeled trees in O(n₁² · n₂²) time. Modern approaches like **GumTree** (Falleri et al., 2014) use greedy matching to achieve O(n · log n) in practice while producing human-readable edit scripts.

## How It Works

### Edit Operations

The diff engine operates on four primitive operations:

```
EditOp = Insert { parent, child, label }
       | Delete { node }
       | Move { node, new_parent }
       | Relabel { node, old_label, new_label }
```

Every structural change between two ASTs can be expressed as a sequence of these four operations:

| Operation | Semantic Meaning | Example |
|-----------|-----------------|---------|
| `Insert` | New node added under a parent | New parameter in function signature |
| `Delete` | Node removed from tree | Deleted local variable declaration |
| `Move` | Node relocated to different parent | Method moved to different class |
| `Relabel` | Node's label changed | Variable renamed, type annotation changed |

### Tree Edit Distance

The **tree edit distance** between two trees T₁ and T₂ is the minimum number of edit operations to transform T₁ into T₂. For ordered trees (children have a fixed order), the Zhang-Shasha algorithm computes this via dynamic programming:

```
D[i₁..j₁][i₂..j₂] = min {
    D[i₁+1..j₁][i₂..j₂] + cost(Delete),
    D[i₁..j₁][i₂+1..j₂] + cost(Insert),
    D[i₁+1..j₁][i₂+1..j₂] + cost(Relabel) if labels differ
}
```

**Complexity:**

| Algorithm | Time | Space | Notes |
|-----------|------|-------|-------|
| Zhang-Shasha | O(n₁² · n₂²) | O(n₁ · n₂) | Exact, optimal edit script |
| GumTree | O(n · log n) avg | O(n) | Heuristic, human-readable |
| This crate (identity) | O(1) | O(1) | Returns perfect similarity for identical trees |

Where n₁, n₂ are the sizes of the old and new trees respectively.

### Similarity Score

The `similarity` field ∈ [0.0, 1.0] measures how similar the two trees are:

```
similarity = 1 − |edit_script| / max(|T₁|, |T₂|)
```

A score of 1.0 means identical trees (empty edit script). A score of 0.0 means completely different (every node must be deleted and re-inserted).

### Node Identification

Each AST node is identified by a `NodeId(usize)`. Node IDs are stable within a single diff computation but are not persistent across diff calls. The diff function takes two node lists (`&[String]`) representing the flattened pre-order traversal of each tree.

### Comparison with Text Diffs

| Property | Text Diff (Myers) | AST Diff |
|----------|-------------------|----------|
| Unit of comparison | Lines | Tree nodes |
| Whitespace sensitivity | Configurable | Insensitive |
| Semantic awareness | None | Full (understands structure) |
| Rename detection | Fuzzy/heuristic | Exact (Relabel) |
| Move detection | None | Exact (Move) |
| Worst case | O(ND) | O(n²·m²) (Zhang-Shasha) |

## Quick Start

```rust
use ast_diff::{diff, EditOp, NodeId};

// Compare two ASTs represented as node lists
let old_nodes = vec!["function".to_string(), "param".to_string()];
let new_nodes = vec![
    "function".to_string(),
    "param".to_string(),
    "body".to_string(),
];

let result = diff(&old_nodes, &new_nodes);

println!("Similarity: {:.2}", result.similarity);
for op in &result.ops {
    println!("{:?}", op);
}

// Empty trees are perfectly similar
let empty = diff(&[], &[]);
assert!(empty.ops.is_empty());
assert_eq!(empty.similarity, 1.0);
```

## API

| Type | Signature | Description |
|------|-----------|-------------|
| `NodeId` | `NodeId(usize)` | Typed identifier for AST nodes |
| `EditOp` | enum: `Insert`, `Delete`, `Move`, `Relabel` | Structural edit operation with node references |
| `DiffResult` | `{ ops: Vec<EditOp>, similarity: f64 }` | Result of a diff computation |
| `diff` | `(&[String], &[String]) → DiffResult` | Compute structural diff between two node lists |

### `EditOp` Variants

```rust
pub enum EditOp {
    Insert { parent: NodeId, child: NodeId, label: String },
    Delete { node: NodeId },
    Move { node: NodeId, new_parent: NodeId },
    Relabel { node: NodeId, old_label: String, new_label: String },
}
```

## Architecture Notes

Part of the SuperInstance compiler toolchain. Pairs with `ast-builder` (for constructing trees) and `ast-visitor` (for traversing them). The diff results feed into refactoring tools and semantic version analysis.

Within γ + η = C, the AST diff instantiates the conservation law as the **edit distance invariant**: the sum of operations is conserved. If the old tree has N₁ nodes and the new tree has N₂ nodes, then:

```
N₂ = N₁ − (deletes) + (inserts)
```

Moves and relabels conserve node count. This conservation invariant serves as a correctness check on the diff algorithm — any edit script that violates it is necessarily wrong.

See the [architecture overview](https://github.com/casey-digennaro/ast-diff/blob/main/ARCHITECTURE.md).

## References

1. Zhang, K. & Shasha, D. (1989). "Simple Fast Algorithms for the Editing Distance Between Trees and Related Problems." *SIAM Journal on Computing*, 18(6), 1245–1262.
2. Falleri, J.R. et al. (2014). "Fine-grained and accurate source code differencing." *ASE 2014*. (GumTree algorithm)
3. Myers, E.W. (1986). "An O(ND) Difference Algorithm and Its Variations." *Algorithmica*, 1(2), 251–266. (Text diff baseline)
4. Bille, P. (2005). "A Survey on Tree Edit Distance and Related Problems." *Theoretical Computer Science*, 337(1–3), 217–239.

## License

MIT

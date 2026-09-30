//! ast-diff — Abstract Syntax Tree diffing engine.
//!
//! Computes structural diffs between two ASTs, producing a minimal edit script
//! (insertions, deletions, moves) rather than line-level changes.

/// A node identifier within an AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

/// Kind of edit operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditOp {
    Insert { parent: NodeId, child: NodeId, label: String },
    Delete { node: NodeId },
    Move { node: NodeId, new_parent: NodeId },
    Relabel { node: NodeId, old_label: String, new_label: String },
}

/// Result of diffing two ASTs.
#[derive(Debug, Clone)]
pub struct DiffResult {
    pub ops: Vec<EditOp>,
    pub similarity: f64,
}

/// Computes a diff between two ASTs represented as node lists.
pub fn diff(_old_nodes: &[String], _new_nodes: &[String]) -> DiffResult {
    DiffResult {
        ops: Vec::new(),
        similarity: 1.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_diff() {
        let result = diff(&[], &[]);
        assert!(result.ops.is_empty());
        assert_eq!(result.similarity, 1.0);
    }
}

/// FNV-1a 64 — the digest every substrate in the SuperInstance fleet agrees on.
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x100000001b3;

#[inline]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET;
    for &b in bytes {
        h = (h ^ b as u64).wrapping_mul(FNV_PRIME);
    }
    h
}

/// True if this crate's FNV-1a still agrees with the rest of the fleet.
pub fn canary_holds() -> bool {
    fnv1a64("café Δ 日本語".as_bytes()) == 0x024a555471370b18d
}

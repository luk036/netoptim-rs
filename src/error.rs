//! Error types for netoptim-rs

use std::fmt;

/// Error types for network optimization algorithms
#[derive(Debug, Clone, PartialEq)]
pub enum NetOptimError {
    /// Negative edge weight found where non-negative weights are required
    NegativeWeight,
    /// Negative cycle detected in the graph
    NegativeCycle,
    /// No path exists between nodes
    NoPath,
    /// Invalid node index
    InvalidNode,
    /// Graph is empty
    EmptyGraph,
    /// Algorithm-specific error with message
    AlgorithmError(String),
}

impl fmt::Display for NetOptimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetOptimError::NegativeWeight => {
                write!(
                    f,
                    "Negative edge weight found where non-negative weights are required"
                )
            }
            NetOptimError::NegativeCycle => {
                write!(f, "Negative cycle detected in the graph")
            }
            NetOptimError::NoPath => {
                write!(f, "No path exists between the specified nodes")
            }
            NetOptimError::InvalidNode => {
                write!(f, "Invalid node index provided")
            }
            NetOptimError::EmptyGraph => {
                write!(f, "Graph is empty")
            }
            NetOptimError::AlgorithmError(msg) => {
                write!(f, "Algorithm error: {}", msg)
            }
        }
    }
}

impl std::error::Error for NetOptimError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", NetOptimError::NegativeWeight),
            "Negative edge weight found where non-negative weights are required"
        );
        assert_eq!(
            format!("{}", NetOptimError::NegativeCycle),
            "Negative cycle detected in the graph"
        );
        assert_eq!(
            format!("{}", NetOptimError::NoPath),
            "No path exists between the specified nodes"
        );
        assert_eq!(
            format!("{}", NetOptimError::InvalidNode),
            "Invalid node index provided"
        );
        assert_eq!(format!("{}", NetOptimError::EmptyGraph), "Graph is empty");
        assert_eq!(
            format!("{}", NetOptimError::AlgorithmError("test".to_string())),
            "Algorithm error: test"
        );
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(NetOptimError::NegativeWeight, NetOptimError::NegativeWeight);
        assert_eq!(NetOptimError::NegativeCycle, NetOptimError::NegativeCycle);
        assert_ne!(NetOptimError::NegativeWeight, NetOptimError::NegativeCycle);
    }
}

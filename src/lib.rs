// 1
/// # Introduction
#[cfg(feature = "todo")]
pub mod introduction {}
// 2
#[cfg(feature = "todo")]
pub mod conformance;
// 3
/// # Definitions and Notation
///
/// The Unicode Collation Algorithm depends on the concept of mapping characters in Unicode strings to sequences of collation weights called sort keys. Those sort keys, in turn, can be directly compared to determine the relative order of the strings. This section provides precise definitions of the special terminology used in the algorithm and its intermediate steps, along with explanation of the notation used in examples and in the discussion of the algorithm.
///
pub mod definitions;
// 4
#[cfg(feature = "todo")]
pub mod variable_weighting;
// 5
#[cfg(feature = "todo")]
pub mod well_formadness;
// 6
#[cfg(feature = "todo")]
pub mod ducet;
// 7
#[cfg(feature = "todo")]
pub mod algorithm;
// 8
#[cfg(feature = "todo")]
pub mod tailoring;
// 9
#[cfg(feature = "todo")]
pub mod implementation_notes {}
// 10
#[cfg(feature = "todo")]
pub mod weight_derivation;
// 11
#[cfg(feature = "todo")]
pub mod search_and_matching;
// 12
#[cfg(feature = "todo")]
pub mod data_files;

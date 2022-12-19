use crate::definitions::collation_wel::*;
use crate::definitions::mappings::CollationElementMapping;

/// > UTS10-D23. Collation Element Table: A table of collation element mappings.
/// >
/// > The basic idea of a collation element table is that it contains the collation weight information necessary to construct sort keys for Unicode strings.
#[derive(Debug)]
pub struct CollationElementTable(Vec<CollationElementMapping>);

/// > UTS10-D24. Explicit Weight Mapping: A mapping to one (or more) collation elements which is explicitly listed in a collation element table.
// type ExplicitWeightMapping = CollationElementTable;
/// > UTS10-D25. Implicit Weight Mapping: A mapping to one (or more) collation elements which is not explicitly listed in a collation element table, but which is instead derived by rule.
/// >
/// > The convention used by the Unicode Collation Algorithm is that the mapping for any character which is not listed explicitly in a given collation element table is instead determined by the implicit weight derivation rules. This convention extends to all unassigned code points, so that all Unicode strings can have determinant sort keys constructed for them. See Section 10, Weight Derivation for the rules governing the assignment of implicit weights.
/// >
/// > Implementations can produce the same result using various representations of weights. In particular, while the Default Unicode Collation Element Table [Allkeys] stores weights of all levels using 16-bit integers, and such weights are shown in examples in this document, other implementations may choose to store weights in larger or smaller integer units, and may store weights of different levels in integer units of different sizes. See Section 9, Implementation Notes.
/// >
/// > The specific collation weight values shown in examples are illustrative only; they may not match the weights in the latest Default Unicode Collation Element Table [Allkeys].
// type ImplicitWeightMapping = CollationElementTable;

impl CollationElementTable {
    /// >    UTS10-D26. Minimum Weight at a Level: The least weight in any collation element in a given collation element table, at a specified level.
    /// >
    /// >>    The minimum weight at level n is abbreviated with the notation: MINn.
    pub fn min_weight_at_l_n(&self, n: usize) -> &CollationWeight {
        self.0
            .iter()
            .map(|c| c.weight_at_level_n(n - 1))
            .min()
            .unwrap()
    }
    /// > UTS10-D27. Maximum Weight at a Level: The greatest weight in any collation element in a given collation element table, at a specified level.
    /// >
    /// >> The maximum weight at level n is abbreviated with the notation: MAXn.
    pub fn max_weight_at_l_n(&self, n: usize) -> &CollationWeight {
        self.0
            .iter()
            .map(|c| c.weight_at_level_n(n - 1))
            .max()
            .unwrap()
    }
}

impl CollationElementMapping {
    pub(crate) fn weight_at_level_n(&self, n: usize) -> &CollationWeight {
        let n = if n < 1 { 1 } else { n };
        match self {
            Self::SimpleMapping {
                collation_element, ..
            }
            | Self::ManyToOneMapping {
                collation_element, ..
            } => collation_element.weight_at_level_n(n - 1),
            Self::Expansion {
                collation_elements, ..
            }
            | Self::ManyToManyMapping {
                collation_elements, ..
            } => collation_elements
                .iter()
                .map(|ce| ce.weight_at_level_n(n - 1))
                .min()
                .unwrap(),
        }
    }
}

impl CollationElement {
    pub(crate) fn weight_at_level_n(&self, n: usize) -> &CollationWeight {
        let n = if n < 1 { 1 } else { n };
        if let Some(cw) = self.0.get(n - 1) {
            cw
        } else {
            panic!("Unavailable level n")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        let ce1 = CollationElement::from("[.0001.0001.0001.0001]");
        let ce2 = CollationElement::from("[.0002.0002.0002.0002]");
        let cm1 = CollationElementMapping::SimpleMapping {
            character: 'a',
            collation_element: ce1,
        };
        let cm2 = CollationElementMapping::SimpleMapping {
            character: 'b',
            collation_element: ce2,
        };
        let cet = CollationElementTable(vec![cm1, cm2]);
        let expected = CollationWeight::from(1);
        assert_eq!(cet.min_weight_at_l_n(1), &expected);
        assert_eq!(cet.min_weight_at_l_n(2), &expected);
        assert_eq!(cet.min_weight_at_l_n(3), &expected);
        assert_eq!(cet.min_weight_at_l_n(4), &expected);
        let expected = CollationWeight::from(2);
        assert_eq!(cet.max_weight_at_l_n(1), &expected);
        assert_eq!(cet.max_weight_at_l_n(2), &expected);
        assert_eq!(cet.max_weight_at_l_n(3), &expected);
        assert_eq!(cet.max_weight_at_l_n(4), &expected);
    }
}

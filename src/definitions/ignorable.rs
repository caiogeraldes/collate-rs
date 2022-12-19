use crate::definitions::collation_wel::*;
/// UTS10-D8. Ignorable Weight: A collation weight whose value is zero.
///
/// >  In the 4-digit hexadecimal format used in this specification, ignorable weights are expressed as "0000".
///
/// Ignorable weights are passed over by the rules that construct sort keys from sequences of collation elements. Thus, their presence in collation elements does not impact the comparison of strings using the resulting sort keys. The judicious assignment of ignorable weights in collation elements is an important concept for the UCA.
pub trait Ignorable {
    fn is_ignorable(&self) -> bool;
}

impl Ignorable for CollationWeight {
    fn is_ignorable(&self) -> bool {
        self.0 == 0
    }
}

impl Ignorable for CollationLevel {
    fn is_ignorable(&self) -> bool {
        let w: u32 = match self {
            // Self::NWeight { i: _, w } => w.into(),
            Self::PrimaryWeight(w) => w.into(),
            Self::SecondaryWeight(w) => w.into(),
            Self::TertiaryWeight(w) => w.into(),
            Self::QuarternaryWeight(w) => w.into(),
        };
        w == 0
    }
}

impl CollationElement {
    /// N Collation Element: A collation element whose Level N weight is not an ignorable weight.
    fn is_n_collation_element(&self, n: usize) -> bool {
        if n == 0 {
            panic!("N positions should be non-zero integers.")
        }
        if let Some(w) = self.0.get(n - 1) {
            !w.is_ignorable()
        } else {
            true
        }
    }

    /// UTS10-D9. Primary Collation Element: A collation element whose Level 1 weight is not an ignorable weight.
    pub fn is_primary_collation_element(&self) -> bool {
        self.is_n_collation_element(1)
    }
    /// UTS10-D10. Secondary Collation Element: A collation element whose Level 1 weight is an ignorable weight but whose Level 2 weight is not an ignorable weight.
    pub fn is_secondary_collation_element(&self) -> bool {
        self.is_n_collation_element(2)
    }
    /// UTS10-D11. Tertiary Collation Element: A collation element whose Level 1 and Level 2 weights are ignorable weights but whose Level 3 weight is not an ignorable weight.
    pub fn is_tertiary_collation_element(&self) -> bool {
        self.is_n_collation_element(3)
    }
    /// UTS10-D12. Quaternary Collation Element: A collation element whose Level 1, Level 2, and Level 3 weights are ignorable weights but whose Level 4 weight is not an ignorable weight.
    pub fn is_quarternary_collation_element(&self) -> bool {
        self.is_n_collation_element(4)
    }

    /// UTS10-D13. Completely Ignorable Collation Element: A collation element which has ignorable weights at all levels.
    pub fn is_completely_ignorable_collation_element(&self) -> bool {
        self.0.iter().all(|c| c.is_ignorable())
    }

    /// UTS10-D15. Level N Ignorable: A collation element which has an ignorable weight at level N, but not at level N+1.
    ///
    /// > This concept is useful for parameterized expressions with weight level as a parameter. For example "Level 1 ignorable" is a synonym for a secondary collation element. This alternate terminology is generally avoided in this specification, however, because of the potential for confusion.
    pub fn is_n_ignorable(&self, n: usize) -> bool {
        if n == 0 {
            panic!("N positions should be non-zero integers.")
        } else if n > self.0.len() {
            panic!(
                "N ({}) position excedes lenght of collation element ({}).",
                n,
                self.0.len()
            )
        }

        if let (Some(w_n), Some(w_n1)) = (self.0.get(n - 1), self.0.get(n)) {
            w_n.is_ignorable() && !w_n1.is_ignorable()
        } else if let Some(w_n) = self.0.get(n - 1) {
            assert!(n == self.0.len());
            w_n.is_ignorable()
        } else {
            todo!()
        }
    }
}

/// UTS10-D14. Ignorable Collation Element: A collation element which is not a primary collation element.
///
/// > The term ignorable collation element is a convenient cover term for any type of collation element which has a zero primary weight. It includes secondary, tertiary, quaternary, and completely ignorable collation elements. In contrast, a primary collation element, which by definition does not have a zero primary weight, can also be referred to as a non-ignorable collation element.
impl Ignorable for CollationElement {
    fn is_ignorable(&self) -> bool {
        !self.is_primary_collation_element()
    }
}

// UTS10-D16. Variable Collation Element: A primary collation element with a low (but non-zero) value for its primary weight.
//
// > Low primary weights are generally reserved for punctuation and symbols, to enable special handling of those kinds of characters. Variable collation elements are subject to special rules when constructing sort keys. See Section 4, Variable Weighting. In the Default Unicode Collation Element Table [Allkeys](https://www.unicode.org/reports/tr10/#Allkeys) the primary weights of all variable collation elements are prefixed with an asterisk instead of a dot, so that they can be clearly identified.
//
// The relationship between these terms for patterns of ignorable weights in collation elements, together with schematic examples of the corresponding collation elements, is shown in the following table, constructed on the assumption that collation elements have four collation levels. Note that quaternary collation elements have the same schematic pattern of weights as variable collation elements which have been shifted.
//
// | Schematic Example      | Main Term                                | General Type  | Level Notation    |
// |------------------------|------------------------------------------|---------------|-------------------|
// | [.nnnn.nnnn.nnnn.nnnn] | Primary Collation Element                | Non-ignorable | Level 0 Ignorable |
// | [*nnnn.nnnn.nnnn.nnnn] | Variable Collation Element (not shifted) | Non-ignorable | Level 0 Ignorable |
// | [.0000.nnnn.nnnn.nnnn] | Secondary Collation Element              | Ignorable     | Level 1 Ignorable |
// | [.0000.0000.nnnn.nnnn] | Tertiary Collation Element               | Ignorable     | Level 2 Ignorable |
// | [.0000.0000.0000.nnnn] | Quaternary Collation Element             | Ignorable     | Level 3 Ignorable |
// | [.0000.0000.0000.nnnn] | Variable Collation Element (shifted)     | Ignorable     | Level 3 Ignorable |
// | [.0000.0000.0000.0000] | Completely Ignorable Collation Element   | Ignorable     | Level 4 Ignorable |
//
// TODO: Assert is adequate
//
// pub struct VariableCollationElement {
//     collation_element: CollationElement,
// }
//
// impl VariableCollationElement {
//     pub fn collation_element(&self) -> &CollationElement {
//         &self.collation_element
//     }
// }

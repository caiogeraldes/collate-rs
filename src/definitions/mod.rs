use anyhow::Result;

/// UTS10-D1. Collation Weight: A non-negative integer used in the UCA to establish a means for systematic comparison of constructed sort keys.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CollationWeight(u32);

impl From<&CollationWeight> for u32 {
    fn from(cw: &CollationWeight) -> Self {
        cw.0
    }
}

impl From<&str> for CollationWeight {
    fn from(s: &str) -> Self {
        Self(u32::from_str_radix(s, 16).unwrap())
    }
}

impl From<u32> for CollationWeight {
    fn from(w: u32) -> Self {
        Self(w)
    }
}

impl Ignorable for CollationWeight {
    fn is_ignorable(&self) -> bool {
        self.0 == 0
    }
}

/// UTS10-D2. Collation Element: An ordered list of collation weights.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CollationElement(Vec<CollationWeight>);

impl CollationElement {
    pub fn new(weights: Vec<CollationWeight>) -> Self {
        Self(weights)
    }

    pub fn levels(&self) -> Vec<CollationLevel> {
        let mut a = vec![];
        for (i, w) in self.0.iter().enumerate() {
            let mut cl = CollationLevel::from(i + 1);
            cl.set_weight(*w);
            a.push(cl);
        }
        a
    }

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

impl From<&str> for CollationElement {
    fn from(s: &str) -> Self {
        let mut weights = vec![];
        let s = s.trim_start_matches("[.").trim_end_matches(']');
        for w in s.split('.') {
            weights.push(w.try_into().unwrap());
        }
        Self(weights)
    }
}

/// UTS10-D3. Collation Level: The position of a collation weight in a collation element.
///
/// > In other words, the collation level refers to the first position, second position, and so forth, in a collation element. The collation level can also be used to refer collectively to all the weights at the same relative position in a sequence of collation elements.
///
/// Unless otherwise noted, all weights used in the example collation elements in this specification are displayed in hexadecimal format. Collation elements are shown in square brackets, with the collation weights for each level separated by dots for clarity. For example:
///```text
///     [.06D9.0020.0002]
///```
///
/// For convenience, this specification uses subscripted numbers after the symbol referring to a particular collation element to refer to the collation weights of that collation element at designated levels. Thus, for a collation element X, X1 refers to the primary weight, X2 refers to the secondary weight, X3 refers to the tertiary weight, and X4 refers to the quaternary weight.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CollationLevel {
    /// UTS10-D4. Primary Weight: The first collation weight in a collation element.
    ///
    /// > A primary weight is also called the Level 1 weight. Level 1 is also abbreviated as L1.
    PrimaryWeight(CollationWeight),
    /// UTS10-D5. Secondary Weight: The second collation weight in a collation element.
    ///
    /// > A secondary weight is also called the Level 2 weight. Level 2 is also abbreviated as L2.
    SecondaryWeight(CollationWeight),
    /// UTS10-D6. Tertiary Weight: The third collation weight in a collation element.
    ///
    /// > A tertiary weight is also called the Level 3 weight. Level 3 is also abbreviated as L3.
    TertiaryWeight(CollationWeight),
    /// UTS10-D7. Quaternary Weight: The fourth collation weight in a collation element.
    ///
    /// > A quaternary weight is also called the Level 4 weight. Level 4 is also abbreviated as L4.
    QuarternaryWeight(CollationWeight),
    // In principle, collation levels can extend past Level 4 to add additional levels, but the specification of the Unicode Collation Algorithm does not require defining more levels. In some special cases, such as support of Japanese collation, an implementation may need to define additional levels.
    // NWeight { i: u32, w: CollationWeight },
}

impl From<usize> for CollationLevel {
    fn from(value: usize) -> Self {
        match value {
            1 => Self::PrimaryWeight(0.into()),
            2 => Self::SecondaryWeight(0.into()),
            3 => Self::TertiaryWeight(0.into()),
            4 => Self::QuarternaryWeight(0.into()),
            _ => unimplemented!(),
            // n => Ok(Self::NWeight { i: n, w: 0.into() }),
        }
    }
}

impl CollationLevel {
    pub(crate) fn set_weight<T: Into<CollationWeight>>(&mut self, w: T) {
        match self {
            // CollationLevel::NWeight { i: _, w: weight } => *weight = w.into(),
            CollationLevel::PrimaryWeight(weight) => *weight = w.into(),
            CollationLevel::SecondaryWeight(weight) => *weight = w.into(),
            CollationLevel::TertiaryWeight(weight) => *weight = w.into(),
            CollationLevel::QuarternaryWeight(weight) => *weight = w.into(),
        };
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

/// UTS10-D8. Ignorable Weight: A collation weight whose value is zero.
///
/// >  In the 4-digit hexadecimal format used in this specification, ignorable weights are expressed as "0000".
///
/// Ignorable weights are passed over by the rules that construct sort keys from sequences of collation elements. Thus, their presence in collation elements does not impact the comparison of strings using the resulting sort keys. The judicious assignment of ignorable weights in collation elements is an important concept for the UCA.
pub trait Ignorable {
    fn is_ignorable(&self) -> bool;
}

/// UTS10-D16. Variable Collation Element: A primary collation element with a low (but non-zero) value for its primary weight.
///
/// > Low primary weights are generally reserved for punctuation and symbols, to enable special handling of those kinds of characters. Variable collation elements are subject to special rules when constructing sort keys. See Section 4, Variable Weighting. In the Default Unicode Collation Element Table [Allkeys](https://www.unicode.org/reports/tr10/#Allkeys) the primary weights of all variable collation elements are prefixed with an asterisk instead of a dot, so that they can be clearly identified.
///
/// The relationship between these terms for patterns of ignorable weights in collation elements, together with schematic examples of the corresponding collation elements, is shown in the following table, constructed on the assumption that collation elements have four collation levels. Note that quaternary collation elements have the same schematic pattern of weights as variable collation elements which have been shifted.
///
/// | Schematic Example      | Main Term                                | General Type  | Level Notation    |
/// |------------------------|------------------------------------------|---------------|-------------------|
/// | [.nnnn.nnnn.nnnn.nnnn] | Primary Collation Element                | Non-ignorable | Level 0 Ignorable |
/// | [*nnnn.nnnn.nnnn.nnnn] | Variable Collation Element (not shifted) | Non-ignorable | Level 0 Ignorable |
/// | [.0000.nnnn.nnnn.nnnn] | Secondary Collation Element              | Ignorable     | Level 1 Ignorable |
/// | [.0000.0000.nnnn.nnnn] | Tertiary Collation Element               | Ignorable     | Level 2 Ignorable |
/// | [.0000.0000.0000.nnnn] | Quaternary Collation Element             | Ignorable     | Level 3 Ignorable |
/// | [.0000.0000.0000.nnnn] | Variable Collation Element (shifted)     | Ignorable     | Level 3 Ignorable |
/// | [.0000.0000.0000.0000] | Completely Ignorable Collation Element   | Ignorable     | Level 4 Ignorable |
///
// TODO: Assert is adequate

pub struct VariableCollationElement {
    collation_element: CollationElement,
}

impl VariableCollationElement {
    pub fn collation_element(&self) -> &CollationElement {
        &self.collation_element
    }
}

/// UTS10-D17. Collation Element Mapping: A mapping from one (or more) Unicode characters to one (or more) collation elements.
///
/// Effectively, a given collation element table defines a mathematical function. It is instantiated as a list of collation element mappings. Collectively, the input for those mappings, which generally consists of all Unicode code points plus some well-defined set of short sequences of Unicode characters, constitutes the domain of the function. Collectively, the output for those mappings consists of a defined list of collation element sequences; the set of which constitutes the codomain of the function. And the collation element table itself constitutes the graph of the function.
///
/// > Note: For formal completeness, a collation element mapping is usually defined to include in its domain all Unicode code points, including noncharacters and unassigned, reserved code points. The Unicode Collation Algorithm specifies the mapping for unassigned code points, as well as some ranges of assigned characters, by means of implicit weights. See Section 10, Weight Derivation for details. However, because specific collation element tables such as the Default Unicode Collation Element Table (DUCET) generally only contain a list of collation element mappings for assigned characters, and maps those assigned characters to collation elements with explicit weights, the definitions in this section are simplified by referring to the input values just in terms of "Unicode characters".
///
/// Collation element mappings are divided into subtypes, based on a distinction between whether the input of the mapping constitutes a single Unicode character or a sequence of Unicode characters, and a separate distinction between whether the output of the mapping constitutes a single collation element or a sequence of collation elements.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CollationElementMapping {
    /// UTS10-D18. Simple Mapping: A collation element mapping from one Unicode character to one collation element.  collation_element: Vec<CollationElement>,
    SimpleMapping {
        character: char,
        collation_element: CollationElement,
    },
    /// UTS10-D19. Expansion: A collation element mapping from one Unicode character to a sequence of more than one collation element.
    Expansion {
        character: char,
        collation_elements: Vec<CollationElement>,
    },
    /// UTS10-D20. Many-to-One Mapping: A collation element mapping from more than one Unicode character to one collation element.
    ManyToOneMapping {
        characters: Vec<char>,
        collation_element: CollationElement,
    },
    /// UTS10-D21. Many-to-Many Mapping: A collation element mapping from more than one Unicode character to a sequence of more than one collation element.
    ManyToManyMapping {
        characters: Vec<char>,
        collation_elements: Vec<CollationElement>,
    },
}

impl CollationElementMapping {
    pub fn new(characters: &[char], collation_elements: &[CollationElement]) -> Result<Self> {
        let mut cem = Self::ManyToManyMapping {
            characters: characters.to_vec(),
            collation_elements: collation_elements.to_vec(),
        };
        cem.simplify()?;
        Ok(cem)
    }

    /// UTS10-D22. Contraction: Either a many-to-one mapping or a many-to-many mapping.
    pub fn is_contraction(&self) -> bool {
        match self {
            Self::ManyToOneMapping { .. } | Self::ManyToManyMapping { .. } => true,
            _ => false,
        }
    }

    /// Helper function to convert misassigned [`CollationElementMapping`]s
    fn simplify(&mut self) -> Result<()> {
        match self {
            Self::SimpleMapping { .. } => {}
            Self::Expansion {
                character,
                collation_elements,
            } => {
                if collation_elements.len() == 1 {
                    *self = Self::SimpleMapping {
                        character: *character,
                        collation_element: collation_elements[0].clone(),
                    };
                }
            }
            Self::ManyToOneMapping {
                characters,
                collation_element,
            } => {
                if characters.len() == 1 {
                    *self = Self::SimpleMapping {
                        character: characters[0],
                        collation_element: collation_element.clone(),
                    }
                }
            }
            Self::ManyToManyMapping {
                characters,
                collation_elements,
            } => {
                if characters.len() == 1 && collation_elements.len() == 1 {
                    *self = Self::SimpleMapping {
                        character: characters[0],
                        collation_element: collation_elements[0].clone(),
                    };
                } else if characters.len() == 1 && collation_elements.len() > 1 {
                    *self = Self::Expansion {
                        character: characters[0],
                        collation_elements: collation_elements.to_vec(),
                    }
                } else if characters.len() > 1 && collation_elements.len() == 1 {
                    *self = Self::ManyToOneMapping {
                        characters: characters.to_vec(),
                        collation_element: collation_elements[0].clone(),
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_1() {
        // UTS10-D1
        let a: CollationWeight = "06D9".into();
        let b: CollationWeight = "0020".into();
        let c: CollationWeight = "0002".into();
        // UTS10-D2
        let f1 = CollationElement::new(vec![a, b, c]);
        // UTS10-D3
        let f2 = CollationElement::from("[.06D9.0020.0002]");
        assert_eq!(f2, f1);

        // UTS10-D4-7
        let f1 = CollationElement::new(vec![a, a, a, a]);
        let levels = f1.levels();
        let help_vec = Vec::from([
            CollationLevel::from(1),
            CollationLevel::from(2),
            CollationLevel::from(3),
            CollationLevel::from(4),
        ]);

        let mut expected = vec![];
        for mut cl in help_vec {
            cl.set_weight(a);
            expected.push(cl.clone())
        }
        assert_eq!(levels[0], expected[0]);
        assert_eq!(levels[1], expected[1]);
        assert_eq!(levels[2], expected[2]);
        assert_eq!(levels[3], expected[3]);
    }
    #[test]
    fn test_3_2() {
        // UTS10-D8
        let a: CollationWeight = "0000".into();
        assert!(a.is_ignorable());

        let a: CollationWeight = "06D9".into();
        let b: CollationWeight = "0020".into();
        let c: CollationWeight = "0002".into();
        let d: CollationWeight = "0002".into();
        let f1 = CollationElement::new(vec![a, b, c, d]);
        // UTS10-D9-12
        assert!(f1.is_primary_collation_element());
        assert!(f1.is_secondary_collation_element());
        assert!(f1.is_tertiary_collation_element());
        assert!(f1.is_quarternary_collation_element());

        let a: CollationWeight = "0000".into();
        let f1 = CollationElement::new(vec![a, a, a, a]);
        // UTS10-D13
        assert!(f1.is_completely_ignorable_collation_element());
        // UTS10-D14
        assert!(f1.is_ignorable());
        // UTS10-D15
        assert!(!f1.is_n_ignorable(1));
        assert!(f1.is_n_ignorable(4));

        let a: CollationWeight = "0000".into();
        let b: CollationWeight = "0020".into();
        let f1 = CollationElement::new(vec![a, b]);
        // UTS10-D15
        assert!(f1.is_n_ignorable(1));
    }

    #[test]
    fn test_3_3() {
        let c = ['a'];
        let ce = [CollationElement::from("[.1C47.0020.0002]")];
        let cem = CollationElementMapping::new(&c, &ce);
        assert!(cem.is_ok());
        let cem = cem.unwrap();
        assert!(!cem.is_contraction());
        assert_eq!(
            cem,
            CollationElementMapping::SimpleMapping {
                character: 'a',
                collation_element: CollationElement::from("[.1C47.0020.002]")
            }
        )
    }
}

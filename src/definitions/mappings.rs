use crate::definitions::collation_wel::CollationElement;
use anyhow::Result;

/// > UTS10-D17. Collation Element Mapping: A mapping from one (or more) Unicode characters to one (or more) collation elements.
/// >
/// > Effectively, a given collation element table defines a mathematical function. It is instantiated as a list of collation element mappings. Collectively, the input for those mappings, which generally consists of all Unicode code points plus some well-defined set of short sequences of Unicode characters, constitutes the domain of the function. Collectively, the output for those mappings consists of a defined list of collation element sequences; the set of which constitutes the codomain of the function. And the collation element table itself constitutes the graph of the function.
/// >
/// >> Note: For formal completeness, a collation element mapping is usually defined to include in its domain all Unicode code points, including noncharacters and unassigned, reserved code points. The Unicode Collation Algorithm specifies the mapping for unassigned code points, as well as some ranges of assigned characters, by means of implicit weights. See Section 10, Weight Derivation for details. However, because specific collation element tables such as the Default Unicode Collation Element Table (DUCET) generally only contain a list of collation element mappings for assigned characters, and maps those assigned characters to collation elements with explicit weights, the definitions in this section are simplified by referring to the input values just in terms of "Unicode characters".
/// >
/// > Collation element mappings are divided into subtypes, based on a distinction between whether the input of the mapping constitutes a single Unicode character or a sequence of Unicode characters, and a separate distinction between whether the output of the mapping constitutes a single collation element or a sequence of collation elements.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CollationElementMapping {
    /// > UTS10-D18. Simple Mapping: A collation element mapping from one Unicode character to one collation element.  collation_element: Vec<CollationElement>,
    SimpleMapping {
        character: char,
        collation_element: CollationElement,
    },
    /// > UTS10-D19. Expansion: A collation element mapping from one Unicode character to a sequence of more than one collation element.
    Expansion {
        character: char,
        collation_elements: Vec<CollationElement>,
    },
    /// > UTS10-D20. Many-to-One Mapping: A collation element mapping from more than one Unicode character to one collation element.
    ManyToOneMapping {
        characters: Vec<char>,
        collation_element: CollationElement,
    },
    /// > UTS10-D21. Many-to-Many Mapping: A collation element mapping from more than one Unicode character to a sequence of more than one collation element.
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

    /// > UTS10-D22. Contraction: Either a many-to-one mapping or a many-to-many mapping.
    pub fn is_contraction(&self) -> bool {
        matches!(
            self,
            Self::ManyToOneMapping { .. } | Self::ManyToManyMapping { .. }
        )
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

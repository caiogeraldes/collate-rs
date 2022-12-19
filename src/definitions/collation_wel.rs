/// > UTS10-D1. Collation Weight: A non-negative integer used in the UCA to establish a means for systematic comparison of constructed sort keys.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CollationWeight(pub(crate) u32);

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

/// > UTS10-D2. Collation Element: An ordered list of collation weights.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CollationElement(pub(crate) Vec<CollationWeight>);

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

/// > UTS10-D3. Collation Level: The position of a collation weight in a collation element.
///
/// >> In other words, the collation level refers to the first position, second position, and so forth, in a collation element. The collation level can also be used to refer collectively to all the weights at the same relative position in a sequence of collation elements.
///
/// > Unless otherwise noted, all weights used in the example collation elements in this specification are displayed in hexadecimal format. Collation elements are shown in square brackets, with the collation weights for each level separated by dots for clarity. For example:
///```text
///     [.06D9.0020.0002]
///```
///
/// > For convenience, this specification uses subscripted numbers after the symbol referring to a particular collation element to refer to the collation weights of that collation element at designated levels. Thus, for a collation element X, X1 refers to the primary weight, X2 refers to the secondary weight, X3 refers to the tertiary weight, and X4 refers to the quaternary weight.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CollationLevel {
    /// > UTS10-D4. Primary Weight: The first collation weight in a collation element.
    /// >
    /// >> A primary weight is also called the Level 1 weight. Level 1 is also abbreviated as L1.
    PrimaryWeight(CollationWeight),
    /// > UTS10-D5. Secondary Weight: The second collation weight in a collation element.
    /// >
    /// >> A secondary weight is also called the Level 2 weight. Level 2 is also abbreviated as L2.
    SecondaryWeight(CollationWeight),
    /// > UTS10-D6. Tertiary Weight: The third collation weight in a collation element.
    /// >
    /// >> A tertiary weight is also called the Level 3 weight. Level 3 is also abbreviated as L3.
    TertiaryWeight(CollationWeight),
    /// > UTS10-D7. Quaternary Weight: The fourth collation weight in a collation element.
    ///
    /// >> A quaternary weight is also called the Level 4 weight. Level 4 is also abbreviated as L4.
    QuarternaryWeight(CollationWeight),
    // > In principle, collation levels can extend past Level 4 to add additional levels, but the specification of the Unicode Collation Algorithm does not require defining more levels. In some special cases, such as support of Japanese collation, an implementation may need to define additional levels.
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

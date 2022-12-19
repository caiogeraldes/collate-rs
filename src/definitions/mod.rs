/// Defines the Collation Weights, Elements and Levels
pub mod collation_wel;

/// Defines the Ignorable trait and related tools.
pub mod ignorable;

pub mod mappings;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_3_1() {
        use collation_wel::*;
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
        use collation_wel::*;
        use ignorable::*;
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
        use collation_wel::*;
        use mappings::*;

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

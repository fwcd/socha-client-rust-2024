macro_rules! assert_xml_parses {
    ($xml:expr, $expected:expr) => {
        {
            use ::std::{str::FromStr, fmt::Debug};

            use crate::util::Element;

            // Workaround to help type inference "see" the generic type.

            fn assert_element_parses<'a, T>(element: &'a Element, expected: T)
            where
                T: Debug + Eq + TryFrom<&'a Element>,
                T::Error: Debug
            {
                assert_eq!(
                    T::try_from(&element).unwrap(),
                    expected
                )
            }

            let element = Element::from_str($xml).unwrap();
            assert_element_parses(&element, $expected)
        }
    };
}

pub(crate) use assert_xml_parses;

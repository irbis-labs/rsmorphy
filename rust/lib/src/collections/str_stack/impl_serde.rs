use std::borrow::Cow;
use std::fmt;

use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;

use crate::collections::str_stack::StrList;

impl<'de> Deserialize<'de> for StrList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StrListVisitor;

        impl<'de> Visitor<'de> for StrListVisitor {
            type Value = StrList;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("list of strings")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<StrList, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut strings = StrList::builder();

                while let Some(s) = seq.next_element::<Cow<str>>()? {
                    strings.push(&s);
                }

                Ok(strings.build())
            }
        }

        deserializer.deserialize_seq(StrListVisitor)
    }
}

use std::borrow::Cow;
use std::fmt;

use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;

use crate::data_struct::string_list::StringArray;

impl<'de> Deserialize<'de> for StringArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringArrayVisitor;

        impl<'de> Visitor<'de> for StringArrayVisitor {
            type Value = StringArray;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("list of strings")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<StringArray, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut strings = StringArray::builder();

                while let Some(s) = seq.next_element::<Cow<str>>()? {
                    strings.push(&s);
                }

                Ok(strings.build())
            }
        }

        deserializer.deserialize_seq(StringArrayVisitor)
    }
}

pub mod data {
    use super::super::Vram;
    use serde::{
        de::{Error, Expected, SeqAccess, Visitor},
        ser::SerializeSeq,
        Deserializer, Serializer,
    };
    use std::fmt;

    pub fn serialize<S>(field: &Vec<[u8; Vram::SIZE]>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(field.len()))?;
        for array in field {
            let vec = Vec::from(array.as_slice());
            seq.serialize_element(&vec)?;
        }
        seq.end()
    }

    struct ExpectedDataBuffer;
    impl Expected for ExpectedDataBuffer {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(formatter, "a buffer with {} bytes value", Vram::SIZE)
        }
    }

    struct DataVisitor;

    impl<'de> Visitor<'de> for DataVisitor {
        type Value = Vec<[u8; Vram::SIZE]>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a data buffer")
        }

        fn visit_seq<S>(self, mut access: S) -> Result<Self::Value, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let mut seq: Vec<[u8; Vram::SIZE]> = Vec::new();
            while let Some(value) = access.next_element::<Vec<u8>>()? {
                let len = value.len();
                let array: [u8; Vram::SIZE] = value
                    .try_into()
                    .map_err(|_| Error::invalid_length(len, &ExpectedDataBuffer))?;
                seq.push(array);
            }
            Ok(seq)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<[u8; Vram::SIZE]>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(DataVisitor {})
    }
}

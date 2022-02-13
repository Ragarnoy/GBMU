pub mod data {
    use super::super::Vram;
    use serde::{
        de::{Error, Expected, SeqAccess, Visitor},
        ser::SerializeSeq,
        Deserializer, Serializer,
    };
    use std::fmt;

    type DataBuffer = Vec<[u8; Vram::SIZE]>;

    pub fn serialize<S>(field: &DataBuffer, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(field.len()))?;
        for array in field {
            let vec = array.to_vec();
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
        type Value = DataBuffer;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a data buffer")
        }

        fn visit_seq<S>(self, mut access: S) -> Result<DataBuffer, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let mut seq: DataBuffer = Vec::new();
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

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DataBuffer, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(DataVisitor {})
    }
}

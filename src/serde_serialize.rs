use {
    super::*,
    ::serde::{de, Deserialize, Deserializer, Serialize, Serializer},
};

impl<O: Identifiable> Serialize for Id<O> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.public_id())
    }
}

impl<'de, O: Identifiable> Deserialize<'de> for Id<O> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_public_id(&s).map_err(de::Error::custom)
    }
}

pub fn deserialize_raw<'de, O: Identifiable, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Id<O>, D::Error> {
    let s = String::deserialize(deserializer)?;
    Id::from_db_id(&s).map_err(de::Error::custom)
}

#[cfg(test)]
mod test {
    use crate::{Id, Ided};
    use crate::{IdClass, Identifiable};
    use rstest::rstest;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use kind_proc::Kind;

    const ID: &str = "86261271-0fc7-46d3-81c6-0b0158628331";

    #[derive(Debug, Kind, Serialize, Deserialize)]
    #[kind(class = "Test")]
    struct TestStruct {
        pub field: String,
        pub answer: i32,
    }

    #[rstest]
    pub fn test_serialize() {
        let val = Ided::new(
            Id::<TestStruct>::from_db_id(ID).unwrap(),
            TestStruct {
                field: "value".to_string(),
                answer: 42,
            },
        );

        let serialized = serde_json::to_value(&val).unwrap();

        let expected = json!({
            "id": format!("Test_{ID}"),
            "field": "value",
            "answer": 42
        });

        assert_eq!(serialized, expected);

        let deserialized: Ided<TestStruct> = serde_json::from_value(serialized).unwrap();

        assert_eq!(val, deserialized)
    }
}

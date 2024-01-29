use {
    super::*,
    ::schemars::{schema::SchemaObject, JsonSchema},
};

impl<O: Identifiable> JsonSchema for Id<O> {
    fn schema_name() -> String {
        format!("{}_uuid", O::class().prefix())
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let mut schema: SchemaObject = <String>::json_schema(gen).into();
        schema.format = Some("string".to_owned());
        schema.into()
    }
}

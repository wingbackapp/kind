use {
    super::*,
    ::schemars::{
        gen::SchemaGenerator,
        schema::{ObjectValidation, SchemaObject},
        JsonSchema,
    },
    std::borrow::Cow,
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

impl<O: Identifiable + JsonSchema> JsonSchema for Ided<O> {
    fn schema_name() -> String {
        format!("{}_ided", O::schema_name())
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Owned(format!("Ided<{}>", O::schema_id()))
    }

    fn json_schema(gen: &mut SchemaGenerator) -> schemars::schema::Schema {
        let subschema = O::json_schema(gen);
        let mut properties = subschema
            .into_object()
            .object
            .map(|obj| obj.properties)
            .unwrap_or_default();
        properties.insert("id".to_string(), Id::<O>::json_schema(gen));

        schemars::schema::Schema::Object(SchemaObject {
            instance_type: Some(schemars::schema::InstanceType::Object.into()),
            metadata: Some(Box::new(schemars::schema::Metadata {
                description: Some(format!("Identified version of {}", O::schema_name())),
                ..Default::default()
            })),
            object: Some(Box::new(ObjectValidation {
                properties,
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

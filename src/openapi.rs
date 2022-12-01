use utoipa::openapi::Schema;

pub fn openapi_schema() -> (&'static str, Schema) {
    let schema =
        utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::SchemaType::String);
    ("Id", schema.into())
}

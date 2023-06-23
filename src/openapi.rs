#[cfg(feature = "serde")]
use serde_json::json;
use utoipa::openapi::Schema;

pub fn openapi_schema() -> (&'static str, Schema) {
    let schema = utoipa::openapi::ObjectBuilder::new()
        .schema_type(utoipa::openapi::SchemaType::String)
        .description(Some(
            "Unique identifier of an object. Consists of object class prefix and a UUID",
        ));
    #[cfg(feature = "serde")]
    let schema = schema.example(Some(json!("Cust_c40bea18-c0c9-44b1-bd0c-43f5283e1670")));
    ("Id", schema.into())
}

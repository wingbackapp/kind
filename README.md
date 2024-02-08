
## Why kind ?

With **kind**, you

- use typed identifiers in Rust, with no overhead over Uuid
- have the type be human readable and obvious in JSON and any export
- still use uuid in your database (if enabling the sqlx feature)
- don't have to add code for that, never explicitly stringify, parse, check types, etc.
- have no boilerplate to declare types and identifiers
- have your ids implement Copy, Debug, Display, FromStr, Serialize, Deserialize, Eq, Hash, etc.
- can safely deal with both identified objects and new ones of same kind


## Optional features

* serde: `Serialize` and `Deserialize` implementations for `Id`, `Ided`, and the `id_enum!` enums
* sqlx: transparent read/write for `Id` (with `uuid` columns) and for `Ided` (with tables having an uuid identifier)
* jsonschema: JSON schema generation
* openapi: openapi ID object type for `Id`

In the current version, the sqlx feature is only complete for postgresql.

## Declare a kind of object

You could implement the `Identifiable` trait, but the easiest solution is to just add attributes to your structs:

```rust
use kind::*;

#[derive(Kind)]
#[kind(class="Cust")]
pub struct Customer {
    // many fields
}

#[derive(Kind)]
#[kind(class="Cont")]
pub struct Contract {
    // many fields
}
```

## Id

A `kind::Id` is strongly typed to avoid misuse of Rust APIs, especially when functions ask for several ids of different types.

The `kind::Id` also prevents the misuse of any string based API, such as Rest or GraphQL, by prefixing the internally used ids with a class prefix.

It's costless: the kind is handled by the type system and doesn't clutter the compiled binary

```rust
assert_eq!(
    std::mem::size_of::<Id<Customer>>(),
    std::mem::size_of::<uuid::Uuid>(),
);
```

You can parse the id from eg JSON, or just a string
```rust
let id: Id<Customer> = "Cust_371c35ec-34d9-4315-ab31-7ea8889a419a"
    .parse().unwrap();
```

The type is checked, so this customer id can't be misused as a contract id
```rust
assert!(
    "Cust_371c35ec-34d9-4315-ab31-7ea8889a419a"
    .parse::<Id<Contract>>()
    .is_err()
);
```

Note: the public id is parsed and checked in a case insensitive way
```rust
assert_eq!(id, "cust_371c35ec-34d9-4315-ab31-7ea8889a419a".parse());
assert_eq!(id, "CUST_371C35EC-34D9-4315-AB31-7EA8889A419A".parse());
```

## Ided

`Ided` is short for "identified".

Sometimes, you have to deal with raw objects without id, because that's what you receive from your REST api for creation, or because you give it an id only when inserting the row in database.

That's why our raw `Customer` type has no id.
Most API don't deal with just the raw `Customer` type but with an `Ided<Customer>`, which is guaranteed to have an id.

An ided can be created from an id and an "entity":

```rust
let new_customer = Customer { name: "John".to_string() };
let customer = Ided::new(id, new_customer);
assert_eq!(
    customer.id().to_string(),
    "Cust_371c35ec-34d9-4315-ab31-7ea8889a419a"
);
assert_eq!(customer.entity().name, "John");
```

An ided automatically derefs into the entity type, so this is valid too:

```rust
assert_eq!(customer.name, "John");
```

## Serde

An `Ided` object is serialized with the id next to the other fields, without unnecessary nesting.

```rust
#[derive(Kind, serde::Serialize, serde::Deserialize)]
#[kind(class="Cust")]
pub struct Customer {
    pub name: String,
}

let json = r#"{
    "id": "Cust_371c35ec-34d9-4315-ab31-7ea8889a419a",
    "name": "John"
}"#;

let customer: Ided<Customer> = serde_json::from_str(&json).unwrap();
assert_eq!(
    customer.id().to_string(),
    "Cust_371c35ec-34d9-4315-ab31-7ea8889a419a"
);
assert_eq!(customer.name, "John");
```

The id kind is checked, the deserialization below fails because the prefix of the id is wrong:
```rust
let json = r#"{
    "id": "Con_371c35ec-34d9-4315-ab31-7ea8889a419a",
    "name": "John"
}"#;
assert!(serde_json::from_str::<Ided<Customer>>(&json).is_err());
```

## sqlx/PostgreSQL

In database, the id is just an `uuid`. The kind of the id in the database is implicitly given by the query and your DB structure, there's no additional check on reading/writing from rust to the DB and you don't have to change the DB structure when starting to use Kind.

The `Id` type implements `Encode` and `Decode`, so that it can be used transparently in sqlx queries just like any other primary type.

As for serde, FromRow implementation on Ided is automatically deduced from the implementation on the raw struct.

So you will usually just declare your struct like this to have the `Ided` loaded from an `sqlx::Row` containing both the `id` column and the ones of the raw struct:

```rust
#[derive(Kind, sqlx::Row)]
#[kind(class="Cust")]
pub struct Customer {
    pub name: String,
}
```
## JSON schema

If you are generating JSON schema for your objects using [schemars crate](https://crates.io/crates/schemars), you can enable `jsonschema` feature, and we will generate definition for the `Id` object and any `Ided` object:

```rust
#[derive(JsonSchema, Kind)]
#[kind(class="Cust")]
pub struct Customer {
    pub name: String
}

fn main() {
    let schema = schema_for!(Ided<Customer>);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
```

will print out

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Customer_ided",
  "description": "Identified version of Customer",
  "type": "object",
  "properties": {
    "id": {
      "type": "string",
      "format": "string"
    },
    "name": {
      "type": "string"
    }
  }
}
```

## Open API

Open APi support (gated behind `openapi` feature flag) is currently extremely rudimentary. So far the only supported feature is defining the schema-level `Id` object that can be referenced by other schemas.

Example for including the `Id` into generated schema:

```rust
pub struct ApiDoc;
impl utoipa::OpenApi for ApiDoc {
    fn openapi() -> utoipa::openapi::OpenApi {
        let mut components = utoipa::openapi::ComponentsBuilder::new();
        let (kind_name, kind_schema) = kind::openapi_schema();
        components = components.schema(kind_name, kind_schema);
        //extra components and paths
        let mut openapi = utoipa::openapi::OpenApiBuilder::new()
            .components(Some(components.build()))
            .build();
        openapi
    }
}
```



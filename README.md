
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
* jsonschema: openapi schema generation

In the current version, the sqlx feature is only complete for postgresql.

## Usage overview

A `kind::Id` is strongly typed to avoid misuse of Rust APIs, especially
when functions ask for several ids of different types.

The `kind::Id` also prevents the misuse of any string based API, such
as Rest or GraphQL, by prefixing the internally used ids with a class
prefix.

```
use kind::*;

// The structs we want to define Id types for are just annotated. The
// Identifiable trait is derived.

#[derive(Debug, Kind)]
#[kind(class="Cust")]
pub struct Customer {
    // many fields
}

#[derive(Debug, Kind)]
#[kind(class="Cont")]
pub struct Contract {
    // many fields
}

// Let's start from an id in the database (we use the string representantion
// but kind natively decodes from postgres' Uuid into Id)
let customer_db_id = "371c35ec-34d9-4315-ab31-7ea8889a419a";

// Now, use it to get our Rust instance of Id:
let customer_id: Id<Customer> = Id::from_db_id(customer_db_id).unwrap();

// If we communicate (via serde, Display, or directly), we
// use the public id
let customer_public_id = customer_id.public_id();
assert_eq!(&customer_public_id, "Cust_371c35ec-34d9-4315-ab31-7ea8889a419a");

// When reading an id withtout prefix, from the db, there was
// no type check. It's (almost) OK because we carefully wrote our
// queries. But we need a type check when we read from a public id.
// Let's try to read our public id as a contract id:
let contract_id: Result<Id<Contract>, IdError> = customer_public_id.parse();
assert!(contract_id.is_err());

// And let's check it's OK as a customer id:
let customer_id: Result<Id<Customer>, IdError> = Id::from_public_id(&customer_public_id);
assert!(customer_id.is_ok());
assert_eq!(customer_id.unwrap().db_id(), "371c35ec-34d9-4315-ab31-7ea8889a419a");

// The public id is parsed and checked in a case insensitive way
assert_eq!(customer_id, "cust_371c35ec-34d9-4315-ab31-7ea8889a419a".parse());
assert_eq!(customer_id, "CUST_371C35EC-34D9-4315-AB31-7EA8889A419A".parse());

```





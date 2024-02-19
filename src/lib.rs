//! A `kind::Id` is strongly typed to avoid misuse of Rust APIs, especially
//! when functions ask for several ids of different types.
//!
//! The `kind::Id` also prevents the misuse of any string based API, such
//! as Rest or GraphQL, by prefixing the internally used ids with a class
//! prefix.
//!
//! ```
//! use kind::*;
//!
//! // The structs we want to define Id types for are just annotated. The
//! // Identifiable trait is derived.
//!
//! #[derive(Debug, Identifiable)]
//! #[kind(class="Cust")]
//! pub struct Customer {
//!     // many fields
//! }
//!
//! #[derive(Debug, Identifiable)]
//! #[kind(class="Cont")]
//! pub struct Contract {
//!     // many fields
//! }
//!
//! // Let's start from an id in the database (we use the string representantion
//! // but kind natively decodes from postgres' Uuid into Id)
//! let customer_db_id = "371c35ec-34d9-4315-ab31-7ea8889a419a";
//!
//! // Now, use it to get our Rust instance of Id:
//! let customer_id: Id<Customer> = Id::from_db_id(customer_db_id).unwrap();
//!
//! // If we communicate (via serde, Display, or directly), we
//! // use the public id
//! let customer_public_id = customer_id.public_id();
//! assert_eq!(&customer_public_id, "Cust_371c35ec-34d9-4315-ab31-7ea8889a419a");
//!
//! // When reading an id withtout prefix, from the db, there was
//! // no type check. It's (almost) OK because we carefully wrote our
//! // queries. But we need a type check when we read from a public id.
//! // Let's try to read our public id as a contract id:
//! let contract_id: Result<Id<Contract>, IdError> = customer_public_id.parse();
//! assert!(contract_id.is_err());
//!
//! // And let's check it's OK as a customer id:
//! let customer_id: Result<Id<Customer>, IdError> = Id::from_public_id(&customer_public_id);
//! assert!(customer_id.is_ok());
//! assert_eq!(customer_id.unwrap().db_id(), "371c35ec-34d9-4315-ab31-7ea8889a419a");
//!
//! // The public id is parsed and checked in a case insensitive way
//! assert_eq!(customer_id, "cust_371c35ec-34d9-4315-ab31-7ea8889a419a".parse());
//! assert_eq!(customer_id, "CUST_371C35EC-34D9-4315-AB31-7EA8889A419A".parse());
//!
//! ```

mod error;
mod id;
mod id_class;
mod ided;
mod identifiable;

#[cfg(feature = "sqlx")]
mod postgres;

#[cfg(feature = "serde")]
mod id_enum;
#[cfg(feature = "jsonschema")]
mod jsonschema;
#[cfg(feature = "openapi")]
mod openapi;
#[cfg(feature = "serde")]
mod serde_serialize;

#[allow(unused_imports)]
pub use {error::*, id::*, id_class::*, ided::*, identifiable::*, kind_proc::*};

#[allow(unused_imports)]
#[cfg(feature = "serde")]
pub use {crate::serde_serialize::*, id_enum::*};

#[allow(unused_imports)]
#[cfg(feature = "jsonschema")]
pub use crate::jsonschema::*;

#[allow(unused_imports)]
#[cfg(feature = "openapi")]
pub use crate::openapi::*;

#[test]
fn test_id_ided() {
    #[derive(Debug, Identifiable)]
    #[kind(class = "Cust")]
    pub struct Customer {
        pub name: String,
    }

    #[derive(Debug, Identifiable)]
    #[kind(class = "Cont")]
    pub struct Contract {
        // many fields
    }

    // It's costless: the kind is handled by the type system
    // and doesn't clutter the compiled binary
    assert_eq!(
        std::mem::size_of::<Id<Customer>>(),
        std::mem::size_of::<uuid::Uuid>(),
    );

    // You can parse the id from eg JSON, or just a string
    let id: Id<Customer> = "Cust_371c35ec-34d9-4315-ab31-7ea8889a419a".parse().unwrap();

    // The type is checked, so this id can't be misused as a contract id
    assert!("Cust_371c35ec-34d9-4315-ab31-7ea8889a419a"
        .parse::<Id<Contract>>()
        .is_err());

    // The public id is parsed and checked in a case insensitive way
    assert_eq!(
        id,
        "cust_371c35ec-34d9-4315-ab31-7ea8889a419a".parse().unwrap()
    );
    assert_eq!(
        id,
        "CUST_371C35EC-34D9-4315-AB31-7EA8889A419A".parse().unwrap()
    );

    // You can build an identified object from just
    // Here's a new customer:
    let new_customer = Customer {
        name: "John".to_string(),
    };
    // Give it an id, by wrapping it in an Ided
    let customer = Ided::new(id, new_customer);

    assert_eq!(customer.name, "John");
    assert_eq!(
        customer.id().to_string(),
        "Cust_371c35ec-34d9-4315-ab31-7ea8889a419a"
    );
}

#[cfg(feature = "serde")]
#[test]
fn test_serde() {
    // deserialize a customer
    #[derive(Debug, Identifiable, serde::Serialize, serde::Deserialize)]
    #[kind(class = "Cust")]
    pub struct Customer {
        pub name: String,
    }

    let json = r#"{
        "id": "Cust_371c35ec-34d9-4315-ab31-7ea8889a419a",
        "name": "John"
    }"#;

    let customer: Ided<Customer> = serde_json::from_str(json).unwrap();
    assert_eq!(customer.entity().name, "John");
    assert_eq!(
        customer.id().to_string(),
        "Cust_371c35ec-34d9-4315-ab31-7ea8889a419a"
    );

    // id kind is checked: this one fails because the prefix of the
    // id is wrong
    let json = r#"{
        "id": "Con_371c35ec-34d9-4315-ab31-7ea8889a419a",
        "name": "John"
    }"#;
    assert!(serde_json::from_str::<Ided<Customer>>(json).is_err());

    assert_eq!(
        serde_json::to_string(&customer).unwrap(),
        r#"{"id":"Cust_371c35ec-34d9-4315-ab31-7ea8889a419a","name":"John"}"#,
    );
}

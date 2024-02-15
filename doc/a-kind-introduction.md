
# A kind introduction

How to leverage the Rust compiler to prevent identifier misuse ?

How to go from `ec2ba151-7acf-43a9-bb98-6f5331992f42` in your database to `"Cust_ec2ba151-7acf-43a9-bb98-6f5331992f42"` in your REST/JSON API ?

How to do it at zero runtime cost and with no boilerplate, with a derive macro ?

See how we did it and how you can too with the library we're making public.

---------------------

[Wingback](https://wingback.com) is a B2B SaaS app that helps B2B SaaS companies price, sell and bill their services. It uses a classical architecture : front-end and other API users communicate in REST with a Rust backend.

We have many tables in our PostgreSQL database. All are mapped to Rust structs which are then, more or less directly, exchanged in JSON through our REST API and described in OpenAPI schemas.

In the first few months of our startup, our Rust objects had Uuid fields everywhere: objects were having ids, and were referring to other objects by their ids. And functions were taking several Uuid as arguments.

This was obviously a dangerously loose typing, and made us miss a lot of the Rust safety.

And even if we could have managed this with conventions in our Rust codebase, our API would have still been unclear and examples unhelpful.

We wanted to

* use typed identifiers in Rust, with no overhead over Uuid (be "zero cost")
* still use uuid in postgresql without having to convert between types
* have the type be obvious and human-readable in JSON and any export
* not have to add code for that, never explicitly stringify, parse, check types, etc.
* no boilerplate to declare types and identifiers
* be able to safely deal with both identified objects and new ones of the same type
* have our ids implement `Copy`, `Debug`, `Display`, `FromStr`, `Serialize`, `Deserialize`, `Eq`, `Hash`, etc.

We made a library to solve this problem, and we're now making it public: **[kind](https://github.com/wingbackapp/kind/)**.


## What it looks like

In database, an id is just your standard `UUID`, and is initialized db-side (we use `gen_random_uuid()` but you can use any function or version of UUID).

It looks like `ec2ba151-7acf-43a9-bb98-6f5331992f42`.

In JSON, URLs, documentations, exports, the identifier is always prefixed.

For example `"Cust_ec2ba151-7acf-43a9-bb98-6f5331992f42"` for a customer.

This makes all examples in documentation and OpenAPI interfaces obvious. This makes exports self documented. And self-checked.

In Rust, the mandatory type prevents any confusion:

```
fn get_contracts(
    customer_id: Id<Customer>,
    plan_id: Id<Plan>,
)
```

You can't pass the wrong id: the compiler will tell you what type is expected.

## What it really is, how it works

We declare a class for each kind of object.

This is done with a derive attribute. To say that a `Customer` has an id prefixed with `Cust_`, we write this:

```
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Identifiable)]
#[kind(class = "Cust")]
pub struct Customer {
    pub name: String,
}
```

With just this declaration, you can use the `Id<Customer>` type:

```
let id: Id<Customer> = "Cust_371c35ec-34d9-4315-ab31-7ea8889a419a"
    .parse()?; // fail if the prefix isn't right
```


What the procedural macro really does is it makes `Customer` implement a special trait, `Identifiable`, which allows it to parameterize an id.

Here's the generic `Id` type:

```
pub struct Id<O: Identifiable> {
    uuid: Uuid,
    phantom: PhantomData<O>,
}
```

The `PhantomData<O>` ensures that `Id<A>` and `Id<B>` are different and incompatible types.

As soon as the compiler has checked your code and prevented any misuse, its strips the `phantom`.
This id is exactly like a single `UUID`, once compiled. This is a zero-cost safety abstraction.

The `Identifiable` trait is

```
pub trait Identifiable {
    fn class() -> IdClass;
}
```

You never see this trait as Kind library user. It's used by Kind to ensure that two ids aren't mixed in Rust , and provides the "Cust" prefix which is appended on serialization and checked on deserialization.

You could easily implement it yourself, `IdClass` just wraps a `&'static str` (eg `"Cust"`), but it's much simpler to use the derive macro.

So the first part of the magic is combining the `PhantomData` of Rust which enables a zero-cost sub typing and a trait returning the kind of id.

And the rest of the Kind magic is made of the derive macro and a bunch of generic implementations for `Id<O>` and `Ided<O>`.

## Identified object

You may have noticed the `Customer` struct has no id field.

This is very important: we split the id part so that we can both have objects with id (most of the time) and objects without id (e.g. before receiving one).

Of course, they’re of different types.

An object with an id is of type `Ided<T>`.

For example, loading a customer could be

```
pub async fn get_customer(id: Id<Customer>) -> Result<Ided<Customer>> {
```

and storing a just created one would be

```
pub async fn store_customer(customer: Customer) -> Result<Ided<Customer>> {
```

The `Ided<Customer>` has functions returning the `&Id<Customer>` and the `&Customer`, but it also dereferences into a `Customer` so you can directly address its fields and functions.

Just like the id, the identified type can be used in functions and structs, for example

```
pub struct SomeThing {
    pub id: Id<Customer>, // we refer to the customer here
    pub plan: Ided<Plan>, // the whole plan is wrapped here
}
```

## Parse/Write, (de)serialize

Parsing an id is as simple as

```
let id: Id<Plan> = "Cust_ec2ba151-7acf-43a9-bb98-6f5331992f42".parse()?;
```

(yes, this one will throw an error, it's not a plan)

Deserialize an id or a struct containing ids is as simple with serde as the Id type.

An `Ided` type is automatically assumed to have an id field on (de)serialization.

So the JSON for an identified customer according to the struct above would be similar to

```
{
    "id": "Cust_ec2ba151-7acf-43a9-bb98-6f5331992f42",
    "name": "Alfred Einstein"
}
```

You can't deserialize an id whose type doesn't match, it would be a deserialization error.

## Query the database


We use [sqlx](https://github.com/launchbadge/sqlx), so we've added some impl to make reading/writing our identifier and identifiable objects transparent and efficient, just as for serialization.

You may have missed the tiny `sqlx::FromRow` in the Customer definition above, so here it is again:

```
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Identifiable)]
#[kind(class = "Cust")]
pub struct Customer {
    pub name: String,
}
```

This is enough to make `Ided<Customer>` implement `FromRow` too: the row just has to contain an id column.

If you use another DB layer, it’s probably easy to add the relevant extension.

## And more

The crate comes with more features.

This introduction is too short to detail them all, the documentation is better suited, but here's for a taste of it.

When you use this kind of struct in the Rust ecosystem, you usually need to implement a few standard traits, like `PartialEq` and `Eq`, `Hash`, `PartialOrd` and `Ord`, `Debug`. Kind comes with those implementations.

We also support the cases when you want the user to pass an id, but it could be of any kind among a selection. For example your REST API lets you link two objects, but they may be customers, plans, payments, invoices, etc.

Kind made it possible at wingback, and for our API users, to have safe and clear identifiers.

We think it could help the Rust communities, as it’s quite a frequent concern. In case there are shortcomings for your use case (maybe a different database?), we probably can extend it together.

At the worst, the core idea and this specific combination of PhantomData, an exported class, and a derive macro, could be applied in a different library.

* [Wingback's blog post about Kind](https://www.wingback.com/blog/kind-library-that-provides-zero-cost-type-safe-identifiers)
* [Kind's README](https://github.com/wingbackapp/kind)

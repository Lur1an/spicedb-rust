# spicedb-rust

An opinionated client for SpiceDB, built on top of the official gRPC API without the suck of using gRPC in rust.

## Disclaimer
API not stable yet, breaking changes are possible in the upcoming days.
(Probably moving all API's to the top level struct)


## API
The API offers builder interfaces for all gRPC requests that leverage the generic trait type system to 
cut down on some request building boilerplate and potential errors/typos.

Some of the most common requests are directly exposed as functions on the `Client` struct like `lookup_resources` directly into a `Vec<R::Id>` or a `check_permission` directly to a `bool`.

As an alternative to builder interfaces the client also exposes/will expose the methods directly with all arguments at once as parameters, if you intend to use the upcoming `mock` feature to not have to run a local `SpiceDB` instance in your tests it is recommended you do it this way.

Regarding `impl Trait` parameters on the `SpiceDBClient`, those have all been removed with either generics or just `String`, `&str`. This little QoL loss allows `mockall` to build a `MockSpiceDBClient`.

## Type System
The type system of this crate allows definition of rust structs with traits that mirror the schema imported into SpiceDB. This cuts down on potential typos and other bugs that can crawl into development when typing raw strings for relationships & permissions and makes it easier to build the quite complex gRPC requests with some compile-time checks.

### Pros
- Never make an error due to passing in the wrong `String` value for a relationship or permission
- Never try to create a relationship that does not exist for one of your entities
- Easy way to share your permission schema across multiple services by centralizing your authz schema in a rust library crate
- Type conversions from `String` into your chosen type of id like `u32` or `Uuid` are done automatically

### Cons
- A bit of Boilerplate

### I don't like the type system, can I just use the raw gRPC API?
Yes you can. `SpiceDBClient` exposes methods to get the underlying `tonic` client, and the `spicedb` module exports all protobuf types. From personal experience, this is not fun.

### Example
Lets take the following SpiceDB schema:
```zed
definition user {}

definition document {
    relation reader: user | user:*
    relation writer: user
    
    permission read = reader + writer
    permission write = writer
}
```
We have 2 entities, `User` and `Document`. This is the boilerplate:

```rust
struct User;

impl Entity for User {
    type Relations = NoRelations;
    type Id = Uuid;

    fn object_type() -> &'static str {
        "user"
    }
}

struct MyActor(Uuid);

impl Actor for MyActor {
    fn to_subject(&self) -> SubjectReference {
        subject_reference_raw(self.0, User::object_type(), None::<String>)
    }
}

struct Document;

pub enum DocumentPermission {
    Read,
    Write,
}

impl Permission for DocumentPermission {
    fn name(&self) -> &'static str {
        match self {
            DocumentPermission::Read => "read",
            DocumentPermission::Write => "write",
        }
    }
}

impl Entity for Document {
    type Relations = DocumentRelation;
    type Id = String;

    fn object_type() -> &'static str {
        "document"
    }
}

pub enum DocumentRelation {
    Reader,
    Writer,
}

impl Relation for DocumentRelation {
    fn name(&self) -> &'static str {
        match self {
            DocumentRelation::Reader => "reader",
            DocumentRelation::Writer => "writer",
        }
    }
}

impl Resource for Document {
    type Permissions = DocumentPermission;
}

```
> **_NOTE:_** Will be working on some `strum` integration to cut down on the quite obvious boilerplate that turns the enum variants into strings.

This type system now makes it impossible to check for a permission that doesn't exist, or create a relationship not supported for an entity. 

> **_NOTE:_** I don't know if its possible to constrain the *Subject* part of a relationship through types, I haven't gone down this road, and I don't feel like I need to yet, I'd be curious if there is an easy way to do this.

Now lets create some relationship and check some permissions!
```rust
let client = SpiceDBClient::new("http://localhost:50051".to_owned(), "randomkey")
    .await?;
let user_id = Uuid::now_v7();
let relationships = [
    relationship_update::<User, Document>(
        RelationshipOperation::Touch,
        user_id,
        None,
        "homework",
        DocumentRelation::Writer,
    ),
    wildcard_relationship_update::<User, Document>(
        RelationshipOperation::Touch,
        "manga",
        DocumentRelation::Reader,
    ),
];
let token = client
    .create_relationships(relationships, [])
    .await
    .unwrap();
let actor = MyActor(user_id);
let authorized = client.check_permission_at::<Document>(
    &actor,
    "homework",
    DocumentPermission::Write,
    token.clone(),
)
.await?;
```

## Mocking
Enable the `mock` feature to instead use a client generated by `mockall`.
All public interfaces will work as expected, but not the request builder interfaces, as those are not mocked, if you are using those currently you have to test against a real instance of SpiceDB.

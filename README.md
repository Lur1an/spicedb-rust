# spicedb-rust

An opinionated client for SpiceDB, built on top of the official gRPC API without the suck of using gRPC in rust.

## Type System
The type system of this crate allows definition of rust structs with traits that mirror the schema imported into SpiceDB. This cuts down on potential typos and other bugs that can crawl into development when typing raw strings for relationships & permissions.
Macros to cut down the boilerplate will be added in the future, probably last after the entire API has been wrapped.

## Example
```rust
use spicedb_rust::{
    Entity, NoRelations, Permission, Relation, RelationshipOperation, Resource, SpiceDBClient,
    Subject,
};

pub struct User;

impl Entity for User {
    type Relations = NoRelations;
    type Id = String;

    fn object_type() -> &'static str {
        "user"
    }
}

impl Subject for User {}

pub struct Document;

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

async fn example() {
    let client = SpiceDBClient::new("localhost:50051", "randomkey")
        .await
        .unwrap();
    let mut request = client.permission_client().create_relationships();
    request.add_relationship::<User, Document>(
        RelationshipOperation::Create,
        "jeff".to_owned(),
        None,
        "homework".to_owned(),
        DocumentRelation::Writer,
    );
    request.send().await.unwrap();
}
```

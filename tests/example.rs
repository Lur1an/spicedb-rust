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

#[tokio::test]
async fn write_relationships() {
    let client = SpiceDBClient::new("http://localhost:50051", "randomkey")
        .await
        .unwrap();
    let schema = include_str!("schema.zed");
    client.schema_client().write_schema(schema).await.unwrap();
    let mut request = client.permission_client().create_relationships();
    request.add_relationship::<User, Document>(
        RelationshipOperation::Create,
        "jeff".to_owned(),
        None,
        "homework".to_owned(),
        DocumentRelation::Writer,
    );
}

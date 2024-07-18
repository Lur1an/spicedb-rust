use spicedb_rust::spicedb::{subject_reference_raw, SubjectReference};
use spicedb_rust::{
    Actor, Entity, NoRelations, Permission, Relation, RelationshipOperation, Resource,
    SpiceDBClient, Subject,
};

pub struct User(String);

impl User {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl Entity for User {
    type Relations = NoRelations;
    type Id = String;

    fn object_type() -> &'static str {
        "user"
    }
}

impl Subject for User {}

impl Actor for User {
    fn to_subject(&self) -> SubjectReference {
        subject_reference_raw(self.0.clone(), User::object_type(), None::<String>)
    }
}

pub struct Document;

#[derive(strum::EnumString)]
pub enum DocumentPermission {
    #[strum(serialize = "read")]
    Read,
    #[strum(serialize = "write")]
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

#[derive(strum::EnumString)]
pub enum DocumentRelation {
    #[strum(serialize = "reader")]
    Reader,
    #[strum(serialize = "writer")]
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

    let user = User::new("jeff");
    let document_id = "homework".to_owned();

    let mut request = client.permission_client().create_relationships_request();
    request.add_relationship::<User, Document>(
        RelationshipOperation::Touch,
        "jeff".to_owned(),
        None,
        document_id.clone(),
        DocumentRelation::Writer,
    );
    request.send().await.unwrap();
    let authorized = client
        .permission_client()
        .check_permission::<Document>(&user, document_id.clone(), DocumentPermission::Write)
        .await
        .unwrap();
    assert!(
        authorized,
        "User should be authorized to write the document"
    );
}

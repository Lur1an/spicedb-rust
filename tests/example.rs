use pretty_assertions::assert_eq;
use spicedb_rust::spicedb::{
    relationship_update, subject_reference_raw, wildcard_relationship_update, SubjectReference,
};
use spicedb_rust::{Actor, Entity, NoRelations, RelationshipOperation, Resource, SpiceDBClient};
use strum::IntoStaticStr;
use uuid::Uuid;

struct User(Uuid);

impl User {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}

impl Entity for User {
    type Relations = NoRelations;
    type Id = Uuid;

    fn object_type() -> &'static str {
        "user"
    }
}

impl Actor for User {
    fn to_subject(&self) -> SubjectReference {
        subject_reference_raw(self.0, User::object_type(), None::<String>)
    }
}

struct Document;

#[derive(IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum DocumentPermission {
    Read,
    Write,
}

impl Entity for Document {
    type Relations = DocumentRelation;
    type Id = String;

    fn object_type() -> &'static str {
        "document"
    }
}

#[derive(IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum DocumentRelation {
    Reader,
    Writer,
}

impl Resource for Document {
    type Permissions = DocumentPermission;
}

#[tokio::test]
async fn example() {
    let client = SpiceDBClient::new("http://localhost:50051".to_owned(), "randomkey")
        .await
        .unwrap();
    let schema = include_str!("schema.zed");
    client.write_schema(schema.to_owned()).await.unwrap();

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

    let actor = User::new(user_id);
    let authorized = client
        .check_permission_at::<_, Document>(
            &actor,
            "homework".to_owned(),
            DocumentPermission::Write,
            token.clone(),
        )
        .await
        .unwrap();
    assert!(
        authorized,
        "User should be authorized to write the document"
    );

    let random_user_actor = User::new(Uuid::now_v7());
    let authorized = client
        .check_permission_at::<_, Document>(
            &random_user_actor,
            "manga".to_owned(),
            DocumentPermission::Read,
            token.clone(),
        )
        .await
        .unwrap();
    assert!(
        authorized,
        "Random user should be authorized to read `manga` due to wildcard"
    );

    let mut resource_ids = client
        .lookup_resources_at::<_, Document>(&actor, DocumentPermission::Read, token.clone())
        .await
        .unwrap();
    let mut expected = vec!["homework", "manga"];
    resource_ids.sort();
    expected.sort();
    assert_eq!(
        resource_ids, expected,
        "Homework and Manga should both appear in documents User can read"
    );

    let subject_ids = client
        .lookup_subjects_at::<User, Document>(
            "homework".to_owned(),
            DocumentPermission::Write,
            token.clone(),
        )
        .await
        .unwrap();
    assert_eq!(subject_ids, vec![user_id]);

    let new_token = client
        .delete_relationships::<Document>(None, None, None)
        .await
        .unwrap();

    let authorized = client
        .check_permission_at::<_, Document>(
            &random_user_actor,
            "manga".to_owned(),
            DocumentPermission::Read,
            new_token,
        )
        .await
        .unwrap();
    assert!(
        !authorized,
        "Nobody should be authorized to read `manga` due to nuking all relationships"
    );
}

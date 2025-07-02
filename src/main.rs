#![doc = include_str!("../README.md")]

use entities::*;
pub use sea_orm;
use sea_orm::{
    ActiveModelTrait, ActiveValue::*, ConnectionTrait, Database, DbBackend, DbConn, DbErr,
    EntityTrait, InsertResult, ModelTrait, Schema,
};

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    // Set connection
    let connection: DbConn = Database::connect("sqlite::memory:").await?;

    // Set table
    let schema = Schema::new(DbBackend::Sqlite);

    //
    //
    //
    //
    //

    // Derive from Entity
    let stmt: sea_orm::sea_query::TableCreateStatement =
        schema.create_table_from_entity(Definitions);

    // Execute create table statement
    connection
        .execute(connection.get_database_backend().build(&stmt))
        .await?;

    // Derive from Entity
    let stmt: sea_orm::sea_query::TableCreateStatement =
        schema.create_table_from_entity(BaseElement);

    // Execute create table statement
    connection
        .execute(connection.get_database_backend().build(&stmt))
        .await?;

    //
    //
    //
    //
    //

    let base_element_user1 = BaseElementModel {
        bpmn_id: Set(String::from("100")),
        ..Default::default()
    };
    let res: InsertResult<BaseElementModel> = BaseElement::insert(base_element_user1)
        .exec(&connection)
        .await?;

    let user1 = DefinitionsModel {
        name: Set(String::from("1")),
        super_base_element: Set(res.last_insert_id),
        target_namespace: Set(String::from(" ")),
        exporter: Set(String::from(" ")),
        exporter_version: Set(String::from(" ")),
        ..Default::default()
    };
    user1.insert(&connection).await?;

    let base_element_user2 = BaseElementModel {
        bpmn_id: Set(String::from("200")),
        ..Default::default()
    };
    let res: InsertResult<BaseElementModel> = BaseElement::insert(base_element_user2)
        .exec(&connection)
        .await?;

    let user2 = DefinitionsModel {
        name: Set(String::from("10")),
        super_base_element: Set(res.last_insert_id),
        target_namespace: Set(String::from(" ")),
        exporter: Set(String::from(" ")),
        exporter_version: Set(String::from(" ")),
        ..Default::default()
    };

    user2.insert(&connection).await?;

    let all_users = Definitions::find().all(&connection).await.unwrap();
    println!("all Definitions : {:?}", all_users);
    println!();

    let u1 = Definitions::find_by_id(1)
        .one(&connection)
        .await
        .unwrap()
        .unwrap();
    println!("u1: {:?}", u1);
    println!();

    u1.delete(&connection).await.unwrap();

    let mut u2: DefinitionsModel = Definitions::find_by_id(2)
        .one(&connection)
        .await
        .unwrap()
        .unwrap()
        .into();

    u2.name = Set(String::from("100"));
    u2.update(&connection).await.unwrap();

    let all_users = Definitions::find().all(&connection).await.unwrap();
    println!("all Definitions : {:?}", all_users);
    println!();
    Ok(())
}

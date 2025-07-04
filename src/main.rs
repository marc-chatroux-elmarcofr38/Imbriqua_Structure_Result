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

    let base_element_def1 = BaseElementModel {
        bpmn_id: Set(String::from("100")),
        ..Default::default()
    };
    let res: InsertResult<BaseElementModel> = BaseElement::insert(base_element_def1)
        .exec(&connection)
        .await?;

    let def1 = DefinitionsModel {
        name: Set(String::from("1")),
        super_base_element: Set(res.last_insert_id),
        target_namespace: Set(String::from(" ")),
        exporter: Set(String::from(" ")),
        exporter_version: Set(String::from(" ")),
        ..Default::default()
    };

    println!(
        "Get BaseElement from Definition : {:?}",
        &def1.super_base_element.TO()
    );

    def1.insert(&connection).await?;

    let base_element_def2 = BaseElementModel {
        bpmn_id: Set(String::from("200")),
        ..Default::default()
    };
    let res: InsertResult<BaseElementModel> = BaseElement::insert(base_element_def2)
        .exec(&connection)
        .await?;

    let def2 = DefinitionsModel {
        name: Set(String::from("10")),
        super_base_element: Set(res.last_insert_id),
        target_namespace: Set(String::from(" ")),
        exporter: Set(String::from(" ")),
        exporter_version: Set(String::from(" ")),
        ..Default::default()
    };

    def2.insert(&connection).await?;

    let all_defs = Definitions::find().all(&connection).await.unwrap();
    println!("all Definitions : {:?}", all_defs);
    println!();

    let d1 = Definitions::find_by_id(1)
        .one(&connection)
        .await
        .unwrap()
        .unwrap();
    println!("d1: {:?}", d1);
    println!();

    d1.delete(&connection).await.unwrap();

    let mut d2: DefinitionsModel = Definitions::find_by_id(2)
        .one(&connection)
        .await
        .unwrap()
        .unwrap()
        .into();

    d2.name = Set(String::from("100"));
    d2.update(&connection).await.unwrap();

    let all_defs = Definitions::find().all(&connection).await.unwrap();
    println!("all Definitions : {:?}", all_defs);
    println!();
    Ok(())
}

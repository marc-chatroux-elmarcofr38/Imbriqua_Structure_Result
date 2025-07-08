#![doc = include_str!("../README.md")]

use entities::*;
pub use sea_orm;
use sea_orm::{
    ActiveModelTrait, ActiveValue::*, ConnectionTrait, Database, DbBackend, DbConn, DbErr,
    EntityOrSelect, EntityTrait, InsertResult, ModelTrait, Schema,
};

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    println!("");

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

    // CREATE BASE ELEMENT AND DEFINITIONS
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
    def1.insert(&connection).await?;
    //

    // CHECKING INSERT
    println!(
        "Get BaseElement from Definition : {:?}\n",
        Definitions::find()
            .find_also_related(BaseElement)
            .all(&connection)
            .await?
    );
    //

    // CREATE BASE ELEMENT AND DEFINITIONS
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
    //

    // CHECKING INSERT
    println!(
        "Get BaseElement from Definition : {:?}\n",
        Definitions::find()
            .find_also_related(BaseElement)
            .all(&connection)
            .await?
    );
    //

    // // DELETION A DEFINITION, WANT CASCADE
    // let d1 = Definitions::find_by_id(1)
    //     .find_also_related(BaseElement)
    //     .one(&connection)
    //     .await
    //     .unwrap()
    //     .unwrap();
    // println!("want to delete d1: {:?}\n", d1);

    // let (d1_0, d1_1) = d1;

    // let d1_base_element = BaseElement::find().all(&connection).await?;
    // println!("before : {:?}\n", d1_base_element);
    // let d1_definitions = Definitions::find().all(&connection).await?;
    // println!("before : {:?}\n", d1_definitions);

    // d1_0.delete(&connection).await?;

    // let d1_base_element = BaseElement::find().all(&connection).await?;
    // println!("checking : {:?}\n", d1_base_element);
    // let d1_definitions = Definitions::find().all(&connection).await?;
    // println!("checking : {:?}\n", d1_definitions);

    let mut d2: DefinitionsModel = Definitions::find_by_id(2)
        .one(&connection)
        .await
        .unwrap()
        .unwrap()
        .into();

    d2.name = Set(String::from("100"));
    d2.update(&connection).await?;

    let all_defs = Definitions::find().all(&connection).await?;
    println!("all Definitions : {:?}", all_defs);
    println!();
    Ok(())
}

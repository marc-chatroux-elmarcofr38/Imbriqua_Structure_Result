#![doc = include_str!("../README.md")]

use entities::*;
pub use sea_orm;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ConnectionTrait, Database, DbBackend, DbConn, EntityTrait,
    ModelTrait, Schema,
};

#[tokio::main]
async fn main() {
    // Set connection
    let connection: DbConn = Database::connect("sqlite::memory:").await.unwrap();

    // Set table
    let schema = Schema::new(DbBackend::Sqlite);

    //
    //
    //
    //
    //

    // Derive from Entity
    let stmt: sea_orm::sea_query::TableCreateStatement = schema.create_table_from_entity(Point);

    // Execute create table statement
    connection
        .execute(connection.get_database_backend().build(&stmt))
        .await
        .unwrap();

    //
    //
    //
    //
    //

    let user1 = PointModel {
        x: Set(1.0),
        y: Set(2.0),
        ..Default::default()
    };

    user1.insert(&connection).await.unwrap();

    let user2 = PointModel {
        x: Set(10.0),
        y: Set(20.0),
        ..Default::default()
    };

    user2.insert(&connection).await.unwrap();

    let all_users = Point::find().all(&connection).await.unwrap();
    println!("all users: {:?}", all_users);
    println!();

    let u1 = Point::find_by_id(1)
        .one(&connection)
        .await
        .unwrap()
        .unwrap();
    println!("u1: {:?}", u1);
    println!();

    u1.delete(&connection).await.unwrap();

    let mut u2: PointModel = Point::find_by_id(2)
        .one(&connection)
        .await
        .unwrap()
        .unwrap()
        .into();

    u2.x = Set(100.0);
    u2.update(&connection).await.unwrap();

    let all_users = Point::find().all(&connection).await.unwrap();
    println!("all users: {:?}", all_users);
    println!();
}

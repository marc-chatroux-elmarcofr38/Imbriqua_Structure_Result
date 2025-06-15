//! Bounds
//! dc_datatype_bounds

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "dc_datatype_bounds")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

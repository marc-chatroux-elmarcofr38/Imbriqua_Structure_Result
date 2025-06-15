//! Font
//! dc_datatype_font

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "dc_datatype_font")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: Option<String>,
    pub size: Option<i64>,
    pub is_bold: Option<bool>,
    pub is_italic: Option<bool>,
    pub is_underline: Option<bool>,
    pub is_strike_through: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

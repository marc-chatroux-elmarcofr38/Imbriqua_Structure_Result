//! bpmn_20_class_end_point

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_end_point")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : RootElement
    pub super_root_element: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_root_element::Entity",
        from = "Column::SuperRootElement",
        to = "super::bpmn_20_root_element::Column::Id"
    )]
    RootElement,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_root_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RootElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "EndPoint",
//     name: "EndPoint",
//     is_abstract: false,
//     super_class: [
//         "RootElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


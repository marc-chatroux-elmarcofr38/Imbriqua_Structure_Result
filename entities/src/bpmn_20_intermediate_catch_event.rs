//! bpmn_20_class_intermediate_catch_event

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_intermediate_catch_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SUPER FIELD : CatchEvent
    pub super_catch_event: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE IntermediateCatchEvent need ONE CatchEvent
    #[sea_orm(
        belongs_to = "super::bpmn_20_catch_event::Entity",
        from = "Column::SuperCatchEvent",
        to = "super::bpmn_20_catch_event::Column::Id"
    )]
    CatchEvent,
}

// SUPER : ONE IntermediateCatchEvent need ONE CatchEvent
impl Related<super::bpmn_20_catch_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CatchEvent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "IntermediateCatchEvent",
//     name: "IntermediateCatchEvent",
//     is_abstract: false,
//     super_class: [
//         "CatchEvent",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


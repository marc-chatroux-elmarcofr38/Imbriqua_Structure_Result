//! bpmn_20_class_end_event

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_end_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : ThrowEvent
    pub super_throw_event: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE EndEvent need ONE ThrowEvent
    #[sea_orm(
        belongs_to = "super::bpmn_20_throw_event::Entity",
        from = "Column::SuperThrowEvent",
        to = "super::bpmn_20_throw_event::Column::Id"
    )]
    ThrowEvent,
}

// SUPER : ONE EndEvent need ONE ThrowEvent
impl Related<super::bpmn_20_throw_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ThrowEvent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "EndEvent",
//     name: "EndEvent",
//     is_abstract: false,
//     super_class: [
//         "ThrowEvent",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


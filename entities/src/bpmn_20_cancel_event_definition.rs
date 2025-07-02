//! bpmn_20_class_cancel_event_definition

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_cancel_event_definition")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : EventDefinition
    pub super_event_definition: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE CancelEventDefinition need ONE EventDefinition
    #[sea_orm(
        belongs_to = "super::bpmn_20_event_definition::Entity",
        from = "Column::SuperEventDefinition",
        to = "super::bpmn_20_event_definition::Column::Id"
    )]
    EventDefinition,
}

// SUPER : ONE CancelEventDefinition need ONE EventDefinition
impl Related<super::bpmn_20_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventDefinition.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "CancelEventDefinition",
//     name: "CancelEventDefinition",
//     is_abstract: false,
//     super_class: [
//         "EventDefinition",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


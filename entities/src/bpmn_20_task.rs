//! bpmn_20_class_task

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub pk_id: i32,
    /// SIMPLE FIELD : Activity
    pub super_activity: i64,
    /// SIMPLE FIELD : InteractionNode
    pub super_interaction_node: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "Task",
//     name: "Task",
//     is_abstract: false,
//     super_class: [
//         "Activity",
//         "InteractionNode",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


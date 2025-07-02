//! bpmn_20_class_manual_task

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_manual_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : Task
    pub super_task: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_task::Entity",
        from = "Column::SuperTask",
        to = "super::bpmn_20_task::Column::Id"
    )]
    Task,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "ManualTask",
//     name: "ManualTask",
//     is_abstract: false,
//     super_class: [
//         "Task",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


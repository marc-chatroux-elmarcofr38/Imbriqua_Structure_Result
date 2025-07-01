//! bpmn_20_class_global_manual_task

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_global_manual_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub pk_id: i32,
    /// SIMPLE FIELD : GlobalTask
    pub super_global_task: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "GlobalManualTask",
//     name: "GlobalManualTask",
//     is_abstract: false,
//     super_class: [
//         "GlobalTask",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


//! bpmn_20_class_intermediate_catch_event

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_intermediate_catch_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub pk_id: i32,
    /// SIMPLE FIELD : CatchEvent
    pub super_catch_event: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
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


//! bpmn_20_class_human_performer

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_human_performer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub pk_id: i32,
    /// SIMPLE FIELD : Performer
    pub super_performer: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "HumanPerformer",
//     name: "HumanPerformer",
//     is_abstract: false,
//     super_class: [
//         "Performer",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


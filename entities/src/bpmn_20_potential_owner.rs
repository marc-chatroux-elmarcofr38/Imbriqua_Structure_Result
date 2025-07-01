//! bpmn_20_class_potential_owner

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_potential_owner")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub pk_id: i32,
    /// SIMPLE FIELD : HumanPerformer
    pub super_human_performer: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "PotentialOwner",
//     name: "PotentialOwner",
//     is_abstract: false,
//     super_class: [
//         "HumanPerformer",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


//! bpmn_20_class_parallel_gateway

use crate::*;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel, Default)]
#[sea_orm(table_name = "bpmn_20_parallel_gateway")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "ParallelGateway",
//     name: "ParallelGateway",
//     is_abstract: false,
//     super_class: Some(
//         "Gateway",
//     ),
//     super_class_link: None,
//     owned_attribute: [],
//     owned_rule: [],
// }


//! bpmn_20_class_parallel_gateway

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_parallel_gateway")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : Gateway
    pub super_gateway: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_gateway::Entity",
        from = "Column::SuperGateway",
        to = "super::bpmn_20_gateway::Column::Id"
    )]
    Gateway,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_gateway::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Gateway.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "ParallelGateway",
//     name: "ParallelGateway",
//     is_abstract: false,
//     super_class: [
//         "Gateway",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


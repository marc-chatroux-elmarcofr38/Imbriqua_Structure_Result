//! bpmn_20_class_gateway

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_gateway")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : FlowNode
    pub super_flow_node: i32,
    /// SIMPLE FIELD : Gateway-gatewayDirection
    #[sea_orm(default_value = "unspecified")]
    pub gateway_direction: GatewayDirection,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_node::Entity",
        from = "Column::SuperFlowNode",
        to = "super::bpmn_20_flow_node::Column::Id"
    )]
    FlowNode,
    #[sea_orm(has_one = "super::bpmn_20_complex_gateway::Entity")]
    ComplexGateway,
    #[sea_orm(has_one = "super::bpmn_20_event_based_gateway::Entity")]
    EventBasedGateway,
    #[sea_orm(has_one = "super::bpmn_20_exclusive_gateway::Entity")]
    ExclusiveGateway,
    #[sea_orm(has_one = "super::bpmn_20_inclusive_gateway::Entity")]
    InclusiveGateway,
    #[sea_orm(has_one = "super::bpmn_20_parallel_gateway::Entity")]
    ParallelGateway,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_flow_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowNode.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_complex_gateway::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ComplexGateway.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_event_based_gateway::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventBasedGateway.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_exclusive_gateway::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExclusiveGateway.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_inclusive_gateway::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InclusiveGateway.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_parallel_gateway::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ParallelGateway.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "Gateway",
//     name: "Gateway",
//     is_abstract: true,
//     super_class: [
//         "FlowNode",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "Gateway-gatewayDirection",
//                 name: "gatewayDirection",
//                 visibility: Public,
//                 simple_type: Some(
//                     "GatewayDirection",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: Some(
//                     "unspecified",
//                 ),
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }


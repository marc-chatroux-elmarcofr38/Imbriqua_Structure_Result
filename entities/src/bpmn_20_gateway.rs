//! bpmn_20_class_gateway

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_gateway")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : FlowNode
    pub super_flow_node: i64,
    /// SIMPLE FIELD : Gateway-gatewayDirection
    #[sea_orm(default_value = "unspecified")]
    pub gateway_direction: GatewayDirection,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Gateway need ONE FlowNode
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_node::Entity",
        from = "Column::SuperFlowNode",
        to = "super::bpmn_20_flow_node::Column::Id",
        on_delete = "Cascade"
    )]
    FlowNode,
    // SUPER : ONE ComplexGateway need ONE Gateway
    #[sea_orm(has_one = "super::bpmn_20_complex_gateway::Entity")]
    ComplexGateway,
    // SUPER : ONE EventBasedGateway need ONE Gateway
    #[sea_orm(has_one = "super::bpmn_20_event_based_gateway::Entity")]
    EventBasedGateway,
    // SUPER : ONE ExclusiveGateway need ONE Gateway
    #[sea_orm(has_one = "super::bpmn_20_exclusive_gateway::Entity")]
    ExclusiveGateway,
    // SUPER : ONE InclusiveGateway need ONE Gateway
    #[sea_orm(has_one = "super::bpmn_20_inclusive_gateway::Entity")]
    InclusiveGateway,
    // SUPER : ONE ParallelGateway need ONE Gateway
    #[sea_orm(has_one = "super::bpmn_20_parallel_gateway::Entity")]
    ParallelGateway,
}

// SUPER : ONE Gateway need ONE FlowNode
impl Related<super::bpmn_20_flow_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowNode.def()
    }
}

// SUPER : ONE ComplexGateway need ONE Gateway
impl Related<super::bpmn_20_complex_gateway::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ComplexGateway.def()
    }
}

// SUPER : ONE EventBasedGateway need ONE Gateway
impl Related<super::bpmn_20_event_based_gateway::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventBasedGateway.def()
    }
}

// SUPER : ONE ExclusiveGateway need ONE Gateway
impl Related<super::bpmn_20_exclusive_gateway::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExclusiveGateway.def()
    }
}

// SUPER : ONE InclusiveGateway need ONE Gateway
impl Related<super::bpmn_20_inclusive_gateway::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InclusiveGateway.def()
    }
}

// SUPER : ONE ParallelGateway need ONE Gateway
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


//! bpmn_20_class_gateway

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_gateway")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperFlowNode
    pub super_flow_node: i64,
    /// SIMPLE FIELD : BPMN20-Gateway-gatewayDirection
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

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-Gateway',
//     name: "Gateway",
//     is_abstract: true,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'BPMN20-FlowNode',
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "Gateway-gatewayDirection": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-Gateway-gatewayDirection',
//                 name: "gatewayDirection",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-GatewayDirection',
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
//                 owning_association: None,
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Gateway",
//     table_name: "bpmn_20_gateway",
//     model_name: "Gateway",
//     full_name: "bpmn_20_class_gateway",
//     reverse_super: RefCell {
//         value: [
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//         ],
//     },
// }


//! bpmn_20_association_a_message_flow_refs_communication

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_message_flow_refs_communication")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub message_flow_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub conversation_node_b_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_message_flow::Entity",
        from = "Column::MessageFlowAId",
        to = "super::bpmn_20_message_flow::Column::Id"
    )]
    MessageFlow,
    #[sea_orm(
        belongs_to = "super::bpmn_20_conversation_node::Entity",
        from = "Column::ConversationNodeBId",
        to = "super::bpmn_20_conversation_node::Column::Id"
    )]
    ConversationNode,
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFAssociation {
//     xmi_id: XMIIdReference {
//         local_id: "A_messageFlowRefs_communication",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "A_messageFlowRefs_communication",
//     visibility: Private,
//     member_end: (
//         "ConversationNode-messageFlowRefs",
//         "A_messageFlowRefs_communication-communication",
//     ),
//     owned_end: {
//         "-A_messageFlowRefs_communication-communication": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "A_messageFlowRefs_communication-communication",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "communication",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ConversationNode",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "A_messageFlowRefs_communication",
//                 association: Some(
//                     "A_messageFlowRefs_communication",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     is_derived: false,
//     technical_name: "BPMN20.cmof#A_messageFlowRefs_communication",
//     table_name: "bpmn_20_a_message_flow_refs_communication",
//     model_name: "AMessageFlowRefsCommunication",
//     full_name: "bpmn_20_association_a_message_flow_refs_communication",
// }


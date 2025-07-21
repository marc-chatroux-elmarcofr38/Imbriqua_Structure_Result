//! bpmn_20_association_a_participant_refs_conversation_node

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_participant_refs_conversation_node")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub participant_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub conversation_node_b_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_participant::Entity",
        from = "Column::ParticipantAId",
        to = "super::bpmn_20_participant::Column::Id"
    )]
    Participant,
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
//     xmi_id: XMIIdLocalReference {
//         object_id: "A_participantRefs_conversationNode",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "A_participantRefs_conversationNode",
//     visibility: Private,
//     member_end: (
//         "RefCell of 'BPMN20-ConversationNode-participantRefs' (loaded : true)",
//         "RefCell of 'BPMN20-A_participantRefs_conversationNode-conversationNode' (loaded : true)",
//     ),
//     owned_end: {
//         "A_participantRefs_conversationNode-conversationNode": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "A_participantRefs_conversationNode-conversationNode",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "conversationNode",
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
//                 owning_association: "A_participantRefs_conversationNode",
//                 association: Some(
//                     "A_participantRefs_conversationNode",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     is_derived: false,
//     technical_name: "BPMN20.cmof#A_participantRefs_conversationNode",
//     table_name: "bpmn_20_a_participant_refs_conversation_node",
//     model_name: "AParticipantRefsConversationNode",
//     full_name: "bpmn_20_association_a_participant_refs_conversation_node",
// }


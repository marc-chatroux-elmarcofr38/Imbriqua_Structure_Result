//! bpmn_20_class_call_conversation

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_call_conversation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SUPER FIELD : ConversationNode
    pub super_conversation_node: i32,
    /// COMPLEX FIELD : CallConversation-calledCollaborationRef
    pub called_collaboration_ref: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE CallConversation need ONE ConversationNode
    #[sea_orm(
        belongs_to = "super::bpmn_20_conversation_node::Entity",
        from = "Column::SuperConversationNode",
        to = "super::bpmn_20_conversation_node::Column::Id"
    )]
    ConversationNode,
}

// SUPER : ONE CallConversation need ONE ConversationNode
impl Related<super::bpmn_20_conversation_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConversationNode.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "CallConversation",
//     name: "CallConversation",
//     is_abstract: false,
//     super_class: [
//         "ConversationNode",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "CallConversation-calledCollaborationRef",
//                 name: "calledCollaborationRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Collaboration",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_calledCollaborationRef_callConversation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "CallConversation-participantAssociations",
//                 name: "participantAssociations",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ParticipantAssociation",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_participantAssociations_callConversation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }


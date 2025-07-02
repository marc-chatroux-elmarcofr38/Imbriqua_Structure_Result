//! bpmn_20_class_conversation_node

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_conversation_node")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : InteractionNode
    pub super_interaction_node: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// SIMPLE FIELD : ConversationNode-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ConversationNode need ONE InteractionNode
    #[sea_orm(
        belongs_to = "super::bpmn_20_interaction_node::Entity",
        from = "Column::SuperInteractionNode",
        to = "super::bpmn_20_interaction_node::Column::Id"
    )]
    InteractionNode,
    // SUPER : ONE ConversationNode need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id"
    )]
    BaseElement,
    // SUPER : ONE CallConversation need ONE ConversationNode
    #[sea_orm(has_one = "super::bpmn_20_call_conversation::Entity")]
    CallConversation,
    // SUPER : ONE Conversation need ONE ConversationNode
    #[sea_orm(has_one = "super::bpmn_20_conversation::Entity")]
    Conversation,
    // SUPER : ONE SubConversation need ONE ConversationNode
    #[sea_orm(has_one = "super::bpmn_20_sub_conversation::Entity")]
    SubConversation,
}

// SUPER : ONE ConversationNode need ONE InteractionNode
impl Related<super::bpmn_20_interaction_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InteractionNode.def()
    }
}

// SUPER : ONE ConversationNode need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// SUPER : ONE CallConversation need ONE ConversationNode
impl Related<super::bpmn_20_call_conversation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CallConversation.def()
    }
}

// SUPER : ONE Conversation need ONE ConversationNode
impl Related<super::bpmn_20_conversation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Conversation.def()
    }
}

// SUPER : ONE SubConversation need ONE ConversationNode
impl Related<super::bpmn_20_sub_conversation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubConversation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "ConversationNode",
//     name: "ConversationNode",
//     is_abstract: true,
//     super_class: [
//         "InteractionNode",
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "ConversationNode-name",
//                 name: "name",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#String",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ConversationNode-participantRefs",
//                 name: "participantRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Participant",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 2,
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
//                 owning_association: "",
//                 association: Some(
//                     "A_participantRefs_conversationNode",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ConversationNode-messageFlowRefs",
//                 name: "messageFlowRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "MessageFlow",
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
//                 owning_association: "",
//                 association: Some(
//                     "A_messageFlowRefs_communication",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ConversationNode-correlationKeys",
//                 name: "correlationKeys",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CorrelationKey",
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
//                     "A_correlationKeys_conversationNode",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }


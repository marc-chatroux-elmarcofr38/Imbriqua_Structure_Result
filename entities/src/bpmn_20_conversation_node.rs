//! bpmn_20_class_conversation_node

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_conversation_node")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// SUPER FIELD : InteractionNode
    pub super_interaction_node: i64,
    /// SIMPLE FIELD : BPMN20-ConversationNode-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ConversationNode need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
    // SUPER : ONE ConversationNode need ONE InteractionNode
    #[sea_orm(
        belongs_to = "super::bpmn_20_interaction_node::Entity",
        from = "Column::SuperInteractionNode",
        to = "super::bpmn_20_interaction_node::Column::Id",
        on_delete = "Cascade"
    )]
    InteractionNode,
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

// SUPER : ONE ConversationNode need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// SUPER : ONE ConversationNode need ONE InteractionNode
impl Related<super::bpmn_20_interaction_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InteractionNode.def()
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

// ManyToMany : with MessageFlow using A_messageFlowRefs_communication
impl Related<super::bpmn_20_a_message_flow_refs_communication::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_message_flow_refs_communication::Relation::MessageFlow.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_message_flow_refs_communication::Relation::ConversationNode
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with Participant using A_participantRefs_conversationNode
impl Related<super::bpmn_20_a_participant_refs_conversation_node::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_participant_refs_conversation_node::Relation::Participant.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_participant_refs_conversation_node::Relation::ConversationNode
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ConversationNode" (bpmn_20_class_conversation_node)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "BPMN20-ConversationNode-name")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __SubConversation__ (__SubConversationModel__) from A_conversationNodes_subConversation
    ///   * one-to-many link : (0-1) __ConversationNode__ need (0-inf) __SubConversation__)
    ///   * callable using find_with_related(__SubConversationModel__) from __ConversationNode__
    ///   * named sub_conversation in BPMN
    /// * __Collaboration__ (__CollaborationModel__) from A_conversations_collaboration
    ///   * one-to-many link : (0-1) __ConversationNode__ need (0-inf) __Collaboration__)
    ///   * callable using find_with_related(__CollaborationModel__) from __ConversationNode__
    ///   * named collaboration in BPMN
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __ConversationNode__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __ConversationNode__
    ///   * saved in __super_base_element__ field as foreing key
    /// * __InteractionNode__ (__InteractionNodeModel__)
    ///   * one-to-one link : one __ConversationNode__ need one __InteractionNode__)
    ///   * callable using find_also_related(__InteractionNodeModel__) from __ConversationNode__
    ///   * saved in __super_interaction_node__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __CallConversation__ (__CallConversationModel__)
    ///   * one-to-one link (reverse) : one __CallConversation__ need one __ConversationNode__)
    ///   * callable using find_also_related(__ConversationNodeModel__) from __CallConversation__
    ///   * saved in __super_conversation_node__ field as foreing key in __CallConversationModel__
    /// * __Conversation__ (__ConversationModel__)
    ///   * one-to-one link (reverse) : one __Conversation__ need one __ConversationNode__)
    ///   * callable using find_also_related(__ConversationNodeModel__) from __Conversation__
    ///   * saved in __super_conversation_node__ field as foreing key in __ConversationModel__
    /// * __SubConversation__ (__SubConversationModel__)
    ///   * one-to-one link (reverse) : one __SubConversation__ need one __ConversationNode__)
    ///   * callable using find_also_related(__ConversationNodeModel__) from __SubConversation__
    ///   * saved in __super_conversation_node__ field as foreing key in __SubConversationModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ConversationNode" (bpmn_20_class_conversation_node)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "BPMN20-ConversationNode-name")
  * type : __std::string::String__


## Relation : One To Many :
* __SubConversation__ (__SubConversationModel__) from A_conversationNodes_subConversation
  * one-to-many link : (0-1) __ConversationNode__ need (0-inf) __SubConversation__)
  * callable using find_with_related(__SubConversationModel__) from __ConversationNode__
  * named sub_conversation in BPMN
* __Collaboration__ (__CollaborationModel__) from A_conversations_collaboration
  * one-to-many link : (0-1) __ConversationNode__ need (0-inf) __Collaboration__)
  * callable using find_with_related(__CollaborationModel__) from __ConversationNode__
  * named collaboration in BPMN

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __ConversationNode__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __ConversationNode__
  * saved in __super_base_element__ field as foreing key
* __InteractionNode__ (__InteractionNodeModel__)
  * one-to-one link : one __ConversationNode__ need one __InteractionNode__)
  * callable using find_also_related(__InteractionNodeModel__) from __ConversationNode__
  * saved in __super_interaction_node__ field as foreing key

## Reverse Super :
* __CallConversation__ (__CallConversationModel__)
  * one-to-one link (reverse) : one __CallConversation__ need one __ConversationNode__)
  * callable using find_also_related(__ConversationNodeModel__) from __CallConversation__
  * saved in __super_conversation_node__ field as foreing key in __CallConversationModel__
* __Conversation__ (__ConversationModel__)
  * one-to-one link (reverse) : one __Conversation__ need one __ConversationNode__)
  * callable using find_also_related(__ConversationNodeModel__) from __Conversation__
  * saved in __super_conversation_node__ field as foreing key in __ConversationModel__
* __SubConversation__ (__SubConversationModel__)
  * one-to-one link (reverse) : one __SubConversation__ need one __ConversationNode__)
  * callable using find_also_related(__ConversationNodeModel__) from __SubConversation__
  * saved in __super_conversation_node__ field as foreing key in __SubConversationModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "ConversationNode",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "ConversationNode",
//     is_abstract: true,
//     super_class: [
//         "InteractionNode",
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-ConversationNode-correlationKeys": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "ConversationNode-correlationKeys",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
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
//         "-ConversationNode-messageFlowRefs": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "ConversationNode-messageFlowRefs",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
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
//         "-ConversationNode-name": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "ConversationNode-name",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "name",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
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
//         "-ConversationNode-participantRefs": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "ConversationNode-participantRefs",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
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
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#ConversationNode",
//     table_name: "bpmn_20_conversation_node",
//     model_name: "ConversationNode",
//     full_name: "bpmn_20_class_conversation_node",
// }


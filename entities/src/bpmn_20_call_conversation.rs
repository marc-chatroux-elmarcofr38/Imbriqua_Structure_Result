//! bpmn_20_class_call_conversation

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_call_conversation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : ConversationNode
    pub super_conversation_node: i64,
    /// COMPLEX FIELD : CallConversation-calledCollaborationRef
    pub called_collaboration_ref: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE CallConversation need ONE ConversationNode
    #[sea_orm(
        belongs_to = "super::bpmn_20_conversation_node::Entity",
        from = "Column::SuperConversationNode",
        to = "super::bpmn_20_conversation_node::Column::Id",
        on_delete = "Cascade"
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

impl ActiveModel {
    /// # Help document for "CallConversation" (bpmn_20_class_call_conversation)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __ConversationNode__ (__ConversationNodeModel__)
    ///   * one-to-one link : one __CallConversation__ need one __ConversationNode__)
    ///   * callable using find_also_related(__ConversationNodeModel__) from __CallConversation__
    ///   * saved in __super_conversation_node__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "CallConversation" (bpmn_20_class_call_conversation)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Direct Super :
* __ConversationNode__ (__ConversationNodeModel__)
  * one-to-one link : one __CallConversation__ need one __ConversationNode__)
  * callable using find_also_related(__ConversationNodeModel__) from __CallConversation__
  * saved in __super_conversation_node__ field as foreing key


"#
    }
}

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


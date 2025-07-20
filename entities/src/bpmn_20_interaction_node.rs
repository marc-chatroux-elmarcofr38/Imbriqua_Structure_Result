//! bpmn_20_class_interaction_node

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_interaction_node")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ConversationNode need ONE InteractionNode
    #[sea_orm(has_one = "super::bpmn_20_conversation_node::Entity")]
    ConversationNode,
    // SUPER : ONE Event need ONE InteractionNode
    #[sea_orm(has_one = "super::bpmn_20_event::Entity")]
    Event,
    // SUPER : ONE Participant need ONE InteractionNode
    #[sea_orm(has_one = "super::bpmn_20_participant::Entity")]
    Participant,
    // SUPER : ONE Task need ONE InteractionNode
    #[sea_orm(has_one = "super::bpmn_20_task::Entity")]
    Task,
}

// SUPER : ONE ConversationNode need ONE InteractionNode
impl Related<super::bpmn_20_conversation_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConversationNode.def()
    }
}

// SUPER : ONE Event need ONE InteractionNode
impl Related<super::bpmn_20_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Event.def()
    }
}

// SUPER : ONE Participant need ONE InteractionNode
impl Related<super::bpmn_20_participant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Participant.def()
    }
}

// SUPER : ONE Task need ONE InteractionNode
impl Related<super::bpmn_20_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "InteractionNode" (bpmn_20_class_interaction_node)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// 
    /// ## Reverse Super :
    /// * __ConversationNode__ (__ConversationNodeModel__)
    ///   * one-to-one link (reverse) : one __ConversationNode__ need one __InteractionNode__)
    ///   * callable using find_also_related(__InteractionNodeModel__) from __ConversationNode__
    ///   * saved in __super_interaction_node__ field as foreing key in __ConversationNodeModel__
    /// * __Event__ (__EventModel__)
    ///   * one-to-one link (reverse) : one __Event__ need one __InteractionNode__)
    ///   * callable using find_also_related(__InteractionNodeModel__) from __Event__
    ///   * saved in __super_interaction_node__ field as foreing key in __EventModel__
    /// * __Participant__ (__ParticipantModel__)
    ///   * one-to-one link (reverse) : one __Participant__ need one __InteractionNode__)
    ///   * callable using find_also_related(__InteractionNodeModel__) from __Participant__
    ///   * saved in __super_interaction_node__ field as foreing key in __ParticipantModel__
    /// * __Task__ (__TaskModel__)
    ///   * one-to-one link (reverse) : one __Task__ need one __InteractionNode__)
    ///   * callable using find_also_related(__InteractionNodeModel__) from __Task__
    ///   * saved in __super_interaction_node__ field as foreing key in __TaskModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "InteractionNode" (bpmn_20_class_interaction_node)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__





## Reverse Super :
* __ConversationNode__ (__ConversationNodeModel__)
  * one-to-one link (reverse) : one __ConversationNode__ need one __InteractionNode__)
  * callable using find_also_related(__InteractionNodeModel__) from __ConversationNode__
  * saved in __super_interaction_node__ field as foreing key in __ConversationNodeModel__
* __Event__ (__EventModel__)
  * one-to-one link (reverse) : one __Event__ need one __InteractionNode__)
  * callable using find_also_related(__InteractionNodeModel__) from __Event__
  * saved in __super_interaction_node__ field as foreing key in __EventModel__
* __Participant__ (__ParticipantModel__)
  * one-to-one link (reverse) : one __Participant__ need one __InteractionNode__)
  * callable using find_also_related(__InteractionNodeModel__) from __Participant__
  * saved in __super_interaction_node__ field as foreing key in __ParticipantModel__
* __Task__ (__TaskModel__)
  * one-to-one link (reverse) : one __Task__ need one __InteractionNode__)
  * callable using find_also_related(__InteractionNodeModel__) from __Task__
  * saved in __super_interaction_node__ field as foreing key in __TaskModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Weak ref of "BPMN20-InteractionNode" (loaded : false)",
//     name: "InteractionNode",
//     is_abstract: true,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: {
//         "-InteractionNode-incomingConversationLinks": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-InteractionNode-incomingConversationLinks" (loaded : false)",
//                 name: "incomingConversationLinks",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ConversationLink",
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
//                 is_derived: true,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_targetRef_incomingConversationLinks",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-InteractionNode-outgoingConversationLinks": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-InteractionNode-outgoingConversationLinks" (loaded : false)",
//                 name: "outgoingConversationLinks",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ConversationLink",
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
//                 is_derived: true,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_sourceRef_outgoingConversationLinks",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#InteractionNode",
//     table_name: "bpmn_20_interaction_node",
//     model_name: "InteractionNode",
//     full_name: "bpmn_20_class_interaction_node",
// }


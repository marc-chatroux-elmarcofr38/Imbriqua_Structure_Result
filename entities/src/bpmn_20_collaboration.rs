//! bpmn_20_class_collaboration

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_collaboration")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : RootElement
    pub super_root_element: i64,
    /// COMPLEX FIELD : Collaboration-conversationAssociations
    pub conversation_associations: i64,
    /// SIMPLE FIELD : Collaboration-name
    pub name: std::string::String,
    /// SIMPLE FIELD : Collaboration-isClosed
    pub is_closed: std::primitive::bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Collaboration need ONE RootElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_root_element::Entity",
        from = "Column::SuperRootElement",
        to = "super::bpmn_20_root_element::Column::Id",
        on_delete = "Cascade"
    )]
    RootElement,
    // SUPER : ONE Choreography need ONE Collaboration
    #[sea_orm(has_one = "super::bpmn_20_choreography::Entity")]
    Choreography,
    // SUPER : ONE GlobalConversation need ONE Collaboration
    #[sea_orm(has_one = "super::bpmn_20_global_conversation::Entity")]
    GlobalConversation,
}

// SUPER : ONE Collaboration need ONE RootElement
impl Related<super::bpmn_20_root_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RootElement.def()
    }
}

// SUPER : ONE Choreography need ONE Collaboration
impl Related<super::bpmn_20_choreography::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Choreography.def()
    }
}

// SUPER : ONE GlobalConversation need ONE Collaboration
impl Related<super::bpmn_20_global_conversation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GlobalConversation.def()
    }
}

// ManyToMany : with Choreography using A_choreographyRef_collaboration
impl Related<super::bpmn_20_a_choreography_ref_collaboration::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_choreography_ref_collaboration::Relation::Choreography.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_choreography_ref_collaboration::Relation::Collaboration
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Collaboration" (bpmn_20_class_collaboration)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "Collaboration-name")
    ///   * type : __std::string::String__
    /// * __is_closed__ (xmi_id : "Collaboration-isClosed")
    ///   * type : __std::primitive::bool__
    /// 
    /// ## Direct One To One :
    /// * __ConversationAssociation__ (__ConversationAssociationModel__) from A_conversationAssociations_converstaionAssociations
    ///   * one-to-one link : (1-1) __Collaboration__ need (1-1) __ConversationAssociation__)
    ///   * callable using find_also_related(__ConversationAssociationModel__) from __Collaboration__
    ///   * saved in __conversation_associations__ field as foreing key
    /// 
    /// 
    /// ## Direct Super :
    /// * __RootElement__ (__RootElementModel__)
    ///   * one-to-one link : one __Collaboration__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __Collaboration__
    ///   * saved in __super_root_element__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __Choreography__ (__ChoreographyModel__)
    ///   * one-to-one link (reverse) : one __Choreography__ need one __Collaboration__)
    ///   * callable using find_also_related(__CollaborationModel__) from __Choreography__
    ///   * saved in __super_collaboration__ field as foreing key in __ChoreographyModel__
    /// * __GlobalConversation__ (__GlobalConversationModel__)
    ///   * one-to-one link (reverse) : one __GlobalConversation__ need one __Collaboration__)
    ///   * callable using find_also_related(__CollaborationModel__) from __GlobalConversation__
    ///   * saved in __super_collaboration__ field as foreing key in __GlobalConversationModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Collaboration" (bpmn_20_class_collaboration)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "Collaboration-name")
  * type : __std::string::String__
* __is_closed__ (xmi_id : "Collaboration-isClosed")
  * type : __std::primitive::bool__

## Direct One To One :
* __ConversationAssociation__ (__ConversationAssociationModel__) from A_conversationAssociations_converstaionAssociations
  * one-to-one link : (1-1) __Collaboration__ need (1-1) __ConversationAssociation__)
  * callable using find_also_related(__ConversationAssociationModel__) from __Collaboration__
  * saved in __conversation_associations__ field as foreing key


## Direct Super :
* __RootElement__ (__RootElementModel__)
  * one-to-one link : one __Collaboration__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __Collaboration__
  * saved in __super_root_element__ field as foreing key

## Reverse Super :
* __Choreography__ (__ChoreographyModel__)
  * one-to-one link (reverse) : one __Choreography__ need one __Collaboration__)
  * callable using find_also_related(__CollaborationModel__) from __Choreography__
  * saved in __super_collaboration__ field as foreing key in __ChoreographyModel__
* __GlobalConversation__ (__GlobalConversationModel__)
  * one-to-one link (reverse) : one __GlobalConversation__ need one __Collaboration__)
  * callable using find_also_related(__CollaborationModel__) from __GlobalConversation__
  * saved in __super_collaboration__ field as foreing key in __GlobalConversationModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Collaboration",
//     name: "Collaboration",
//     is_abstract: false,
//     super_class: [
//         "RootElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "Collaboration-name",
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
//                 xmi_id: "Collaboration-isClosed",
//                 name: "isClosed",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#Boolean",
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
//                 xmi_id: "Collaboration-choreographyRef",
//                 name: "choreographyRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Choreography",
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
//                     "A_choreographyRef_collaboration",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "Collaboration-artifacts",
//                 name: "artifacts",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Artifact",
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
//                     "A_artifacts_collaboration",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "Collaboration-participantAssociations",
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
//                     "A_participantAssociations_collaboration",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "Collaboration-messageFlowAssociations",
//                 name: "messageFlowAssociations",
//                 visibility: Public,
//                 simple_type: Some(
//                     "MessageFlowAssociation",
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
//                     "A_messageFlowAssociations_collaboration",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "Collaboration-conversationAssociations",
//                 name: "conversationAssociations",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ConversationAssociation",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
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
//                     "A_conversationAssociations_converstaionAssociations",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "Collaboration-participants",
//                 name: "participants",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Participant",
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
//                     "A_participants_collaboration",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "Collaboration-messageFlows",
//                 name: "messageFlows",
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
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_messageFlows_collaboration",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "Collaboration-correlationKeys",
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
//                     "A_correlationKeys_collaboration",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "Collaboration-conversations",
//                 name: "conversations",
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
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_conversations_collaboration",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "Collaboration-conversationLinks",
//                 name: "conversationLinks",
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
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_conversationLinks_collaboration",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }


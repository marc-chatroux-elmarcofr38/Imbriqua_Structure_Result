//! bpmn_20_class_message_flow

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_message_flow")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : MessageFlow-sourceRef
    pub source_ref: i64,
    /// COMPLEX FIELD : MessageFlow-targetRef
    pub target_ref: i64,
    /// COMPLEX FIELD : MessageFlow-messageRef
    pub message_ref: Option<i64>,
    /// SIMPLE FIELD : MessageFlow-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE MessageFlow need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE MessageFlow need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// ManyToMany : with ConversationNode using A_messageFlowRefs_communication
impl Related<super::bpmn_20_a_message_flow_refs_communication::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_message_flow_refs_communication::Relation::ConversationNode.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_message_flow_refs_communication::Relation::MessageFlow
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "MessageFlow" (bpmn_20_class_message_flow)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "MessageFlow-name")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __InteractionNode__ (__InteractionNodeModel__) from A_sourceRef_messageFlow
    ///   * one-to-many link : (1-1) __MessageFlow__ need (0-inf) __InteractionNode__)
    ///   * callable using find_with_related(__InteractionNodeModel__) from __MessageFlow__
    /// * __Message__ (__MessageModel__) from A_messageRef_messageFlow
    ///   * one-to-many link : (0-1) __MessageFlow__ need (0-inf) __Message__)
    ///   * callable using find_with_related(__MessageModel__) from __MessageFlow__
    /// * __Collaboration__ (__CollaborationModel__) from A_messageFlows_collaboration
    ///   * one-to-many link : (1-1) __MessageFlow__ need (0-inf) __Collaboration__)
    ///   * callable using find_with_related(__CollaborationModel__) from __MessageFlow__
    ///   * named collaboration in BPMN
    /// * __InteractionNode__ (__InteractionNodeModel__) from A_targetRef_messageFlow
    ///   * one-to-many link : (1-1) __MessageFlow__ need (0-inf) __InteractionNode__)
    ///   * callable using find_with_related(__InteractionNodeModel__) from __MessageFlow__
    /// * __ChoreographyTask__ (__ChoreographyTaskModel__) from A_messageFlowRef_choreographyTask
    ///   * one-to-many link : (0-1) __MessageFlow__ need (1-2) __ChoreographyTask__)
    ///   * callable using find_with_related(__ChoreographyTaskModel__) from __MessageFlow__
    ///   * named choreography_task in BPMN
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __MessageFlow__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __MessageFlow__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "MessageFlow" (bpmn_20_class_message_flow)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "MessageFlow-name")
  * type : __std::string::String__


## Relation : One To Many :
* __InteractionNode__ (__InteractionNodeModel__) from A_sourceRef_messageFlow
  * one-to-many link : (1-1) __MessageFlow__ need (0-inf) __InteractionNode__)
  * callable using find_with_related(__InteractionNodeModel__) from __MessageFlow__
* __Message__ (__MessageModel__) from A_messageRef_messageFlow
  * one-to-many link : (0-1) __MessageFlow__ need (0-inf) __Message__)
  * callable using find_with_related(__MessageModel__) from __MessageFlow__
* __Collaboration__ (__CollaborationModel__) from A_messageFlows_collaboration
  * one-to-many link : (1-1) __MessageFlow__ need (0-inf) __Collaboration__)
  * callable using find_with_related(__CollaborationModel__) from __MessageFlow__
  * named collaboration in BPMN
* __InteractionNode__ (__InteractionNodeModel__) from A_targetRef_messageFlow
  * one-to-many link : (1-1) __MessageFlow__ need (0-inf) __InteractionNode__)
  * callable using find_with_related(__InteractionNodeModel__) from __MessageFlow__
* __ChoreographyTask__ (__ChoreographyTaskModel__) from A_messageFlowRef_choreographyTask
  * one-to-many link : (0-1) __MessageFlow__ need (1-2) __ChoreographyTask__)
  * callable using find_with_related(__ChoreographyTaskModel__) from __MessageFlow__
  * named choreography_task in BPMN

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __MessageFlow__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __MessageFlow__
  * saved in __super_base_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "MessageFlow",
//     name: "MessageFlow",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "MessageFlow-name",
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
//                 xmi_id: "MessageFlow-sourceRef",
//                 name: "sourceRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "InteractionNode",
//                 ),
//                 complex_type: None,
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
//                 association: Some(
//                     "A_sourceRef_messageFlow",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "MessageFlow-targetRef",
//                 name: "targetRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "InteractionNode",
//                 ),
//                 complex_type: None,
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
//                 association: Some(
//                     "A_targetRef_messageFlow",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "MessageFlow-messageRef",
//                 name: "messageRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Message",
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
//                     "A_messageRef_messageFlow",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }


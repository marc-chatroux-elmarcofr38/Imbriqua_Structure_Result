//! bpmn_20_class_conversation_link

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_conversation_link")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : ConversationLink-sourceRef
    pub source_ref: i64,
    /// COMPLEX FIELD : ConversationLink-targetRef
    pub target_ref: i64,
    /// SIMPLE FIELD : ConversationLink-name
    pub name: Option<std::string::String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ConversationLink need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE ConversationLink need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ConversationLink" (bpmn_20_class_conversation_link)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "ConversationLink-name")
    ///   * type : __Option<std::string::String>__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __InteractionNode__ (__InteractionNodeModel__) from A_targetRef_incomingConversationLinks
    ///   * one-to-many link : (1-1) __ConversationLink__ need (0-inf) __InteractionNode__)
    ///   * callable using find_with_related(__InteractionNodeModel__) from __ConversationLink__
    ///   * named target_ref in BPMN
    /// * __Collaboration__ (__CollaborationModel__) from A_conversationLinks_collaboration
    ///   * one-to-many link : (1-1) __ConversationLink__ need (0-inf) __Collaboration__)
    ///   * callable using find_with_related(__CollaborationModel__) from __ConversationLink__
    ///   * named collaboration in BPMN
    /// * __InteractionNode__ (__InteractionNodeModel__) from A_sourceRef_outgoingConversationLinks
    ///   * one-to-many link : (1-1) __ConversationLink__ need (0-inf) __InteractionNode__)
    ///   * callable using find_with_related(__InteractionNodeModel__) from __ConversationLink__
    ///   * named source_ref in BPMN
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __ConversationLink__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __ConversationLink__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ConversationLink" (bpmn_20_class_conversation_link)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "ConversationLink-name")
  * type : __Option<std::string::String>__


## Relation : One To Many :
* __InteractionNode__ (__InteractionNodeModel__) from A_targetRef_incomingConversationLinks
  * one-to-many link : (1-1) __ConversationLink__ need (0-inf) __InteractionNode__)
  * callable using find_with_related(__InteractionNodeModel__) from __ConversationLink__
  * named target_ref in BPMN
* __Collaboration__ (__CollaborationModel__) from A_conversationLinks_collaboration
  * one-to-many link : (1-1) __ConversationLink__ need (0-inf) __Collaboration__)
  * callable using find_with_related(__CollaborationModel__) from __ConversationLink__
  * named collaboration in BPMN
* __InteractionNode__ (__InteractionNodeModel__) from A_sourceRef_outgoingConversationLinks
  * one-to-many link : (1-1) __ConversationLink__ need (0-inf) __InteractionNode__)
  * callable using find_with_related(__InteractionNodeModel__) from __ConversationLink__
  * named source_ref in BPMN

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __ConversationLink__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __ConversationLink__
  * saved in __super_base_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "ConversationLink",
//     name: "ConversationLink",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "ConversationLink-sourceRef",
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
//                     "A_sourceRef_outgoingConversationLinks",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ConversationLink-targetRef",
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
//                     "A_targetRef_incomingConversationLinks",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ConversationLink-name",
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }


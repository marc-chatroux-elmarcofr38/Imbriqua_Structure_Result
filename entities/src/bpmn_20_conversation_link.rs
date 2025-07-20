//! bpmn_20_class_conversation_link

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_conversation_link")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : BPMN20-ConversationLink-sourceRef
    pub source_ref: i64,
    /// COMPLEX FIELD : BPMN20-ConversationLink-targetRef
    pub target_ref: i64,
    /// SIMPLE FIELD : BPMN20-ConversationLink-name
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
    /// * __name__ (xmi_id : "BPMN20-ConversationLink-name")
    ///   * type : __Option<std::string::String>__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __Collaboration__ (__CollaborationModel__) from A_conversationLinks_collaboration
    ///   * one-to-many link : (1-1) __ConversationLink__ need (0-inf) __Collaboration__)
    ///   * callable using find_with_related(__CollaborationModel__) from __ConversationLink__
    ///   * named collaboration in BPMN
    /// * __InteractionNode__ (__InteractionNodeModel__) from A_sourceRef_outgoingConversationLinks
    ///   * one-to-many link : (1-1) __ConversationLink__ need (0-inf) __InteractionNode__)
    ///   * callable using find_with_related(__InteractionNodeModel__) from __ConversationLink__
    ///   * named source_ref in BPMN
    /// * __InteractionNode__ (__InteractionNodeModel__) from A_targetRef_incomingConversationLinks
    ///   * one-to-many link : (1-1) __ConversationLink__ need (0-inf) __InteractionNode__)
    ///   * callable using find_with_related(__InteractionNodeModel__) from __ConversationLink__
    ///   * named target_ref in BPMN
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
* __name__ (xmi_id : "BPMN20-ConversationLink-name")
  * type : __Option<std::string::String>__


## Relation : One To Many :
* __Collaboration__ (__CollaborationModel__) from A_conversationLinks_collaboration
  * one-to-many link : (1-1) __ConversationLink__ need (0-inf) __Collaboration__)
  * callable using find_with_related(__CollaborationModel__) from __ConversationLink__
  * named collaboration in BPMN
* __InteractionNode__ (__InteractionNodeModel__) from A_sourceRef_outgoingConversationLinks
  * one-to-many link : (1-1) __ConversationLink__ need (0-inf) __InteractionNode__)
  * callable using find_with_related(__InteractionNodeModel__) from __ConversationLink__
  * named source_ref in BPMN
* __InteractionNode__ (__InteractionNodeModel__) from A_targetRef_incomingConversationLinks
  * one-to-many link : (1-1) __ConversationLink__ need (0-inf) __InteractionNode__)
  * callable using find_with_related(__InteractionNodeModel__) from __ConversationLink__
  * named target_ref in BPMN

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
//     xmi_id: "Weak ref of "BPMN20-ConversationLink" (loaded : false)",
//     name: "ConversationLink",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-ConversationLink-name": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-ConversationLink-name" (loaded : false)",
//                 name: "name",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "Weak ref of "DC-String" (loaded : false)",
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
//         "-ConversationLink-sourceRef": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-ConversationLink-sourceRef" (loaded : false)",
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
//         "-ConversationLink-targetRef": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-ConversationLink-targetRef" (loaded : false)",
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
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#ConversationLink",
//     table_name: "bpmn_20_conversation_link",
//     model_name: "ConversationLink",
//     full_name: "bpmn_20_class_conversation_link",
// }


//! bpmn_20_class_conversation_association

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_conversation_association")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : ConversationAssociation-innerConversationNodeRef
    pub inner_conversation_node_ref: i64,
    /// COMPLEX FIELD : ConversationAssociation-outerConversationNodeRef
    pub outer_conversation_node_ref: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ConversationAssociation need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE ConversationAssociation need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ConversationAssociation" (bpmn_20_class_conversation_association)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __ConversationNode__ (__ConversationNodeModel__) from A_innerConversationNodeRef_conversationAssociation
    ///   * one-to-many link : (1-1) __ConversationAssociation__ need (0-inf) __ConversationNode__)
    ///   * callable using find_with_related(__ConversationNodeModel__) from __ConversationAssociation__
    /// * __ConversationNode__ (__ConversationNodeModel__) from A_outerConversationNodeRef_conversationAssociation
    ///   * one-to-many link : (1-1) __ConversationAssociation__ need (0-inf) __ConversationNode__)
    ///   * callable using find_with_related(__ConversationNodeModel__) from __ConversationAssociation__
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __ConversationAssociation__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __ConversationAssociation__
    ///   * saved in __super_base_element__ field as foreing key
    /// ## Reverse One To One :
    /// * __Collaboration__ (__CollaborationModel__) from A_conversationAssociations_converstaionAssociations
    ///   * one-to-one link : (1-1) __Collaboration__ need (1-1) __ConversationAssociation__)
    ///   * callable using find_also_related(__ConversationAssociationModel__) from __Collaboration__
    ///   * saved in __conversation_associations__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ConversationAssociation" (bpmn_20_class_conversation_association)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Relation : One To Many :
* __ConversationNode__ (__ConversationNodeModel__) from A_innerConversationNodeRef_conversationAssociation
  * one-to-many link : (1-1) __ConversationAssociation__ need (0-inf) __ConversationNode__)
  * callable using find_with_related(__ConversationNodeModel__) from __ConversationAssociation__
* __ConversationNode__ (__ConversationNodeModel__) from A_outerConversationNodeRef_conversationAssociation
  * one-to-many link : (1-1) __ConversationAssociation__ need (0-inf) __ConversationNode__)
  * callable using find_with_related(__ConversationNodeModel__) from __ConversationAssociation__

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __ConversationAssociation__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __ConversationAssociation__
  * saved in __super_base_element__ field as foreing key
## Reverse One To One :
* __Collaboration__ (__CollaborationModel__) from A_conversationAssociations_converstaionAssociations
  * one-to-one link : (1-1) __Collaboration__ need (1-1) __ConversationAssociation__)
  * callable using find_also_related(__ConversationAssociationModel__) from __Collaboration__
  * saved in __conversation_associations__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "ConversationAssociation",
//     name: "ConversationAssociation",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "ConversationAssociation-innerConversationNodeRef",
//                 name: "innerConversationNodeRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ConversationNode",
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
//                     "A_innerConversationNodeRef_conversationAssociation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ConversationAssociation-outerConversationNodeRef",
//                 name: "outerConversationNodeRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ConversationNode",
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
//                     "A_outerConversationNodeRef_conversationAssociation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }


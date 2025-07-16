//! bpmn_20_class_message_flow_association

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_message_flow_association")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : MessageFlowAssociation-innerMessageFlowRef
    pub inner_message_flow_ref: i64,
    /// COMPLEX FIELD : MessageFlowAssociation-outerMessageFlowRef
    pub outer_message_flow_ref: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE MessageFlowAssociation need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE MessageFlowAssociation need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "MessageFlowAssociation" (bpmn_20_class_message_flow_association)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __MessageFlow__ (__MessageFlowModel__) from A_innerMessageFlowRef_messageFlowAssociation
    ///   * one-to-many link : (1-1) __MessageFlowAssociation__ need (0-inf) __MessageFlow__)
    ///   * callable using find_with_related(__MessageFlowModel__) from __MessageFlowAssociation__
    /// * __Collaboration__ (__CollaborationModel__) from A_messageFlowAssociations_collaboration
    ///   * one-to-many link : (1-1) __MessageFlowAssociation__ need (0-inf) __Collaboration__)
    ///   * callable using find_with_related(__CollaborationModel__) from __MessageFlowAssociation__
    ///   * named collaboration in BPMN
    /// * __MessageFlow__ (__MessageFlowModel__) from A_outerMessageFlowRef_messageFlowAssociation
    ///   * one-to-many link : (1-1) __MessageFlowAssociation__ need (0-inf) __MessageFlow__)
    ///   * callable using find_with_related(__MessageFlowModel__) from __MessageFlowAssociation__
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __MessageFlowAssociation__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __MessageFlowAssociation__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "MessageFlowAssociation" (bpmn_20_class_message_flow_association)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Relation : One To Many :
* __MessageFlow__ (__MessageFlowModel__) from A_innerMessageFlowRef_messageFlowAssociation
  * one-to-many link : (1-1) __MessageFlowAssociation__ need (0-inf) __MessageFlow__)
  * callable using find_with_related(__MessageFlowModel__) from __MessageFlowAssociation__
* __Collaboration__ (__CollaborationModel__) from A_messageFlowAssociations_collaboration
  * one-to-many link : (1-1) __MessageFlowAssociation__ need (0-inf) __Collaboration__)
  * callable using find_with_related(__CollaborationModel__) from __MessageFlowAssociation__
  * named collaboration in BPMN
* __MessageFlow__ (__MessageFlowModel__) from A_outerMessageFlowRef_messageFlowAssociation
  * one-to-many link : (1-1) __MessageFlowAssociation__ need (0-inf) __MessageFlow__)
  * callable using find_with_related(__MessageFlowModel__) from __MessageFlowAssociation__

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __MessageFlowAssociation__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __MessageFlowAssociation__
  * saved in __super_base_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "MessageFlowAssociation",
//     name: "MessageFlowAssociation",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "MessageFlowAssociation-innerMessageFlowRef": Property(
//             CMOFProperty {
//                 xmi_id: "MessageFlowAssociation-innerMessageFlowRef",
//                 name: "innerMessageFlowRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "MessageFlow",
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
//                     "A_innerMessageFlowRef_messageFlowAssociation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "MessageFlowAssociation-outerMessageFlowRef": Property(
//             CMOFProperty {
//                 xmi_id: "MessageFlowAssociation-outerMessageFlowRef",
//                 name: "outerMessageFlowRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "MessageFlow",
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
//                     "A_outerMessageFlowRef_messageFlowAssociation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#MessageFlowAssociation",
//     table_name: "bpmn_20_message_flow_association",
//     model_name: "MessageFlowAssociation",
//     full_name: "bpmn_20_class_message_flow_association",
// }


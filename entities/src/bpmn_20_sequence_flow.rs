//! bpmn_20_class_sequence_flow

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_sequence_flow")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : FlowElement
    pub super_flow_element: i64,
    /// COMPLEX FIELD : BPMN20-SequenceFlow-conditionExpression
    pub condition_expression: Option<i64>,
    /// COMPLEX FIELD : BPMN20-SequenceFlow-sourceRef
    pub source_ref: i64,
    /// COMPLEX FIELD : BPMN20-SequenceFlow-targetRef
    pub target_ref: i64,
    /// SIMPLE FIELD : BPMN20-SequenceFlow-isImmediate
    pub is_immediate: Option<std::primitive::bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE SequenceFlow need ONE FlowElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_element::Entity",
        from = "Column::SuperFlowElement",
        to = "super::bpmn_20_flow_element::Column::Id",
        on_delete = "Cascade"
    )]
    FlowElement,
}

// SUPER : ONE SequenceFlow need ONE FlowElement
impl Related<super::bpmn_20_flow_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "SequenceFlow" (bpmn_20_class_sequence_flow)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __is_immediate__ (xmi_id : "BPMN20-SequenceFlow-isImmediate")
    ///   * type : __Option<std::primitive::bool>__
    /// 
    /// ## Direct One To One :
    /// * __Expression__ (__ExpressionModel__) from A_conditionExpression_sequenceFlow
    ///   * one-to-one link : (0-1) __SequenceFlow__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __SequenceFlow__
    ///   * saved in __condition_expression__ field as foreing key
    /// 
    /// ## Relation : One To Many :
    /// * __FlowNode__ (__FlowNodeModel__) from A_sourceRef_outgoing_flow
    ///   * one-to-many link : (1-1) __SequenceFlow__ need (0-inf) __FlowNode__)
    ///   * callable using find_with_related(__FlowNodeModel__) from __SequenceFlow__
    ///   * named source_ref in BPMN
    /// * __FlowNode__ (__FlowNodeModel__) from A_targetRef_incoming_flow
    ///   * one-to-many link : (1-1) __SequenceFlow__ need (0-inf) __FlowNode__)
    ///   * callable using find_with_related(__FlowNodeModel__) from __SequenceFlow__
    ///   * named target_ref in BPMN
    /// 
    /// ## Direct Super :
    /// * __FlowElement__ (__FlowElementModel__)
    ///   * one-to-one link : one __SequenceFlow__ need one __FlowElement__)
    ///   * callable using find_also_related(__FlowElementModel__) from __SequenceFlow__
    ///   * saved in __super_flow_element__ field as foreing key
    /// ## Reverse One To One :
    /// * __Activity__ (__ActivityModel__) from A_default_activity
    ///   * one-to-one link : (0-1) __Activity__ need (1-1) __SequenceFlow__)
    ///   * callable using find_also_related(__SequenceFlowModel__) from __Activity__
    ///   * saved in __default__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "SequenceFlow" (bpmn_20_class_sequence_flow)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __is_immediate__ (xmi_id : "BPMN20-SequenceFlow-isImmediate")
  * type : __Option<std::primitive::bool>__

## Direct One To One :
* __Expression__ (__ExpressionModel__) from A_conditionExpression_sequenceFlow
  * one-to-one link : (0-1) __SequenceFlow__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __SequenceFlow__
  * saved in __condition_expression__ field as foreing key

## Relation : One To Many :
* __FlowNode__ (__FlowNodeModel__) from A_sourceRef_outgoing_flow
  * one-to-many link : (1-1) __SequenceFlow__ need (0-inf) __FlowNode__)
  * callable using find_with_related(__FlowNodeModel__) from __SequenceFlow__
  * named source_ref in BPMN
* __FlowNode__ (__FlowNodeModel__) from A_targetRef_incoming_flow
  * one-to-many link : (1-1) __SequenceFlow__ need (0-inf) __FlowNode__)
  * callable using find_with_related(__FlowNodeModel__) from __SequenceFlow__
  * named target_ref in BPMN

## Direct Super :
* __FlowElement__ (__FlowElementModel__)
  * one-to-one link : one __SequenceFlow__ need one __FlowElement__)
  * callable using find_also_related(__FlowElementModel__) from __SequenceFlow__
  * saved in __super_flow_element__ field as foreing key
## Reverse One To One :
* __Activity__ (__ActivityModel__) from A_default_activity
  * one-to-one link : (0-1) __Activity__ need (1-1) __SequenceFlow__)
  * callable using find_also_related(__SequenceFlowModel__) from __Activity__
  * saved in __default__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "SequenceFlow",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "SequenceFlow",
//     is_abstract: false,
//     super_class: [
//         "FlowElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-SequenceFlow-conditionExpression": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "SequenceFlow-conditionExpression",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "conditionExpression",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Expression",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
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
//                     "A_conditionExpression_sequenceFlow",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-SequenceFlow-isImmediate": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "SequenceFlow-isImmediate",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "isImmediate",
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
//         "-SequenceFlow-sourceRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "SequenceFlow-sourceRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "sourceRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "FlowNode",
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
//                     "A_sourceRef_outgoing_flow",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-SequenceFlow-targetRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "SequenceFlow-targetRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "targetRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "FlowNode",
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
//                     "A_targetRef_incoming_flow",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#SequenceFlow",
//     table_name: "bpmn_20_sequence_flow",
//     model_name: "SequenceFlow",
//     full_name: "bpmn_20_class_sequence_flow",
// }


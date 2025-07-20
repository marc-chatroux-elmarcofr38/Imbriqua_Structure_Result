//! bpmn_20_class_activity

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_activity")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : FlowNode
    pub super_flow_node: i64,
    /// COMPLEX FIELD : BPMN20-Activity-default
    pub default: Option<i64>,
    /// COMPLEX FIELD : BPMN20-Activity-ioSpecification
    pub io_specification: Option<i64>,
    /// COMPLEX FIELD : BPMN20-Activity-loopCharacteristics
    pub loop_characteristics: Option<i64>,
    /// SIMPLE FIELD : BPMN20-Activity-completionQuantity
    #[sea_orm(default_value = "1")]
    pub completion_quantity: std::primitive::u64,
    /// SIMPLE FIELD : BPMN20-Activity-isForCompensation
    #[sea_orm(default_value = "false")]
    pub is_for_compensation: std::primitive::bool,
    /// SIMPLE FIELD : BPMN20-Activity-startQuantity
    #[sea_orm(default_value = "1")]
    pub start_quantity: std::primitive::u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Activity need ONE FlowNode
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_node::Entity",
        from = "Column::SuperFlowNode",
        to = "super::bpmn_20_flow_node::Column::Id",
        on_delete = "Cascade"
    )]
    FlowNode,
    // SUPER : ONE CallActivity need ONE Activity
    #[sea_orm(has_one = "super::bpmn_20_call_activity::Entity")]
    CallActivity,
    // SUPER : ONE SubProcess need ONE Activity
    #[sea_orm(has_one = "super::bpmn_20_sub_process::Entity")]
    SubProcess,
    // SUPER : ONE Task need ONE Activity
    #[sea_orm(has_one = "super::bpmn_20_task::Entity")]
    Task,
}

// SUPER : ONE Activity need ONE FlowNode
impl Related<super::bpmn_20_flow_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowNode.def()
    }
}

// SUPER : ONE CallActivity need ONE Activity
impl Related<super::bpmn_20_call_activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CallActivity.def()
    }
}

// SUPER : ONE SubProcess need ONE Activity
impl Related<super::bpmn_20_sub_process::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubProcess.def()
    }
}

// SUPER : ONE Task need ONE Activity
impl Related<super::bpmn_20_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Activity" (bpmn_20_class_activity)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __completion_quantity__ (xmi_id : "BPMN20-Activity-completionQuantity")
    ///   * type : __std::primitive::u64__
    ///   * default : "1"
    /// * __is_for_compensation__ (xmi_id : "BPMN20-Activity-isForCompensation")
    ///   * type : __std::primitive::bool__
    ///   * default : "false"
    /// * __start_quantity__ (xmi_id : "BPMN20-Activity-startQuantity")
    ///   * type : __std::primitive::u64__
    ///   * default : "1"
    /// 
    /// ## Direct One To One :
    /// * __SequenceFlow__ (__SequenceFlowModel__) from A_default_activity
    ///   * one-to-one link : (0-1) __Activity__ need (1-1) __SequenceFlow__)
    ///   * callable using find_also_related(__SequenceFlowModel__) from __Activity__
    ///   * saved in __default__ field as foreing key
    /// * __InputOutputSpecification__ (__InputOutputSpecificationModel__) from A_ioSpecification_activity
    ///   * one-to-one link : (0-1) __Activity__ need (0-1) __InputOutputSpecification__)
    ///   * callable using find_also_related(__InputOutputSpecificationModel__) from __Activity__
    ///   * saved in __io_specification__ field as foreing key
    /// * __LoopCharacteristics__ (__LoopCharacteristicsModel__) from A_loopCharacteristics_activity
    ///   * one-to-one link : (0-1) __Activity__ need (0-1) __LoopCharacteristics__)
    ///   * callable using find_also_related(__LoopCharacteristicsModel__) from __Activity__
    ///   * saved in __loop_characteristics__ field as foreing key
    /// 
    /// 
    /// ## Direct Super :
    /// * __FlowNode__ (__FlowNodeModel__)
    ///   * one-to-one link : one __Activity__ need one __FlowNode__)
    ///   * callable using find_also_related(__FlowNodeModel__) from __Activity__
    ///   * saved in __super_flow_node__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __CallActivity__ (__CallActivityModel__)
    ///   * one-to-one link (reverse) : one __CallActivity__ need one __Activity__)
    ///   * callable using find_also_related(__ActivityModel__) from __CallActivity__
    ///   * saved in __super_activity__ field as foreing key in __CallActivityModel__
    /// * __SubProcess__ (__SubProcessModel__)
    ///   * one-to-one link (reverse) : one __SubProcess__ need one __Activity__)
    ///   * callable using find_also_related(__ActivityModel__) from __SubProcess__
    ///   * saved in __super_activity__ field as foreing key in __SubProcessModel__
    /// * __Task__ (__TaskModel__)
    ///   * one-to-one link (reverse) : one __Task__ need one __Activity__)
    ///   * callable using find_also_related(__ActivityModel__) from __Task__
    ///   * saved in __super_activity__ field as foreing key in __TaskModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Activity" (bpmn_20_class_activity)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __completion_quantity__ (xmi_id : "BPMN20-Activity-completionQuantity")
  * type : __std::primitive::u64__
  * default : "1"
* __is_for_compensation__ (xmi_id : "BPMN20-Activity-isForCompensation")
  * type : __std::primitive::bool__
  * default : "false"
* __start_quantity__ (xmi_id : "BPMN20-Activity-startQuantity")
  * type : __std::primitive::u64__
  * default : "1"

## Direct One To One :
* __SequenceFlow__ (__SequenceFlowModel__) from A_default_activity
  * one-to-one link : (0-1) __Activity__ need (1-1) __SequenceFlow__)
  * callable using find_also_related(__SequenceFlowModel__) from __Activity__
  * saved in __default__ field as foreing key
* __InputOutputSpecification__ (__InputOutputSpecificationModel__) from A_ioSpecification_activity
  * one-to-one link : (0-1) __Activity__ need (0-1) __InputOutputSpecification__)
  * callable using find_also_related(__InputOutputSpecificationModel__) from __Activity__
  * saved in __io_specification__ field as foreing key
* __LoopCharacteristics__ (__LoopCharacteristicsModel__) from A_loopCharacteristics_activity
  * one-to-one link : (0-1) __Activity__ need (0-1) __LoopCharacteristics__)
  * callable using find_also_related(__LoopCharacteristicsModel__) from __Activity__
  * saved in __loop_characteristics__ field as foreing key


## Direct Super :
* __FlowNode__ (__FlowNodeModel__)
  * one-to-one link : one __Activity__ need one __FlowNode__)
  * callable using find_also_related(__FlowNodeModel__) from __Activity__
  * saved in __super_flow_node__ field as foreing key

## Reverse Super :
* __CallActivity__ (__CallActivityModel__)
  * one-to-one link (reverse) : one __CallActivity__ need one __Activity__)
  * callable using find_also_related(__ActivityModel__) from __CallActivity__
  * saved in __super_activity__ field as foreing key in __CallActivityModel__
* __SubProcess__ (__SubProcessModel__)
  * one-to-one link (reverse) : one __SubProcess__ need one __Activity__)
  * callable using find_also_related(__ActivityModel__) from __SubProcess__
  * saved in __super_activity__ field as foreing key in __SubProcessModel__
* __Task__ (__TaskModel__)
  * one-to-one link (reverse) : one __Task__ need one __Activity__)
  * callable using find_also_related(__ActivityModel__) from __Task__
  * saved in __super_activity__ field as foreing key in __TaskModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         object_id: "Activity",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "Activity",
//     is_abstract: true,
//     super_class: [
//         "FlowNode",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-Activity-boundaryEventRefs": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Activity-boundaryEventRefs",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "boundaryEventRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "BoundaryEvent",
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
//                     "A_boundaryEventRefs_attachedToRef",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Activity-completionQuantity": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Activity-completionQuantity",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "completionQuantity",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "DC.cmof#Integer",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: Some(
//                     "1",
//                 ),
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
//         "-Activity-dataInputAssociations": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Activity-dataInputAssociations",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "dataInputAssociations",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataInputAssociation",
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
//                     "A_dataInputAssociations_activity",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Activity-dataOutputAssociations": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Activity-dataOutputAssociations",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "dataOutputAssociations",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataOutputAssociation",
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
//                     "A_dataOutputAssociations_activity",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Activity-default": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Activity-default",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "default",
//                 visibility: Public,
//                 simple_type: Some(
//                     "SequenceFlow",
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
//                     "A_default_activity",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Activity-ioSpecification": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Activity-ioSpecification",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "ioSpecification",
//                 visibility: Public,
//                 simple_type: Some(
//                     "InputOutputSpecification",
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
//                     "A_ioSpecification_activity",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Activity-isForCompensation": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Activity-isForCompensation",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "isForCompensation",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "DC.cmof#Boolean",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: Some(
//                     "false",
//                 ),
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
//         "-Activity-loopCharacteristics": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Activity-loopCharacteristics",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "loopCharacteristics",
//                 visibility: Public,
//                 simple_type: Some(
//                     "LoopCharacteristics",
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
//                     "A_loopCharacteristics_activity",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Activity-properties": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Activity-properties",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "properties",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Property",
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
//                     "A_properties_activity",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Activity-resources": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Activity-resources",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "resources",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ResourceRole",
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
//                     "A_resources_activity",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Activity-startQuantity": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Activity-startQuantity",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "startQuantity",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "DC.cmof#Integer",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: Some(
//                     "1",
//                 ),
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
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Activity",
//     table_name: "bpmn_20_activity",
//     model_name: "Activity",
//     full_name: "bpmn_20_class_activity",
// }


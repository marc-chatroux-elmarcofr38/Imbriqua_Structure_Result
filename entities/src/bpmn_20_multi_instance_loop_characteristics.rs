//! bpmn_20_class_multi_instance_loop_characteristics

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_multi_instance_loop_characteristics")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : LoopCharacteristics
    pub super_loop_characteristics: i64,
    /// COMPLEX FIELD : BPMN20-MultiInstanceLoopCharacteristics-completionCondition
    pub completion_condition: Option<i64>,
    /// COMPLEX FIELD : BPMN20-MultiInstanceLoopCharacteristics-inputDataItem
    pub input_data_item: Option<i64>,
    /// COMPLEX FIELD : BPMN20-MultiInstanceLoopCharacteristics-loopCardinality
    pub loop_cardinality: Option<i64>,
    /// COMPLEX FIELD : BPMN20-MultiInstanceLoopCharacteristics-loopDataInputRef
    pub loop_data_input_ref: Option<i64>,
    /// COMPLEX FIELD : BPMN20-MultiInstanceLoopCharacteristics-loopDataOutputRef
    pub loop_data_output_ref: Option<i64>,
    /// COMPLEX FIELD : BPMN20-MultiInstanceLoopCharacteristics-noneBehaviorEventRef
    pub none_behavior_event_ref: Option<i64>,
    /// COMPLEX FIELD : BPMN20-MultiInstanceLoopCharacteristics-oneBehaviorEventRef
    pub one_behavior_event_ref: Option<i64>,
    /// COMPLEX FIELD : BPMN20-MultiInstanceLoopCharacteristics-outputDataItem
    pub output_data_item: Option<i64>,
    /// SIMPLE FIELD : BPMN20-MultiInstanceLoopCharacteristics-behavior
    #[sea_orm(default_value = "All")]
    pub behavior: MultiInstanceBehavior,
    /// SIMPLE FIELD : BPMN20-MultiInstanceLoopCharacteristics-isSequential
    #[sea_orm(default_value = "false")]
    pub is_sequential: std::primitive::bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE MultiInstanceLoopCharacteristics need ONE LoopCharacteristics
    #[sea_orm(
        belongs_to = "super::bpmn_20_loop_characteristics::Entity",
        from = "Column::SuperLoopCharacteristics",
        to = "super::bpmn_20_loop_characteristics::Column::Id",
        on_delete = "Cascade"
    )]
    LoopCharacteristics,
}

// SUPER : ONE MultiInstanceLoopCharacteristics need ONE LoopCharacteristics
impl Related<super::bpmn_20_loop_characteristics::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LoopCharacteristics.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "MultiInstanceLoopCharacteristics" (bpmn_20_class_multi_instance_loop_characteristics)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __behavior__ (xmi_id : "BPMN20-MultiInstanceLoopCharacteristics-behavior")
    ///   * type : __MultiInstanceBehavior__
    ///   * default : "All"
    /// * __is_sequential__ (xmi_id : "BPMN20-MultiInstanceLoopCharacteristics-isSequential")
    ///   * type : __std::primitive::bool__
    ///   * default : "false"
    /// 
    /// ## Direct One To One :
    /// * __Expression__ (__ExpressionModel__) from A_completionCondition_multiInstanceLoopCharacteristics
    ///   * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __MultiInstanceLoopCharacteristics__
    ///   * saved in __completion_condition__ field as foreing key
    /// * __DataInput__ (__DataInputModel__) from A_inputDataItem_multiInstanceLoopCharacteristics
    ///   * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __DataInput__)
    ///   * callable using find_also_related(__DataInputModel__) from __MultiInstanceLoopCharacteristics__
    ///   * saved in __input_data_item__ field as foreing key
    /// * __Expression__ (__ExpressionModel__) from A_loopCardinality_multiInstanceLoopCharacteristics
    ///   * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __MultiInstanceLoopCharacteristics__
    ///   * saved in __loop_cardinality__ field as foreing key
    /// * __ItemAwareElement__ (__ItemAwareElementModel__) from A_loopDataInputRef_multiInstanceLoopCharacteristics
    ///   * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __ItemAwareElement__)
    ///   * callable using find_also_related(__ItemAwareElementModel__) from __MultiInstanceLoopCharacteristics__
    ///   * saved in __loop_data_input_ref__ field as foreing key
    /// * __ItemAwareElement__ (__ItemAwareElementModel__) from A_loopDataOutputRef_multiInstanceLoopCharacteristics
    ///   * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __ItemAwareElement__)
    ///   * callable using find_also_related(__ItemAwareElementModel__) from __MultiInstanceLoopCharacteristics__
    ///   * saved in __loop_data_output_ref__ field as foreing key
    /// * __DataOutput__ (__DataOutputModel__) from A_outputDataItem_multiInstanceLoopCharacteristics
    ///   * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __DataOutput__)
    ///   * callable using find_also_related(__DataOutputModel__) from __MultiInstanceLoopCharacteristics__
    ///   * saved in __output_data_item__ field as foreing key
    /// 
    /// ## Relation : One To Many :
    /// * __EventDefinition__ (__EventDefinitionModel__) from A_noneBehaviorEventRef_multiInstanceLoopCharacteristics
    ///   * one-to-many link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-inf) __EventDefinition__)
    ///   * callable using find_with_related(__EventDefinitionModel__) from __MultiInstanceLoopCharacteristics__
    /// * __EventDefinition__ (__EventDefinitionModel__) from A_oneBehaviorEventRef_multiInstanceLoopCharacteristics
    ///   * one-to-many link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-inf) __EventDefinition__)
    ///   * callable using find_with_related(__EventDefinitionModel__) from __MultiInstanceLoopCharacteristics__
    /// 
    /// ## Direct Super :
    /// * __LoopCharacteristics__ (__LoopCharacteristicsModel__)
    ///   * one-to-one link : one __MultiInstanceLoopCharacteristics__ need one __LoopCharacteristics__)
    ///   * callable using find_also_related(__LoopCharacteristicsModel__) from __MultiInstanceLoopCharacteristics__
    ///   * saved in __super_loop_characteristics__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "MultiInstanceLoopCharacteristics" (bpmn_20_class_multi_instance_loop_characteristics)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __behavior__ (xmi_id : "BPMN20-MultiInstanceLoopCharacteristics-behavior")
  * type : __MultiInstanceBehavior__
  * default : "All"
* __is_sequential__ (xmi_id : "BPMN20-MultiInstanceLoopCharacteristics-isSequential")
  * type : __std::primitive::bool__
  * default : "false"

## Direct One To One :
* __Expression__ (__ExpressionModel__) from A_completionCondition_multiInstanceLoopCharacteristics
  * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __MultiInstanceLoopCharacteristics__
  * saved in __completion_condition__ field as foreing key
* __DataInput__ (__DataInputModel__) from A_inputDataItem_multiInstanceLoopCharacteristics
  * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __DataInput__)
  * callable using find_also_related(__DataInputModel__) from __MultiInstanceLoopCharacteristics__
  * saved in __input_data_item__ field as foreing key
* __Expression__ (__ExpressionModel__) from A_loopCardinality_multiInstanceLoopCharacteristics
  * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __MultiInstanceLoopCharacteristics__
  * saved in __loop_cardinality__ field as foreing key
* __ItemAwareElement__ (__ItemAwareElementModel__) from A_loopDataInputRef_multiInstanceLoopCharacteristics
  * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __ItemAwareElement__)
  * callable using find_also_related(__ItemAwareElementModel__) from __MultiInstanceLoopCharacteristics__
  * saved in __loop_data_input_ref__ field as foreing key
* __ItemAwareElement__ (__ItemAwareElementModel__) from A_loopDataOutputRef_multiInstanceLoopCharacteristics
  * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __ItemAwareElement__)
  * callable using find_also_related(__ItemAwareElementModel__) from __MultiInstanceLoopCharacteristics__
  * saved in __loop_data_output_ref__ field as foreing key
* __DataOutput__ (__DataOutputModel__) from A_outputDataItem_multiInstanceLoopCharacteristics
  * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __DataOutput__)
  * callable using find_also_related(__DataOutputModel__) from __MultiInstanceLoopCharacteristics__
  * saved in __output_data_item__ field as foreing key

## Relation : One To Many :
* __EventDefinition__ (__EventDefinitionModel__) from A_noneBehaviorEventRef_multiInstanceLoopCharacteristics
  * one-to-many link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-inf) __EventDefinition__)
  * callable using find_with_related(__EventDefinitionModel__) from __MultiInstanceLoopCharacteristics__
* __EventDefinition__ (__EventDefinitionModel__) from A_oneBehaviorEventRef_multiInstanceLoopCharacteristics
  * one-to-many link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-inf) __EventDefinition__)
  * callable using find_with_related(__EventDefinitionModel__) from __MultiInstanceLoopCharacteristics__

## Direct Super :
* __LoopCharacteristics__ (__LoopCharacteristicsModel__)
  * one-to-one link : one __MultiInstanceLoopCharacteristics__ need one __LoopCharacteristics__)
  * callable using find_also_related(__LoopCharacteristicsModel__) from __MultiInstanceLoopCharacteristics__
  * saved in __super_loop_characteristics__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "MultiInstanceLoopCharacteristics",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "MultiInstanceLoopCharacteristics",
//     is_abstract: false,
//     super_class: [
//         "LoopCharacteristics",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-MultiInstanceLoopCharacteristics-behavior": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "MultiInstanceLoopCharacteristics-behavior",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "behavior",
//                 visibility: Public,
//                 simple_type: Some(
//                     "MultiInstanceBehavior",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: Some(
//                     "All",
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
//         "-MultiInstanceLoopCharacteristics-completionCondition": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "MultiInstanceLoopCharacteristics-completionCondition",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "completionCondition",
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
//                     "A_completionCondition_multiInstanceLoopCharacteristics",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-MultiInstanceLoopCharacteristics-complexBehaviorDefinition": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "MultiInstanceLoopCharacteristics-complexBehaviorDefinition",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "complexBehaviorDefinition",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ComplexBehaviorDefinition",
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
//                     "A_complexBehaviorDefinition_multiInstanceLoopCharacteristics",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-MultiInstanceLoopCharacteristics-inputDataItem": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "MultiInstanceLoopCharacteristics-inputDataItem",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "inputDataItem",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataInput",
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
//                     "A_inputDataItem_multiInstanceLoopCharacteristics",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-MultiInstanceLoopCharacteristics-isSequential": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "MultiInstanceLoopCharacteristics-isSequential",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "isSequential",
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
//         "-MultiInstanceLoopCharacteristics-loopCardinality": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "MultiInstanceLoopCharacteristics-loopCardinality",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "loopCardinality",
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
//                     "A_loopCardinality_multiInstanceLoopCharacteristics",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-MultiInstanceLoopCharacteristics-loopDataInputRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "MultiInstanceLoopCharacteristics-loopDataInputRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "loopDataInputRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ItemAwareElement",
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
//                     "A_loopDataInputRef_multiInstanceLoopCharacteristics",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-MultiInstanceLoopCharacteristics-loopDataOutputRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "MultiInstanceLoopCharacteristics-loopDataOutputRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "loopDataOutputRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ItemAwareElement",
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
//                     "A_loopDataOutputRef_multiInstanceLoopCharacteristics",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-MultiInstanceLoopCharacteristics-noneBehaviorEventRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "MultiInstanceLoopCharacteristics-noneBehaviorEventRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "noneBehaviorEventRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "EventDefinition",
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
//                     "A_noneBehaviorEventRef_multiInstanceLoopCharacteristics",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-MultiInstanceLoopCharacteristics-oneBehaviorEventRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "MultiInstanceLoopCharacteristics-oneBehaviorEventRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "oneBehaviorEventRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "EventDefinition",
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
//                     "A_oneBehaviorEventRef_multiInstanceLoopCharacteristics",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-MultiInstanceLoopCharacteristics-outputDataItem": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "MultiInstanceLoopCharacteristics-outputDataItem",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "outputDataItem",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataOutput",
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
//                     "A_outputDataItem_multiInstanceLoopCharacteristics",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#MultiInstanceLoopCharacteristics",
//     table_name: "bpmn_20_multi_instance_loop_characteristics",
//     model_name: "MultiInstanceLoopCharacteristics",
//     full_name: "bpmn_20_class_multi_instance_loop_characteristics",
// }


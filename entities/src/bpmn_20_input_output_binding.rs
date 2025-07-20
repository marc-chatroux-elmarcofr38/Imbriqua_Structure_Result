//! bpmn_20_class_input_output_binding

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_input_output_binding")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// COMPLEX FIELD : BPMN20-InputOutputBinding-inputDataRef
    pub input_data_ref: i64,
    /// COMPLEX FIELD : BPMN20-InputOutputBinding-operationRef
    pub operation_ref: i64,
    /// COMPLEX FIELD : BPMN20-InputOutputBinding-outputDataRef
    pub output_data_ref: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "InputOutputBinding" (bpmn_20_class_input_output_binding)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __InputSet__ (__InputSetModel__) from A_inputDataRef_inputOutputBinding
    ///   * one-to-many link : (1-1) __InputOutputBinding__ need (0-inf) __InputSet__)
    ///   * callable using find_with_related(__InputSetModel__) from __InputOutputBinding__
    /// * __CallableElement__ (__CallableElementModel__) from A_ioBinding_callableElement
    ///   * one-to-many link : (0-1) __InputOutputBinding__ need (0-inf) __CallableElement__)
    ///   * callable using find_with_related(__CallableElementModel__) from __InputOutputBinding__
    ///   * named callable_element in BPMN
    /// * __Operation__ (__OperationModel__) from A_operationRef_ioBinding
    ///   * one-to-many link : (1-1) __InputOutputBinding__ need (0-inf) __Operation__)
    ///   * callable using find_with_related(__OperationModel__) from __InputOutputBinding__
    /// * __OutputSet__ (__OutputSetModel__) from A_outputDataRef_inputOutputBinding
    ///   * one-to-many link : (1-1) __InputOutputBinding__ need (0-inf) __OutputSet__)
    ///   * callable using find_with_related(__OutputSetModel__) from __InputOutputBinding__
    /// 
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "InputOutputBinding" (bpmn_20_class_input_output_binding)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Relation : One To Many :
* __InputSet__ (__InputSetModel__) from A_inputDataRef_inputOutputBinding
  * one-to-many link : (1-1) __InputOutputBinding__ need (0-inf) __InputSet__)
  * callable using find_with_related(__InputSetModel__) from __InputOutputBinding__
* __CallableElement__ (__CallableElementModel__) from A_ioBinding_callableElement
  * one-to-many link : (0-1) __InputOutputBinding__ need (0-inf) __CallableElement__)
  * callable using find_with_related(__CallableElementModel__) from __InputOutputBinding__
  * named callable_element in BPMN
* __Operation__ (__OperationModel__) from A_operationRef_ioBinding
  * one-to-many link : (1-1) __InputOutputBinding__ need (0-inf) __Operation__)
  * callable using find_with_related(__OperationModel__) from __InputOutputBinding__
* __OutputSet__ (__OutputSetModel__) from A_outputDataRef_inputOutputBinding
  * one-to-many link : (1-1) __InputOutputBinding__ need (0-inf) __OutputSet__)
  * callable using find_with_related(__OutputSetModel__) from __InputOutputBinding__



"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         object_id: "InputOutputBinding",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "InputOutputBinding",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: {
//         "-InputOutputBinding-inputDataRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "InputOutputBinding-inputDataRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "inputDataRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "InputSet",
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
//                     "A_inputDataRef_inputOutputBinding",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-InputOutputBinding-operationRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "InputOutputBinding-operationRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "operationRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Operation",
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
//                     "A_operationRef_ioBinding",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-InputOutputBinding-outputDataRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "InputOutputBinding-outputDataRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "outputDataRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "OutputSet",
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
//                     "A_outputDataRef_inputOutputBinding",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#InputOutputBinding",
//     table_name: "bpmn_20_input_output_binding",
//     model_name: "InputOutputBinding",
//     full_name: "bpmn_20_class_input_output_binding",
// }


//! bpmn_20_class_data_input

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_data_input")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : ItemAwareElement
    pub super_item_aware_element: i64,
    /// SIMPLE FIELD : BPMN20-DataInput-isCollection
    #[sea_orm(default_value = "false")]
    pub is_collection: std::primitive::bool,
    /// SIMPLE FIELD : BPMN20-DataInput-name
    pub name: Option<std::string::String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE DataInput need ONE ItemAwareElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_item_aware_element::Entity",
        from = "Column::SuperItemAwareElement",
        to = "super::bpmn_20_item_aware_element::Column::Id",
        on_delete = "Cascade"
    )]
    ItemAwareElement,
}

// SUPER : ONE DataInput need ONE ItemAwareElement
impl Related<super::bpmn_20_item_aware_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemAwareElement.def()
    }
}

// ManyToMany : with InputSet using A_dataInputRefs_inputSetRefs
impl Related<super::bpmn_20_a_data_input_refs_input_set_refs::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_data_input_refs_input_set_refs::Relation::InputSet.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_data_input_refs_input_set_refs::Relation::DataInput
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with InputSet using A_optionalInputRefs_inputSetWithOptional
impl Related<super::bpmn_20_a_optional_input_refs_input_set_with_optional::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_optional_input_refs_input_set_with_optional::Relation::InputSet.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_optional_input_refs_input_set_with_optional::Relation::DataInput
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with InputSet using A_whileExecutingInputRefs_inputSetWithWhileExecuting
impl Related<super::bpmn_20_a_while_executing_input_refs_input_set_with_while_executing::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_while_executing_input_refs_input_set_with_while_executing::Relation::InputSet.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_while_executing_input_refs_input_set_with_while_executing::Relation::DataInput
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "DataInput" (bpmn_20_class_data_input)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __is_collection__ (xmi_id : "BPMN20-DataInput-isCollection")
    ///   * type : __std::primitive::bool__
    ///   * default : "false"
    /// * __name__ (xmi_id : "BPMN20-DataInput-name")
    ///   * type : __Option<std::string::String>__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __InputOutputSpecification__ (__InputOutputSpecificationModel__) from A_dataInputs_inputOutputSpecification
    ///   * one-to-many link : (1-1) __DataInput__ need (0-inf) __InputOutputSpecification__)
    ///   * callable using find_with_related(__InputOutputSpecificationModel__) from __DataInput__
    ///   * named input_output_specification in BPMN
    /// * __ThrowEvent__ (__ThrowEventModel__) from A_dataInputs_throwEvent
    ///   * one-to-many link : (0-1) __DataInput__ need (0-inf) __ThrowEvent__)
    ///   * callable using find_with_related(__ThrowEventModel__) from __DataInput__
    ///   * named throw_event in BPMN
    /// 
    /// ## Direct Super :
    /// * __ItemAwareElement__ (__ItemAwareElementModel__)
    ///   * one-to-one link : one __DataInput__ need one __ItemAwareElement__)
    ///   * callable using find_also_related(__ItemAwareElementModel__) from __DataInput__
    ///   * saved in __super_item_aware_element__ field as foreing key
    /// ## Reverse One To One :
    /// * __MultiInstanceLoopCharacteristics__ (__MultiInstanceLoopCharacteristicsModel__) from A_inputDataItem_multiInstanceLoopCharacteristics
    ///   * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __DataInput__)
    ///   * callable using find_also_related(__DataInputModel__) from __MultiInstanceLoopCharacteristics__
    ///   * saved in __input_data_item__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "DataInput" (bpmn_20_class_data_input)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __is_collection__ (xmi_id : "BPMN20-DataInput-isCollection")
  * type : __std::primitive::bool__
  * default : "false"
* __name__ (xmi_id : "BPMN20-DataInput-name")
  * type : __Option<std::string::String>__


## Relation : One To Many :
* __InputOutputSpecification__ (__InputOutputSpecificationModel__) from A_dataInputs_inputOutputSpecification
  * one-to-many link : (1-1) __DataInput__ need (0-inf) __InputOutputSpecification__)
  * callable using find_with_related(__InputOutputSpecificationModel__) from __DataInput__
  * named input_output_specification in BPMN
* __ThrowEvent__ (__ThrowEventModel__) from A_dataInputs_throwEvent
  * one-to-many link : (0-1) __DataInput__ need (0-inf) __ThrowEvent__)
  * callable using find_with_related(__ThrowEventModel__) from __DataInput__
  * named throw_event in BPMN

## Direct Super :
* __ItemAwareElement__ (__ItemAwareElementModel__)
  * one-to-one link : one __DataInput__ need one __ItemAwareElement__)
  * callable using find_also_related(__ItemAwareElementModel__) from __DataInput__
  * saved in __super_item_aware_element__ field as foreing key
## Reverse One To One :
* __MultiInstanceLoopCharacteristics__ (__MultiInstanceLoopCharacteristicsModel__) from A_inputDataItem_multiInstanceLoopCharacteristics
  * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __DataInput__)
  * callable using find_also_related(__DataInputModel__) from __MultiInstanceLoopCharacteristics__
  * saved in __input_data_item__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdLocalReference {
//         object_id: "DataInput",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "DataInput",
//     is_abstract: false,
//     super_class: [
//         "ItemAwareElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "DataInput-inputSetRefs": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "DataInput-inputSetRefs",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "inputSetRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "InputSet",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: true,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_dataInputRefs_inputSetRefs",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "DataInput-inputSetWithOptional": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "DataInput-inputSetWithOptional",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "inputSetWithOptional",
//                 visibility: Public,
//                 simple_type: Some(
//                     "InputSet",
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
//                 is_derived: true,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_optionalInputRefs_inputSetWithOptional",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "DataInput-inputSetWithWhileExecuting": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "DataInput-inputSetWithWhileExecuting",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "inputSetWithWhileExecuting",
//                 visibility: Public,
//                 simple_type: Some(
//                     "InputSet",
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
//                 is_derived: true,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_whileExecutingInputRefs_inputSetWithWhileExecuting",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "DataInput-isCollection": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "DataInput-isCollection",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "isCollection",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "Weak ref of 'DC-Boolean' (loaded : true)",
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
//         "DataInput-name": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "DataInput-name",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "name",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "Weak ref of 'DC-String' (loaded : true)",
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
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#DataInput",
//     table_name: "bpmn_20_data_input",
//     model_name: "DataInput",
//     full_name: "bpmn_20_class_data_input",
// }


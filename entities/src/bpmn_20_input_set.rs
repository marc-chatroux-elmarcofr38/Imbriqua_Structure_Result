//! bpmn_20_class_input_set

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_input_set")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// SIMPLE FIELD : BPMN20-InputSet-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE InputSet need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE InputSet need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// ManyToMany : with DataInput using A_dataInputRefs_inputSetRefs
impl Related<super::bpmn_20_a_data_input_refs_input_set_refs::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_data_input_refs_input_set_refs::Relation::DataInput.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_data_input_refs_input_set_refs::Relation::InputSet
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with OutputSet using A_inputSetRefs_outputSetRefs
impl Related<super::bpmn_20_a_input_set_refs_output_set_refs::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_input_set_refs_output_set_refs::Relation::OutputSet.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_input_set_refs_output_set_refs::Relation::InputSet
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with DataInput using A_optionalInputRefs_inputSetWithOptional
impl Related<super::bpmn_20_a_optional_input_refs_input_set_with_optional::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_optional_input_refs_input_set_with_optional::Relation::DataInput.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_optional_input_refs_input_set_with_optional::Relation::InputSet
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with DataInput using A_whileExecutingInputRefs_inputSetWithWhileExecuting
impl Related<super::bpmn_20_a_while_executing_input_refs_input_set_with_while_executing::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_while_executing_input_refs_input_set_with_while_executing::Relation::DataInput.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_while_executing_input_refs_input_set_with_while_executing::Relation::InputSet
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "InputSet" (bpmn_20_class_input_set)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "BPMN20-InputSet-name")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __InputOutputSpecification__ (__InputOutputSpecificationModel__) from A_inputSets_inputOutputSpecification
    ///   * one-to-many link : (1-1) __InputSet__ need (1-inf) __InputOutputSpecification__)
    ///   * callable using find_with_related(__InputOutputSpecificationModel__) from __InputSet__
    ///   * named input_output_specification in BPMN
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __InputSet__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __InputSet__
    ///   * saved in __super_base_element__ field as foreing key
    /// ## Reverse One To One :
    /// * __ThrowEvent__ (__ThrowEventModel__) from A_inputSet_throwEvent
    ///   * one-to-one link : (0-1) __ThrowEvent__ need (0-1) __InputSet__)
    ///   * callable using find_also_related(__InputSetModel__) from __ThrowEvent__
    ///   * saved in __input_set__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "InputSet" (bpmn_20_class_input_set)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "BPMN20-InputSet-name")
  * type : __std::string::String__


## Relation : One To Many :
* __InputOutputSpecification__ (__InputOutputSpecificationModel__) from A_inputSets_inputOutputSpecification
  * one-to-many link : (1-1) __InputSet__ need (1-inf) __InputOutputSpecification__)
  * callable using find_with_related(__InputOutputSpecificationModel__) from __InputSet__
  * named input_output_specification in BPMN

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __InputSet__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __InputSet__
  * saved in __super_base_element__ field as foreing key
## Reverse One To One :
* __ThrowEvent__ (__ThrowEventModel__) from A_inputSet_throwEvent
  * one-to-one link : (0-1) __ThrowEvent__ need (0-1) __InputSet__)
  * callable using find_also_related(__InputSetModel__) from __ThrowEvent__
  * saved in __input_set__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         object_id: "InputSet",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "InputSet",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-InputSet-dataInputRefs": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "InputSet-dataInputRefs",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "dataInputRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataInput",
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
//                     "A_dataInputRefs_inputSetRefs",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-InputSet-name": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "InputSet-name",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "name",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
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
//         "-InputSet-optionalInputRefs": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "InputSet-optionalInputRefs",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "optionalInputRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataInput",
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
//                     "A_optionalInputRefs_inputSetWithOptional",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-InputSet-outputSetRefs": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "InputSet-outputSetRefs",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "outputSetRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "OutputSet",
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
//                     "A_inputSetRefs_outputSetRefs",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-InputSet-whileExecutingInputRefs": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "InputSet-whileExecutingInputRefs",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "whileExecutingInputRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataInput",
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
//                     "A_whileExecutingInputRefs_inputSetWithWhileExecuting",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#InputSet",
//     table_name: "bpmn_20_input_set",
//     model_name: "InputSet",
//     full_name: "bpmn_20_class_input_set",
// }


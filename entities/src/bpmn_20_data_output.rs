//! bpmn_20_class_data_output

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_data_output")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : ItemAwareElement
    pub super_item_aware_element: i64,
    /// SIMPLE FIELD : BPMN20-DataOutput-isCollection
    #[sea_orm(default_value = "false")]
    pub is_collection: std::primitive::bool,
    /// SIMPLE FIELD : BPMN20-DataOutput-name
    pub name: Option<std::string::String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE DataOutput need ONE ItemAwareElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_item_aware_element::Entity",
        from = "Column::SuperItemAwareElement",
        to = "super::bpmn_20_item_aware_element::Column::Id",
        on_delete = "Cascade"
    )]
    ItemAwareElement,
}

// SUPER : ONE DataOutput need ONE ItemAwareElement
impl Related<super::bpmn_20_item_aware_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemAwareElement.def()
    }
}

// ManyToMany : with OutputSet using A_dataOutputRefs_outputSetRefs
impl Related<super::bpmn_20_a_data_output_refs_output_set_refs::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_data_output_refs_output_set_refs::Relation::OutputSet.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_data_output_refs_output_set_refs::Relation::DataOutput
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with OutputSet using A_outputSetWithOptional_optionalOutputRefs
impl Related<super::bpmn_20_a_output_set_with_optional_optional_output_refs::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_output_set_with_optional_optional_output_refs::Relation::OutputSet.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_output_set_with_optional_optional_output_refs::Relation::DataOutput
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with OutputSet using A_outputSetWithWhileExecuting_whileExecutingOutputRefs
impl Related<super::bpmn_20_a_output_set_with_while_executing_while_executing_output_refs::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_output_set_with_while_executing_while_executing_output_refs::Relation::OutputSet.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_output_set_with_while_executing_while_executing_output_refs::Relation::DataOutput
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "DataOutput" (bpmn_20_class_data_output)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __is_collection__ (xmi_id : "BPMN20-DataOutput-isCollection")
    ///   * type : __std::primitive::bool__
    ///   * default : "false"
    /// * __name__ (xmi_id : "BPMN20-DataOutput-name")
    ///   * type : __Option<std::string::String>__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __CatchEvent__ (__CatchEventModel__) from A_dataOutputs_catchEvent
    ///   * one-to-many link : (0-1) __DataOutput__ need (0-inf) __CatchEvent__)
    ///   * callable using find_with_related(__CatchEventModel__) from __DataOutput__
    ///   * named catch_event in BPMN
    /// * __InputOutputSpecification__ (__InputOutputSpecificationModel__) from A_dataOutputs_inputOutputSpecification
    ///   * one-to-many link : (1-1) __DataOutput__ need (0-inf) __InputOutputSpecification__)
    ///   * callable using find_with_related(__InputOutputSpecificationModel__) from __DataOutput__
    ///   * named input_output_specification in BPMN
    /// 
    /// ## Direct Super :
    /// * __ItemAwareElement__ (__ItemAwareElementModel__)
    ///   * one-to-one link : one __DataOutput__ need one __ItemAwareElement__)
    ///   * callable using find_also_related(__ItemAwareElementModel__) from __DataOutput__
    ///   * saved in __super_item_aware_element__ field as foreing key
    /// ## Reverse One To One :
    /// * __MultiInstanceLoopCharacteristics__ (__MultiInstanceLoopCharacteristicsModel__) from A_outputDataItem_multiInstanceLoopCharacteristics
    ///   * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __DataOutput__)
    ///   * callable using find_also_related(__DataOutputModel__) from __MultiInstanceLoopCharacteristics__
    ///   * saved in __output_data_item__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "DataOutput" (bpmn_20_class_data_output)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __is_collection__ (xmi_id : "BPMN20-DataOutput-isCollection")
  * type : __std::primitive::bool__
  * default : "false"
* __name__ (xmi_id : "BPMN20-DataOutput-name")
  * type : __Option<std::string::String>__


## Relation : One To Many :
* __CatchEvent__ (__CatchEventModel__) from A_dataOutputs_catchEvent
  * one-to-many link : (0-1) __DataOutput__ need (0-inf) __CatchEvent__)
  * callable using find_with_related(__CatchEventModel__) from __DataOutput__
  * named catch_event in BPMN
* __InputOutputSpecification__ (__InputOutputSpecificationModel__) from A_dataOutputs_inputOutputSpecification
  * one-to-many link : (1-1) __DataOutput__ need (0-inf) __InputOutputSpecification__)
  * callable using find_with_related(__InputOutputSpecificationModel__) from __DataOutput__
  * named input_output_specification in BPMN

## Direct Super :
* __ItemAwareElement__ (__ItemAwareElementModel__)
  * one-to-one link : one __DataOutput__ need one __ItemAwareElement__)
  * callable using find_also_related(__ItemAwareElementModel__) from __DataOutput__
  * saved in __super_item_aware_element__ field as foreing key
## Reverse One To One :
* __MultiInstanceLoopCharacteristics__ (__MultiInstanceLoopCharacteristicsModel__) from A_outputDataItem_multiInstanceLoopCharacteristics
  * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __DataOutput__)
  * callable using find_also_related(__DataOutputModel__) from __MultiInstanceLoopCharacteristics__
  * saved in __output_data_item__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "DataOutput",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "DataOutput",
//     is_abstract: false,
//     super_class: [
//         "ItemAwareElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-DataOutput-isCollection": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "DataOutput-isCollection",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "isCollection",
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
//         "-DataOutput-name": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "DataOutput-name",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
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
//         "-DataOutput-outputSetRefs": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "DataOutput-outputSetRefs",
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
//                     "A_dataOutputRefs_outputSetRefs",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-DataOutput-outputSetWithOptional": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "DataOutput-outputSetWithOptional",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "outputSetWithOptional",
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
//                 is_derived: true,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_outputSetWithOptional_optionalOutputRefs",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-DataOutput-outputSetWithWhileExecuting": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "DataOutput-outputSetWithWhileExecuting",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "outputSetWithWhileExecuting",
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
//                 is_derived: true,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_outputSetWithWhileExecuting_whileExecutingOutputRefs",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#DataOutput",
//     table_name: "bpmn_20_data_output",
//     model_name: "DataOutput",
//     full_name: "bpmn_20_class_data_output",
// }


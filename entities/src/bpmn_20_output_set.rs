//! bpmn_20_class_output_set

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_output_set")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// SIMPLE FIELD : OutputSet-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE OutputSet need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE OutputSet need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// ManyToMany : with DataOutput using A_dataOutputRefs_outputSetRefs
impl Related<super::bpmn_20_a_data_output_refs_output_set_refs::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_data_output_refs_output_set_refs::Relation::DataOutput.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_data_output_refs_output_set_refs::Relation::OutputSet
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with InputSet using A_inputSetRefs_outputSetRefs
impl Related<super::bpmn_20_a_input_set_refs_output_set_refs::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_input_set_refs_output_set_refs::Relation::InputSet.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_input_set_refs_output_set_refs::Relation::OutputSet
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with DataOutput using A_outputSetWithOptional_optionalOutputRefs
impl Related<super::bpmn_20_a_output_set_with_optional_optional_output_refs::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_output_set_with_optional_optional_output_refs::Relation::DataOutput.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_output_set_with_optional_optional_output_refs::Relation::OutputSet
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with DataOutput using A_outputSetWithWhileExecuting_whileExecutingOutputRefs
impl Related<super::bpmn_20_a_output_set_with_while_executing_while_executing_output_refs::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_output_set_with_while_executing_while_executing_output_refs::Relation::DataOutput.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_output_set_with_while_executing_while_executing_output_refs::Relation::OutputSet
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "OutputSet" (bpmn_20_class_output_set)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "OutputSet-name")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __OutputSet__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __OutputSet__
    ///   * saved in __super_base_element__ field as foreing key
    /// ## Reverse One To One :
    /// * __CatchEvent__ (__CatchEventModel__) from A_outputSet_catchEvent
    ///   * one-to-one link : one __CatchEvent__ need one __OutputSet__)
    ///   * callable using find_also_related(__OutputSetModel__) from __CatchEvent__
    ///   * saved in __output_set__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "OutputSet" (bpmn_20_class_output_set)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "OutputSet-name")
  * type : __std::string::String__


## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __OutputSet__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __OutputSet__
  * saved in __super_base_element__ field as foreing key
## Reverse One To One :
* __CatchEvent__ (__CatchEventModel__) from A_outputSet_catchEvent
  * one-to-one link : one __CatchEvent__ need one __OutputSet__)
  * callable using find_also_related(__OutputSetModel__) from __CatchEvent__
  * saved in __output_set__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "OutputSet",
//     name: "OutputSet",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "OutputSet-dataOutputRefs",
//                 name: "dataOutputRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataOutput",
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
//                     "A_dataOutputRefs_outputSetRefs",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "OutputSet-name",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "OutputSet-inputSetRefs",
//                 name: "inputSetRefs",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "OutputSet-optionalOutputRefs",
//                 name: "optionalOutputRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataOutput",
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
//                     "A_outputSetWithOptional_optionalOutputRefs",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "OutputSet-whileExecutingOutputRefs",
//                 name: "whileExecutingOutputRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataOutput",
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
//                     "A_outputSetWithWhileExecuting_whileExecutingOutputRefs",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }


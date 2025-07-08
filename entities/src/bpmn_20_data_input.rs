//! bpmn_20_class_data_input

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_data_input")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : ItemAwareElement
    pub super_item_aware_element: i64,
    /// SIMPLE FIELD : DataInput-name
    pub name: Option<std::string::String>,
    /// SIMPLE FIELD : DataInput-isCollection
    #[sea_orm(default_value = "false")]
    pub is_collection: std::primitive::bool,
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

// RAW :
// CMOFClass {
//     xmi_id: "DataInput",
//     name: "DataInput",
//     is_abstract: false,
//     super_class: [
//         "ItemAwareElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "DataInput-name",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "DataInput-isCollection",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "DataInput-inputSetRefs",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "DataInput-inputSetWithOptional",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "DataInput-inputSetWithWhileExecuting",
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
//     ],
//     owned_rule: [],
// }


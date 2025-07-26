//! bpmn_20_class_callable_element

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_callable_element")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperRootElement
    pub super_root_element: i64,
    /// COMPLEX FIELD : BPMN20-CallableElement-ioSpecification
    pub io_specification: Option<i64>,
    /// SIMPLE FIELD : BPMN20-CallableElement-name
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE CallableElement need ONE RootElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_root_element::Entity",
        from = "Column::SuperRootElement",
        to = "super::bpmn_20_root_element::Column::Id",
        on_delete = "Cascade"
    )]
    RootElement,
    // SUPER : ONE GlobalTask need ONE CallableElement
    #[sea_orm(has_one = "super::bpmn_20_global_task::Entity")]
    GlobalTask,
    // SUPER : ONE Process need ONE CallableElement
    #[sea_orm(has_one = "super::bpmn_20_process::Entity")]
    Process,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-CallableElement',
//     name: "CallableElement",
//     is_abstract: true,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'BPMN20-RootElement',
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "CallableElement-ioBinding": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-CallableElement-ioBinding',
//                 name: "ioBinding",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-InputOutputBinding',
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
//                 owning_association: None,
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-A_ioBinding_callableElement',
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "CallableElement-ioSpecification": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-CallableElement-ioSpecification',
//                 name: "ioSpecification",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-InputOutputSpecification',
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
//                 owning_association: None,
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-A_ioSpecification_callableElement',
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "CallableElement-name": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-CallableElement-name',
//                 name: "name",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "Loaded XMIIdReference RefCell of 'DC-String',
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
//                 owning_association: None,
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "CallableElement-supportedInterfaceRefs": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-CallableElement-supportedInterfaceRefs',
//                 name: "supportedInterfaceRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-Interface',
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
//                 owning_association: None,
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-A_supportedInterfaceRefs_callableElements',
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#CallableElement",
//     table_name: "bpmn_20_callable_element",
//     model_name: "CallableElement",
//     full_name: "bpmn_20_class_callable_element",
//     reverse_super: RefCell {
//         value: [
//             (Weak),
//             (Weak),
//         ],
//     },
// }


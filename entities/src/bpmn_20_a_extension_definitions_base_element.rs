//! bpmn_20_association_a_extension_definitions_base_element

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_extension_definitions_base_element")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub extension_definition_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub base_element_b_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_extension_definition::Entity",
        from = "Column::ExtensionDefinitionAId",
        to = "super::bpmn_20_extension_definition::Column::Id"
    )]
    ExtensionDefinition,
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::BaseElementBId",
        to = "super::bpmn_20_base_element::Column::Id"
    )]
    BaseElement,
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFAssociation {
//     xmi_id: "A_extensionDefinitions_baseElement",
//     name: "A_extensionDefinitions_baseElement",
//     visibility: Private,
//     member_end: (
//         "BaseElement-extensionDefinitions",
//         "A_extensionDefinitions_baseElement-baseElement",
//     ),
//     owned_end: {
//         "A_extensionDefinitions_baseElement-baseElement": Property(
//             CMOFProperty {
//                 xmi_id: "A_extensionDefinitions_baseElement-baseElement",
//                 name: "baseElement",
//                 visibility: Public,
//                 simple_type: Some(
//                     "BaseElement",
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
//                 owning_association: "A_extensionDefinitions_baseElement",
//                 association: Some(
//                     "A_extensionDefinitions_baseElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     is_derived: false,
//     technical_name: "BPMN20.cmof#A_extensionDefinitions_baseElement",
//     table_name: "bpmn_20_a_extension_definitions_base_element",
//     model_name: "AExtensionDefinitionsBaseElement",
//     full_name: "bpmn_20_association_a_extension_definitions_base_element",
// }


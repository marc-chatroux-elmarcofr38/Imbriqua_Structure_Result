//! bpmn_20_association_a_supported_interface_refs_callable_elements

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_supported_interface_refs_callable_elements")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub interface_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub callable_element_b_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_interface::Entity",
        from = "Column::InterfaceAId",
        to = "super::bpmn_20_interface::Column::Id"
    )]
    Interface,
    #[sea_orm(
        belongs_to = "super::bpmn_20_callable_element::Entity",
        from = "Column::CallableElementBId",
        to = "super::bpmn_20_callable_element::Column::Id"
    )]
    CallableElement,
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFAssociation {
//     xmi_id: XMIIdLocalReference {
//         object_id: "A_supportedInterfaceRefs_callableElements",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "A_supportedInterfaceRefs_callableElements",
//     visibility: Private,
//     member_end: (
//         "CallableElement-supportedInterfaceRefs",
//         "A_supportedInterfaceRefs_callableElements-callableElements",
//     ),
//     owned_end: {
//         "A_supportedInterfaceRefs_callableElements-callableElements": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "A_supportedInterfaceRefs_callableElements-callableElements",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "callableElements",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CallableElement",
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
//                 owning_association: "A_supportedInterfaceRefs_callableElements",
//                 association: Some(
//                     "A_supportedInterfaceRefs_callableElements",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     is_derived: false,
//     technical_name: "BPMN20.cmof#A_supportedInterfaceRefs_callableElements",
//     table_name: "bpmn_20_a_supported_interface_refs_callable_elements",
//     model_name: "ASupportedInterfaceRefsCallableElements",
//     full_name: "bpmn_20_association_a_supported_interface_refs_callable_elements",
// }


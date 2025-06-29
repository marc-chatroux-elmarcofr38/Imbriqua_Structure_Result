//! bpmndi_class_bpmn_diagram

use crate::*;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel, Default)]
#[sea_orm(table_name = "bpmndi_bpmn_diagram")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "BPMNDiagram",
//     name: "BPMNDiagram",
//     is_abstract: false,
//     super_class: None,
//     super_class_link: Some(
//         Class(
//             SuperClass {
//                 href: "DI.cmof#Diagram",
//             },
//         ),
//     ),
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "BPMNDiagram-plane",
//                 name: "plane",
//                 visibility: Public,
//                 simple_type: Some(
//                     "BPMNPlane",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
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
//                     "A_plane_diagram",
//                 ),
//                 redefined_property_link: Some(
//                     Property(
//                         RedefinedProperty {
//                             href: "DI.cmof#Diagram-rootElement",
//                         },
//                     ),
//                 ),
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "BPMNDiagram-labelStyle",
//                 name: "labelStyle",
//                 visibility: Public,
//                 simple_type: Some(
//                     "BPMNLabelStyle",
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
//                     "A_labelStyle_diagram",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: Some(
//                     Property(
//                         SubsettedProperty {
//                             href: "DI.cmof#Diagram-ownedStyle",
//                         },
//                     ),
//                 ),
//             },
//         ),
//     ],
//     owned_rule: [],
// }


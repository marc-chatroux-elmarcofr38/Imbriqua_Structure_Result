//! di_class_plane

use crate::*;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel, Default)]
#[sea_orm(table_name = "di_plane")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "Plane",
//     name: "Plane",
//     is_abstract: true,
//     super_class: Some(
//         "Node",
//     ),
//     super_class_link: None,
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "Plane-planeElement",
//                 name: "planeElement",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DiagramElement",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: true,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: Some(
//                     "DiagramElement-ownedElement",
//                 ),
//                 owning_association: "",
//                 association: Some(
//                     "A_planeElement_plane",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [
//         Constraint(
//             CMOFConstraint {
//                 xmi_id: "Plane-plane_element_type",
//                 name: "plane_element_type",
//                 constrained_element: "Plane",
//                 namespace: "Plane",
//                 specification: OpaqueExpression(
//                     CMOFOpaqueExpression {
//                         xmi_id: "Plane-plane_element_type-_specification",
//                         body: "planeElement->forAll(oclIsKindOf(Shape) or oclIsKindOf(Edge))",
//                         language: "OCL",
//                     },
//                 ),
//             },
//         ),
//     ],
// }


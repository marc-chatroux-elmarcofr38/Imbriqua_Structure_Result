//! di_class_plane

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "di_plane")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperNode
    pub super_node: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Plane need ONE Node
    #[sea_orm(
        belongs_to = "super::di_node::Entity",
        from = "Column::SuperNode",
        to = "super::di_node::Column::Id",
        on_delete = "Cascade"
    )]
    Node,
    // SUPER : ONE BpmnPlane need ONE Plane
    #[sea_orm(has_one = "super::bpmndi_bpmn_plane::Entity")]
    BpmnPlane,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'DI-Plane',
//     name: "Plane",
//     is_abstract: true,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'DI-Node',
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "Plane-planeElement": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'DI-Plane-planeElement',
//                 name: "planeElement",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'DI-DiagramElement',
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
//                 owning_association: None,
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'DI-A_planeElement_plane',
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {
//         "Plane-plane_element_type": Constraint(
//             CMOFConstraint {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'DI-Plane-plane_element_type',
//                 _name: "plane_element_type",
//                 constrained_element: "Plane",
//                 namespace: "Plane",
//                 specification: OpaqueExpression(
//                     CMOFOpaqueExpression {
//                         xmi_id: "Complete XMIIdLocalReference RefCell of 'DI-Plane-plane_element_type-_specification',
//                         body: "planeElement->forAll(oclIsKindOf(Shape) or oclIsKindOf(Edge))",
//                         language: "OCL",
//                     },
//                 ),
//             },
//         ),
//     },
//     technical_name: "DI.cmof#Plane",
//     table_name: "di_plane",
//     model_name: "Plane",
//     full_name: "di_class_plane",
//     reverse_super: RefCell {
//         value: [
//             (Weak),
//         ],
//     },
// }


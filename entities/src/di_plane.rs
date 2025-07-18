//! di_class_plane

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "di_plane")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Node
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

// SUPER : ONE Plane need ONE Node
impl Related<super::di_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Node.def()
    }
}

// SUPER : ONE BpmnPlane need ONE Plane
impl Related<super::bpmndi_bpmn_plane::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BpmnPlane.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Plane" (di_class_plane)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __Node__ (__NodeModel__)
    ///   * one-to-one link : one __Plane__ need one __Node__)
    ///   * callable using find_also_related(__NodeModel__) from __Plane__
    ///   * saved in __super_node__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __BpmnPlane__ (__BpmnPlaneModel__)
    ///   * one-to-one link (reverse) : one __BpmnPlane__ need one __Plane__)
    ///   * callable using find_also_related(__PlaneModel__) from __BpmnPlane__
    ///   * saved in __super_plane__ field as foreing key in __BpmnPlaneModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Plane" (di_class_plane)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __Node__ (__NodeModel__)
  * one-to-one link : one __Plane__ need one __Node__)
  * callable using find_also_related(__NodeModel__) from __Plane__
  * saved in __super_node__ field as foreing key

## Reverse Super :
* __BpmnPlane__ (__BpmnPlaneModel__)
  * one-to-one link (reverse) : one __BpmnPlane__ need one __Plane__)
  * callable using find_also_related(__PlaneModel__) from __BpmnPlane__
  * saved in __super_plane__ field as foreing key in __BpmnPlaneModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "Plane",
//         package_id: "DI",
//         is_set: true,
//     },
//     name: "Plane",
//     is_abstract: true,
//     super_class: [
//         "Node",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-Plane-planeElement": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Plane-planeElement",
//                     package_id: "DI",
//                     is_set: true,
//                 },
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
//     },
//     owned_rule: {
//         "-Plane-plane_element_type": Constraint(
//             CMOFConstraint {
//                 xmi_id: XMIIdReference {
//                     local_id: "Plane-plane_element_type",
//                     package_id: "DI",
//                     is_set: true,
//                 },
//                 name: "plane_element_type",
//                 constrained_element: "Plane",
//                 namespace: "Plane",
//                 specification: OpaqueExpression(
//                     CMOFOpaqueExpression {
//                         xmi_id: XMIIdReference {
//                             local_id: "Plane-plane_element_type-_specification",
//                             package_id: "DI",
//                             is_set: true,
//                         },
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
// }


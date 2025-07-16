//! bpmndi_class_bpmn_plane

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmndi_bpmn_plane")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Plane
    pub super_plane: i64,
    /// COMPLEX FIELD : BPMNPlane-bpmnElement
    pub bpmn_element: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE BpmnPlane need ONE Plane
    #[sea_orm(
        belongs_to = "super::di_plane::Entity",
        from = "Column::SuperPlane",
        to = "super::di_plane::Column::Id",
        on_delete = "Cascade"
    )]
    Plane,
}

// SUPER : ONE BpmnPlane need ONE Plane
impl Related<super::di_plane::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Plane.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "BpmnPlane" (bpmndi_class_bpmn_plane)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __Plane__ (__PlaneModel__)
    ///   * one-to-one link : one __BpmnPlane__ need one __Plane__)
    ///   * callable using find_also_related(__PlaneModel__) from __BpmnPlane__
    ///   * saved in __super_plane__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "BpmnPlane" (bpmndi_class_bpmn_plane)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __Plane__ (__PlaneModel__)
  * one-to-one link : one __BpmnPlane__ need one __Plane__)
  * callable using find_also_related(__PlaneModel__) from __BpmnPlane__
  * saved in __super_plane__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "BPMNPlane",
//     name: "BPMNPlane",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [
//         Class(
//             SuperClass {
//                 href: "DI.cmof#Plane",
//             },
//         ),
//     ],
//     owned_attribute: {
//         "BPMNPlane-bpmnElement": Property(
//             CMOFProperty {
//                 xmi_id: "BPMNPlane-bpmnElement",
//                 name: "bpmnElement",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     ClassLink(
//                         ClassLink {
//                             href: "BPMN20.cmof#BaseElement",
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
//                 association: Some(
//                     "A_bpmnElement_plane",
//                 ),
//                 redefined_property_link: Some(
//                     Property(
//                         RedefinedProperty {
//                             href: "DI.cmof#DiagramElement-modelElement",
//                         },
//                     ),
//                 ),
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMNDI.cmof#BPMNPlane",
//     table_name: "bpmndi_bpmn_plane",
//     model_name: "BpmnPlane",
//     full_name: "bpmndi_class_bpmn_plane",
// }


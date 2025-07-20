//! bpmn_20_class_boundary_event

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_boundary_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : CatchEvent
    pub super_catch_event: i64,
    /// COMPLEX FIELD : BPMN20-BoundaryEvent-attachedToRef
    pub attached_to_ref: i64,
    /// SIMPLE FIELD : BPMN20-BoundaryEvent-cancelActivity
    #[sea_orm(default_value = "true")]
    pub cancel_activity: std::primitive::bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE BoundaryEvent need ONE CatchEvent
    #[sea_orm(
        belongs_to = "super::bpmn_20_catch_event::Entity",
        from = "Column::SuperCatchEvent",
        to = "super::bpmn_20_catch_event::Column::Id",
        on_delete = "Cascade"
    )]
    CatchEvent,
}

// SUPER : ONE BoundaryEvent need ONE CatchEvent
impl Related<super::bpmn_20_catch_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CatchEvent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "BoundaryEvent" (bpmn_20_class_boundary_event)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __cancel_activity__ (xmi_id : "BPMN20-BoundaryEvent-cancelActivity")
    ///   * type : __std::primitive::bool__
    ///   * default : "true"
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __Activity__ (__ActivityModel__) from A_boundaryEventRefs_attachedToRef
    ///   * one-to-many link : (1-1) __BoundaryEvent__ need (0-inf) __Activity__)
    ///   * callable using find_with_related(__ActivityModel__) from __BoundaryEvent__
    ///   * named attached_to_ref in BPMN
    /// 
    /// ## Direct Super :
    /// * __CatchEvent__ (__CatchEventModel__)
    ///   * one-to-one link : one __BoundaryEvent__ need one __CatchEvent__)
    ///   * callable using find_also_related(__CatchEventModel__) from __BoundaryEvent__
    ///   * saved in __super_catch_event__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "BoundaryEvent" (bpmn_20_class_boundary_event)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __cancel_activity__ (xmi_id : "BPMN20-BoundaryEvent-cancelActivity")
  * type : __std::primitive::bool__
  * default : "true"


## Relation : One To Many :
* __Activity__ (__ActivityModel__) from A_boundaryEventRefs_attachedToRef
  * one-to-many link : (1-1) __BoundaryEvent__ need (0-inf) __Activity__)
  * callable using find_with_related(__ActivityModel__) from __BoundaryEvent__
  * named attached_to_ref in BPMN

## Direct Super :
* __CatchEvent__ (__CatchEventModel__)
  * one-to-one link : one __BoundaryEvent__ need one __CatchEvent__)
  * callable using find_also_related(__CatchEventModel__) from __BoundaryEvent__
  * saved in __super_catch_event__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         object_id: "BoundaryEvent",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "BoundaryEvent",
//     is_abstract: false,
//     super_class: [
//         "CatchEvent",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-BoundaryEvent-attachedToRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "BoundaryEvent-attachedToRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "attachedToRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Activity",
//                 ),
//                 complex_type: None,
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
//                 association: Some(
//                     "A_boundaryEventRefs_attachedToRef",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-BoundaryEvent-cancelActivity": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "BoundaryEvent-cancelActivity",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "cancelActivity",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
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
//                     "true",
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
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#BoundaryEvent",
//     table_name: "bpmn_20_boundary_event",
//     model_name: "BoundaryEvent",
//     full_name: "bpmn_20_class_boundary_event",
// }


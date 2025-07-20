//! di_class_edge

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "di_edge")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : DiagramElement
    pub super_diagram_element: i64,
    /// COMPLEX FIELD : DI-Edge-source
    pub source: Option<i64>,
    /// COMPLEX FIELD : DI-Edge-target
    pub target: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Edge need ONE DiagramElement
    #[sea_orm(
        belongs_to = "super::di_diagram_element::Entity",
        from = "Column::SuperDiagramElement",
        to = "super::di_diagram_element::Column::Id",
        on_delete = "Cascade"
    )]
    DiagramElement,
    // SUPER : ONE LabeledEdge need ONE Edge
    #[sea_orm(has_one = "super::di_labeled_edge::Entity")]
    LabeledEdge,
}

// SUPER : ONE Edge need ONE DiagramElement
impl Related<super::di_diagram_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DiagramElement.def()
    }
}

// SUPER : ONE LabeledEdge need ONE Edge
impl Related<super::di_labeled_edge::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LabeledEdge.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Edge" (di_class_edge)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __DiagramElement__ (__DiagramElementModel__) from A_source_sourceEdge
    ///   * one-to-many link : (0-1) __Edge__ need (0-inf) __DiagramElement__)
    ///   * callable using find_with_related(__DiagramElementModel__) from __Edge__
    /// * __DiagramElement__ (__DiagramElementModel__) from A_target_targetEdge
    ///   * one-to-many link : (0-1) __Edge__ need (0-inf) __DiagramElement__)
    ///   * callable using find_with_related(__DiagramElementModel__) from __Edge__
    /// 
    /// ## Direct Super :
    /// * __DiagramElement__ (__DiagramElementModel__)
    ///   * one-to-one link : one __Edge__ need one __DiagramElement__)
    ///   * callable using find_also_related(__DiagramElementModel__) from __Edge__
    ///   * saved in __super_diagram_element__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __LabeledEdge__ (__LabeledEdgeModel__)
    ///   * one-to-one link (reverse) : one __LabeledEdge__ need one __Edge__)
    ///   * callable using find_also_related(__EdgeModel__) from __LabeledEdge__
    ///   * saved in __super_edge__ field as foreing key in __LabeledEdgeModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Edge" (di_class_edge)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Relation : One To Many :
* __DiagramElement__ (__DiagramElementModel__) from A_source_sourceEdge
  * one-to-many link : (0-1) __Edge__ need (0-inf) __DiagramElement__)
  * callable using find_with_related(__DiagramElementModel__) from __Edge__
* __DiagramElement__ (__DiagramElementModel__) from A_target_targetEdge
  * one-to-many link : (0-1) __Edge__ need (0-inf) __DiagramElement__)
  * callable using find_with_related(__DiagramElementModel__) from __Edge__

## Direct Super :
* __DiagramElement__ (__DiagramElementModel__)
  * one-to-one link : one __Edge__ need one __DiagramElement__)
  * callable using find_also_related(__DiagramElementModel__) from __Edge__
  * saved in __super_diagram_element__ field as foreing key

## Reverse Super :
* __LabeledEdge__ (__LabeledEdgeModel__)
  * one-to-one link (reverse) : one __LabeledEdge__ need one __Edge__)
  * callable using find_also_related(__EdgeModel__) from __LabeledEdge__
  * saved in __super_edge__ field as foreing key in __LabeledEdgeModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         object_id: "Edge",
//         package_id: "DI",
//         is_set: true,
//     },
//     name: "Edge",
//     is_abstract: true,
//     super_class: [
//         "DiagramElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-Edge-source": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Edge-source",
//                     package_id: "DI",
//                     is_set: true,
//                 },
//                 name: "source",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DiagramElement",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: true,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: true,
//                 is_derived_union: true,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_source_sourceEdge",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Edge-target": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Edge-target",
//                     package_id: "DI",
//                     is_set: true,
//                 },
//                 name: "target",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DiagramElement",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: true,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: true,
//                 is_derived_union: true,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_target_targetEdge",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Edge-waypoint": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Edge-waypoint",
//                     package_id: "DI",
//                     is_set: true,
//                 },
//                 name: "waypoint",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefDataType(
//                         HRefDataType {
//                             href: "DC.cmof#Point",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 2,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: true,
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
//     technical_name: "DI.cmof#Edge",
//     table_name: "di_edge",
//     model_name: "Edge",
//     full_name: "di_class_edge",
// }


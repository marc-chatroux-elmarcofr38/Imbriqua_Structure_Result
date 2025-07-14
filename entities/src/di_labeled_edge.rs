//! di_class_labeled_edge

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "di_labeled_edge")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Edge
    pub super_edge: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE LabeledEdge need ONE Edge
    #[sea_orm(
        belongs_to = "super::di_edge::Entity",
        from = "Column::SuperEdge",
        to = "super::di_edge::Column::Id",
        on_delete = "Cascade"
    )]
    Edge,
    // SUPER : ONE BpmnEdge need ONE LabeledEdge
    #[sea_orm(has_one = "super::bpmndi_bpmn_edge::Entity")]
    BpmnEdge,
}

// SUPER : ONE LabeledEdge need ONE Edge
impl Related<super::di_edge::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Edge.def()
    }
}

// SUPER : ONE BpmnEdge need ONE LabeledEdge
impl Related<super::bpmndi_bpmn_edge::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BpmnEdge.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "LabeledEdge" (di_class_labeled_edge)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __Edge__ (__EdgeModel__)
    ///   * one-to-one link : one __LabeledEdge__ need one __Edge__)
    ///   * callable using find_also_related(__EdgeModel__) from __LabeledEdge__
    ///   * saved in __super_edge__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __BpmnEdge__ (__BpmnEdgeModel__)
    ///   * one-to-one link (reverse) : one __BpmnEdge__ need one __LabeledEdge__)
    ///   * callable using find_also_related(__LabeledEdgeModel__) from __BpmnEdge__
    ///   * saved in __super_labeled_edge__ field as foreing key in __BpmnEdgeModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "LabeledEdge" (di_class_labeled_edge)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __Edge__ (__EdgeModel__)
  * one-to-one link : one __LabeledEdge__ need one __Edge__)
  * callable using find_also_related(__EdgeModel__) from __LabeledEdge__
  * saved in __super_edge__ field as foreing key

## Reverse Super :
* __BpmnEdge__ (__BpmnEdgeModel__)
  * one-to-one link (reverse) : one __BpmnEdge__ need one __LabeledEdge__)
  * callable using find_also_related(__LabeledEdgeModel__) from __BpmnEdge__
  * saved in __super_labeled_edge__ field as foreing key in __BpmnEdgeModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "LabeledEdge",
//     name: "LabeledEdge",
//     is_abstract: true,
//     super_class: [
//         "Edge",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "LabeledEdge-ownedLabel",
//                 name: "ownedLabel",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Label",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: true,
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: true,
//                 is_derived_union: true,
//                 subsetted_property: Some(
//                     "DiagramElement-ownedElement",
//                 ),
//                 owning_association: "",
//                 association: Some(
//                     "A_ownedLabel_owningEdge",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }


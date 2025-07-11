//! di_class_label

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "di_label")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Node
    pub super_node: i64,
    /// COMPLEX FIELD : Label-bounds
    pub bounds: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Label need ONE Node
    #[sea_orm(
        belongs_to = "super::di_node::Entity",
        from = "Column::SuperNode",
        to = "super::di_node::Column::Id",
        on_delete = "Cascade"
    )]
    Node,
    // SUPER : ONE BpmnLabel need ONE Label
    #[sea_orm(has_one = "super::bpmndi_bpmn_label::Entity")]
    BpmnLabel,
}

// SUPER : ONE Label need ONE Node
impl Related<super::di_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Node.def()
    }
}

// SUPER : ONE BpmnLabel need ONE Label
impl Related<super::bpmndi_bpmn_label::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BpmnLabel.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Label" (di_class_label)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __Node__ (__NodeModel__)
    ///   * one-to-one link : one __Label__ need one __Node__)
    ///   * callable using find_also_related(__NodeModel__) from __Label__
    ///   * saved in __super_node__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __BpmnLabel__ (__BpmnLabelModel__)
    ///   * one-to-one link (reverse) : one __BpmnLabel__ need one __Label__)
    ///   * callable using find_also_related(__LabelModel__) from __BpmnLabel__
    ///   * saved in __super_label__ field as foreing key in __BpmnLabelModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Label" (di_class_label)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Direct Super :
* __Node__ (__NodeModel__)
  * one-to-one link : one __Label__ need one __Node__)
  * callable using find_also_related(__NodeModel__) from __Label__
  * saved in __super_node__ field as foreing key

## Reverse Super :
* __BpmnLabel__ (__BpmnLabelModel__)
  * one-to-one link (reverse) : one __BpmnLabel__ need one __Label__)
  * callable using find_also_related(__LabelModel__) from __BpmnLabel__
  * saved in __super_label__ field as foreing key in __BpmnLabelModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Label",
//     name: "Label",
//     is_abstract: true,
//     super_class: [
//         "Node",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "Label-bounds",
//                 name: "bounds",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     DataTypeLink(
//                         DataTypeLink {
//                             href: "DC.cmof#Bounds",
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }


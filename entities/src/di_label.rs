//! di_class_label

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "di_label")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SUPER FIELD : Node
    pub super_node: i32,
    /// COMPLEX FIELD : Label-bounds
    pub bounds: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Label need ONE Node
    #[sea_orm(
        belongs_to = "super::di_node::Entity",
        from = "Column::SuperNode",
        to = "super::di_node::Column::Id"
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


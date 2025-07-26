//! di_class_node

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "di_node")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperDiagramElement
    pub super_diagram_element: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Node need ONE DiagramElement
    #[sea_orm(
        belongs_to = "super::di_diagram_element::Entity",
        from = "Column::SuperDiagramElement",
        to = "super::di_diagram_element::Column::Id",
        on_delete = "Cascade"
    )]
    DiagramElement,
    // SUPER : ONE Label need ONE Node
    #[sea_orm(has_one = "super::di_label::Entity")]
    Label,
    // SUPER : ONE Plane need ONE Node
    #[sea_orm(has_one = "super::di_plane::Entity")]
    Plane,
    // SUPER : ONE Shape need ONE Node
    #[sea_orm(has_one = "super::di_shape::Entity")]
    Shape,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'DI-Node',
//     name: "Node",
//     is_abstract: true,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'DI-DiagramElement',
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "DI.cmof#Node",
//     table_name: "di_node",
//     model_name: "Node",
//     full_name: "di_class_node",
//     reverse_super: RefCell {
//         value: [
//             (Weak),
//             (Weak),
//             (Weak),
//         ],
//     },
// }


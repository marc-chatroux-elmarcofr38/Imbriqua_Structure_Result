//! di_class_node

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "di_node")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : DiagramElement
    pub super_diagram_element: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::di_diagram_element::Entity",
        from = "Column::SuperDiagramElement",
        to = "super::di_diagram_element::Column::Id"
    )]
    DiagramElement,
    #[sea_orm(has_one = "super::di_label::Entity")]
    Label,
    #[sea_orm(has_one = "super::di_plane::Entity")]
    Plane,
    #[sea_orm(has_one = "super::di_shape::Entity")]
    Shape,
}

// `Related` trait has to be implemented by hand
impl Related<super::di_diagram_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DiagramElement.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::di_label::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Label.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::di_plane::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Plane.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::di_shape::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Shape.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "Node",
//     name: "Node",
//     is_abstract: true,
//     super_class: [
//         "DiagramElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


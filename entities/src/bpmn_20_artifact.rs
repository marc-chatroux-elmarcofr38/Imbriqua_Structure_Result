//! bpmn_20_class_artifact

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_artifact")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Artifact need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id"
    )]
    BaseElement,
    // SUPER : ONE Association need ONE Artifact
    #[sea_orm(has_one = "super::bpmn_20_association::Entity")]
    Association,
    // SUPER : ONE Group need ONE Artifact
    #[sea_orm(has_one = "super::bpmn_20_group::Entity")]
    Group,
    // SUPER : ONE TextAnnotation need ONE Artifact
    #[sea_orm(has_one = "super::bpmn_20_text_annotation::Entity")]
    TextAnnotation,
}

// SUPER : ONE Artifact need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// SUPER : ONE Association need ONE Artifact
impl Related<super::bpmn_20_association::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Association.def()
    }
}

// SUPER : ONE Group need ONE Artifact
impl Related<super::bpmn_20_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

// SUPER : ONE TextAnnotation need ONE Artifact
impl Related<super::bpmn_20_text_annotation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TextAnnotation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "Artifact",
//     name: "Artifact",
//     is_abstract: true,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


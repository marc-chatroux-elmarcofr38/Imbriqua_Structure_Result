//! bpmn_20_class_performer

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_performer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : ResourceRole
    pub super_resource_role: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_resource_role::Entity",
        from = "Column::SuperResourceRole",
        to = "super::bpmn_20_resource_role::Column::Id"
    )]
    ResourceRole,
    #[sea_orm(has_one = "super::bpmn_20_human_performer::Entity")]
    HumanPerformer,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_resource_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ResourceRole.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_human_performer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::HumanPerformer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "Performer",
//     name: "Performer",
//     is_abstract: false,
//     super_class: [
//         "ResourceRole",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


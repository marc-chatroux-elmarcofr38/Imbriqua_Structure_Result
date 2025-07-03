//! bpmn_20_class_performer

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_performer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : ResourceRole
    pub super_resource_role: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Performer need ONE ResourceRole
    #[sea_orm(
        belongs_to = "super::bpmn_20_resource_role::Entity",
        from = "Column::SuperResourceRole",
        to = "super::bpmn_20_resource_role::Column::Id"
    )]
    ResourceRole,
    // SUPER : ONE HumanPerformer need ONE Performer
    #[sea_orm(has_one = "super::bpmn_20_human_performer::Entity")]
    HumanPerformer,
}

// SUPER : ONE Performer need ONE ResourceRole
impl Related<super::bpmn_20_resource_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ResourceRole.def()
    }
}

// SUPER : ONE HumanPerformer need ONE Performer
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


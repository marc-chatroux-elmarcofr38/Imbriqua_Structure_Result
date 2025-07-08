//! bpmn_20_class_human_performer

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_human_performer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Performer
    pub super_performer: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE HumanPerformer need ONE Performer
    #[sea_orm(
        belongs_to = "super::bpmn_20_performer::Entity",
        from = "Column::SuperPerformer",
        to = "super::bpmn_20_performer::Column::Id",
        on_delete = "Cascade"
    )]
    Performer,
    // SUPER : ONE PotentialOwner need ONE HumanPerformer
    #[sea_orm(has_one = "super::bpmn_20_potential_owner::Entity")]
    PotentialOwner,
}

// SUPER : ONE HumanPerformer need ONE Performer
impl Related<super::bpmn_20_performer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Performer.def()
    }
}

// SUPER : ONE PotentialOwner need ONE HumanPerformer
impl Related<super::bpmn_20_potential_owner::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PotentialOwner.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "HumanPerformer",
//     name: "HumanPerformer",
//     is_abstract: false,
//     super_class: [
//         "Performer",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


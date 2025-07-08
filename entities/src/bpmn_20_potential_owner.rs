//! bpmn_20_class_potential_owner

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_potential_owner")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : HumanPerformer
    pub super_human_performer: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE PotentialOwner need ONE HumanPerformer
    #[sea_orm(
        belongs_to = "super::bpmn_20_human_performer::Entity",
        from = "Column::SuperHumanPerformer",
        to = "super::bpmn_20_human_performer::Column::Id",
        on_delete = "Cascade"
    )]
    HumanPerformer,
}

// SUPER : ONE PotentialOwner need ONE HumanPerformer
impl Related<super::bpmn_20_human_performer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::HumanPerformer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "PotentialOwner",
//     name: "PotentialOwner",
//     is_abstract: false,
//     super_class: [
//         "HumanPerformer",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


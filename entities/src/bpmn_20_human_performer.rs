//! bpmn_20_class_human_performer

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_human_performer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperPerformer
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

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-HumanPerformer',
//     name: "HumanPerformer",
//     is_abstract: false,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'BPMN20-Performer',
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#HumanPerformer",
//     table_name: "bpmn_20_human_performer",
//     model_name: "HumanPerformer",
//     full_name: "bpmn_20_class_human_performer",
//     reverse_super: RefCell {
//         value: [
//             (Weak),
//         ],
//     },
// }


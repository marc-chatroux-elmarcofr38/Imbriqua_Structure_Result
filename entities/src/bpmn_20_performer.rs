//! bpmn_20_class_performer

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_performer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperResourceRole
    pub super_resource_role: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Performer need ONE ResourceRole
    #[sea_orm(
        belongs_to = "super::bpmn_20_resource_role::Entity",
        from = "Column::SuperResourceRole",
        to = "super::bpmn_20_resource_role::Column::Id",
        on_delete = "Cascade"
    )]
    ResourceRole,
    // SUPER : ONE HumanPerformer need ONE Performer
    #[sea_orm(has_one = "super::bpmn_20_human_performer::Entity")]
    HumanPerformer,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-Performer',
//     name: "Performer",
//     is_abstract: false,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'BPMN20-ResourceRole',
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Performer",
//     table_name: "bpmn_20_performer",
//     model_name: "Performer",
//     full_name: "bpmn_20_class_performer",
//     reverse_super: RefCell {
//         value: [
//             (Weak),
//         ],
//     },
// }


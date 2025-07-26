//! bpmn_20_class_task

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperActivity
    pub super_activity: i64,
    /// SUPER FIELD : SuperInteractionNode
    pub super_interaction_node: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Task need ONE Activity
    #[sea_orm(
        belongs_to = "super::bpmn_20_activity::Entity",
        from = "Column::SuperActivity",
        to = "super::bpmn_20_activity::Column::Id",
        on_delete = "Cascade"
    )]
    Activity,
    // SUPER : ONE Task need ONE InteractionNode
    #[sea_orm(
        belongs_to = "super::bpmn_20_interaction_node::Entity",
        from = "Column::SuperInteractionNode",
        to = "super::bpmn_20_interaction_node::Column::Id",
        on_delete = "Cascade"
    )]
    InteractionNode,
    // SUPER : ONE BusinessRuleTask need ONE Task
    #[sea_orm(has_one = "super::bpmn_20_business_rule_task::Entity")]
    BusinessRuleTask,
    // SUPER : ONE ManualTask need ONE Task
    #[sea_orm(has_one = "super::bpmn_20_manual_task::Entity")]
    ManualTask,
    // SUPER : ONE ReceiveTask need ONE Task
    #[sea_orm(has_one = "super::bpmn_20_receive_task::Entity")]
    ReceiveTask,
    // SUPER : ONE ScriptTask need ONE Task
    #[sea_orm(has_one = "super::bpmn_20_script_task::Entity")]
    ScriptTask,
    // SUPER : ONE SendTask need ONE Task
    #[sea_orm(has_one = "super::bpmn_20_send_task::Entity")]
    SendTask,
    // SUPER : ONE ServiceTask need ONE Task
    #[sea_orm(has_one = "super::bpmn_20_service_task::Entity")]
    ServiceTask,
    // SUPER : ONE UserTask need ONE Task
    #[sea_orm(has_one = "super::bpmn_20_user_task::Entity")]
    UserTask,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-Task',
//     name: "Task",
//     is_abstract: false,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'BPMN20-Activity',
//         "Loaded XMIIdReference RefCell of 'BPMN20-InteractionNode',
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Task",
//     table_name: "bpmn_20_task",
//     model_name: "Task",
//     full_name: "bpmn_20_class_task",
//     reverse_super: RefCell {
//         value: [
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//         ],
//     },
// }


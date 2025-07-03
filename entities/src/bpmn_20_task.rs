//! bpmn_20_class_task

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Activity
    pub super_activity: i64,
    /// SUPER FIELD : InteractionNode
    pub super_interaction_node: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Task need ONE Activity
    #[sea_orm(
        belongs_to = "super::bpmn_20_activity::Entity",
        from = "Column::SuperActivity",
        to = "super::bpmn_20_activity::Column::Id"
    )]
    Activity,
    // SUPER : ONE Task need ONE InteractionNode
    #[sea_orm(
        belongs_to = "super::bpmn_20_interaction_node::Entity",
        from = "Column::SuperInteractionNode",
        to = "super::bpmn_20_interaction_node::Column::Id"
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

// SUPER : ONE Task need ONE Activity
impl Related<super::bpmn_20_activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Activity.def()
    }
}

// SUPER : ONE Task need ONE InteractionNode
impl Related<super::bpmn_20_interaction_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InteractionNode.def()
    }
}

// SUPER : ONE BusinessRuleTask need ONE Task
impl Related<super::bpmn_20_business_rule_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BusinessRuleTask.def()
    }
}

// SUPER : ONE ManualTask need ONE Task
impl Related<super::bpmn_20_manual_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ManualTask.def()
    }
}

// SUPER : ONE ReceiveTask need ONE Task
impl Related<super::bpmn_20_receive_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReceiveTask.def()
    }
}

// SUPER : ONE ScriptTask need ONE Task
impl Related<super::bpmn_20_script_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ScriptTask.def()
    }
}

// SUPER : ONE SendTask need ONE Task
impl Related<super::bpmn_20_send_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SendTask.def()
    }
}

// SUPER : ONE ServiceTask need ONE Task
impl Related<super::bpmn_20_service_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ServiceTask.def()
    }
}

// SUPER : ONE UserTask need ONE Task
impl Related<super::bpmn_20_user_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserTask.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "Task",
//     name: "Task",
//     is_abstract: false,
//     super_class: [
//         "Activity",
//         "InteractionNode",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


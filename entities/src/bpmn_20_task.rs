//! bpmn_20_class_task

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : Activity
    pub super_activity: i32,
    /// SIMPLE FIELD : InteractionNode
    pub super_interaction_node: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_activity::Entity",
        from = "Column::SuperActivity",
        to = "super::bpmn_20_activity::Column::Id"
    )]
    Activity,
    #[sea_orm(
        belongs_to = "super::bpmn_20_interaction_node::Entity",
        from = "Column::SuperInteractionNode",
        to = "super::bpmn_20_interaction_node::Column::Id"
    )]
    InteractionNode,
    #[sea_orm(has_one = "super::bpmn_20_business_rule_task::Entity")]
    BusinessRuleTask,
    #[sea_orm(has_one = "super::bpmn_20_manual_task::Entity")]
    ManualTask,
    #[sea_orm(has_one = "super::bpmn_20_receive_task::Entity")]
    ReceiveTask,
    #[sea_orm(has_one = "super::bpmn_20_script_task::Entity")]
    ScriptTask,
    #[sea_orm(has_one = "super::bpmn_20_send_task::Entity")]
    SendTask,
    #[sea_orm(has_one = "super::bpmn_20_service_task::Entity")]
    ServiceTask,
    #[sea_orm(has_one = "super::bpmn_20_user_task::Entity")]
    UserTask,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Activity.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_interaction_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InteractionNode.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_business_rule_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BusinessRuleTask.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_manual_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ManualTask.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_receive_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReceiveTask.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_script_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ScriptTask.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_send_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SendTask.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_service_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ServiceTask.def()
    }
}
// `Related` trait has to be implemented by hand
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


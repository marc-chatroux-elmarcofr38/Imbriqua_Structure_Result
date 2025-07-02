//! bpmn_20_class_event_definition

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_event_definition")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : RootElement
    pub super_root_element: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_root_element::Entity",
        from = "Column::SuperRootElement",
        to = "super::bpmn_20_root_element::Column::Id"
    )]
    RootElement,
    #[sea_orm(has_one = "super::bpmn_20_cancel_event_definition::Entity")]
    CancelEventDefinition,
    #[sea_orm(has_one = "super::bpmn_20_compensate_event_definition::Entity")]
    CompensateEventDefinition,
    #[sea_orm(has_one = "super::bpmn_20_conditional_event_definition::Entity")]
    ConditionalEventDefinition,
    #[sea_orm(has_one = "super::bpmn_20_error_event_definition::Entity")]
    ErrorEventDefinition,
    #[sea_orm(has_one = "super::bpmn_20_escalation_event_definition::Entity")]
    EscalationEventDefinition,
    #[sea_orm(has_one = "super::bpmn_20_link_event_definition::Entity")]
    LinkEventDefinition,
    #[sea_orm(has_one = "super::bpmn_20_message_event_definition::Entity")]
    MessageEventDefinition,
    #[sea_orm(has_one = "super::bpmn_20_signal_event_definition::Entity")]
    SignalEventDefinition,
    #[sea_orm(has_one = "super::bpmn_20_terminate_event_definition::Entity")]
    TerminateEventDefinition,
    #[sea_orm(has_one = "super::bpmn_20_timer_event_definition::Entity")]
    TimerEventDefinition,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_root_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RootElement.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_cancel_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CancelEventDefinition.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_compensate_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CompensateEventDefinition.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_conditional_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConditionalEventDefinition.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_error_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ErrorEventDefinition.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_escalation_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EscalationEventDefinition.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_link_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LinkEventDefinition.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_message_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MessageEventDefinition.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_signal_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SignalEventDefinition.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_terminate_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TerminateEventDefinition.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_timer_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TimerEventDefinition.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "EventDefinition",
//     name: "EventDefinition",
//     is_abstract: true,
//     super_class: [
//         "RootElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


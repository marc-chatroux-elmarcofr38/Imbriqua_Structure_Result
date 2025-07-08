//! bpmn_20_class_event_definition

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_event_definition")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : RootElement
    pub super_root_element: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE EventDefinition need ONE RootElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_root_element::Entity",
        from = "Column::SuperRootElement",
        to = "super::bpmn_20_root_element::Column::Id",
        on_delete = "Cascade"
    )]
    RootElement,
    // SUPER : ONE CancelEventDefinition need ONE EventDefinition
    #[sea_orm(has_one = "super::bpmn_20_cancel_event_definition::Entity")]
    CancelEventDefinition,
    // SUPER : ONE CompensateEventDefinition need ONE EventDefinition
    #[sea_orm(has_one = "super::bpmn_20_compensate_event_definition::Entity")]
    CompensateEventDefinition,
    // SUPER : ONE ConditionalEventDefinition need ONE EventDefinition
    #[sea_orm(has_one = "super::bpmn_20_conditional_event_definition::Entity")]
    ConditionalEventDefinition,
    // SUPER : ONE ErrorEventDefinition need ONE EventDefinition
    #[sea_orm(has_one = "super::bpmn_20_error_event_definition::Entity")]
    ErrorEventDefinition,
    // SUPER : ONE EscalationEventDefinition need ONE EventDefinition
    #[sea_orm(has_one = "super::bpmn_20_escalation_event_definition::Entity")]
    EscalationEventDefinition,
    // SUPER : ONE LinkEventDefinition need ONE EventDefinition
    #[sea_orm(has_one = "super::bpmn_20_link_event_definition::Entity")]
    LinkEventDefinition,
    // SUPER : ONE MessageEventDefinition need ONE EventDefinition
    #[sea_orm(has_one = "super::bpmn_20_message_event_definition::Entity")]
    MessageEventDefinition,
    // SUPER : ONE SignalEventDefinition need ONE EventDefinition
    #[sea_orm(has_one = "super::bpmn_20_signal_event_definition::Entity")]
    SignalEventDefinition,
    // SUPER : ONE TerminateEventDefinition need ONE EventDefinition
    #[sea_orm(has_one = "super::bpmn_20_terminate_event_definition::Entity")]
    TerminateEventDefinition,
    // SUPER : ONE TimerEventDefinition need ONE EventDefinition
    #[sea_orm(has_one = "super::bpmn_20_timer_event_definition::Entity")]
    TimerEventDefinition,
}

// SUPER : ONE EventDefinition need ONE RootElement
impl Related<super::bpmn_20_root_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RootElement.def()
    }
}

// SUPER : ONE CancelEventDefinition need ONE EventDefinition
impl Related<super::bpmn_20_cancel_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CancelEventDefinition.def()
    }
}

// SUPER : ONE CompensateEventDefinition need ONE EventDefinition
impl Related<super::bpmn_20_compensate_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CompensateEventDefinition.def()
    }
}

// SUPER : ONE ConditionalEventDefinition need ONE EventDefinition
impl Related<super::bpmn_20_conditional_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConditionalEventDefinition.def()
    }
}

// SUPER : ONE ErrorEventDefinition need ONE EventDefinition
impl Related<super::bpmn_20_error_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ErrorEventDefinition.def()
    }
}

// SUPER : ONE EscalationEventDefinition need ONE EventDefinition
impl Related<super::bpmn_20_escalation_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EscalationEventDefinition.def()
    }
}

// SUPER : ONE LinkEventDefinition need ONE EventDefinition
impl Related<super::bpmn_20_link_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LinkEventDefinition.def()
    }
}

// SUPER : ONE MessageEventDefinition need ONE EventDefinition
impl Related<super::bpmn_20_message_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MessageEventDefinition.def()
    }
}

// SUPER : ONE SignalEventDefinition need ONE EventDefinition
impl Related<super::bpmn_20_signal_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SignalEventDefinition.def()
    }
}

// SUPER : ONE TerminateEventDefinition need ONE EventDefinition
impl Related<super::bpmn_20_terminate_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TerminateEventDefinition.def()
    }
}

// SUPER : ONE TimerEventDefinition need ONE EventDefinition
impl Related<super::bpmn_20_timer_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TimerEventDefinition.def()
    }
}

// ManyToMany : with CatchEvent using A_eventDefinitionRefs_catchEvent
impl Related<super::bpmn_20_a_event_definition_refs_catch_event::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_event_definition_refs_catch_event::Relation::CatchEvent.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_event_definition_refs_catch_event::Relation::EventDefinition
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with ThrowEvent using A_eventDefinitionRefs_throwEvent
impl Related<super::bpmn_20_a_event_definition_refs_throw_event::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_event_definition_refs_throw_event::Relation::ThrowEvent.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_event_definition_refs_throw_event::Relation::EventDefinition
                .def()
                .rev(),
        )
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


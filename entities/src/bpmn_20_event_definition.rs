//! bpmn_20_class_event_definition

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_event_definition")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperRootElement
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

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-EventDefinition',
//     name: "EventDefinition",
//     is_abstract: true,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'BPMN20-RootElement',
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#EventDefinition",
//     table_name: "bpmn_20_event_definition",
//     model_name: "EventDefinition",
//     full_name: "bpmn_20_class_event_definition",
//     reverse_super: RefCell {
//         value: [
//             (Weak),
//             (Weak),
//             (Weak),
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


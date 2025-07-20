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

impl ActiveModel {
    /// # Help document for "EventDefinition" (bpmn_20_class_event_definition)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __CatchEvent__ (__CatchEventModel__) from A_eventDefinitions_catchEvent
    ///   * one-to-many link : (0-1) __EventDefinition__ need (0-inf) __CatchEvent__)
    ///   * callable using find_with_related(__CatchEventModel__) from __EventDefinition__
    ///   * named catch_event in BPMN
    /// * __ThrowEvent__ (__ThrowEventModel__) from A_eventDefinitions_throwEvent
    ///   * one-to-many link : (0-1) __EventDefinition__ need (0-inf) __ThrowEvent__)
    ///   * callable using find_with_related(__ThrowEventModel__) from __EventDefinition__
    ///   * named throw_event in BPMN
    /// 
    /// ## Direct Super :
    /// * __RootElement__ (__RootElementModel__)
    ///   * one-to-one link : one __EventDefinition__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __EventDefinition__
    ///   * saved in __super_root_element__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __CancelEventDefinition__ (__CancelEventDefinitionModel__)
    ///   * one-to-one link (reverse) : one __CancelEventDefinition__ need one __EventDefinition__)
    ///   * callable using find_also_related(__EventDefinitionModel__) from __CancelEventDefinition__
    ///   * saved in __super_event_definition__ field as foreing key in __CancelEventDefinitionModel__
    /// * __CompensateEventDefinition__ (__CompensateEventDefinitionModel__)
    ///   * one-to-one link (reverse) : one __CompensateEventDefinition__ need one __EventDefinition__)
    ///   * callable using find_also_related(__EventDefinitionModel__) from __CompensateEventDefinition__
    ///   * saved in __super_event_definition__ field as foreing key in __CompensateEventDefinitionModel__
    /// * __ConditionalEventDefinition__ (__ConditionalEventDefinitionModel__)
    ///   * one-to-one link (reverse) : one __ConditionalEventDefinition__ need one __EventDefinition__)
    ///   * callable using find_also_related(__EventDefinitionModel__) from __ConditionalEventDefinition__
    ///   * saved in __super_event_definition__ field as foreing key in __ConditionalEventDefinitionModel__
    /// * __ErrorEventDefinition__ (__ErrorEventDefinitionModel__)
    ///   * one-to-one link (reverse) : one __ErrorEventDefinition__ need one __EventDefinition__)
    ///   * callable using find_also_related(__EventDefinitionModel__) from __ErrorEventDefinition__
    ///   * saved in __super_event_definition__ field as foreing key in __ErrorEventDefinitionModel__
    /// * __EscalationEventDefinition__ (__EscalationEventDefinitionModel__)
    ///   * one-to-one link (reverse) : one __EscalationEventDefinition__ need one __EventDefinition__)
    ///   * callable using find_also_related(__EventDefinitionModel__) from __EscalationEventDefinition__
    ///   * saved in __super_event_definition__ field as foreing key in __EscalationEventDefinitionModel__
    /// * __LinkEventDefinition__ (__LinkEventDefinitionModel__)
    ///   * one-to-one link (reverse) : one __LinkEventDefinition__ need one __EventDefinition__)
    ///   * callable using find_also_related(__EventDefinitionModel__) from __LinkEventDefinition__
    ///   * saved in __super_event_definition__ field as foreing key in __LinkEventDefinitionModel__
    /// * __MessageEventDefinition__ (__MessageEventDefinitionModel__)
    ///   * one-to-one link (reverse) : one __MessageEventDefinition__ need one __EventDefinition__)
    ///   * callable using find_also_related(__EventDefinitionModel__) from __MessageEventDefinition__
    ///   * saved in __super_event_definition__ field as foreing key in __MessageEventDefinitionModel__
    /// * __SignalEventDefinition__ (__SignalEventDefinitionModel__)
    ///   * one-to-one link (reverse) : one __SignalEventDefinition__ need one __EventDefinition__)
    ///   * callable using find_also_related(__EventDefinitionModel__) from __SignalEventDefinition__
    ///   * saved in __super_event_definition__ field as foreing key in __SignalEventDefinitionModel__
    /// * __TerminateEventDefinition__ (__TerminateEventDefinitionModel__)
    ///   * one-to-one link (reverse) : one __TerminateEventDefinition__ need one __EventDefinition__)
    ///   * callable using find_also_related(__EventDefinitionModel__) from __TerminateEventDefinition__
    ///   * saved in __super_event_definition__ field as foreing key in __TerminateEventDefinitionModel__
    /// * __TimerEventDefinition__ (__TimerEventDefinitionModel__)
    ///   * one-to-one link (reverse) : one __TimerEventDefinition__ need one __EventDefinition__)
    ///   * callable using find_also_related(__EventDefinitionModel__) from __TimerEventDefinition__
    ///   * saved in __super_event_definition__ field as foreing key in __TimerEventDefinitionModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "EventDefinition" (bpmn_20_class_event_definition)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Relation : One To Many :
* __CatchEvent__ (__CatchEventModel__) from A_eventDefinitions_catchEvent
  * one-to-many link : (0-1) __EventDefinition__ need (0-inf) __CatchEvent__)
  * callable using find_with_related(__CatchEventModel__) from __EventDefinition__
  * named catch_event in BPMN
* __ThrowEvent__ (__ThrowEventModel__) from A_eventDefinitions_throwEvent
  * one-to-many link : (0-1) __EventDefinition__ need (0-inf) __ThrowEvent__)
  * callable using find_with_related(__ThrowEventModel__) from __EventDefinition__
  * named throw_event in BPMN

## Direct Super :
* __RootElement__ (__RootElementModel__)
  * one-to-one link : one __EventDefinition__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __EventDefinition__
  * saved in __super_root_element__ field as foreing key

## Reverse Super :
* __CancelEventDefinition__ (__CancelEventDefinitionModel__)
  * one-to-one link (reverse) : one __CancelEventDefinition__ need one __EventDefinition__)
  * callable using find_also_related(__EventDefinitionModel__) from __CancelEventDefinition__
  * saved in __super_event_definition__ field as foreing key in __CancelEventDefinitionModel__
* __CompensateEventDefinition__ (__CompensateEventDefinitionModel__)
  * one-to-one link (reverse) : one __CompensateEventDefinition__ need one __EventDefinition__)
  * callable using find_also_related(__EventDefinitionModel__) from __CompensateEventDefinition__
  * saved in __super_event_definition__ field as foreing key in __CompensateEventDefinitionModel__
* __ConditionalEventDefinition__ (__ConditionalEventDefinitionModel__)
  * one-to-one link (reverse) : one __ConditionalEventDefinition__ need one __EventDefinition__)
  * callable using find_also_related(__EventDefinitionModel__) from __ConditionalEventDefinition__
  * saved in __super_event_definition__ field as foreing key in __ConditionalEventDefinitionModel__
* __ErrorEventDefinition__ (__ErrorEventDefinitionModel__)
  * one-to-one link (reverse) : one __ErrorEventDefinition__ need one __EventDefinition__)
  * callable using find_also_related(__EventDefinitionModel__) from __ErrorEventDefinition__
  * saved in __super_event_definition__ field as foreing key in __ErrorEventDefinitionModel__
* __EscalationEventDefinition__ (__EscalationEventDefinitionModel__)
  * one-to-one link (reverse) : one __EscalationEventDefinition__ need one __EventDefinition__)
  * callable using find_also_related(__EventDefinitionModel__) from __EscalationEventDefinition__
  * saved in __super_event_definition__ field as foreing key in __EscalationEventDefinitionModel__
* __LinkEventDefinition__ (__LinkEventDefinitionModel__)
  * one-to-one link (reverse) : one __LinkEventDefinition__ need one __EventDefinition__)
  * callable using find_also_related(__EventDefinitionModel__) from __LinkEventDefinition__
  * saved in __super_event_definition__ field as foreing key in __LinkEventDefinitionModel__
* __MessageEventDefinition__ (__MessageEventDefinitionModel__)
  * one-to-one link (reverse) : one __MessageEventDefinition__ need one __EventDefinition__)
  * callable using find_also_related(__EventDefinitionModel__) from __MessageEventDefinition__
  * saved in __super_event_definition__ field as foreing key in __MessageEventDefinitionModel__
* __SignalEventDefinition__ (__SignalEventDefinitionModel__)
  * one-to-one link (reverse) : one __SignalEventDefinition__ need one __EventDefinition__)
  * callable using find_also_related(__EventDefinitionModel__) from __SignalEventDefinition__
  * saved in __super_event_definition__ field as foreing key in __SignalEventDefinitionModel__
* __TerminateEventDefinition__ (__TerminateEventDefinitionModel__)
  * one-to-one link (reverse) : one __TerminateEventDefinition__ need one __EventDefinition__)
  * callable using find_also_related(__EventDefinitionModel__) from __TerminateEventDefinition__
  * saved in __super_event_definition__ field as foreing key in __TerminateEventDefinitionModel__
* __TimerEventDefinition__ (__TimerEventDefinitionModel__)
  * one-to-one link (reverse) : one __TimerEventDefinition__ need one __EventDefinition__)
  * callable using find_also_related(__EventDefinitionModel__) from __TimerEventDefinition__
  * saved in __super_event_definition__ field as foreing key in __TimerEventDefinitionModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Weak ref of "BPMN20-EventDefinition" (loaded : false)",
//     name: "EventDefinition",
//     is_abstract: true,
//     super_class: [
//         "RootElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#EventDefinition",
//     table_name: "bpmn_20_event_definition",
//     model_name: "EventDefinition",
//     full_name: "bpmn_20_class_event_definition",
// }


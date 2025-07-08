//! bpmn_20_class_throw_event

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_throw_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Event
    pub super_event: i64,
    /// COMPLEX FIELD : ThrowEvent-inputSet
    pub input_set: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ThrowEvent need ONE Event
    #[sea_orm(
        belongs_to = "super::bpmn_20_event::Entity",
        from = "Column::SuperEvent",
        to = "super::bpmn_20_event::Column::Id",
        on_delete = "Cascade"
    )]
    Event,
    // SUPER : ONE EndEvent need ONE ThrowEvent
    #[sea_orm(has_one = "super::bpmn_20_end_event::Entity")]
    EndEvent,
    // SUPER : ONE ImplicitThrowEvent need ONE ThrowEvent
    #[sea_orm(has_one = "super::bpmn_20_implicit_throw_event::Entity")]
    ImplicitThrowEvent,
    // SUPER : ONE IntermediateThrowEvent need ONE ThrowEvent
    #[sea_orm(has_one = "super::bpmn_20_intermediate_throw_event::Entity")]
    IntermediateThrowEvent,
}

// SUPER : ONE ThrowEvent need ONE Event
impl Related<super::bpmn_20_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Event.def()
    }
}

// SUPER : ONE EndEvent need ONE ThrowEvent
impl Related<super::bpmn_20_end_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EndEvent.def()
    }
}

// SUPER : ONE ImplicitThrowEvent need ONE ThrowEvent
impl Related<super::bpmn_20_implicit_throw_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ImplicitThrowEvent.def()
    }
}

// SUPER : ONE IntermediateThrowEvent need ONE ThrowEvent
impl Related<super::bpmn_20_intermediate_throw_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IntermediateThrowEvent.def()
    }
}

// ManyToMany : with EventDefinition using A_eventDefinitionRefs_throwEvent
impl Related<super::bpmn_20_a_event_definition_refs_throw_event::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_event_definition_refs_throw_event::Relation::EventDefinition.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_event_definition_refs_throw_event::Relation::ThrowEvent
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "ThrowEvent",
//     name: "ThrowEvent",
//     is_abstract: true,
//     super_class: [
//         "Event",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "ThrowEvent-inputSet",
//                 name: "inputSet",
//                 visibility: Public,
//                 simple_type: Some(
//                     "InputSet",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_inputSet_throwEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ThrowEvent-eventDefinitionRefs",
//                 name: "eventDefinitionRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "EventDefinition",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_eventDefinitionRefs_throwEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ThrowEvent-dataInputAssociation",
//                 name: "dataInputAssociation",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataInputAssociation",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_dataInputAssociation_throwEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ThrowEvent-dataInputs",
//                 name: "dataInputs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataInput",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_dataInputs_throwEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ThrowEvent-eventDefinitions",
//                 name: "eventDefinitions",
//                 visibility: Public,
//                 simple_type: Some(
//                     "EventDefinition",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_eventDefinitions_throwEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }


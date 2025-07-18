//! bpmn_20_association_a_event_definition_refs_throw_event

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_event_definition_refs_throw_event")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub event_definition_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub throw_event_b_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_event_definition::Entity",
        from = "Column::EventDefinitionAId",
        to = "super::bpmn_20_event_definition::Column::Id"
    )]
    EventDefinition,
    #[sea_orm(
        belongs_to = "super::bpmn_20_throw_event::Entity",
        from = "Column::ThrowEventBId",
        to = "super::bpmn_20_throw_event::Column::Id"
    )]
    ThrowEvent,
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFAssociation {
//     xmi_id: XMIIdReference {
//         local_id: "A_eventDefinitionRefs_throwEvent",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "A_eventDefinitionRefs_throwEvent",
//     visibility: Private,
//     member_end: (
//         "ThrowEvent-eventDefinitionRefs",
//         "A_eventDefinitionRefs_throwEvent-throwEvent",
//     ),
//     owned_end: {
//         "-A_eventDefinitionRefs_throwEvent-throwEvent": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "A_eventDefinitionRefs_throwEvent-throwEvent",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "throwEvent",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ThrowEvent",
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
//                 owning_association: "A_eventDefinitionRefs_throwEvent",
//                 association: Some(
//                     "A_eventDefinitionRefs_throwEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     is_derived: false,
//     technical_name: "BPMN20.cmof#A_eventDefinitionRefs_throwEvent",
//     table_name: "bpmn_20_a_event_definition_refs_throw_event",
//     model_name: "AEventDefinitionRefsThrowEvent",
//     full_name: "bpmn_20_association_a_event_definition_refs_throw_event",
// }


//! bpmn_20_association_a_event_definition_refs_catch_event

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_event_definition_refs_catch_event")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub event_definition_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub catch_event_b_id: i64,
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
        belongs_to = "super::bpmn_20_catch_event::Entity",
        from = "Column::CatchEventBId",
        to = "super::bpmn_20_catch_event::Column::Id"
    )]
    CatchEvent,
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFAssociation {
//     xmi_id: XMIIdReference {
//         local_id: "A_eventDefinitionRefs_catchEvent",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "A_eventDefinitionRefs_catchEvent",
//     visibility: Private,
//     member_end: (
//         "CatchEvent-eventDefinitionRefs",
//         "A_eventDefinitionRefs_catchEvent-catchEvent",
//     ),
//     owned_end: {
//         "-A_eventDefinitionRefs_catchEvent-catchEvent": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "A_eventDefinitionRefs_catchEvent-catchEvent",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "catchEvent",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CatchEvent",
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
//                 owning_association: "A_eventDefinitionRefs_catchEvent",
//                 association: Some(
//                     "A_eventDefinitionRefs_catchEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     is_derived: false,
//     technical_name: "BPMN20.cmof#A_eventDefinitionRefs_catchEvent",
//     table_name: "bpmn_20_a_event_definition_refs_catch_event",
//     model_name: "AEventDefinitionRefsCatchEvent",
//     full_name: "bpmn_20_association_a_event_definition_refs_catch_event",
// }


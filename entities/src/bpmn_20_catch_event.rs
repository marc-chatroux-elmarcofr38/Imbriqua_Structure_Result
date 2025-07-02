//! bpmn_20_class_catch_event

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_catch_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SUPER FIELD : Event
    pub super_event: i32,
    /// COMPLEX FIELD : CatchEvent-outputSet
    pub output_set: Option<i32>,
    /// SIMPLE FIELD : CatchEvent-parallelMultiple
    pub parallel_multiple: std::primitive::bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE CatchEvent need ONE Event
    #[sea_orm(
        belongs_to = "super::bpmn_20_event::Entity",
        from = "Column::SuperEvent",
        to = "super::bpmn_20_event::Column::Id"
    )]
    Event,
    // SUPER : ONE BoundaryEvent need ONE CatchEvent
    #[sea_orm(has_one = "super::bpmn_20_boundary_event::Entity")]
    BoundaryEvent,
    // SUPER : ONE IntermediateCatchEvent need ONE CatchEvent
    #[sea_orm(has_one = "super::bpmn_20_intermediate_catch_event::Entity")]
    IntermediateCatchEvent,
    // SUPER : ONE StartEvent need ONE CatchEvent
    #[sea_orm(has_one = "super::bpmn_20_start_event::Entity")]
    StartEvent,
}

// SUPER : ONE CatchEvent need ONE Event
impl Related<super::bpmn_20_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Event.def()
    }
}

// SUPER : ONE BoundaryEvent need ONE CatchEvent
impl Related<super::bpmn_20_boundary_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BoundaryEvent.def()
    }
}

// SUPER : ONE IntermediateCatchEvent need ONE CatchEvent
impl Related<super::bpmn_20_intermediate_catch_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IntermediateCatchEvent.def()
    }
}

// SUPER : ONE StartEvent need ONE CatchEvent
impl Related<super::bpmn_20_start_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StartEvent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "CatchEvent",
//     name: "CatchEvent",
//     is_abstract: true,
//     super_class: [
//         "Event",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "CatchEvent-parallelMultiple",
//                 name: "parallelMultiple",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#Boolean",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "CatchEvent-outputSet",
//                 name: "outputSet",
//                 visibility: Public,
//                 simple_type: Some(
//                     "OutputSet",
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
//                     "A_outputSet_catchEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "CatchEvent-eventDefinitionRefs",
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
//                     "A_eventDefinitionRefs_catchEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "CatchEvent-dataOutputAssociation",
//                 name: "dataOutputAssociation",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataOutputAssociation",
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
//                     "A_dataOutputAssociation_catchEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "CatchEvent-dataOutputs",
//                 name: "dataOutputs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataOutput",
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
//                     "A_dataOutputs_catchEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "CatchEvent-eventDefinitions",
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
//                     "A_eventDefinitions_catchEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }


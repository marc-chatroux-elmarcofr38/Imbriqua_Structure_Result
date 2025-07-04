//! bpmn_20_class_choreography_activity

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_choreography_activity")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : FlowNode
    pub super_flow_node: i64,
    /// COMPLEX FIELD : ChoreographyActivity-initiatingParticipantRef
    pub initiating_participant_ref: i64,
    /// SIMPLE FIELD : ChoreographyActivity-loopType
    #[sea_orm(default_value = "None")]
    pub loop_type: ChoreographyLoopType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ChoreographyActivity need ONE FlowNode
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_node::Entity",
        from = "Column::SuperFlowNode",
        to = "super::bpmn_20_flow_node::Column::Id"
    )]
    FlowNode,
    // SUPER : ONE CallChoreography need ONE ChoreographyActivity
    #[sea_orm(has_one = "super::bpmn_20_call_choreography::Entity")]
    CallChoreography,
    // SUPER : ONE ChoreographyTask need ONE ChoreographyActivity
    #[sea_orm(has_one = "super::bpmn_20_choreography_task::Entity")]
    ChoreographyTask,
    // SUPER : ONE SubChoreography need ONE ChoreographyActivity
    #[sea_orm(has_one = "super::bpmn_20_sub_choreography::Entity")]
    SubChoreography,
}

// SUPER : ONE ChoreographyActivity need ONE FlowNode
impl Related<super::bpmn_20_flow_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowNode.def()
    }
}

// SUPER : ONE CallChoreography need ONE ChoreographyActivity
impl Related<super::bpmn_20_call_choreography::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CallChoreography.def()
    }
}

// SUPER : ONE ChoreographyTask need ONE ChoreographyActivity
impl Related<super::bpmn_20_choreography_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChoreographyTask.def()
    }
}

// SUPER : ONE SubChoreography need ONE ChoreographyActivity
impl Related<super::bpmn_20_sub_choreography::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubChoreography.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "ChoreographyActivity",
//     name: "ChoreographyActivity",
//     is_abstract: true,
//     super_class: [
//         "FlowNode",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "ChoreographyActivity-participantRefs",
//                 name: "participantRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Participant",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 2,
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
//                     "A_participantRefs_choreographyActivity",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ChoreographyActivity-initiatingParticipantRef",
//                 name: "initiatingParticipantRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Participant",
//                 ),
//                 complex_type: None,
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
//                 association: Some(
//                     "A_initiatingParticipantRef_choreographyActivity",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ChoreographyActivity-correlationKeys",
//                 name: "correlationKeys",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CorrelationKey",
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
//                     "A_correlationKeys_choreographyActivity",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ChoreographyActivity-loopType",
//                 name: "loopType",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ChoreographyLoopType",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: Some(
//                     "None",
//                 ),
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
//     ],
//     owned_rule: [],
// }


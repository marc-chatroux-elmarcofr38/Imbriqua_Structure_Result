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
        to = "super::bpmn_20_flow_node::Column::Id",
        on_delete = "Cascade"
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

// ManyToMany : with Participant using A_participantRefs_choreographyActivity
impl Related<super::bpmn_20_a_participant_refs_choreography_activity::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_participant_refs_choreography_activity::Relation::Participant.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_participant_refs_choreography_activity::Relation::ChoreographyActivity
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ChoreographyActivity" (bpmn_20_class_choreography_activity)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __loop_type__ (xmi_id : "ChoreographyActivity-loopType")
    ///   * type : __ChoreographyLoopType__
    ///   * default : "None"
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __Participant__ (__ParticipantModel__) from A_initiatingParticipantRef_choreographyActivity
    ///   * one-to-many link : (1-1) __ChoreographyActivity__ need (0-inf) __Participant__)
    ///   * callable using find_with_related(__ParticipantModel__) from __ChoreographyActivity__
    /// 
    /// ## Direct Super :
    /// * __FlowNode__ (__FlowNodeModel__)
    ///   * one-to-one link : one __ChoreographyActivity__ need one __FlowNode__)
    ///   * callable using find_also_related(__FlowNodeModel__) from __ChoreographyActivity__
    ///   * saved in __super_flow_node__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __CallChoreography__ (__CallChoreographyModel__)
    ///   * one-to-one link (reverse) : one __CallChoreography__ need one __ChoreographyActivity__)
    ///   * callable using find_also_related(__ChoreographyActivityModel__) from __CallChoreography__
    ///   * saved in __super_choreography_activity__ field as foreing key in __CallChoreographyModel__
    /// * __ChoreographyTask__ (__ChoreographyTaskModel__)
    ///   * one-to-one link (reverse) : one __ChoreographyTask__ need one __ChoreographyActivity__)
    ///   * callable using find_also_related(__ChoreographyActivityModel__) from __ChoreographyTask__
    ///   * saved in __super_choreography_activity__ field as foreing key in __ChoreographyTaskModel__
    /// * __SubChoreography__ (__SubChoreographyModel__)
    ///   * one-to-one link (reverse) : one __SubChoreography__ need one __ChoreographyActivity__)
    ///   * callable using find_also_related(__ChoreographyActivityModel__) from __SubChoreography__
    ///   * saved in __super_choreography_activity__ field as foreing key in __SubChoreographyModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ChoreographyActivity" (bpmn_20_class_choreography_activity)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __loop_type__ (xmi_id : "ChoreographyActivity-loopType")
  * type : __ChoreographyLoopType__
  * default : "None"


## Relation : One To Many :
* __Participant__ (__ParticipantModel__) from A_initiatingParticipantRef_choreographyActivity
  * one-to-many link : (1-1) __ChoreographyActivity__ need (0-inf) __Participant__)
  * callable using find_with_related(__ParticipantModel__) from __ChoreographyActivity__

## Direct Super :
* __FlowNode__ (__FlowNodeModel__)
  * one-to-one link : one __ChoreographyActivity__ need one __FlowNode__)
  * callable using find_also_related(__FlowNodeModel__) from __ChoreographyActivity__
  * saved in __super_flow_node__ field as foreing key

## Reverse Super :
* __CallChoreography__ (__CallChoreographyModel__)
  * one-to-one link (reverse) : one __CallChoreography__ need one __ChoreographyActivity__)
  * callable using find_also_related(__ChoreographyActivityModel__) from __CallChoreography__
  * saved in __super_choreography_activity__ field as foreing key in __CallChoreographyModel__
* __ChoreographyTask__ (__ChoreographyTaskModel__)
  * one-to-one link (reverse) : one __ChoreographyTask__ need one __ChoreographyActivity__)
  * callable using find_also_related(__ChoreographyActivityModel__) from __ChoreographyTask__
  * saved in __super_choreography_activity__ field as foreing key in __ChoreographyTaskModel__
* __SubChoreography__ (__SubChoreographyModel__)
  * one-to-one link (reverse) : one __SubChoreography__ need one __ChoreographyActivity__)
  * callable using find_also_related(__ChoreographyActivityModel__) from __SubChoreography__
  * saved in __super_choreography_activity__ field as foreing key in __SubChoreographyModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "ChoreographyActivity",
//     name: "ChoreographyActivity",
//     is_abstract: true,
//     super_class: [
//         "FlowNode",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "ChoreographyActivity-correlationKeys": Property(
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
//         "ChoreographyActivity-initiatingParticipantRef": Property(
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
//         "ChoreographyActivity-loopType": Property(
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
//         "ChoreographyActivity-participantRefs": Property(
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
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#ChoreographyActivity",
//     table_name: "bpmn_20_choreography_activity",
//     model_name: "ChoreographyActivity",
//     full_name: "bpmn_20_class_choreography_activity",
// }


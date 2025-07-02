//! bpmn_20_class_flow_node

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_flow_node")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : FlowElement
    pub super_flow_element: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_element::Entity",
        from = "Column::SuperFlowElement",
        to = "super::bpmn_20_flow_element::Column::Id"
    )]
    FlowElement,
    #[sea_orm(has_one = "super::bpmn_20_activity::Entity")]
    Activity,
    #[sea_orm(has_one = "super::bpmn_20_choreography_activity::Entity")]
    ChoreographyActivity,
    #[sea_orm(has_one = "super::bpmn_20_event::Entity")]
    Event,
    #[sea_orm(has_one = "super::bpmn_20_gateway::Entity")]
    Gateway,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_flow_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowElement.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Activity.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_choreography_activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChoreographyActivity.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Event.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_gateway::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Gateway.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "FlowNode",
//     name: "FlowNode",
//     is_abstract: true,
//     super_class: [
//         "FlowElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "FlowNode-outgoing",
//                 name: "outgoing",
//                 visibility: Public,
//                 simple_type: Some(
//                     "SequenceFlow",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: true,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_sourceRef_outgoing_flow",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "FlowNode-incoming",
//                 name: "incoming",
//                 visibility: Public,
//                 simple_type: Some(
//                     "SequenceFlow",
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
//                     "A_targetRef_incoming_flow",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "FlowNode-lanes",
//                 name: "lanes",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Lane",
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
//                 is_derived: true,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_flowNodeRefs_lanes",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }


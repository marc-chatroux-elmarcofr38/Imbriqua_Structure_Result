//! bpmn_20_class_flow_node

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_flow_node")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : FlowElement
    pub super_flow_element: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE FlowNode need ONE FlowElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_element::Entity",
        from = "Column::SuperFlowElement",
        to = "super::bpmn_20_flow_element::Column::Id"
    )]
    FlowElement,
    // SUPER : ONE Activity need ONE FlowNode
    #[sea_orm(has_one = "super::bpmn_20_activity::Entity")]
    Activity,
    // SUPER : ONE ChoreographyActivity need ONE FlowNode
    #[sea_orm(has_one = "super::bpmn_20_choreography_activity::Entity")]
    ChoreographyActivity,
    // SUPER : ONE Event need ONE FlowNode
    #[sea_orm(has_one = "super::bpmn_20_event::Entity")]
    Event,
    // SUPER : ONE Gateway need ONE FlowNode
    #[sea_orm(has_one = "super::bpmn_20_gateway::Entity")]
    Gateway,
}

// SUPER : ONE FlowNode need ONE FlowElement
impl Related<super::bpmn_20_flow_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowElement.def()
    }
}

// SUPER : ONE Activity need ONE FlowNode
impl Related<super::bpmn_20_activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Activity.def()
    }
}

// SUPER : ONE ChoreographyActivity need ONE FlowNode
impl Related<super::bpmn_20_choreography_activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChoreographyActivity.def()
    }
}

// SUPER : ONE Event need ONE FlowNode
impl Related<super::bpmn_20_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Event.def()
    }
}

// SUPER : ONE Gateway need ONE FlowNode
impl Related<super::bpmn_20_gateway::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Gateway.def()
    }
}

// ManyToMany : with Lane using A_flowNodeRefs_lanes
impl Related<super::bpmn_20_a_flow_node_refs_lanes::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_flow_node_refs_lanes::Relation::Lane.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_flow_node_refs_lanes::Relation::FlowNode
                .def()
                .rev(),
        )
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


//! bpmn_20_class_event

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : FlowNode
    pub super_flow_node: i64,
    /// SUPER FIELD : InteractionNode
    pub super_interaction_node: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Event need ONE FlowNode
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_node::Entity",
        from = "Column::SuperFlowNode",
        to = "super::bpmn_20_flow_node::Column::Id",
        on_delete = "Cascade"
    )]
    FlowNode,
    // SUPER : ONE Event need ONE InteractionNode
    #[sea_orm(
        belongs_to = "super::bpmn_20_interaction_node::Entity",
        from = "Column::SuperInteractionNode",
        to = "super::bpmn_20_interaction_node::Column::Id",
        on_delete = "Cascade"
    )]
    InteractionNode,
    // SUPER : ONE CatchEvent need ONE Event
    #[sea_orm(has_one = "super::bpmn_20_catch_event::Entity")]
    CatchEvent,
    // SUPER : ONE ThrowEvent need ONE Event
    #[sea_orm(has_one = "super::bpmn_20_throw_event::Entity")]
    ThrowEvent,
}

// SUPER : ONE Event need ONE FlowNode
impl Related<super::bpmn_20_flow_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowNode.def()
    }
}

// SUPER : ONE Event need ONE InteractionNode
impl Related<super::bpmn_20_interaction_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InteractionNode.def()
    }
}

// SUPER : ONE CatchEvent need ONE Event
impl Related<super::bpmn_20_catch_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CatchEvent.def()
    }
}

// SUPER : ONE ThrowEvent need ONE Event
impl Related<super::bpmn_20_throw_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ThrowEvent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "Event",
//     name: "Event",
//     is_abstract: true,
//     super_class: [
//         "FlowNode",
//         "InteractionNode",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "Event-properties",
//                 name: "properties",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Property",
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
//                     "A_properties_event",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }


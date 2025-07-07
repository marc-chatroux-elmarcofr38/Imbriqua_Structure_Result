//! bpmn_20_association_a_flow_node_refs_lanes

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_flow_node_refs_lanes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub flow_node_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub lane_b_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_node::Entity",
        from = "Column::FlowNodeAId",
        to = "super::bpmn_20_flow_node::Column::Id"
    )]
    FlowNode,
    #[sea_orm(
        belongs_to = "super::bpmn_20_lane::Entity",
        from = "Column::LaneBId",
        to = "super::bpmn_20_lane::Column::Id"
    )]
    Lane,
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFAssociation {
//     xmi_id: "A_flowNodeRefs_lanes",
//     name: "A_flowNodeRefs_lanes",
//     visibility: Private,
//     member_end: (
//         "Lane-flowNodeRefs",
//         "FlowNode-lanes",
//     ),
//     owned_end: [],
//     is_derived: false,
// }


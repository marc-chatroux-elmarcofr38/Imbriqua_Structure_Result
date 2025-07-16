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
        to = "super::bpmn_20_flow_element::Column::Id",
        on_delete = "Cascade"
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

impl ActiveModel {
    /// # Help document for "FlowNode" (bpmn_20_class_flow_node)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __FlowElement__ (__FlowElementModel__)
    ///   * one-to-one link : one __FlowNode__ need one __FlowElement__)
    ///   * callable using find_also_related(__FlowElementModel__) from __FlowNode__
    ///   * saved in __super_flow_element__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __Activity__ (__ActivityModel__)
    ///   * one-to-one link (reverse) : one __Activity__ need one __FlowNode__)
    ///   * callable using find_also_related(__FlowNodeModel__) from __Activity__
    ///   * saved in __super_flow_node__ field as foreing key in __ActivityModel__
    /// * __ChoreographyActivity__ (__ChoreographyActivityModel__)
    ///   * one-to-one link (reverse) : one __ChoreographyActivity__ need one __FlowNode__)
    ///   * callable using find_also_related(__FlowNodeModel__) from __ChoreographyActivity__
    ///   * saved in __super_flow_node__ field as foreing key in __ChoreographyActivityModel__
    /// * __Event__ (__EventModel__)
    ///   * one-to-one link (reverse) : one __Event__ need one __FlowNode__)
    ///   * callable using find_also_related(__FlowNodeModel__) from __Event__
    ///   * saved in __super_flow_node__ field as foreing key in __EventModel__
    /// * __Gateway__ (__GatewayModel__)
    ///   * one-to-one link (reverse) : one __Gateway__ need one __FlowNode__)
    ///   * callable using find_also_related(__FlowNodeModel__) from __Gateway__
    ///   * saved in __super_flow_node__ field as foreing key in __GatewayModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "FlowNode" (bpmn_20_class_flow_node)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __FlowElement__ (__FlowElementModel__)
  * one-to-one link : one __FlowNode__ need one __FlowElement__)
  * callable using find_also_related(__FlowElementModel__) from __FlowNode__
  * saved in __super_flow_element__ field as foreing key

## Reverse Super :
* __Activity__ (__ActivityModel__)
  * one-to-one link (reverse) : one __Activity__ need one __FlowNode__)
  * callable using find_also_related(__FlowNodeModel__) from __Activity__
  * saved in __super_flow_node__ field as foreing key in __ActivityModel__
* __ChoreographyActivity__ (__ChoreographyActivityModel__)
  * one-to-one link (reverse) : one __ChoreographyActivity__ need one __FlowNode__)
  * callable using find_also_related(__FlowNodeModel__) from __ChoreographyActivity__
  * saved in __super_flow_node__ field as foreing key in __ChoreographyActivityModel__
* __Event__ (__EventModel__)
  * one-to-one link (reverse) : one __Event__ need one __FlowNode__)
  * callable using find_also_related(__FlowNodeModel__) from __Event__
  * saved in __super_flow_node__ field as foreing key in __EventModel__
* __Gateway__ (__GatewayModel__)
  * one-to-one link (reverse) : one __Gateway__ need one __FlowNode__)
  * callable using find_also_related(__FlowNodeModel__) from __Gateway__
  * saved in __super_flow_node__ field as foreing key in __GatewayModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "FlowNode",
//     name: "FlowNode",
//     is_abstract: true,
//     super_class: [
//         "FlowElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "FlowNode-incoming": Property(
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
//         "FlowNode-lanes": Property(
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
//         "FlowNode-outgoing": Property(
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
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#FlowNode",
//     table_name: "bpmn_20_flow_node",
//     model_name: "FlowNode",
//     full_name: "bpmn_20_class_flow_node",
// }


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

impl ActiveModel {
    /// # Help document for "Event" (bpmn_20_class_event)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __FlowNode__ (__FlowNodeModel__)
    ///   * one-to-one link : one __Event__ need one __FlowNode__)
    ///   * callable using find_also_related(__FlowNodeModel__) from __Event__
    ///   * saved in __super_flow_node__ field as foreing key
    /// * __InteractionNode__ (__InteractionNodeModel__)
    ///   * one-to-one link : one __Event__ need one __InteractionNode__)
    ///   * callable using find_also_related(__InteractionNodeModel__) from __Event__
    ///   * saved in __super_interaction_node__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __CatchEvent__ (__CatchEventModel__)
    ///   * one-to-one link (reverse) : one __CatchEvent__ need one __Event__)
    ///   * callable using find_also_related(__EventModel__) from __CatchEvent__
    ///   * saved in __super_event__ field as foreing key in __CatchEventModel__
    /// * __ThrowEvent__ (__ThrowEventModel__)
    ///   * one-to-one link (reverse) : one __ThrowEvent__ need one __Event__)
    ///   * callable using find_also_related(__EventModel__) from __ThrowEvent__
    ///   * saved in __super_event__ field as foreing key in __ThrowEventModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Event" (bpmn_20_class_event)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __FlowNode__ (__FlowNodeModel__)
  * one-to-one link : one __Event__ need one __FlowNode__)
  * callable using find_also_related(__FlowNodeModel__) from __Event__
  * saved in __super_flow_node__ field as foreing key
* __InteractionNode__ (__InteractionNodeModel__)
  * one-to-one link : one __Event__ need one __InteractionNode__)
  * callable using find_also_related(__InteractionNodeModel__) from __Event__
  * saved in __super_interaction_node__ field as foreing key

## Reverse Super :
* __CatchEvent__ (__CatchEventModel__)
  * one-to-one link (reverse) : one __CatchEvent__ need one __Event__)
  * callable using find_also_related(__EventModel__) from __CatchEvent__
  * saved in __super_event__ field as foreing key in __CatchEventModel__
* __ThrowEvent__ (__ThrowEventModel__)
  * one-to-one link (reverse) : one __ThrowEvent__ need one __Event__)
  * callable using find_also_related(__EventModel__) from __ThrowEvent__
  * saved in __super_event__ field as foreing key in __ThrowEventModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         object_id: "Event",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "Event",
//     is_abstract: true,
//     super_class: [
//         "FlowNode",
//         "InteractionNode",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-Event-properties": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Event-properties",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
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
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Event",
//     table_name: "bpmn_20_event",
//     model_name: "Event",
//     full_name: "bpmn_20_class_event",
// }


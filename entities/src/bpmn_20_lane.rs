//! bpmn_20_class_lane

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_lane")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : BPMN20-Lane-childLaneSet
    pub child_lane_set: Option<i64>,
    /// COMPLEX FIELD : BPMN20-Lane-partitionElement
    pub partition_element: Option<i64>,
    /// COMPLEX FIELD : BPMN20-Lane-partitionElementRef
    pub partition_element_ref: Option<i64>,
    /// SIMPLE FIELD : BPMN20-Lane-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Lane need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE Lane need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// ManyToMany : with FlowNode using A_flowNodeRefs_lanes
impl Related<super::bpmn_20_a_flow_node_refs_lanes::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_flow_node_refs_lanes::Relation::FlowNode.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_flow_node_refs_lanes::Relation::Lane
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Lane" (bpmn_20_class_lane)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "BPMN20-Lane-name")
    ///   * type : __std::string::String__
    /// 
    /// ## Direct One To One :
    /// * __LaneSet__ (__LaneSetModel__) from A_childLaneSet_parentLane
    ///   * one-to-one link : (0-1) __Lane__ need (0-1) __LaneSet__)
    ///   * callable using find_also_related(__LaneSetModel__) from __Lane__
    ///   * saved in __child_lane_set__ field as foreing key
    /// * __BaseElement__ (__BaseElementModel__) from A_partitionElement_lane
    ///   * one-to-one link : (0-1) __Lane__ need (0-1) __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Lane__
    ///   * saved in __partition_element__ field as foreing key
    /// 
    /// ## Relation : One To Many :
    /// * __LaneSet__ (__LaneSetModel__) from A_lanes_laneSet
    ///   * one-to-many link : (1-1) __Lane__ need (0-inf) __LaneSet__)
    ///   * callable using find_with_related(__LaneSetModel__) from __Lane__
    ///   * named lane_set in BPMN
    /// * __BaseElement__ (__BaseElementModel__) from A_partitionElementRef_lane
    ///   * one-to-many link : (0-1) __Lane__ need (0-inf) __BaseElement__)
    ///   * callable using find_with_related(__BaseElementModel__) from __Lane__
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __Lane__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Lane__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Lane" (bpmn_20_class_lane)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "BPMN20-Lane-name")
  * type : __std::string::String__

## Direct One To One :
* __LaneSet__ (__LaneSetModel__) from A_childLaneSet_parentLane
  * one-to-one link : (0-1) __Lane__ need (0-1) __LaneSet__)
  * callable using find_also_related(__LaneSetModel__) from __Lane__
  * saved in __child_lane_set__ field as foreing key
* __BaseElement__ (__BaseElementModel__) from A_partitionElement_lane
  * one-to-one link : (0-1) __Lane__ need (0-1) __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Lane__
  * saved in __partition_element__ field as foreing key

## Relation : One To Many :
* __LaneSet__ (__LaneSetModel__) from A_lanes_laneSet
  * one-to-many link : (1-1) __Lane__ need (0-inf) __LaneSet__)
  * callable using find_with_related(__LaneSetModel__) from __Lane__
  * named lane_set in BPMN
* __BaseElement__ (__BaseElementModel__) from A_partitionElementRef_lane
  * one-to-many link : (0-1) __Lane__ need (0-inf) __BaseElement__)
  * callable using find_with_related(__BaseElementModel__) from __Lane__

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __Lane__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Lane__
  * saved in __super_base_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "Lane",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "Lane",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-Lane-childLaneSet": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Lane-childLaneSet",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "childLaneSet",
//                 visibility: Public,
//                 simple_type: Some(
//                     "LaneSet",
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
//                     "A_childLaneSet_parentLane",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Lane-flowNodeRefs": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Lane-flowNodeRefs",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "flowNodeRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "FlowNode",
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
//                     "A_flowNodeRefs_lanes",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Lane-name": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Lane-name",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "name",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "DC.cmof#String",
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
//         "-Lane-partitionElement": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Lane-partitionElement",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "partitionElement",
//                 visibility: Public,
//                 simple_type: Some(
//                     "BaseElement",
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
//                     "A_partitionElement_lane",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Lane-partitionElementRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Lane-partitionElementRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "partitionElementRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "BaseElement",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
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
//                     "A_partitionElementRef_lane",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Lane",
//     table_name: "bpmn_20_lane",
//     model_name: "Lane",
//     full_name: "bpmn_20_class_lane",
// }


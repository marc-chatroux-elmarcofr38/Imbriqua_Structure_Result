//! bpmn_20_class_lane

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_lane")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : Lane-childLaneSet
    pub child_lane_set: Option<i64>,
    /// COMPLEX FIELD : Lane-partitionElementRef
    pub partition_element_ref: Option<i64>,
    /// COMPLEX FIELD : Lane-partitionElement
    pub partition_element: Option<i64>,
    /// SIMPLE FIELD : Lane-name
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
    /// * __name__ (xmi_id : "Lane-name")
    ///   * type : __std::string::String__
    /// 
    /// ## Direct One To One :
    /// * __LaneSet__ (__LaneSetModel__) from A_childLaneSet_parentLane
    ///   * one-to-one link : one __Lane__ need one __LaneSet__)
    ///   * callable using find_also_related(__LaneSetModel__) from __Lane__
    ///   * saved in __child_lane_set__ field as foreing key
    /// * __BaseElement__ (__BaseElementModel__) from A_partitionElement_lane
    ///   * one-to-one link : one __Lane__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Lane__
    ///   * saved in __partition_element__ field as foreing key
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
* __name__ (xmi_id : "Lane-name")
  * type : __std::string::String__

## Direct One To One :
* __LaneSet__ (__LaneSetModel__) from A_childLaneSet_parentLane
  * one-to-one link : one __Lane__ need one __LaneSet__)
  * callable using find_also_related(__LaneSetModel__) from __Lane__
  * saved in __child_lane_set__ field as foreing key
* __BaseElement__ (__BaseElementModel__) from A_partitionElement_lane
  * one-to-one link : one __Lane__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Lane__
  * saved in __partition_element__ field as foreing key

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
//     xmi_id: "Lane",
//     name: "Lane",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "Lane-name",
//                 name: "name",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "Lane-childLaneSet",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "Lane-partitionElementRef",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "Lane-flowNodeRefs",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "Lane-partitionElement",
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
//     ],
//     owned_rule: [],
// }


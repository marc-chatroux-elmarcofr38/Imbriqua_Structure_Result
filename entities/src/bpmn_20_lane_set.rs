//! bpmn_20_class_lane_set

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_lane_set")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// SIMPLE FIELD : BPMN20-LaneSet-name
    pub name: Option<std::string::String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE LaneSet need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE LaneSet need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "LaneSet" (bpmn_20_class_lane_set)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "BPMN20-LaneSet-name")
    ///   * type : __Option<std::string::String>__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __FlowElementsContainer__ (__FlowElementsContainerModel__) from A_laneSets_flowElementsContainer
    ///   * one-to-many link : (0-1) __LaneSet__ need (0-inf) __FlowElementsContainer__)
    ///   * callable using find_with_related(__FlowElementsContainerModel__) from __LaneSet__
    ///   * named flow_elements_container in BPMN
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __LaneSet__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __LaneSet__
    ///   * saved in __super_base_element__ field as foreing key
    /// ## Reverse One To One :
    /// * __Lane__ (__LaneModel__) from A_childLaneSet_parentLane
    ///   * one-to-one link : (0-1) __Lane__ need (0-1) __LaneSet__)
    ///   * callable using find_also_related(__LaneSetModel__) from __Lane__
    ///   * saved in __child_lane_set__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "LaneSet" (bpmn_20_class_lane_set)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "BPMN20-LaneSet-name")
  * type : __Option<std::string::String>__


## Relation : One To Many :
* __FlowElementsContainer__ (__FlowElementsContainerModel__) from A_laneSets_flowElementsContainer
  * one-to-many link : (0-1) __LaneSet__ need (0-inf) __FlowElementsContainer__)
  * callable using find_with_related(__FlowElementsContainerModel__) from __LaneSet__
  * named flow_elements_container in BPMN

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __LaneSet__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __LaneSet__
  * saved in __super_base_element__ field as foreing key
## Reverse One To One :
* __Lane__ (__LaneModel__) from A_childLaneSet_parentLane
  * one-to-one link : (0-1) __Lane__ need (0-1) __LaneSet__)
  * callable using find_also_related(__LaneSetModel__) from __Lane__
  * saved in __child_lane_set__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         object_id: "LaneSet",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "LaneSet",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-LaneSet-lanes": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "LaneSet-lanes",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
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
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_lanes_laneSet",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-LaneSet-name": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "LaneSet-name",
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#LaneSet",
//     table_name: "bpmn_20_lane_set",
//     model_name: "LaneSet",
//     full_name: "bpmn_20_class_lane_set",
// }


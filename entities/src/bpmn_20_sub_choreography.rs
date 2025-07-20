//! bpmn_20_class_sub_choreography

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_sub_choreography")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : ChoreographyActivity
    pub super_choreography_activity: i64,
    /// SUPER FIELD : FlowElementsContainer
    pub super_flow_elements_container: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE SubChoreography need ONE ChoreographyActivity
    #[sea_orm(
        belongs_to = "super::bpmn_20_choreography_activity::Entity",
        from = "Column::SuperChoreographyActivity",
        to = "super::bpmn_20_choreography_activity::Column::Id",
        on_delete = "Cascade"
    )]
    ChoreographyActivity,
    // SUPER : ONE SubChoreography need ONE FlowElementsContainer
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_elements_container::Entity",
        from = "Column::SuperFlowElementsContainer",
        to = "super::bpmn_20_flow_elements_container::Column::Id",
        on_delete = "Cascade"
    )]
    FlowElementsContainer,
}

// SUPER : ONE SubChoreography need ONE ChoreographyActivity
impl Related<super::bpmn_20_choreography_activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChoreographyActivity.def()
    }
}

// SUPER : ONE SubChoreography need ONE FlowElementsContainer
impl Related<super::bpmn_20_flow_elements_container::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowElementsContainer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "SubChoreography" (bpmn_20_class_sub_choreography)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __ChoreographyActivity__ (__ChoreographyActivityModel__)
    ///   * one-to-one link : one __SubChoreography__ need one __ChoreographyActivity__)
    ///   * callable using find_also_related(__ChoreographyActivityModel__) from __SubChoreography__
    ///   * saved in __super_choreography_activity__ field as foreing key
    /// * __FlowElementsContainer__ (__FlowElementsContainerModel__)
    ///   * one-to-one link : one __SubChoreography__ need one __FlowElementsContainer__)
    ///   * callable using find_also_related(__FlowElementsContainerModel__) from __SubChoreography__
    ///   * saved in __super_flow_elements_container__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "SubChoreography" (bpmn_20_class_sub_choreography)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __ChoreographyActivity__ (__ChoreographyActivityModel__)
  * one-to-one link : one __SubChoreography__ need one __ChoreographyActivity__)
  * callable using find_also_related(__ChoreographyActivityModel__) from __SubChoreography__
  * saved in __super_choreography_activity__ field as foreing key
* __FlowElementsContainer__ (__FlowElementsContainerModel__)
  * one-to-one link : one __SubChoreography__ need one __FlowElementsContainer__)
  * callable using find_also_related(__FlowElementsContainerModel__) from __SubChoreography__
  * saved in __super_flow_elements_container__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         object_id: "SubChoreography",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "SubChoreography",
//     is_abstract: false,
//     super_class: [
//         "ChoreographyActivity",
//         "FlowElementsContainer",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-SubChoreography-artifacts": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "SubChoreography-artifacts",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "artifacts",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Artifact",
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
//                     "A_artifacts_subChoreography",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#SubChoreography",
//     table_name: "bpmn_20_sub_choreography",
//     model_name: "SubChoreography",
//     full_name: "bpmn_20_class_sub_choreography",
// }


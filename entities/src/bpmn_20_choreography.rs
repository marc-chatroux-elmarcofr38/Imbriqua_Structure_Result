//! bpmn_20_class_choreography

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_choreography")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Collaboration
    pub super_collaboration: i64,
    /// SUPER FIELD : FlowElementsContainer
    pub super_flow_elements_container: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Choreography need ONE Collaboration
    #[sea_orm(
        belongs_to = "super::bpmn_20_collaboration::Entity",
        from = "Column::SuperCollaboration",
        to = "super::bpmn_20_collaboration::Column::Id",
        on_delete = "Cascade"
    )]
    Collaboration,
    // SUPER : ONE Choreography need ONE FlowElementsContainer
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_elements_container::Entity",
        from = "Column::SuperFlowElementsContainer",
        to = "super::bpmn_20_flow_elements_container::Column::Id",
        on_delete = "Cascade"
    )]
    FlowElementsContainer,
    // SUPER : ONE GlobalChoreographyTask need ONE Choreography
    #[sea_orm(has_one = "super::bpmn_20_global_choreography_task::Entity")]
    GlobalChoreographyTask,
}

// SUPER : ONE Choreography need ONE Collaboration
impl Related<super::bpmn_20_collaboration::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Collaboration.def()
    }
}

// SUPER : ONE Choreography need ONE FlowElementsContainer
impl Related<super::bpmn_20_flow_elements_container::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowElementsContainer.def()
    }
}

// SUPER : ONE GlobalChoreographyTask need ONE Choreography
impl Related<super::bpmn_20_global_choreography_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GlobalChoreographyTask.def()
    }
}

// ManyToMany : with Collaboration using A_choreographyRef_collaboration
impl Related<super::bpmn_20_a_choreography_ref_collaboration::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_choreography_ref_collaboration::Relation::Collaboration.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_choreography_ref_collaboration::Relation::Choreography
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Choreography" (bpmn_20_class_choreography)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __Collaboration__ (__CollaborationModel__)
    ///   * one-to-one link : one __Choreography__ need one __Collaboration__)
    ///   * callable using find_also_related(__CollaborationModel__) from __Choreography__
    ///   * saved in __super_collaboration__ field as foreing key
    /// * __FlowElementsContainer__ (__FlowElementsContainerModel__)
    ///   * one-to-one link : one __Choreography__ need one __FlowElementsContainer__)
    ///   * callable using find_also_related(__FlowElementsContainerModel__) from __Choreography__
    ///   * saved in __super_flow_elements_container__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __GlobalChoreographyTask__ (__GlobalChoreographyTaskModel__)
    ///   * one-to-one link (reverse) : one __GlobalChoreographyTask__ need one __Choreography__)
    ///   * callable using find_also_related(__ChoreographyModel__) from __GlobalChoreographyTask__
    ///   * saved in __super_choreography__ field as foreing key in __GlobalChoreographyTaskModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Choreography" (bpmn_20_class_choreography)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __Collaboration__ (__CollaborationModel__)
  * one-to-one link : one __Choreography__ need one __Collaboration__)
  * callable using find_also_related(__CollaborationModel__) from __Choreography__
  * saved in __super_collaboration__ field as foreing key
* __FlowElementsContainer__ (__FlowElementsContainerModel__)
  * one-to-one link : one __Choreography__ need one __FlowElementsContainer__)
  * callable using find_also_related(__FlowElementsContainerModel__) from __Choreography__
  * saved in __super_flow_elements_container__ field as foreing key

## Reverse Super :
* __GlobalChoreographyTask__ (__GlobalChoreographyTaskModel__)
  * one-to-one link (reverse) : one __GlobalChoreographyTask__ need one __Choreography__)
  * callable using find_also_related(__ChoreographyModel__) from __GlobalChoreographyTask__
  * saved in __super_choreography__ field as foreing key in __GlobalChoreographyTaskModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Choreography",
//     name: "Choreography",
//     is_abstract: false,
//     super_class: [
//         "FlowElementsContainer",
//         "Collaboration",
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Choreography",
//     table_name: "bpmn_20_choreography",
//     model_name: "Choreography",
//     full_name: "bpmn_20_class_choreography",
// }


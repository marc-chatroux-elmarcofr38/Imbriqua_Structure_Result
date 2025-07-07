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
        to = "super::bpmn_20_collaboration::Column::Id"
    )]
    Collaboration,
    // SUPER : ONE Choreography need ONE FlowElementsContainer
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_elements_container::Entity",
        from = "Column::SuperFlowElementsContainer",
        to = "super::bpmn_20_flow_elements_container::Column::Id"
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
//     owned_attribute: [],
//     owned_rule: [],
// }


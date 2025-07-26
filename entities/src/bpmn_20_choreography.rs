//! bpmn_20_class_choreography

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_choreography")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperCollaboration
    pub super_collaboration: i64,
    /// SUPER FIELD : SuperFlowElementsContainer
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

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-Choreography',
//     name: "Choreography",
//     is_abstract: false,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'BPMN20-FlowElementsContainer',
//         "Loaded XMIIdReference RefCell of 'BPMN20-Collaboration',
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Choreography",
//     table_name: "bpmn_20_choreography",
//     model_name: "Choreography",
//     full_name: "bpmn_20_class_choreography",
//     reverse_super: RefCell {
//         value: [
//             (Weak),
//         ],
//     },
// }


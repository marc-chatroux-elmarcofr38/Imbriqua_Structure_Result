//! bpmn_20_class_choreography

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_choreography")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub pk_id: i32,
    /// SIMPLE FIELD : FlowElementsContainer
    pub super_flow_elements_container: i64,
    /// SIMPLE FIELD : Collaboration
    pub super_collaboration: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
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


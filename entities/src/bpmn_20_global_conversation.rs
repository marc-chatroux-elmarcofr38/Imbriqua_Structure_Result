//! bpmn_20_class_global_conversation

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_global_conversation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : Collaboration
    pub super_collaboration: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_collaboration::Entity",
        from = "Column::SuperCollaboration",
        to = "super::bpmn_20_collaboration::Column::Id"
    )]
    Collaboration,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_collaboration::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Collaboration.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "GlobalConversation",
//     name: "GlobalConversation",
//     is_abstract: false,
//     super_class: [
//         "Collaboration",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


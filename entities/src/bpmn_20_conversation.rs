//! bpmn_20_class_conversation

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_conversation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : ConversationNode
    pub super_conversation_node: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_conversation_node::Entity",
        from = "Column::SuperConversationNode",
        to = "super::bpmn_20_conversation_node::Column::Id"
    )]
    ConversationNode,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_conversation_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConversationNode.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "Conversation",
//     name: "Conversation",
//     is_abstract: false,
//     super_class: [
//         "ConversationNode",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


//! bpmn_20_class_conversation

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_conversation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : ConversationNode
    pub super_conversation_node: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Conversation need ONE ConversationNode
    #[sea_orm(
        belongs_to = "super::bpmn_20_conversation_node::Entity",
        from = "Column::SuperConversationNode",
        to = "super::bpmn_20_conversation_node::Column::Id",
        on_delete = "Cascade"
    )]
    ConversationNode,
}

// SUPER : ONE Conversation need ONE ConversationNode
impl Related<super::bpmn_20_conversation_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConversationNode.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Conversation" (bpmn_20_class_conversation)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __ConversationNode__ (__ConversationNodeModel__)
    ///   * one-to-one link : one __Conversation__ need one __ConversationNode__)
    ///   * callable using find_also_related(__ConversationNodeModel__) from __Conversation__
    ///   * saved in __super_conversation_node__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Conversation" (bpmn_20_class_conversation)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __ConversationNode__ (__ConversationNodeModel__)
  * one-to-one link : one __Conversation__ need one __ConversationNode__)
  * callable using find_also_related(__ConversationNodeModel__) from __Conversation__
  * saved in __super_conversation_node__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Weak ref of "BPMN20-Conversation" (loaded : false)",
//     name: "Conversation",
//     is_abstract: false,
//     super_class: [
//         "ConversationNode",
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Conversation",
//     table_name: "bpmn_20_conversation",
//     model_name: "Conversation",
//     full_name: "bpmn_20_class_conversation",
// }


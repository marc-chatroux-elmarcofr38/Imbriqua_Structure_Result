//! bpmn_20_class_global_conversation

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_global_conversation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperCollaboration
    pub super_collaboration: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE GlobalConversation need ONE Collaboration
    #[sea_orm(
        belongs_to = "super::bpmn_20_collaboration::Entity",
        from = "Column::SuperCollaboration",
        to = "super::bpmn_20_collaboration::Column::Id",
        on_delete = "Cascade"
    )]
    Collaboration,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-GlobalConversation',
//     name: "GlobalConversation",
//     is_abstract: false,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'BPMN20-Collaboration',
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#GlobalConversation",
//     table_name: "bpmn_20_global_conversation",
//     model_name: "GlobalConversation",
//     full_name: "bpmn_20_class_global_conversation",
//     reverse_super: RefCell {
//         value: [],
//     },
// }


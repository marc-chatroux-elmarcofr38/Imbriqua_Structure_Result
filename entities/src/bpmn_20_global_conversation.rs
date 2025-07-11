//! bpmn_20_class_global_conversation

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_global_conversation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Collaboration
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

// SUPER : ONE GlobalConversation need ONE Collaboration
impl Related<super::bpmn_20_collaboration::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Collaboration.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "GlobalConversation" (bpmn_20_class_global_conversation)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __Collaboration__ (__CollaborationModel__)
    ///   * one-to-one link : one __GlobalConversation__ need one __Collaboration__)
    ///   * callable using find_also_related(__CollaborationModel__) from __GlobalConversation__
    ///   * saved in __super_collaboration__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "GlobalConversation" (bpmn_20_class_global_conversation)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Direct Super :
* __Collaboration__ (__CollaborationModel__)
  * one-to-one link : one __GlobalConversation__ need one __Collaboration__)
  * callable using find_also_related(__CollaborationModel__) from __GlobalConversation__
  * saved in __super_collaboration__ field as foreing key


"#
    }
}

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


//! bpmn_20_class_auditing

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_auditing")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Auditing need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE Auditing need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Auditing" (bpmn_20_class_auditing)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __Auditing__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Auditing__
    ///   * saved in __super_base_element__ field as foreing key
    /// ## Reverse One To One :
    /// * __FlowElement__ (__FlowElementModel__) from A_auditing_flowElement
    ///   * one-to-one link : (0-1) __FlowElement__ need (0-1) __Auditing__)
    ///   * callable using find_also_related(__AuditingModel__) from __FlowElement__
    ///   * saved in __auditing__ field as foreing key
    /// * __Process__ (__ProcessModel__) from A_auditing_process
    ///   * one-to-one link : (0-1) __Process__ need (0-1) __Auditing__)
    ///   * callable using find_also_related(__AuditingModel__) from __Process__
    ///   * saved in __auditing__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Auditing" (bpmn_20_class_auditing)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __Auditing__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Auditing__
  * saved in __super_base_element__ field as foreing key
## Reverse One To One :
* __FlowElement__ (__FlowElementModel__) from A_auditing_flowElement
  * one-to-one link : (0-1) __FlowElement__ need (0-1) __Auditing__)
  * callable using find_also_related(__AuditingModel__) from __FlowElement__
  * saved in __auditing__ field as foreing key
* __Process__ (__ProcessModel__) from A_auditing_process
  * one-to-one link : (0-1) __Process__ need (0-1) __Auditing__)
  * callable using find_also_related(__AuditingModel__) from __Process__
  * saved in __auditing__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Auditing",
//     name: "Auditing",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


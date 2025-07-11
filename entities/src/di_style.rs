//! di_class_style

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "di_style")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE BpmnLabelStyle need ONE Style
    #[sea_orm(has_one = "super::bpmndi_bpmn_label_style::Entity")]
    BpmnLabelStyle,
}

// SUPER : ONE BpmnLabelStyle need ONE Style
impl Related<super::bpmndi_bpmn_label_style::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BpmnLabelStyle.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Style" (di_class_style)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Reverse Super :
    /// * __BpmnLabelStyle__ (__BpmnLabelStyleModel__)
    ///   * one-to-one link (reverse) : one __BpmnLabelStyle__ need one __Style__)
    ///   * callable using find_also_related(__StyleModel__) from __BpmnLabelStyle__
    ///   * saved in __super_style__ field as foreing key in __BpmnLabelStyleModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Style" (di_class_style)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Reverse Super :
* __BpmnLabelStyle__ (__BpmnLabelStyleModel__)
  * one-to-one link (reverse) : one __BpmnLabelStyle__ need one __Style__)
  * callable using find_also_related(__StyleModel__) from __BpmnLabelStyle__
  * saved in __super_style__ field as foreing key in __BpmnLabelStyleModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Style",
//     name: "Style",
//     is_abstract: true,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


//! di_class_style

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "di_style")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
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


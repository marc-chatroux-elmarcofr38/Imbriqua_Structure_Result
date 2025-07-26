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

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'DI-Style',
//     name: "Style",
//     is_abstract: true,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "DI.cmof#Style",
//     table_name: "di_style",
//     model_name: "Style",
//     full_name: "di_class_style",
//     reverse_super: RefCell {
//         value: [
//             (Weak),
//         ],
//     },
// }


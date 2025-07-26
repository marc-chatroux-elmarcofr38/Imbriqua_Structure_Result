//! bpmn_20_class_global_task

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_global_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperCallableElement
    pub super_callable_element: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE GlobalTask need ONE CallableElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_callable_element::Entity",
        from = "Column::SuperCallableElement",
        to = "super::bpmn_20_callable_element::Column::Id",
        on_delete = "Cascade"
    )]
    CallableElement,
    // SUPER : ONE GlobalBusinessRuleTask need ONE GlobalTask
    #[sea_orm(has_one = "super::bpmn_20_global_business_rule_task::Entity")]
    GlobalBusinessRuleTask,
    // SUPER : ONE GlobalManualTask need ONE GlobalTask
    #[sea_orm(has_one = "super::bpmn_20_global_manual_task::Entity")]
    GlobalManualTask,
    // SUPER : ONE GlobalScriptTask need ONE GlobalTask
    #[sea_orm(has_one = "super::bpmn_20_global_script_task::Entity")]
    GlobalScriptTask,
    // SUPER : ONE GlobalUserTask need ONE GlobalTask
    #[sea_orm(has_one = "super::bpmn_20_global_user_task::Entity")]
    GlobalUserTask,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-GlobalTask',
//     name: "GlobalTask",
//     is_abstract: false,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'BPMN20-CallableElement',
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "GlobalTask-resources": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-GlobalTask-resources',
//                 name: "resources",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-ResourceRole',
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: None,
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-A_resources_globalTask',
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#GlobalTask",
//     table_name: "bpmn_20_global_task",
//     model_name: "GlobalTask",
//     full_name: "bpmn_20_class_global_task",
//     reverse_super: RefCell {
//         value: [
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//         ],
//     },
// }


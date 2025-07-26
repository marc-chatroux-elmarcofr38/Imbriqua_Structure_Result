//! bpmn_20_class_global_script_task

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_global_script_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperGlobalTask
    pub super_global_task: i64,
    /// SIMPLE FIELD : BPMN20-GlobalScriptTask-script
    pub script: String,
    /// SIMPLE FIELD : BPMN20-GlobalScriptTask-scriptLanguage
    pub script_language: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE GlobalScriptTask need ONE GlobalTask
    #[sea_orm(
        belongs_to = "super::bpmn_20_global_task::Entity",
        from = "Column::SuperGlobalTask",
        to = "super::bpmn_20_global_task::Column::Id",
        on_delete = "Cascade"
    )]
    GlobalTask,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-GlobalScriptTask',
//     name: "GlobalScriptTask",
//     is_abstract: false,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'BPMN20-GlobalTask',
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "GlobalScriptTask-script": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-GlobalScriptTask-script',
//                 name: "script",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "Loaded XMIIdReference RefCell of 'DC-String',
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: None,
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "GlobalScriptTask-scriptLanguage": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-GlobalScriptTask-scriptLanguage',
//                 name: "scriptLanguage",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "Loaded XMIIdReference RefCell of 'DC-String',
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: None,
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#GlobalScriptTask",
//     table_name: "bpmn_20_global_script_task",
//     model_name: "GlobalScriptTask",
//     full_name: "bpmn_20_class_global_script_task",
//     reverse_super: RefCell {
//         value: [],
//     },
// }


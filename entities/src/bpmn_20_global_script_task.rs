//! bpmn_20_class_global_script_task

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_global_script_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : GlobalTask
    pub super_global_task: i64,
    /// SIMPLE FIELD : GlobalScriptTask-scriptLanguage
    pub script_language: std::string::String,
    /// SIMPLE FIELD : GlobalScriptTask-script
    pub script: std::string::String,
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

// SUPER : ONE GlobalScriptTask need ONE GlobalTask
impl Related<super::bpmn_20_global_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GlobalTask.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "GlobalScriptTask" (bpmn_20_class_global_script_task)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __script_language__ (xmi_id : "GlobalScriptTask-scriptLanguage")
    ///   * type : __std::string::String__
    /// * __script__ (xmi_id : "GlobalScriptTask-script")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// ## Direct Super :
    /// * __GlobalTask__ (__GlobalTaskModel__)
    ///   * one-to-one link : one __GlobalScriptTask__ need one __GlobalTask__)
    ///   * callable using find_also_related(__GlobalTaskModel__) from __GlobalScriptTask__
    ///   * saved in __super_global_task__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "GlobalScriptTask" (bpmn_20_class_global_script_task)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __script_language__ (xmi_id : "GlobalScriptTask-scriptLanguage")
  * type : __std::string::String__
* __script__ (xmi_id : "GlobalScriptTask-script")
  * type : __std::string::String__


## Direct Super :
* __GlobalTask__ (__GlobalTaskModel__)
  * one-to-one link : one __GlobalScriptTask__ need one __GlobalTask__)
  * callable using find_also_related(__GlobalTaskModel__) from __GlobalScriptTask__
  * saved in __super_global_task__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "GlobalScriptTask",
//     name: "GlobalScriptTask",
//     is_abstract: false,
//     super_class: [
//         "GlobalTask",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "GlobalScriptTask-scriptLanguage",
//                 name: "scriptLanguage",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#String",
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
//                 owning_association: "",
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "GlobalScriptTask-script",
//                 name: "script",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#String",
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
//                 owning_association: "",
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }


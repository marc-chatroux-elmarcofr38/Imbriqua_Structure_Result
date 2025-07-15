//! bpmn_20_class_script_task

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_script_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Task
    pub super_task: i64,
    /// SIMPLE FIELD : ScriptTask-script
    pub script: std::string::String,
    /// SIMPLE FIELD : ScriptTask-scriptFormat
    pub script_format: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ScriptTask need ONE Task
    #[sea_orm(
        belongs_to = "super::bpmn_20_task::Entity",
        from = "Column::SuperTask",
        to = "super::bpmn_20_task::Column::Id",
        on_delete = "Cascade"
    )]
    Task,
}

// SUPER : ONE ScriptTask need ONE Task
impl Related<super::bpmn_20_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ScriptTask" (bpmn_20_class_script_task)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __script__ (xmi_id : "ScriptTask-script")
    ///   * type : __std::string::String__
    /// * __script_format__ (xmi_id : "ScriptTask-scriptFormat")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __Task__ (__TaskModel__)
    ///   * one-to-one link : one __ScriptTask__ need one __Task__)
    ///   * callable using find_also_related(__TaskModel__) from __ScriptTask__
    ///   * saved in __super_task__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ScriptTask" (bpmn_20_class_script_task)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __script__ (xmi_id : "ScriptTask-script")
  * type : __std::string::String__
* __script_format__ (xmi_id : "ScriptTask-scriptFormat")
  * type : __std::string::String__



## Direct Super :
* __Task__ (__TaskModel__)
  * one-to-one link : one __ScriptTask__ need one __Task__)
  * callable using find_also_related(__TaskModel__) from __ScriptTask__
  * saved in __super_task__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "ScriptTask",
//     name: "ScriptTask",
//     is_abstract: false,
//     super_class: [
//         "Task",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "ScriptTask-script": Property(
//             CMOFProperty {
//                 xmi_id: "ScriptTask-script",
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
//         "ScriptTask-scriptFormat": Property(
//             CMOFProperty {
//                 xmi_id: "ScriptTask-scriptFormat",
//                 name: "scriptFormat",
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
//     },
//     owned_rule: {},
// }


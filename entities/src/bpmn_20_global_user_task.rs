//! bpmn_20_class_global_user_task

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_global_user_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : GlobalTask
    pub super_global_task: i64,
    /// SIMPLE FIELD : BPMN20-GlobalUserTask-implementation
    pub implementation: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE GlobalUserTask need ONE GlobalTask
    #[sea_orm(
        belongs_to = "super::bpmn_20_global_task::Entity",
        from = "Column::SuperGlobalTask",
        to = "super::bpmn_20_global_task::Column::Id",
        on_delete = "Cascade"
    )]
    GlobalTask,
}

// SUPER : ONE GlobalUserTask need ONE GlobalTask
impl Related<super::bpmn_20_global_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GlobalTask.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "GlobalUserTask" (bpmn_20_class_global_user_task)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __implementation__ (xmi_id : "BPMN20-GlobalUserTask-implementation")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __GlobalTask__ (__GlobalTaskModel__)
    ///   * one-to-one link : one __GlobalUserTask__ need one __GlobalTask__)
    ///   * callable using find_also_related(__GlobalTaskModel__) from __GlobalUserTask__
    ///   * saved in __super_global_task__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "GlobalUserTask" (bpmn_20_class_global_user_task)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __implementation__ (xmi_id : "BPMN20-GlobalUserTask-implementation")
  * type : __std::string::String__



## Direct Super :
* __GlobalTask__ (__GlobalTaskModel__)
  * one-to-one link : one __GlobalUserTask__ need one __GlobalTask__)
  * callable using find_also_related(__GlobalTaskModel__) from __GlobalUserTask__
  * saved in __super_global_task__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "GlobalUserTask",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "GlobalUserTask",
//     is_abstract: false,
//     super_class: [
//         "GlobalTask",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-GlobalUserTask-implementation": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "GlobalUserTask-implementation",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "implementation",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
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
//         "-GlobalUserTask-renderings": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "GlobalUserTask-renderings",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "renderings",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Rendering",
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
//                 owning_association: "",
//                 association: Some(
//                     "A_renderings_globalUserTask",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#GlobalUserTask",
//     table_name: "bpmn_20_global_user_task",
//     model_name: "GlobalUserTask",
//     full_name: "bpmn_20_class_global_user_task",
// }


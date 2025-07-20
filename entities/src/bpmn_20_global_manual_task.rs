//! bpmn_20_class_global_manual_task

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_global_manual_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : GlobalTask
    pub super_global_task: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE GlobalManualTask need ONE GlobalTask
    #[sea_orm(
        belongs_to = "super::bpmn_20_global_task::Entity",
        from = "Column::SuperGlobalTask",
        to = "super::bpmn_20_global_task::Column::Id",
        on_delete = "Cascade"
    )]
    GlobalTask,
}

// SUPER : ONE GlobalManualTask need ONE GlobalTask
impl Related<super::bpmn_20_global_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GlobalTask.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "GlobalManualTask" (bpmn_20_class_global_manual_task)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __GlobalTask__ (__GlobalTaskModel__)
    ///   * one-to-one link : one __GlobalManualTask__ need one __GlobalTask__)
    ///   * callable using find_also_related(__GlobalTaskModel__) from __GlobalManualTask__
    ///   * saved in __super_global_task__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "GlobalManualTask" (bpmn_20_class_global_manual_task)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __GlobalTask__ (__GlobalTaskModel__)
  * one-to-one link : one __GlobalManualTask__ need one __GlobalTask__)
  * callable using find_also_related(__GlobalTaskModel__) from __GlobalManualTask__
  * saved in __super_global_task__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Weak ref of "BPMN20-GlobalManualTask" (loaded : false)",
//     name: "GlobalManualTask",
//     is_abstract: false,
//     super_class: [
//         "GlobalTask",
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#GlobalManualTask",
//     table_name: "bpmn_20_global_manual_task",
//     model_name: "GlobalManualTask",
//     full_name: "bpmn_20_class_global_manual_task",
// }


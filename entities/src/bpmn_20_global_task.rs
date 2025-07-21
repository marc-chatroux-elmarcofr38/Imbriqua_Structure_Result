//! bpmn_20_class_global_task

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_global_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : CallableElement
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

// SUPER : ONE GlobalTask need ONE CallableElement
impl Related<super::bpmn_20_callable_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CallableElement.def()
    }
}

// SUPER : ONE GlobalBusinessRuleTask need ONE GlobalTask
impl Related<super::bpmn_20_global_business_rule_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GlobalBusinessRuleTask.def()
    }
}

// SUPER : ONE GlobalManualTask need ONE GlobalTask
impl Related<super::bpmn_20_global_manual_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GlobalManualTask.def()
    }
}

// SUPER : ONE GlobalScriptTask need ONE GlobalTask
impl Related<super::bpmn_20_global_script_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GlobalScriptTask.def()
    }
}

// SUPER : ONE GlobalUserTask need ONE GlobalTask
impl Related<super::bpmn_20_global_user_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GlobalUserTask.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "GlobalTask" (bpmn_20_class_global_task)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __CallableElement__ (__CallableElementModel__)
    ///   * one-to-one link : one __GlobalTask__ need one __CallableElement__)
    ///   * callable using find_also_related(__CallableElementModel__) from __GlobalTask__
    ///   * saved in __super_callable_element__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __GlobalBusinessRuleTask__ (__GlobalBusinessRuleTaskModel__)
    ///   * one-to-one link (reverse) : one __GlobalBusinessRuleTask__ need one __GlobalTask__)
    ///   * callable using find_also_related(__GlobalTaskModel__) from __GlobalBusinessRuleTask__
    ///   * saved in __super_global_task__ field as foreing key in __GlobalBusinessRuleTaskModel__
    /// * __GlobalManualTask__ (__GlobalManualTaskModel__)
    ///   * one-to-one link (reverse) : one __GlobalManualTask__ need one __GlobalTask__)
    ///   * callable using find_also_related(__GlobalTaskModel__) from __GlobalManualTask__
    ///   * saved in __super_global_task__ field as foreing key in __GlobalManualTaskModel__
    /// * __GlobalScriptTask__ (__GlobalScriptTaskModel__)
    ///   * one-to-one link (reverse) : one __GlobalScriptTask__ need one __GlobalTask__)
    ///   * callable using find_also_related(__GlobalTaskModel__) from __GlobalScriptTask__
    ///   * saved in __super_global_task__ field as foreing key in __GlobalScriptTaskModel__
    /// * __GlobalUserTask__ (__GlobalUserTaskModel__)
    ///   * one-to-one link (reverse) : one __GlobalUserTask__ need one __GlobalTask__)
    ///   * callable using find_also_related(__GlobalTaskModel__) from __GlobalUserTask__
    ///   * saved in __super_global_task__ field as foreing key in __GlobalUserTaskModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "GlobalTask" (bpmn_20_class_global_task)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __CallableElement__ (__CallableElementModel__)
  * one-to-one link : one __GlobalTask__ need one __CallableElement__)
  * callable using find_also_related(__CallableElementModel__) from __GlobalTask__
  * saved in __super_callable_element__ field as foreing key

## Reverse Super :
* __GlobalBusinessRuleTask__ (__GlobalBusinessRuleTaskModel__)
  * one-to-one link (reverse) : one __GlobalBusinessRuleTask__ need one __GlobalTask__)
  * callable using find_also_related(__GlobalTaskModel__) from __GlobalBusinessRuleTask__
  * saved in __super_global_task__ field as foreing key in __GlobalBusinessRuleTaskModel__
* __GlobalManualTask__ (__GlobalManualTaskModel__)
  * one-to-one link (reverse) : one __GlobalManualTask__ need one __GlobalTask__)
  * callable using find_also_related(__GlobalTaskModel__) from __GlobalManualTask__
  * saved in __super_global_task__ field as foreing key in __GlobalManualTaskModel__
* __GlobalScriptTask__ (__GlobalScriptTaskModel__)
  * one-to-one link (reverse) : one __GlobalScriptTask__ need one __GlobalTask__)
  * callable using find_also_related(__GlobalTaskModel__) from __GlobalScriptTask__
  * saved in __super_global_task__ field as foreing key in __GlobalScriptTaskModel__
* __GlobalUserTask__ (__GlobalUserTaskModel__)
  * one-to-one link (reverse) : one __GlobalUserTask__ need one __GlobalTask__)
  * callable using find_also_related(__GlobalTaskModel__) from __GlobalUserTask__
  * saved in __super_global_task__ field as foreing key in __GlobalUserTaskModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdLocalReference {
//         object_id: "GlobalTask",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "GlobalTask",
//     is_abstract: false,
//     super_class: [
//         "CallableElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "GlobalTask-resources": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "GlobalTask-resources",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "resources",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ResourceRole",
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
//                     "A_resources_globalTask",
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
// }


//! bpmn_20_class_rendering

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_rendering")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Rendering need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE Rendering need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Rendering" (bpmn_20_class_rendering)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __GlobalUserTask__ (__GlobalUserTaskModel__) from A_renderings_globalUserTask
    ///   * one-to-many link : (0-1) __Rendering__ need (0-inf) __GlobalUserTask__)
    ///   * callable using find_with_related(__GlobalUserTaskModel__) from __Rendering__
    ///   * named global_user_task in BPMN
    /// * __UserTask__ (__UserTaskModel__) from A_renderings_usertask
    ///   * one-to-many link : (0-1) __Rendering__ need (0-inf) __UserTask__)
    ///   * callable using find_with_related(__UserTaskModel__) from __Rendering__
    ///   * named usertask in BPMN
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __Rendering__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Rendering__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Rendering" (bpmn_20_class_rendering)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Relation : One To Many :
* __GlobalUserTask__ (__GlobalUserTaskModel__) from A_renderings_globalUserTask
  * one-to-many link : (0-1) __Rendering__ need (0-inf) __GlobalUserTask__)
  * callable using find_with_related(__GlobalUserTaskModel__) from __Rendering__
  * named global_user_task in BPMN
* __UserTask__ (__UserTaskModel__) from A_renderings_usertask
  * one-to-many link : (0-1) __Rendering__ need (0-inf) __UserTask__)
  * callable using find_with_related(__UserTaskModel__) from __Rendering__
  * named usertask in BPMN

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __Rendering__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Rendering__
  * saved in __super_base_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Weak ref of "BPMN20-Rendering" (loaded : false)",
//     name: "Rendering",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Rendering",
//     table_name: "bpmn_20_rendering",
//     model_name: "Rendering",
//     full_name: "bpmn_20_class_rendering",
// }


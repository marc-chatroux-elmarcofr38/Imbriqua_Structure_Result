//! bpmn_20_class_loop_characteristics

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_loop_characteristics")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE LoopCharacteristics need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
    // SUPER : ONE MultiInstanceLoopCharacteristics need ONE LoopCharacteristics
    #[sea_orm(has_one = "super::bpmn_20_multi_instance_loop_characteristics::Entity")]
    MultiInstanceLoopCharacteristics,
    // SUPER : ONE StandardLoopCharacteristics need ONE LoopCharacteristics
    #[sea_orm(has_one = "super::bpmn_20_standard_loop_characteristics::Entity")]
    StandardLoopCharacteristics,
}

// SUPER : ONE LoopCharacteristics need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// SUPER : ONE MultiInstanceLoopCharacteristics need ONE LoopCharacteristics
impl Related<super::bpmn_20_multi_instance_loop_characteristics::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MultiInstanceLoopCharacteristics.def()
    }
}

// SUPER : ONE StandardLoopCharacteristics need ONE LoopCharacteristics
impl Related<super::bpmn_20_standard_loop_characteristics::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StandardLoopCharacteristics.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "LoopCharacteristics" (bpmn_20_class_loop_characteristics)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __LoopCharacteristics__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __LoopCharacteristics__
    ///   * saved in __super_base_element__ field as foreing key
    /// ## Reverse One To One :
    /// * __Activity__ (__ActivityModel__) from A_loopCharacteristics_activity
    ///   * one-to-one link : (0-1) __Activity__ need (0-1) __LoopCharacteristics__)
    ///   * callable using find_also_related(__LoopCharacteristicsModel__) from __Activity__
    ///   * saved in __loop_characteristics__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __MultiInstanceLoopCharacteristics__ (__MultiInstanceLoopCharacteristicsModel__)
    ///   * one-to-one link (reverse) : one __MultiInstanceLoopCharacteristics__ need one __LoopCharacteristics__)
    ///   * callable using find_also_related(__LoopCharacteristicsModel__) from __MultiInstanceLoopCharacteristics__
    ///   * saved in __super_loop_characteristics__ field as foreing key in __MultiInstanceLoopCharacteristicsModel__
    /// * __StandardLoopCharacteristics__ (__StandardLoopCharacteristicsModel__)
    ///   * one-to-one link (reverse) : one __StandardLoopCharacteristics__ need one __LoopCharacteristics__)
    ///   * callable using find_also_related(__LoopCharacteristicsModel__) from __StandardLoopCharacteristics__
    ///   * saved in __super_loop_characteristics__ field as foreing key in __StandardLoopCharacteristicsModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "LoopCharacteristics" (bpmn_20_class_loop_characteristics)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __LoopCharacteristics__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __LoopCharacteristics__
  * saved in __super_base_element__ field as foreing key
## Reverse One To One :
* __Activity__ (__ActivityModel__) from A_loopCharacteristics_activity
  * one-to-one link : (0-1) __Activity__ need (0-1) __LoopCharacteristics__)
  * callable using find_also_related(__LoopCharacteristicsModel__) from __Activity__
  * saved in __loop_characteristics__ field as foreing key

## Reverse Super :
* __MultiInstanceLoopCharacteristics__ (__MultiInstanceLoopCharacteristicsModel__)
  * one-to-one link (reverse) : one __MultiInstanceLoopCharacteristics__ need one __LoopCharacteristics__)
  * callable using find_also_related(__LoopCharacteristicsModel__) from __MultiInstanceLoopCharacteristics__
  * saved in __super_loop_characteristics__ field as foreing key in __MultiInstanceLoopCharacteristicsModel__
* __StandardLoopCharacteristics__ (__StandardLoopCharacteristicsModel__)
  * one-to-one link (reverse) : one __StandardLoopCharacteristics__ need one __LoopCharacteristics__)
  * callable using find_also_related(__LoopCharacteristicsModel__) from __StandardLoopCharacteristics__
  * saved in __super_loop_characteristics__ field as foreing key in __StandardLoopCharacteristicsModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "LoopCharacteristics",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "LoopCharacteristics",
//     is_abstract: true,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#LoopCharacteristics",
//     table_name: "bpmn_20_loop_characteristics",
//     model_name: "LoopCharacteristics",
//     full_name: "bpmn_20_class_loop_characteristics",
// }


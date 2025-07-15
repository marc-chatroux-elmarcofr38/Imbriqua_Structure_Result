//! bpmn_20_class_intermediate_catch_event

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_intermediate_catch_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : CatchEvent
    pub super_catch_event: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE IntermediateCatchEvent need ONE CatchEvent
    #[sea_orm(
        belongs_to = "super::bpmn_20_catch_event::Entity",
        from = "Column::SuperCatchEvent",
        to = "super::bpmn_20_catch_event::Column::Id",
        on_delete = "Cascade"
    )]
    CatchEvent,
}

// SUPER : ONE IntermediateCatchEvent need ONE CatchEvent
impl Related<super::bpmn_20_catch_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CatchEvent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "IntermediateCatchEvent" (bpmn_20_class_intermediate_catch_event)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __CatchEvent__ (__CatchEventModel__)
    ///   * one-to-one link : one __IntermediateCatchEvent__ need one __CatchEvent__)
    ///   * callable using find_also_related(__CatchEventModel__) from __IntermediateCatchEvent__
    ///   * saved in __super_catch_event__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "IntermediateCatchEvent" (bpmn_20_class_intermediate_catch_event)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __CatchEvent__ (__CatchEventModel__)
  * one-to-one link : one __IntermediateCatchEvent__ need one __CatchEvent__)
  * callable using find_also_related(__CatchEventModel__) from __IntermediateCatchEvent__
  * saved in __super_catch_event__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "IntermediateCatchEvent",
//     name: "IntermediateCatchEvent",
//     is_abstract: false,
//     super_class: [
//         "CatchEvent",
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
// }


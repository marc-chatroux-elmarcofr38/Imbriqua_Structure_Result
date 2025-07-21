//! bpmn_20_class_end_event

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_end_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : ThrowEvent
    pub super_throw_event: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE EndEvent need ONE ThrowEvent
    #[sea_orm(
        belongs_to = "super::bpmn_20_throw_event::Entity",
        from = "Column::SuperThrowEvent",
        to = "super::bpmn_20_throw_event::Column::Id",
        on_delete = "Cascade"
    )]
    ThrowEvent,
}

// SUPER : ONE EndEvent need ONE ThrowEvent
impl Related<super::bpmn_20_throw_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ThrowEvent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "EndEvent" (bpmn_20_class_end_event)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __ThrowEvent__ (__ThrowEventModel__)
    ///   * one-to-one link : one __EndEvent__ need one __ThrowEvent__)
    ///   * callable using find_also_related(__ThrowEventModel__) from __EndEvent__
    ///   * saved in __super_throw_event__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "EndEvent" (bpmn_20_class_end_event)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __ThrowEvent__ (__ThrowEventModel__)
  * one-to-one link : one __EndEvent__ need one __ThrowEvent__)
  * callable using find_also_related(__ThrowEventModel__) from __EndEvent__
  * saved in __super_throw_event__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdLocalReference {
//         object_id: "EndEvent",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "EndEvent",
//     is_abstract: false,
//     super_class: [
//         "ThrowEvent",
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#EndEvent",
//     table_name: "bpmn_20_end_event",
//     model_name: "EndEvent",
//     full_name: "bpmn_20_class_end_event",
// }


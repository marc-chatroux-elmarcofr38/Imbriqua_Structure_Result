//! bpmn_20_class_intermediate_throw_event

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_intermediate_throw_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : ThrowEvent
    pub super_throw_event: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE IntermediateThrowEvent need ONE ThrowEvent
    #[sea_orm(
        belongs_to = "super::bpmn_20_throw_event::Entity",
        from = "Column::SuperThrowEvent",
        to = "super::bpmn_20_throw_event::Column::Id",
        on_delete = "Cascade"
    )]
    ThrowEvent,
}

// SUPER : ONE IntermediateThrowEvent need ONE ThrowEvent
impl Related<super::bpmn_20_throw_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ThrowEvent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "IntermediateThrowEvent" (bpmn_20_class_intermediate_throw_event)
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
    ///   * one-to-one link : one __IntermediateThrowEvent__ need one __ThrowEvent__)
    ///   * callable using find_also_related(__ThrowEventModel__) from __IntermediateThrowEvent__
    ///   * saved in __super_throw_event__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "IntermediateThrowEvent" (bpmn_20_class_intermediate_throw_event)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __ThrowEvent__ (__ThrowEventModel__)
  * one-to-one link : one __IntermediateThrowEvent__ need one __ThrowEvent__)
  * callable using find_also_related(__ThrowEventModel__) from __IntermediateThrowEvent__
  * saved in __super_throw_event__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "IntermediateThrowEvent",
//     name: "IntermediateThrowEvent",
//     is_abstract: false,
//     super_class: [
//         "ThrowEvent",
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#IntermediateThrowEvent",
//     table_name: "bpmn_20_intermediate_throw_event",
//     model_name: "IntermediateThrowEvent",
//     full_name: "bpmn_20_class_intermediate_throw_event",
// }


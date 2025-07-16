//! bpmn_20_class_human_performer

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_human_performer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Performer
    pub super_performer: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE HumanPerformer need ONE Performer
    #[sea_orm(
        belongs_to = "super::bpmn_20_performer::Entity",
        from = "Column::SuperPerformer",
        to = "super::bpmn_20_performer::Column::Id",
        on_delete = "Cascade"
    )]
    Performer,
    // SUPER : ONE PotentialOwner need ONE HumanPerformer
    #[sea_orm(has_one = "super::bpmn_20_potential_owner::Entity")]
    PotentialOwner,
}

// SUPER : ONE HumanPerformer need ONE Performer
impl Related<super::bpmn_20_performer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Performer.def()
    }
}

// SUPER : ONE PotentialOwner need ONE HumanPerformer
impl Related<super::bpmn_20_potential_owner::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PotentialOwner.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "HumanPerformer" (bpmn_20_class_human_performer)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __Performer__ (__PerformerModel__)
    ///   * one-to-one link : one __HumanPerformer__ need one __Performer__)
    ///   * callable using find_also_related(__PerformerModel__) from __HumanPerformer__
    ///   * saved in __super_performer__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __PotentialOwner__ (__PotentialOwnerModel__)
    ///   * one-to-one link (reverse) : one __PotentialOwner__ need one __HumanPerformer__)
    ///   * callable using find_also_related(__HumanPerformerModel__) from __PotentialOwner__
    ///   * saved in __super_human_performer__ field as foreing key in __PotentialOwnerModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "HumanPerformer" (bpmn_20_class_human_performer)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __Performer__ (__PerformerModel__)
  * one-to-one link : one __HumanPerformer__ need one __Performer__)
  * callable using find_also_related(__PerformerModel__) from __HumanPerformer__
  * saved in __super_performer__ field as foreing key

## Reverse Super :
* __PotentialOwner__ (__PotentialOwnerModel__)
  * one-to-one link (reverse) : one __PotentialOwner__ need one __HumanPerformer__)
  * callable using find_also_related(__HumanPerformerModel__) from __PotentialOwner__
  * saved in __super_human_performer__ field as foreing key in __PotentialOwnerModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "HumanPerformer",
//     name: "HumanPerformer",
//     is_abstract: false,
//     super_class: [
//         "Performer",
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#HumanPerformer",
//     table_name: "bpmn_20_human_performer",
//     model_name: "HumanPerformer",
//     full_name: "bpmn_20_class_human_performer",
// }


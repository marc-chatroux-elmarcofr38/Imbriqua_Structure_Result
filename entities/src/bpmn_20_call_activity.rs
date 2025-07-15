//! bpmn_20_class_call_activity

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_call_activity")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Activity
    pub super_activity: i64,
    /// COMPLEX FIELD : CallActivity-calledElementRef
    pub called_element_ref: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE CallActivity need ONE Activity
    #[sea_orm(
        belongs_to = "super::bpmn_20_activity::Entity",
        from = "Column::SuperActivity",
        to = "super::bpmn_20_activity::Column::Id",
        on_delete = "Cascade"
    )]
    Activity,
}

// SUPER : ONE CallActivity need ONE Activity
impl Related<super::bpmn_20_activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Activity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "CallActivity" (bpmn_20_class_call_activity)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __CallableElement__ (__CallableElementModel__) from A_calledElementRef_callActivity
    ///   * one-to-many link : (0-1) __CallActivity__ need (0-inf) __CallableElement__)
    ///   * callable using find_with_related(__CallableElementModel__) from __CallActivity__
    /// 
    /// ## Direct Super :
    /// * __Activity__ (__ActivityModel__)
    ///   * one-to-one link : one __CallActivity__ need one __Activity__)
    ///   * callable using find_also_related(__ActivityModel__) from __CallActivity__
    ///   * saved in __super_activity__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "CallActivity" (bpmn_20_class_call_activity)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Relation : One To Many :
* __CallableElement__ (__CallableElementModel__) from A_calledElementRef_callActivity
  * one-to-many link : (0-1) __CallActivity__ need (0-inf) __CallableElement__)
  * callable using find_with_related(__CallableElementModel__) from __CallActivity__

## Direct Super :
* __Activity__ (__ActivityModel__)
  * one-to-one link : one __CallActivity__ need one __Activity__)
  * callable using find_also_related(__ActivityModel__) from __CallActivity__
  * saved in __super_activity__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "CallActivity",
//     name: "CallActivity",
//     is_abstract: false,
//     super_class: [
//         "Activity",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "CallActivity-calledElementRef": Property(
//             CMOFProperty {
//                 xmi_id: "CallActivity-calledElementRef",
//                 name: "calledElementRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CallableElement",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
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
//                 association: Some(
//                     "A_calledElementRef_callActivity",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
// }


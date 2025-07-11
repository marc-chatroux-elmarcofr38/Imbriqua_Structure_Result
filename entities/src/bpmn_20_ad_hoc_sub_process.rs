//! bpmn_20_class_ad_hoc_sub_process

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_ad_hoc_sub_process")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SubProcess
    pub super_sub_process: i64,
    /// COMPLEX FIELD : AdHocSubProcess-completionCondition
    pub completion_condition: i64,
    /// SIMPLE FIELD : AdHocSubProcess-ordering
    pub ordering: AdHocOrdering,
    /// SIMPLE FIELD : AdHocSubProcess-cancelRemainingInstances
    #[sea_orm(default_value = "true")]
    pub cancel_remaining_instances: std::primitive::bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE AdHocSubProcess need ONE SubProcess
    #[sea_orm(
        belongs_to = "super::bpmn_20_sub_process::Entity",
        from = "Column::SuperSubProcess",
        to = "super::bpmn_20_sub_process::Column::Id",
        on_delete = "Cascade"
    )]
    SubProcess,
}

// SUPER : ONE AdHocSubProcess need ONE SubProcess
impl Related<super::bpmn_20_sub_process::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubProcess.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "AdHocSubProcess" (bpmn_20_class_ad_hoc_sub_process)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __ordering__ (xmi_id : "AdHocSubProcess-ordering")
    ///   * type : __AdHocOrdering__
    /// * __cancel_remaining_instances__ (xmi_id : "AdHocSubProcess-cancelRemainingInstances")
    ///   * type : __std::primitive::bool__
    ///   * default : "true"
    /// 
    /// ## Direct One To One :
    /// * __Expression__ (__ExpressionModel__) from A_completionCondition_adHocSubProcess
    ///   * one-to-one link : one __AdHocSubProcess__ need one __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __AdHocSubProcess__
    ///   * saved in __completion_condition__ field as foreing key
    /// 
    /// ## Direct Super :
    /// * __SubProcess__ (__SubProcessModel__)
    ///   * one-to-one link : one __AdHocSubProcess__ need one __SubProcess__)
    ///   * callable using find_also_related(__SubProcessModel__) from __AdHocSubProcess__
    ///   * saved in __super_sub_process__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "AdHocSubProcess" (bpmn_20_class_ad_hoc_sub_process)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __ordering__ (xmi_id : "AdHocSubProcess-ordering")
  * type : __AdHocOrdering__
* __cancel_remaining_instances__ (xmi_id : "AdHocSubProcess-cancelRemainingInstances")
  * type : __std::primitive::bool__
  * default : "true"

## Direct One To One :
* __Expression__ (__ExpressionModel__) from A_completionCondition_adHocSubProcess
  * one-to-one link : one __AdHocSubProcess__ need one __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __AdHocSubProcess__
  * saved in __completion_condition__ field as foreing key

## Direct Super :
* __SubProcess__ (__SubProcessModel__)
  * one-to-one link : one __AdHocSubProcess__ need one __SubProcess__)
  * callable using find_also_related(__SubProcessModel__) from __AdHocSubProcess__
  * saved in __super_sub_process__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "AdHocSubProcess",
//     name: "AdHocSubProcess",
//     is_abstract: false,
//     super_class: [
//         "SubProcess",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "AdHocSubProcess-completionCondition",
//                 name: "completionCondition",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Expression",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
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
//                     "A_completionCondition_adHocSubProcess",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "AdHocSubProcess-ordering",
//                 name: "ordering",
//                 visibility: Public,
//                 simple_type: Some(
//                     "AdHocOrdering",
//                 ),
//                 complex_type: None,
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "AdHocSubProcess-cancelRemainingInstances",
//                 name: "cancelRemainingInstances",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#Boolean",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: Some(
//                     "true",
//                 ),
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
//     ],
//     owned_rule: [],
// }


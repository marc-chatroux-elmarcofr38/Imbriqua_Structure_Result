//! bpmn_20_class_sub_process

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_sub_process")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Activity
    pub super_activity: i64,
    /// SUPER FIELD : FlowElementsContainer
    pub super_flow_elements_container: i64,
    /// SIMPLE FIELD : SubProcess-triggeredByEvent
    #[sea_orm(default_value = "false")]
    pub triggered_by_event: std::primitive::bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE SubProcess need ONE Activity
    #[sea_orm(
        belongs_to = "super::bpmn_20_activity::Entity",
        from = "Column::SuperActivity",
        to = "super::bpmn_20_activity::Column::Id",
        on_delete = "Cascade"
    )]
    Activity,
    // SUPER : ONE SubProcess need ONE FlowElementsContainer
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_elements_container::Entity",
        from = "Column::SuperFlowElementsContainer",
        to = "super::bpmn_20_flow_elements_container::Column::Id",
        on_delete = "Cascade"
    )]
    FlowElementsContainer,
    // SUPER : ONE AdHocSubProcess need ONE SubProcess
    #[sea_orm(has_one = "super::bpmn_20_ad_hoc_sub_process::Entity")]
    AdHocSubProcess,
    // SUPER : ONE Transaction need ONE SubProcess
    #[sea_orm(has_one = "super::bpmn_20_transaction::Entity")]
    Transaction,
}

// SUPER : ONE SubProcess need ONE Activity
impl Related<super::bpmn_20_activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Activity.def()
    }
}

// SUPER : ONE SubProcess need ONE FlowElementsContainer
impl Related<super::bpmn_20_flow_elements_container::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowElementsContainer.def()
    }
}

// SUPER : ONE AdHocSubProcess need ONE SubProcess
impl Related<super::bpmn_20_ad_hoc_sub_process::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AdHocSubProcess.def()
    }
}

// SUPER : ONE Transaction need ONE SubProcess
impl Related<super::bpmn_20_transaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transaction.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "SubProcess" (bpmn_20_class_sub_process)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __triggered_by_event__ (xmi_id : "SubProcess-triggeredByEvent")
    ///   * type : __std::primitive::bool__
    ///   * default : "false"
    /// 
    /// 
    /// ## Direct Super :
    /// * __Activity__ (__ActivityModel__)
    ///   * one-to-one link : one __SubProcess__ need one __Activity__)
    ///   * callable using find_also_related(__ActivityModel__) from __SubProcess__
    ///   * saved in __super_activity__ field as foreing key
    /// * __FlowElementsContainer__ (__FlowElementsContainerModel__)
    ///   * one-to-one link : one __SubProcess__ need one __FlowElementsContainer__)
    ///   * callable using find_also_related(__FlowElementsContainerModel__) from __SubProcess__
    ///   * saved in __super_flow_elements_container__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __AdHocSubProcess__ (__AdHocSubProcessModel__)
    ///   * one-to-one link (reverse) : one __AdHocSubProcess__ need one __SubProcess__)
    ///   * callable using find_also_related(__SubProcessModel__) from __AdHocSubProcess__
    ///   * saved in __super_sub_process__ field as foreing key in __AdHocSubProcessModel__
    /// * __Transaction__ (__TransactionModel__)
    ///   * one-to-one link (reverse) : one __Transaction__ need one __SubProcess__)
    ///   * callable using find_also_related(__SubProcessModel__) from __Transaction__
    ///   * saved in __super_sub_process__ field as foreing key in __TransactionModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "SubProcess" (bpmn_20_class_sub_process)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __triggered_by_event__ (xmi_id : "SubProcess-triggeredByEvent")
  * type : __std::primitive::bool__
  * default : "false"


## Direct Super :
* __Activity__ (__ActivityModel__)
  * one-to-one link : one __SubProcess__ need one __Activity__)
  * callable using find_also_related(__ActivityModel__) from __SubProcess__
  * saved in __super_activity__ field as foreing key
* __FlowElementsContainer__ (__FlowElementsContainerModel__)
  * one-to-one link : one __SubProcess__ need one __FlowElementsContainer__)
  * callable using find_also_related(__FlowElementsContainerModel__) from __SubProcess__
  * saved in __super_flow_elements_container__ field as foreing key

## Reverse Super :
* __AdHocSubProcess__ (__AdHocSubProcessModel__)
  * one-to-one link (reverse) : one __AdHocSubProcess__ need one __SubProcess__)
  * callable using find_also_related(__SubProcessModel__) from __AdHocSubProcess__
  * saved in __super_sub_process__ field as foreing key in __AdHocSubProcessModel__
* __Transaction__ (__TransactionModel__)
  * one-to-one link (reverse) : one __Transaction__ need one __SubProcess__)
  * callable using find_also_related(__SubProcessModel__) from __Transaction__
  * saved in __super_sub_process__ field as foreing key in __TransactionModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "SubProcess",
//     name: "SubProcess",
//     is_abstract: false,
//     super_class: [
//         "Activity",
//         "FlowElementsContainer",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "SubProcess-triggeredByEvent",
//                 name: "triggeredByEvent",
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
//                     "false",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "SubProcess-artifacts",
//                 name: "artifacts",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Artifact",
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
//                     "A_artifacts_subProcess",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }


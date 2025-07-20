//! bpmn_20_class_correlation_subscription

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_correlation_subscription")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : BPMN20-CorrelationSubscription-correlationKeyRef
    pub correlation_key_ref: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE CorrelationSubscription need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE CorrelationSubscription need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "CorrelationSubscription" (bpmn_20_class_correlation_subscription)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __CorrelationKey__ (__CorrelationKeyModel__) from A_correlationKeyRef_correlationSubscription
    ///   * one-to-many link : (1-1) __CorrelationSubscription__ need (0-inf) __CorrelationKey__)
    ///   * callable using find_with_related(__CorrelationKeyModel__) from __CorrelationSubscription__
    /// * __Process__ (__ProcessModel__) from A_correlationSubscriptions_process
    ///   * one-to-many link : (1-1) __CorrelationSubscription__ need (0-inf) __Process__)
    ///   * callable using find_with_related(__ProcessModel__) from __CorrelationSubscription__
    ///   * named process in BPMN
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __CorrelationSubscription__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __CorrelationSubscription__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "CorrelationSubscription" (bpmn_20_class_correlation_subscription)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Relation : One To Many :
* __CorrelationKey__ (__CorrelationKeyModel__) from A_correlationKeyRef_correlationSubscription
  * one-to-many link : (1-1) __CorrelationSubscription__ need (0-inf) __CorrelationKey__)
  * callable using find_with_related(__CorrelationKeyModel__) from __CorrelationSubscription__
* __Process__ (__ProcessModel__) from A_correlationSubscriptions_process
  * one-to-many link : (1-1) __CorrelationSubscription__ need (0-inf) __Process__)
  * callable using find_with_related(__ProcessModel__) from __CorrelationSubscription__
  * named process in BPMN

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __CorrelationSubscription__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __CorrelationSubscription__
  * saved in __super_base_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Weak ref of "BPMN20-CorrelationSubscription" (loaded : false)",
//     name: "CorrelationSubscription",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-CorrelationSubscription-correlationKeyRef": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-CorrelationSubscription-correlationKeyRef" (loaded : false)",
//                 name: "correlationKeyRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CorrelationKey",
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
//                 association: Some(
//                     "A_correlationKeyRef_correlationSubscription",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-CorrelationSubscription-correlationPropertyBinding": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-CorrelationSubscription-correlationPropertyBinding" (loaded : false)",
//                 name: "correlationPropertyBinding",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CorrelationPropertyBinding",
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
//                     "A_correlationPropertyBinding_correlationSubscription",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#CorrelationSubscription",
//     table_name: "bpmn_20_correlation_subscription",
//     model_name: "CorrelationSubscription",
//     full_name: "bpmn_20_class_correlation_subscription",
// }


//! bpmn_20_class_correlation_property_binding

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_correlation_property_binding")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : CorrelationPropertyBinding-correlationPropertyRef
    pub correlation_property_ref: i64,
    /// COMPLEX FIELD : CorrelationPropertyBinding-dataPath
    pub data_path: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE CorrelationPropertyBinding need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE CorrelationPropertyBinding need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "CorrelationPropertyBinding" (bpmn_20_class_correlation_property_binding)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// ## Direct One To One :
    /// * __FormalExpression__ (__FormalExpressionModel__) from A_dataPath_correlationPropertyBinding
    ///   * one-to-one link : (1-1) __CorrelationPropertyBinding__ need (0-1) __FormalExpression__)
    ///   * callable using find_also_related(__FormalExpressionModel__) from __CorrelationPropertyBinding__
    ///   * saved in __data_path__ field as foreing key
    /// 
    /// ## Relation : One To Many :
    /// * __CorrelationSubscription__ (__CorrelationSubscriptionModel__) from A_correlationPropertyBinding_correlationSubscription
    ///   * one-to-many link : (0-1) __CorrelationPropertyBinding__ need (0-inf) __CorrelationSubscription__)
    ///   * callable using find_with_related(__CorrelationSubscriptionModel__) from __CorrelationPropertyBinding__
    ///   * named correlation_subscription in BPMN
    /// * __CorrelationProperty__ (__CorrelationPropertyModel__) from A_correlationPropertyRef_correlationPropertyBinding
    ///   * one-to-many link : (1-1) __CorrelationPropertyBinding__ need (0-inf) __CorrelationProperty__)
    ///   * callable using find_with_related(__CorrelationPropertyModel__) from __CorrelationPropertyBinding__
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __CorrelationPropertyBinding__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __CorrelationPropertyBinding__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "CorrelationPropertyBinding" (bpmn_20_class_correlation_property_binding)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__


## Direct One To One :
* __FormalExpression__ (__FormalExpressionModel__) from A_dataPath_correlationPropertyBinding
  * one-to-one link : (1-1) __CorrelationPropertyBinding__ need (0-1) __FormalExpression__)
  * callable using find_also_related(__FormalExpressionModel__) from __CorrelationPropertyBinding__
  * saved in __data_path__ field as foreing key

## Relation : One To Many :
* __CorrelationSubscription__ (__CorrelationSubscriptionModel__) from A_correlationPropertyBinding_correlationSubscription
  * one-to-many link : (0-1) __CorrelationPropertyBinding__ need (0-inf) __CorrelationSubscription__)
  * callable using find_with_related(__CorrelationSubscriptionModel__) from __CorrelationPropertyBinding__
  * named correlation_subscription in BPMN
* __CorrelationProperty__ (__CorrelationPropertyModel__) from A_correlationPropertyRef_correlationPropertyBinding
  * one-to-many link : (1-1) __CorrelationPropertyBinding__ need (0-inf) __CorrelationProperty__)
  * callable using find_with_related(__CorrelationPropertyModel__) from __CorrelationPropertyBinding__

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __CorrelationPropertyBinding__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __CorrelationPropertyBinding__
  * saved in __super_base_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "CorrelationPropertyBinding",
//     name: "CorrelationPropertyBinding",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "CorrelationPropertyBinding-correlationPropertyRef": Property(
//             CMOFProperty {
//                 xmi_id: "CorrelationPropertyBinding-correlationPropertyRef",
//                 name: "correlationPropertyRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CorrelationProperty",
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
//                     "A_correlationPropertyRef_correlationPropertyBinding",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "CorrelationPropertyBinding-dataPath": Property(
//             CMOFProperty {
//                 xmi_id: "CorrelationPropertyBinding-dataPath",
//                 name: "dataPath",
//                 visibility: Public,
//                 simple_type: Some(
//                     "FormalExpression",
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
//                     "A_dataPath_correlationPropertyBinding",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#CorrelationPropertyBinding",
//     table_name: "bpmn_20_correlation_property_binding",
//     model_name: "CorrelationPropertyBinding",
//     full_name: "bpmn_20_class_correlation_property_binding",
// }


//! bpmn_20_class_correlation_property_retrieval_expression

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_correlation_property_retrieval_expression")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : BPMN20-CorrelationPropertyRetrievalExpression-messagePath
    pub message_path: i64,
    /// COMPLEX FIELD : BPMN20-CorrelationPropertyRetrievalExpression-messageRef
    pub message_ref: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE CorrelationPropertyRetrievalExpression need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE CorrelationPropertyRetrievalExpression need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "CorrelationPropertyRetrievalExpression" (bpmn_20_class_correlation_property_retrieval_expression)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// ## Direct One To One :
    /// * __FormalExpression__ (__FormalExpressionModel__) from A_messagePath_correlationset
    ///   * one-to-one link : (1-1) __CorrelationPropertyRetrievalExpression__ need (0-1) __FormalExpression__)
    ///   * callable using find_also_related(__FormalExpressionModel__) from __CorrelationPropertyRetrievalExpression__
    ///   * saved in __message_path__ field as foreing key
    /// 
    /// ## Relation : One To Many :
    /// * __CorrelationProperty__ (__CorrelationPropertyModel__) from A_correlationPropertyRetrievalExpression_correlationproperty
    ///   * one-to-many link : (1-1) __CorrelationPropertyRetrievalExpression__ need (1-inf) __CorrelationProperty__)
    ///   * callable using find_with_related(__CorrelationPropertyModel__) from __CorrelationPropertyRetrievalExpression__
    ///   * named correlationproperty in BPMN
    /// * __Message__ (__MessageModel__) from A_messageRef_correlationPropertyRetrievalExpression
    ///   * one-to-many link : (1-1) __CorrelationPropertyRetrievalExpression__ need (0-inf) __Message__)
    ///   * callable using find_with_related(__MessageModel__) from __CorrelationPropertyRetrievalExpression__
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __CorrelationPropertyRetrievalExpression__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __CorrelationPropertyRetrievalExpression__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "CorrelationPropertyRetrievalExpression" (bpmn_20_class_correlation_property_retrieval_expression)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__


## Direct One To One :
* __FormalExpression__ (__FormalExpressionModel__) from A_messagePath_correlationset
  * one-to-one link : (1-1) __CorrelationPropertyRetrievalExpression__ need (0-1) __FormalExpression__)
  * callable using find_also_related(__FormalExpressionModel__) from __CorrelationPropertyRetrievalExpression__
  * saved in __message_path__ field as foreing key

## Relation : One To Many :
* __CorrelationProperty__ (__CorrelationPropertyModel__) from A_correlationPropertyRetrievalExpression_correlationproperty
  * one-to-many link : (1-1) __CorrelationPropertyRetrievalExpression__ need (1-inf) __CorrelationProperty__)
  * callable using find_with_related(__CorrelationPropertyModel__) from __CorrelationPropertyRetrievalExpression__
  * named correlationproperty in BPMN
* __Message__ (__MessageModel__) from A_messageRef_correlationPropertyRetrievalExpression
  * one-to-many link : (1-1) __CorrelationPropertyRetrievalExpression__ need (0-inf) __Message__)
  * callable using find_with_related(__MessageModel__) from __CorrelationPropertyRetrievalExpression__

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __CorrelationPropertyRetrievalExpression__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __CorrelationPropertyRetrievalExpression__
  * saved in __super_base_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Weak ref of "BPMN20-CorrelationPropertyRetrievalExpression" (loaded : false)",
//     name: "CorrelationPropertyRetrievalExpression",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-CorrelationPropertyRetrievalExpression-messagePath": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-CorrelationPropertyRetrievalExpression-messagePath" (loaded : false)",
//                 name: "messagePath",
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
//                     "A_messagePath_correlationset",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-CorrelationPropertyRetrievalExpression-messageRef": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-CorrelationPropertyRetrievalExpression-messageRef" (loaded : false)",
//                 name: "messageRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Message",
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
//                     "A_messageRef_correlationPropertyRetrievalExpression",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#CorrelationPropertyRetrievalExpression",
//     table_name: "bpmn_20_correlation_property_retrieval_expression",
//     model_name: "CorrelationPropertyRetrievalExpression",
//     full_name: "bpmn_20_class_correlation_property_retrieval_expression",
// }


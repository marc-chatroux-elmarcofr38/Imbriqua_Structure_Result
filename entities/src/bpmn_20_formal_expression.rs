//! bpmn_20_class_formal_expression

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_formal_expression")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Expression
    pub super_expression: i64,
    /// COMPLEX FIELD : BPMN20-FormalExpression-body
    pub body: i64,
    /// COMPLEX FIELD : BPMN20-FormalExpression-evaluatesToTypeRef
    pub evaluates_to_type_ref: i64,
    /// SIMPLE FIELD : BPMN20-FormalExpression-language
    pub language: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE FormalExpression need ONE Expression
    #[sea_orm(
        belongs_to = "super::bpmn_20_expression::Entity",
        from = "Column::SuperExpression",
        to = "super::bpmn_20_expression::Column::Id",
        on_delete = "Cascade"
    )]
    Expression,
}

// SUPER : ONE FormalExpression need ONE Expression
impl Related<super::bpmn_20_expression::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Expression.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "FormalExpression" (bpmn_20_class_formal_expression)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __language__ (xmi_id : "BPMN20-FormalExpression-language")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __ItemDefinition__ (__ItemDefinitionModel__) from A_evaluatesToTypeRef_formalExpression
    ///   * one-to-many link : (1-1) __FormalExpression__ need (0-inf) __ItemDefinition__)
    ///   * callable using find_with_related(__ItemDefinitionModel__) from __FormalExpression__
    /// 
    /// ## Direct Super :
    /// * __Expression__ (__ExpressionModel__)
    ///   * one-to-one link : one __FormalExpression__ need one __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __FormalExpression__
    ///   * saved in __super_expression__ field as foreing key
    /// ## Reverse One To One :
    /// * __ComplexBehaviorDefinition__ (__ComplexBehaviorDefinitionModel__) from A_condition_complexBehaviorDefinition
    ///   * one-to-one link : (1-1) __ComplexBehaviorDefinition__ need (0-1) __FormalExpression__)
    ///   * callable using find_also_related(__FormalExpressionModel__) from __ComplexBehaviorDefinition__
    ///   * saved in __condition__ field as foreing key
    /// * __CorrelationPropertyBinding__ (__CorrelationPropertyBindingModel__) from A_dataPath_correlationPropertyBinding
    ///   * one-to-one link : (1-1) __CorrelationPropertyBinding__ need (0-1) __FormalExpression__)
    ///   * callable using find_also_related(__FormalExpressionModel__) from __CorrelationPropertyBinding__
    ///   * saved in __data_path__ field as foreing key
    /// * __CorrelationPropertyRetrievalExpression__ (__CorrelationPropertyRetrievalExpressionModel__) from A_messagePath_correlationset
    ///   * one-to-one link : (1-1) __CorrelationPropertyRetrievalExpression__ need (0-1) __FormalExpression__)
    ///   * callable using find_also_related(__FormalExpressionModel__) from __CorrelationPropertyRetrievalExpression__
    ///   * saved in __message_path__ field as foreing key
    /// * __DataAssociation__ (__DataAssociationModel__) from A_transformation_dataAssociation
    ///   * one-to-one link : (0-1) __DataAssociation__ need (0-1) __FormalExpression__)
    ///   * callable using find_also_related(__FormalExpressionModel__) from __DataAssociation__
    ///   * saved in __transformation__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "FormalExpression" (bpmn_20_class_formal_expression)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __language__ (xmi_id : "BPMN20-FormalExpression-language")
  * type : __std::string::String__


## Relation : One To Many :
* __ItemDefinition__ (__ItemDefinitionModel__) from A_evaluatesToTypeRef_formalExpression
  * one-to-many link : (1-1) __FormalExpression__ need (0-inf) __ItemDefinition__)
  * callable using find_with_related(__ItemDefinitionModel__) from __FormalExpression__

## Direct Super :
* __Expression__ (__ExpressionModel__)
  * one-to-one link : one __FormalExpression__ need one __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __FormalExpression__
  * saved in __super_expression__ field as foreing key
## Reverse One To One :
* __ComplexBehaviorDefinition__ (__ComplexBehaviorDefinitionModel__) from A_condition_complexBehaviorDefinition
  * one-to-one link : (1-1) __ComplexBehaviorDefinition__ need (0-1) __FormalExpression__)
  * callable using find_also_related(__FormalExpressionModel__) from __ComplexBehaviorDefinition__
  * saved in __condition__ field as foreing key
* __CorrelationPropertyBinding__ (__CorrelationPropertyBindingModel__) from A_dataPath_correlationPropertyBinding
  * one-to-one link : (1-1) __CorrelationPropertyBinding__ need (0-1) __FormalExpression__)
  * callable using find_also_related(__FormalExpressionModel__) from __CorrelationPropertyBinding__
  * saved in __data_path__ field as foreing key
* __CorrelationPropertyRetrievalExpression__ (__CorrelationPropertyRetrievalExpressionModel__) from A_messagePath_correlationset
  * one-to-one link : (1-1) __CorrelationPropertyRetrievalExpression__ need (0-1) __FormalExpression__)
  * callable using find_also_related(__FormalExpressionModel__) from __CorrelationPropertyRetrievalExpression__
  * saved in __message_path__ field as foreing key
* __DataAssociation__ (__DataAssociationModel__) from A_transformation_dataAssociation
  * one-to-one link : (0-1) __DataAssociation__ need (0-1) __FormalExpression__)
  * callable using find_also_related(__FormalExpressionModel__) from __DataAssociation__
  * saved in __transformation__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "FormalExpression",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "FormalExpression",
//     is_abstract: false,
//     super_class: [
//         "Expression",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-FormalExpression-body": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "FormalExpression-body",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "body",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     ClassLink(
//                         ClassLink {
//                             href: "Extensibilty.cmof#Element",
//                         },
//                     ),
//                 ),
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
//         "-FormalExpression-evaluatesToTypeRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "FormalExpression-evaluatesToTypeRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "evaluatesToTypeRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ItemDefinition",
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
//                     "A_evaluatesToTypeRef_formalExpression",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-FormalExpression-language": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "FormalExpression-language",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "language",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#String",
//                         },
//                     ),
//                 ),
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
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#FormalExpression",
//     table_name: "bpmn_20_formal_expression",
//     model_name: "FormalExpression",
//     full_name: "bpmn_20_class_formal_expression",
// }


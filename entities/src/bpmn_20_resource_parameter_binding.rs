//! bpmn_20_class_resource_parameter_binding

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_resource_parameter_binding")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// COMPLEX FIELD : ResourceParameterBinding-expression
    pub expression: i64,
    /// COMPLEX FIELD : ResourceParameterBinding-parameterRef
    pub parameter_ref: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ResourceParameterBinding" (bpmn_20_class_resource_parameter_binding)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// ## Direct One To One :
    /// * __Expression__ (__ExpressionModel__) from A_expression_resourceParameterBinding
    ///   * one-to-one link : (1-1) __ResourceParameterBinding__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __ResourceParameterBinding__
    ///   * saved in __expression__ field as foreing key
    /// 
    /// ## Relation : One To Many :
    /// * __ResourceParameter__ (__ResourceParameterModel__) from A_parameterRef_resourceParameterBinding
    ///   * one-to-many link : (1-1) __ResourceParameterBinding__ need (0-inf) __ResourceParameter__)
    ///   * callable using find_with_related(__ResourceParameterModel__) from __ResourceParameterBinding__
    /// * __ResourceRole__ (__ResourceRoleModel__) from A_resourceParameterBindings_activityResource
    ///   * one-to-many link : (1-1) __ResourceParameterBinding__ need (0-inf) __ResourceRole__)
    ///   * callable using find_with_related(__ResourceRoleModel__) from __ResourceParameterBinding__
    ///   * named activity_resource in BPMN
    /// 
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ResourceParameterBinding" (bpmn_20_class_resource_parameter_binding)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__


## Direct One To One :
* __Expression__ (__ExpressionModel__) from A_expression_resourceParameterBinding
  * one-to-one link : (1-1) __ResourceParameterBinding__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __ResourceParameterBinding__
  * saved in __expression__ field as foreing key

## Relation : One To Many :
* __ResourceParameter__ (__ResourceParameterModel__) from A_parameterRef_resourceParameterBinding
  * one-to-many link : (1-1) __ResourceParameterBinding__ need (0-inf) __ResourceParameter__)
  * callable using find_with_related(__ResourceParameterModel__) from __ResourceParameterBinding__
* __ResourceRole__ (__ResourceRoleModel__) from A_resourceParameterBindings_activityResource
  * one-to-many link : (1-1) __ResourceParameterBinding__ need (0-inf) __ResourceRole__)
  * callable using find_with_related(__ResourceRoleModel__) from __ResourceParameterBinding__
  * named activity_resource in BPMN



"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "ResourceParameterBinding",
//     name: "ResourceParameterBinding",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: {
//         "ResourceParameterBinding-expression": Property(
//             CMOFProperty {
//                 xmi_id: "ResourceParameterBinding-expression",
//                 name: "expression",
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
//                     "A_expression_resourceParameterBinding",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "ResourceParameterBinding-parameterRef": Property(
//             CMOFProperty {
//                 xmi_id: "ResourceParameterBinding-parameterRef",
//                 name: "parameterRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ResourceParameter",
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
//                     "A_parameterRef_resourceParameterBinding",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#ResourceParameterBinding",
//     table_name: "bpmn_20_resource_parameter_binding",
//     model_name: "ResourceParameterBinding",
//     full_name: "bpmn_20_class_resource_parameter_binding",
// }


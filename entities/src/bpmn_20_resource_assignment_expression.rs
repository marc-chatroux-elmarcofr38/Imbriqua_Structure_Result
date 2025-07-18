//! bpmn_20_class_resource_assignment_expression

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_resource_assignment_expression")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// COMPLEX FIELD : BPMN20-ResourceAssignmentExpression-expression
    pub expression: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ResourceAssignmentExpression" (bpmn_20_class_resource_assignment_expression)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// ## Direct One To One :
    /// * __Expression__ (__ExpressionModel__) from A_expression_resourceAssignmentExpression
    ///   * one-to-one link : (1-1) __ResourceAssignmentExpression__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __ResourceAssignmentExpression__
    ///   * saved in __expression__ field as foreing key
    /// 
    /// 
    /// ## Reverse One To One :
    /// * __ResourceRole__ (__ResourceRoleModel__) from A_resourceAssignmentExpression_activityResource
    ///   * one-to-one link : (0-1) __ResourceRole__ need (1-1) __ResourceAssignmentExpression__)
    ///   * callable using find_also_related(__ResourceAssignmentExpressionModel__) from __ResourceRole__
    ///   * saved in __resource_assignment_expression__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ResourceAssignmentExpression" (bpmn_20_class_resource_assignment_expression)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__


## Direct One To One :
* __Expression__ (__ExpressionModel__) from A_expression_resourceAssignmentExpression
  * one-to-one link : (1-1) __ResourceAssignmentExpression__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __ResourceAssignmentExpression__
  * saved in __expression__ field as foreing key


## Reverse One To One :
* __ResourceRole__ (__ResourceRoleModel__) from A_resourceAssignmentExpression_activityResource
  * one-to-one link : (0-1) __ResourceRole__ need (1-1) __ResourceAssignmentExpression__)
  * callable using find_also_related(__ResourceAssignmentExpressionModel__) from __ResourceRole__
  * saved in __resource_assignment_expression__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "ResourceAssignmentExpression",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "ResourceAssignmentExpression",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: {
//         "-ResourceAssignmentExpression-expression": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "ResourceAssignmentExpression-expression",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
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
//                     "A_expression_resourceAssignmentExpression",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#ResourceAssignmentExpression",
//     table_name: "bpmn_20_resource_assignment_expression",
//     model_name: "ResourceAssignmentExpression",
//     full_name: "bpmn_20_class_resource_assignment_expression",
// }


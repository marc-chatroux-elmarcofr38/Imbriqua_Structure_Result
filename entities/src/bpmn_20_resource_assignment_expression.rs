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

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-ResourceAssignmentExpression',
//     name: "ResourceAssignmentExpression",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: {
//         "ResourceAssignmentExpression-expression": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-ResourceAssignmentExpression-expression',
//                 name: "expression",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-Expression',
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
//                 owning_association: None,
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-A_expression_resourceAssignmentExpression',
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
//     reverse_super: RefCell {
//         value: [],
//     },
// }


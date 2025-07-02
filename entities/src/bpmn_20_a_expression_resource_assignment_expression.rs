//! bpmn_20_association_a_expression_resource_assignment_expression

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_expression_resourceAssignmentExpression",
//     name: "A_expression_resourceAssignmentExpression",
//     visibility: Private,
//     member_end: (
//         "ResourceAssignmentExpression-expression",
//         "A_expression_resourceAssignmentExpression-resourceAssignmentExpression",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_expression_resourceAssignmentExpression-resourceAssignmentExpression",
//                 name: "resourceAssignmentExpression",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ResourceAssignmentExpression",
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
//                 owning_association: "A_expression_resourceAssignmentExpression",
//                 association: Some(
//                     "A_expression_resourceAssignmentExpression",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }


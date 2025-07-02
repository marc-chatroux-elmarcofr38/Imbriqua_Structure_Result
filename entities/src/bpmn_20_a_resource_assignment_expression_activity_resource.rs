//! bpmn_20_association_a_resource_assignment_expression_activity_resource

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_resourceAssignmentExpression_activityResource",
//     name: "A_resourceAssignmentExpression_activityResource",
//     visibility: Private,
//     member_end: (
//         "ResourceRole-resourceAssignmentExpression",
//         "A_resourceAssignmentExpression_activityResource-activityResource",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_resourceAssignmentExpression_activityResource-activityResource",
//                 name: "activityResource",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ResourceRole",
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
//                 owning_association: "A_resourceAssignmentExpression_activityResource",
//                 association: Some(
//                     "A_resourceAssignmentExpression_activityResource",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }


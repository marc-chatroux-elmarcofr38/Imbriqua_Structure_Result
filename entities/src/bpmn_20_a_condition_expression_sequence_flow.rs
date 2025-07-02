//! bpmn_20_association_a_condition_expression_sequence_flow

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_conditionExpression_sequenceFlow",
//     name: "A_conditionExpression_sequenceFlow",
//     visibility: Private,
//     member_end: (
//         "SequenceFlow-conditionExpression",
//         "A_conditionExpression_sequenceFlow-sequenceFlow",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_conditionExpression_sequenceFlow-sequenceFlow",
//                 name: "sequenceFlow",
//                 visibility: Public,
//                 simple_type: Some(
//                     "SequenceFlow",
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
//                 owning_association: "A_conditionExpression_sequenceFlow",
//                 association: Some(
//                     "A_conditionExpression_sequenceFlow",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }


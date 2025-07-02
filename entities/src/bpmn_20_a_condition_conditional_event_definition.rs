//! bpmn_20_association_a_condition_conditional_event_definition

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_condition_conditionalEventDefinition",
//     name: "A_condition_conditionalEventDefinition",
//     visibility: Private,
//     member_end: (
//         "ConditionalEventDefinition-condition",
//         "A_condition_conditionalEventDefinition-conditionalEventDefinition",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_condition_conditionalEventDefinition-conditionalEventDefinition",
//                 name: "conditionalEventDefinition",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ConditionalEventDefinition",
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
//                 owning_association: "A_condition_conditionalEventDefinition",
//                 association: Some(
//                     "A_condition_conditionalEventDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }


//! bpmn_20_association_a_default_activity

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_default_activity",
//     name: "A_default_activity",
//     visibility: Private,
//     member_end: (
//         "Activity-default",
//         "A_default_activity-activity",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_default_activity-activity",
//                 name: "activity",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Activity",
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
//                 owning_association: "A_default_activity",
//                 association: Some(
//                     "A_default_activity",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }


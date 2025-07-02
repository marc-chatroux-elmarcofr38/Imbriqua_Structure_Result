//! bpmn_20_association_a_called_choreography_ref_call_choreography_activity

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_calledChoreographyRef_callChoreographyActivity",
//     name: "A_calledChoreographyRef_callChoreographyActivity",
//     visibility: Private,
//     member_end: (
//         "CallChoreography-calledChoreographyRef",
//         "A_calledChoreographyRef_callChoreographyActivity-callChoreographyActivity",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_calledChoreographyRef_callChoreographyActivity-callChoreographyActivity",
//                 name: "callChoreographyActivity",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CallChoreography",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "A_calledChoreographyRef_callChoreographyActivity",
//                 association: Some(
//                     "A_calledChoreographyRef_callChoreographyActivity",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }


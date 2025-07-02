//! bpmn_20_association_a_message_flow_ref_choreography_task

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_messageFlowRef_choreographyTask",
//     name: "A_messageFlowRef_choreographyTask",
//     visibility: Private,
//     member_end: (
//         "ChoreographyTask-messageFlowRef",
//         "A_messageFlowRef_choreographyTask-choreographyTask",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_messageFlowRef_choreographyTask-choreographyTask",
//                 name: "choreographyTask",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ChoreographyTask",
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
//                 owning_association: "A_messageFlowRef_choreographyTask",
//                 association: Some(
//                     "A_messageFlowRef_choreographyTask",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }


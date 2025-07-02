//! bpmn_20_association_a_definitional_collaboration_ref_process

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_definitionalCollaborationRef_process",
//     name: "A_definitionalCollaborationRef_process",
//     visibility: Private,
//     member_end: (
//         "Process-definitionalCollaborationRef",
//         "A_definitionalCollaborationRef_process-process",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_definitionalCollaborationRef_process-process",
//                 name: "process",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Process",
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
//                 owning_association: "A_definitionalCollaborationRef_process",
//                 association: Some(
//                     "A_definitionalCollaborationRef_process",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }


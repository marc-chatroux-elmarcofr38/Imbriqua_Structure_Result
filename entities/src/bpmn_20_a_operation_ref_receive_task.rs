//! bpmn_20_association_a_operation_ref_receive_task

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_operationRef_receiveTask",
//     name: "A_operationRef_receiveTask",
//     visibility: Private,
//     member_end: (
//         "ReceiveTask-operationRef",
//         "A_operationRef_receiveTask-receiveTask",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_operationRef_receiveTask-receiveTask",
//                 name: "receiveTask",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ReceiveTask",
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
//                 owning_association: "A_operationRef_receiveTask",
//                 association: Some(
//                     "A_operationRef_receiveTask",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }


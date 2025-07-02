//! bpmn_20_association_a_operation_ref_service_task

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_operationRef_serviceTask",
//     name: "A_operationRef_serviceTask",
//     visibility: Private,
//     member_end: (
//         "ServiceTask-operationRef",
//         "A_operationRef_serviceTask-serviceTask",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_operationRef_serviceTask-serviceTask",
//                 name: "serviceTask",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ServiceTask",
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
//                 owning_association: "A_operationRef_serviceTask",
//                 association: Some(
//                     "A_operationRef_serviceTask",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }


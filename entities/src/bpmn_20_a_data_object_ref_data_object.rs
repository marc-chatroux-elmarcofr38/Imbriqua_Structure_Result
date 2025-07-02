//! bpmn_20_association_a_data_object_ref_data_object

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_dataObjectRef_dataObject",
//     name: "A_dataObjectRef_dataObject",
//     visibility: Private,
//     member_end: (
//         "DataObjectReference-dataObjectRef",
//         "A_dataObjectRef_dataObject-dataObject",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_dataObjectRef_dataObject-dataObject",
//                 name: "dataObject",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataObjectReference",
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
//                 owning_association: "A_dataObjectRef_dataObject",
//                 association: Some(
//                     "A_dataObjectRef_dataObject",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }


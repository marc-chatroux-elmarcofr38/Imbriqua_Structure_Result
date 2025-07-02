//! bpmn_20_association_a_time_cycle_timer_event_definition

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_timeCycle_timerEventDefinition",
//     name: "A_timeCycle_timerEventDefinition",
//     visibility: Private,
//     member_end: (
//         "TimerEventDefinition-timeCycle",
//         "A_timeCycle_timerEventDefinition-timerEventDefinition",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_timeCycle_timerEventDefinition-timerEventDefinition",
//                 name: "timerEventDefinition",
//                 visibility: Public,
//                 simple_type: Some(
//                     "TimerEventDefinition",
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
//                 owning_association: "A_timeCycle_timerEventDefinition",
//                 association: Some(
//                     "A_timeCycle_timerEventDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }


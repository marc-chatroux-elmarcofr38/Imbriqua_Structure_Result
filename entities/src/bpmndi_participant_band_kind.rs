//! bpmndi_enumeration_participant_band_kind

use sea_orm::entity::prelude::*;

#[derive(Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ParticipantBandKind {
    #[sea_orm(string_value = "top_initiating")]
    TopInitiating,
    #[sea_orm(string_value = "middle_initiating")]
    MiddleInitiating,
    #[sea_orm(string_value = "bottom_initiating")]
    BottomInitiating,
    #[sea_orm(string_value = "top_non_initiating")]
    TopNonInitiating,
    #[sea_orm(string_value = "middle_non_initiating")]
    MiddleNonInitiating,
    #[sea_orm(string_value = "bottom_non_initiating")]
    BottomNonInitiating,
}

// RAW :
// CMOFEnumeration {
//     xmi_id: "ParticipantBandKind",
//     name: "ParticipantBandKind",
//     owned_attribute: [
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ParticipantBandKind-top_initiating",
//                 name: "top_initiating",
//                 classifier: "ParticipantBandKind",
//                 enumeration: "ParticipantBandKind",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ParticipantBandKind-middle_initiating",
//                 name: "middle_initiating",
//                 classifier: "ParticipantBandKind",
//                 enumeration: "ParticipantBandKind",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ParticipantBandKind-bottom_initiating",
//                 name: "bottom_initiating",
//                 classifier: "ParticipantBandKind",
//                 enumeration: "ParticipantBandKind",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ParticipantBandKind-top_non_initiating",
//                 name: "top_non_initiating",
//                 classifier: "ParticipantBandKind",
//                 enumeration: "ParticipantBandKind",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ParticipantBandKind-middle_non_initiating",
//                 name: "middle_non_initiating",
//                 classifier: "ParticipantBandKind",
//                 enumeration: "ParticipantBandKind",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ParticipantBandKind-bottom_non_initiating",
//                 name: "bottom_non_initiating",
//                 classifier: "ParticipantBandKind",
//                 enumeration: "ParticipantBandKind",
//             },
//         ),
//     ],
// }


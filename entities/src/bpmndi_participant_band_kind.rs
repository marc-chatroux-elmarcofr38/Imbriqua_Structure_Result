//! bpmndi_enumeration_participant_band_kind

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ParticipantBandKind {
    /// ENUMERATION LITERAL : ParticipantBandKind-top_initiating
    #[default]
    #[sea_orm(string_value = "top_initiating")]
    TopInitiating,
    /// ENUMERATION LITERAL : ParticipantBandKind-middle_initiating
    #[sea_orm(string_value = "middle_initiating")]
    MiddleInitiating,
    /// ENUMERATION LITERAL : ParticipantBandKind-bottom_initiating
    #[sea_orm(string_value = "bottom_initiating")]
    BottomInitiating,
    /// ENUMERATION LITERAL : ParticipantBandKind-top_non_initiating
    #[sea_orm(string_value = "top_non_initiating")]
    TopNonInitiating,
    /// ENUMERATION LITERAL : ParticipantBandKind-middle_non_initiating
    #[sea_orm(string_value = "middle_non_initiating")]
    MiddleNonInitiating,
    /// ENUMERATION LITERAL : ParticipantBandKind-bottom_non_initiating
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


//! bpmndi_enumeration_participant_band_kind

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ParticipantBandKind {
    /// ENUMERATION LITERAL : BPMNDI-ParticipantBandKind-bottom_initiating
    #[sea_orm(string_value = "bottom_initiating")]
    BottomInitiating,
    /// ENUMERATION LITERAL : BPMNDI-ParticipantBandKind-bottom_non_initiating
    #[sea_orm(string_value = "bottom_non_initiating")]
    BottomNonInitiating,
    /// ENUMERATION LITERAL : BPMNDI-ParticipantBandKind-middle_initiating
    #[sea_orm(string_value = "middle_initiating")]
    MiddleInitiating,
    /// ENUMERATION LITERAL : BPMNDI-ParticipantBandKind-middle_non_initiating
    #[sea_orm(string_value = "middle_non_initiating")]
    MiddleNonInitiating,
    /// ENUMERATION LITERAL : BPMNDI-ParticipantBandKind-top_initiating
    #[default]
    #[sea_orm(string_value = "top_initiating")]
    TopInitiating,
    /// ENUMERATION LITERAL : BPMNDI-ParticipantBandKind-top_non_initiating
    #[sea_orm(string_value = "top_non_initiating")]
    TopNonInitiating,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: "Weak ref of "BPMNDI-ParticipantBandKind" (loaded : false)",
//     name: "ParticipantBandKind",
//     owned_attribute: {
//         "-ParticipantBandKind-bottom_initiating": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Weak ref of "BPMNDI-ParticipantBandKind-bottom_initiating" (loaded : false)",
//                 name: "bottom_initiating",
//                 classifier: "ParticipantBandKind",
//                 enumeration: "ParticipantBandKind",
//             },
//         ),
//         "-ParticipantBandKind-bottom_non_initiating": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Weak ref of "BPMNDI-ParticipantBandKind-bottom_non_initiating" (loaded : false)",
//                 name: "bottom_non_initiating",
//                 classifier: "ParticipantBandKind",
//                 enumeration: "ParticipantBandKind",
//             },
//         ),
//         "-ParticipantBandKind-middle_initiating": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Weak ref of "BPMNDI-ParticipantBandKind-middle_initiating" (loaded : false)",
//                 name: "middle_initiating",
//                 classifier: "ParticipantBandKind",
//                 enumeration: "ParticipantBandKind",
//             },
//         ),
//         "-ParticipantBandKind-middle_non_initiating": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Weak ref of "BPMNDI-ParticipantBandKind-middle_non_initiating" (loaded : false)",
//                 name: "middle_non_initiating",
//                 classifier: "ParticipantBandKind",
//                 enumeration: "ParticipantBandKind",
//             },
//         ),
//         "-ParticipantBandKind-top_initiating": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Weak ref of "BPMNDI-ParticipantBandKind-top_initiating" (loaded : false)",
//                 name: "top_initiating",
//                 classifier: "ParticipantBandKind",
//                 enumeration: "ParticipantBandKind",
//             },
//         ),
//         "-ParticipantBandKind-top_non_initiating": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Weak ref of "BPMNDI-ParticipantBandKind-top_non_initiating" (loaded : false)",
//                 name: "top_non_initiating",
//                 classifier: "ParticipantBandKind",
//                 enumeration: "ParticipantBandKind",
//             },
//         ),
//     },
//     technical_name: "BPMNDI.cmof#ParticipantBandKind",
//     table_name: "bpmndi_participant_band_kind",
//     model_name: "ParticipantBandKind",
//     full_name: "bpmndi_enumeration_participant_band_kind",
// }


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
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-ParticipantBandKind',
//     name: "ParticipantBandKind",
//     owned_attribute: {
//         "ParticipantBandKind-bottom_initiating": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-ParticipantBandKind-bottom_initiating',
//                 name: "bottom_initiating",
//                 _classifier: "ParticipantBandKind",
//                 _enumeration: "ParticipantBandKind",
//                 litteral_name: "BottomInitiating",
//                 litteral_designation: "bottom_initiating",
//             },
//         ),
//         "ParticipantBandKind-bottom_non_initiating": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-ParticipantBandKind-bottom_non_initiating',
//                 name: "bottom_non_initiating",
//                 _classifier: "ParticipantBandKind",
//                 _enumeration: "ParticipantBandKind",
//                 litteral_name: "BottomNonInitiating",
//                 litteral_designation: "bottom_non_initiating",
//             },
//         ),
//         "ParticipantBandKind-middle_initiating": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-ParticipantBandKind-middle_initiating',
//                 name: "middle_initiating",
//                 _classifier: "ParticipantBandKind",
//                 _enumeration: "ParticipantBandKind",
//                 litteral_name: "MiddleInitiating",
//                 litteral_designation: "middle_initiating",
//             },
//         ),
//         "ParticipantBandKind-middle_non_initiating": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-ParticipantBandKind-middle_non_initiating',
//                 name: "middle_non_initiating",
//                 _classifier: "ParticipantBandKind",
//                 _enumeration: "ParticipantBandKind",
//                 litteral_name: "MiddleNonInitiating",
//                 litteral_designation: "middle_non_initiating",
//             },
//         ),
//         "ParticipantBandKind-top_initiating": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-ParticipantBandKind-top_initiating',
//                 name: "top_initiating",
//                 _classifier: "ParticipantBandKind",
//                 _enumeration: "ParticipantBandKind",
//                 litteral_name: "TopInitiating",
//                 litteral_designation: "top_initiating",
//             },
//         ),
//         "ParticipantBandKind-top_non_initiating": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-ParticipantBandKind-top_non_initiating',
//                 name: "top_non_initiating",
//                 _classifier: "ParticipantBandKind",
//                 _enumeration: "ParticipantBandKind",
//                 litteral_name: "TopNonInitiating",
//                 litteral_designation: "top_non_initiating",
//             },
//         ),
//     },
//     technical_name: "BPMNDI.cmof#ParticipantBandKind",
//     table_name: "bpmndi_participant_band_kind",
//     model_name: "ParticipantBandKind",
//     full_name: "bpmndi_enumeration_participant_band_kind",
// }


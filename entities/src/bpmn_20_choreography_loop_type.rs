//! bpmn_20_enumeration_choreography_loop_type

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ChoreographyLoopType {
    /// ENUMERATION LITERAL : BPMN20-ChoreographyLoopType-MultiInstanceParallel
    #[sea_orm(string_value = "MultiInstanceParallel")]
    MultiInstanceParallel,
    /// ENUMERATION LITERAL : BPMN20-ChoreographyLoopType-MultiInstanceSequential
    #[sea_orm(string_value = "MultiInstanceSequential")]
    MultiInstanceSequential,
    /// ENUMERATION LITERAL : BPMN20-ChoreographyLoopType-None
    #[default]
    #[sea_orm(string_value = "None")]
    None,
    /// ENUMERATION LITERAL : BPMN20-ChoreographyLoopType-Standard
    #[sea_orm(string_value = "Standard")]
    Standard,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: XMIIdReference {
//         object_id: "ChoreographyLoopType",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "ChoreographyLoopType",
//     owned_attribute: {
//         "-ChoreographyLoopType-MultiInstanceParallel": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdReference {
//                     object_id: "ChoreographyLoopType-MultiInstanceParallel",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "MultiInstanceParallel",
//                 classifier: "ChoreographyLoopType",
//                 enumeration: "ChoreographyLoopType",
//             },
//         ),
//         "-ChoreographyLoopType-MultiInstanceSequential": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdReference {
//                     object_id: "ChoreographyLoopType-MultiInstanceSequential",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "MultiInstanceSequential",
//                 classifier: "ChoreographyLoopType",
//                 enumeration: "ChoreographyLoopType",
//             },
//         ),
//         "-ChoreographyLoopType-None": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdReference {
//                     object_id: "ChoreographyLoopType-None",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "None",
//                 classifier: "ChoreographyLoopType",
//                 enumeration: "ChoreographyLoopType",
//             },
//         ),
//         "-ChoreographyLoopType-Standard": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdReference {
//                     object_id: "ChoreographyLoopType-Standard",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "Standard",
//                 classifier: "ChoreographyLoopType",
//                 enumeration: "ChoreographyLoopType",
//             },
//         ),
//     },
//     technical_name: "BPMN20.cmof#ChoreographyLoopType",
//     table_name: "bpmn_20_choreography_loop_type",
//     model_name: "ChoreographyLoopType",
//     full_name: "bpmn_20_enumeration_choreography_loop_type",
// }


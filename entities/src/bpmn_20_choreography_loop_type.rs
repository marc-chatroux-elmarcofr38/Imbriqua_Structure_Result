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
//     xmi_id: "Weak ref of "BPMN20-ChoreographyLoopType" (loaded : false)",
//     name: "ChoreographyLoopType",
//     owned_attribute: {
//         "-ChoreographyLoopType-MultiInstanceParallel": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Weak ref of "BPMN20-ChoreographyLoopType-MultiInstanceParallel" (loaded : false)",
//                 name: "MultiInstanceParallel",
//                 classifier: "ChoreographyLoopType",
//                 enumeration: "ChoreographyLoopType",
//             },
//         ),
//         "-ChoreographyLoopType-MultiInstanceSequential": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Weak ref of "BPMN20-ChoreographyLoopType-MultiInstanceSequential" (loaded : false)",
//                 name: "MultiInstanceSequential",
//                 classifier: "ChoreographyLoopType",
//                 enumeration: "ChoreographyLoopType",
//             },
//         ),
//         "-ChoreographyLoopType-None": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Weak ref of "BPMN20-ChoreographyLoopType-None" (loaded : false)",
//                 name: "None",
//                 classifier: "ChoreographyLoopType",
//                 enumeration: "ChoreographyLoopType",
//             },
//         ),
//         "-ChoreographyLoopType-Standard": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Weak ref of "BPMN20-ChoreographyLoopType-Standard" (loaded : false)",
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


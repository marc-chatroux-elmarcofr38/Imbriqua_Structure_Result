//! bpmn_20_enumeration_choreography_loop_type

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ChoreographyLoopType {
    /// ENUMERATION LITERAL : ChoreographyLoopType-MultiInstanceParallel
    #[sea_orm(string_value = "MultiInstanceParallel")]
    MultiInstanceParallel,
    /// ENUMERATION LITERAL : ChoreographyLoopType-MultiInstanceSequential
    #[sea_orm(string_value = "MultiInstanceSequential")]
    MultiInstanceSequential,
    /// ENUMERATION LITERAL : ChoreographyLoopType-None
    #[default]
    #[sea_orm(string_value = "None")]
    None,
    /// ENUMERATION LITERAL : ChoreographyLoopType-Standard
    #[sea_orm(string_value = "Standard")]
    Standard,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: "ChoreographyLoopType",
//     name: "ChoreographyLoopType",
//     owned_attribute: {
//         "ChoreographyLoopType-MultiInstanceParallel": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ChoreographyLoopType-MultiInstanceParallel",
//                 name: "MultiInstanceParallel",
//                 classifier: "ChoreographyLoopType",
//                 enumeration: "ChoreographyLoopType",
//             },
//         ),
//         "ChoreographyLoopType-MultiInstanceSequential": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ChoreographyLoopType-MultiInstanceSequential",
//                 name: "MultiInstanceSequential",
//                 classifier: "ChoreographyLoopType",
//                 enumeration: "ChoreographyLoopType",
//             },
//         ),
//         "ChoreographyLoopType-None": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ChoreographyLoopType-None",
//                 name: "None",
//                 classifier: "ChoreographyLoopType",
//                 enumeration: "ChoreographyLoopType",
//             },
//         ),
//         "ChoreographyLoopType-Standard": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ChoreographyLoopType-Standard",
//                 name: "Standard",
//                 classifier: "ChoreographyLoopType",
//                 enumeration: "ChoreographyLoopType",
//             },
//         ),
//     },
// }


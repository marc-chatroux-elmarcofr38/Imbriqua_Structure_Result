//! bpmn_20_enumeration_choreography_loop_type

use sea_orm::entity::prelude::*;

#[derive(Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ChoreographyLoopType {
    #[sea_orm(string_value = "None")]
    None,
    #[sea_orm(string_value = "Standard")]
    Standard,
    #[sea_orm(string_value = "MultiInstanceSequential")]
    MultiInstanceSequential,
    #[sea_orm(string_value = "MultiInstanceParallel")]
    MultiInstanceParallel,
}

// RAW :
// CMOFEnumeration {
//     xmi_id: "ChoreographyLoopType",
//     name: "ChoreographyLoopType",
//     owned_attribute: [
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ChoreographyLoopType-None",
//                 name: "None",
//                 classifier: "ChoreographyLoopType",
//                 enumeration: "ChoreographyLoopType",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ChoreographyLoopType-Standard",
//                 name: "Standard",
//                 classifier: "ChoreographyLoopType",
//                 enumeration: "ChoreographyLoopType",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ChoreographyLoopType-MultiInstanceSequential",
//                 name: "MultiInstanceSequential",
//                 classifier: "ChoreographyLoopType",
//                 enumeration: "ChoreographyLoopType",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ChoreographyLoopType-MultiInstanceParallel",
//                 name: "MultiInstanceParallel",
//                 classifier: "ChoreographyLoopType",
//                 enumeration: "ChoreographyLoopType",
//             },
//         ),
//     ],
// }


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
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-ChoreographyLoopType',
//     name: "ChoreographyLoopType",
//     owned_attribute: {
//         "ChoreographyLoopType-MultiInstanceParallel": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-ChoreographyLoopType-MultiInstanceParallel',
//                 name: "MultiInstanceParallel",
//                 _classifier: "ChoreographyLoopType",
//                 _enumeration: "ChoreographyLoopType",
//                 litteral_name: "MultiInstanceParallel",
//                 litteral_designation: "MultiInstanceParallel",
//             },
//         ),
//         "ChoreographyLoopType-MultiInstanceSequential": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-ChoreographyLoopType-MultiInstanceSequential',
//                 name: "MultiInstanceSequential",
//                 _classifier: "ChoreographyLoopType",
//                 _enumeration: "ChoreographyLoopType",
//                 litteral_name: "MultiInstanceSequential",
//                 litteral_designation: "MultiInstanceSequential",
//             },
//         ),
//         "ChoreographyLoopType-None": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-ChoreographyLoopType-None',
//                 name: "None",
//                 _classifier: "ChoreographyLoopType",
//                 _enumeration: "ChoreographyLoopType",
//                 litteral_name: "None",
//                 litteral_designation: "None",
//             },
//         ),
//         "ChoreographyLoopType-Standard": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-ChoreographyLoopType-Standard',
//                 name: "Standard",
//                 _classifier: "ChoreographyLoopType",
//                 _enumeration: "ChoreographyLoopType",
//                 litteral_name: "Standard",
//                 litteral_designation: "Standard",
//             },
//         ),
//     },
//     technical_name: "BPMN20.cmof#ChoreographyLoopType",
//     table_name: "bpmn_20_choreography_loop_type",
//     model_name: "ChoreographyLoopType",
//     full_name: "bpmn_20_enumeration_choreography_loop_type",
// }


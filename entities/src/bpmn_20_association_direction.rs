//! bpmn_20_enumeration_association_direction

use sea_orm::entity::prelude::*;

#[derive(Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum AssociationDirection {
    #[sea_orm(string_value = "None")]
    None,
    #[sea_orm(string_value = "One")]
    One,
    #[sea_orm(string_value = "Both")]
    Both,
}

// RAW :
// CMOFEnumeration {
//     xmi_id: "AssociationDirection",
//     name: "AssociationDirection",
//     owned_attribute: [
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "AssociationDirection-None",
//                 name: "None",
//                 classifier: "AssociationDirection",
//                 enumeration: "AssociationDirection",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "AssociationDirection-One",
//                 name: "One",
//                 classifier: "AssociationDirection",
//                 enumeration: "AssociationDirection",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "AssociationDirection-Both",
//                 name: "Both",
//                 classifier: "AssociationDirection",
//                 enumeration: "AssociationDirection",
//             },
//         ),
//     ],
// }


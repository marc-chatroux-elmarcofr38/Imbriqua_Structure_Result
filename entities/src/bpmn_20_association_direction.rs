//! bpmn_20_enumeration_association_direction

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum AssociationDirection {
    /// ENUMERATION LITERAL : AssociationDirection-None
    #[default]
    #[sea_orm(string_value = "None")]
    None,
    /// ENUMERATION LITERAL : AssociationDirection-One
    #[sea_orm(string_value = "One")]
    One,
    /// ENUMERATION LITERAL : AssociationDirection-Both
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


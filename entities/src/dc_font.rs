//! dc_datatype_font

use crate::*;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "dc_font")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// RUST DATA TYPE : DC-Font-isBold
    pub is_bold: Option<Boolean>,
    /// RUST DATA TYPE : DC-Font-isItalic
    pub is_italic: Option<Boolean>,
    /// RUST DATA TYPE : DC-Font-isStrikeThrough
    pub is_strike_through: Option<Boolean>,
    /// RUST DATA TYPE : DC-Font-isUnderline
    pub is_underline: Option<Boolean>,
    /// RUST DATA TYPE : DC-Font-name
    pub name: Option<String>,
    /// RUST DATA TYPE : DC-Font-size
    pub size: Option<Real>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFDataType {
//     xmi_id: "Weak ref of "DC-Font" (loaded : false)",
//     name: "Font",
//     owned_attribute: {
//         "-Font-isBold": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "DC-Font-isBold" (loaded : false)",
//                 name: "isBold",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Boolean",
//                 ),
//                 complex_type: None,
//                 datatype: Some(
//                     "Font",
//                 ),
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Font-isItalic": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "DC-Font-isItalic" (loaded : false)",
//                 name: "isItalic",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Boolean",
//                 ),
//                 complex_type: None,
//                 datatype: Some(
//                     "Font",
//                 ),
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Font-isStrikeThrough": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "DC-Font-isStrikeThrough" (loaded : false)",
//                 name: "isStrikeThrough",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Boolean",
//                 ),
//                 complex_type: None,
//                 datatype: Some(
//                     "Font",
//                 ),
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Font-isUnderline": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "DC-Font-isUnderline" (loaded : false)",
//                 name: "isUnderline",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Boolean",
//                 ),
//                 complex_type: None,
//                 datatype: Some(
//                     "Font",
//                 ),
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Font-name": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "DC-Font-name" (loaded : false)",
//                 name: "name",
//                 visibility: Public,
//                 simple_type: Some(
//                     "String",
//                 ),
//                 complex_type: None,
//                 datatype: Some(
//                     "Font",
//                 ),
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Font-size": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "DC-Font-size" (loaded : false)",
//                 name: "size",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Real",
//                 ),
//                 complex_type: None,
//                 datatype: Some(
//                     "Font",
//                 ),
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {
//         "-Font-non_negative_size": Constraint(
//             CMOFConstraint {
//                 xmi_id: "Weak ref of "DC-Font-non_negative_size" (loaded : false)",
//                 name: "non_negative_size",
//                 constrained_element: "Font",
//                 namespace: "Font",
//                 specification: OpaqueExpression(
//                     CMOFOpaqueExpression {
//                         xmi_id: "Weak ref of "DC-Font-non_negative_size-_specification" (loaded : false)",
//                         body: "size >=  0",
//                         language: "OCL",
//                     },
//                 ),
//             },
//         ),
//     },
//     technical_name: "DC.cmof#Font",
//     table_name: "dc_font",
//     model_name: "Font",
//     full_name: "dc_datatype_font",
// }


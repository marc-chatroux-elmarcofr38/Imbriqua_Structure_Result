//! dc_datatype_font

use crate::*;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel, Default)]
#[sea_orm(table_name = "dc_font")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    name : Option<String>,
    size : Option<Real>,
    is_bold : Option<Boolean>,
    is_italic : Option<Boolean>,
    is_underline : Option<Boolean>,
    is_strike_through : Option<Boolean>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFDataType {
//     xmi_id: "Font",
//     name: "Font",
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "Font-name",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "Font-size",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "Font-isBold",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "Font-isItalic",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "Font-isUnderline",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "Font-isStrikeThrough",
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
//     ],
//     owned_rule: [
//         Constraint(
//             CMOFConstraint {
//                 xmi_id: "Font-non_negative_size",
//                 name: "non_negative_size",
//                 constrained_element: "Font",
//                 namespace: "Font",
//                 specification: OpaqueExpression(
//                     CMOFOpaqueExpression {
//                         xmi_id: "Font-non_negative_size-_specification",
//                         body: "size >=  0",
//                         language: "OCL",
//                     },
//                 ),
//             },
//         ),
//     ],
// }


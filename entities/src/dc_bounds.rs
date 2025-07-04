//! dc_datatype_bounds

use crate::*;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "dc_bounds")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// RUST DATA TYPE : Bounds-x
    #[sea_orm(default_value = "0")]
    pub x: Real,
    /// RUST DATA TYPE : Bounds-y
    #[sea_orm(default_value = "0")]
    pub y: Real,
    /// RUST DATA TYPE : Bounds-width
    pub width: Real,
    /// RUST DATA TYPE : Bounds-height
    pub height: Real,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFDataType {
//     xmi_id: "Bounds",
//     name: "Bounds",
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "Bounds-x",
//                 name: "x",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Real",
//                 ),
//                 complex_type: None,
//                 datatype: Some(
//                     "Bounds",
//                 ),
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: Some(
//                     "0",
//                 ),
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
//                 xmi_id: "Bounds-y",
//                 name: "y",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Real",
//                 ),
//                 complex_type: None,
//                 datatype: Some(
//                     "Bounds",
//                 ),
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: Some(
//                     "0",
//                 ),
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
//                 xmi_id: "Bounds-width",
//                 name: "width",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Real",
//                 ),
//                 complex_type: None,
//                 datatype: Some(
//                     "Bounds",
//                 ),
//                 lower: 1,
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
//                 xmi_id: "Bounds-height",
//                 name: "height",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Real",
//                 ),
//                 complex_type: None,
//                 datatype: Some(
//                     "Bounds",
//                 ),
//                 lower: 1,
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
//     owned_rule: [],
// }


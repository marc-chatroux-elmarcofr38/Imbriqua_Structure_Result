//! dc_datatype_point

use crate::*;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "dc_point")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// RUST DATA TYPE : DC-Point-x
    #[sea_orm(default_value = "0")]
    pub x: Real,
    /// RUST DATA TYPE : DC-Point-y
    #[sea_orm(default_value = "0")]
    pub y: Real,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFDataType {
//     xmi_id: XMIIdLocalReference {
//         object_id: "Point",
//         package_id: "DC",
//         is_set: true,
//     },
//     name: "Point",
//     owned_attribute: {
//         "Point-x": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "Point-x",
//                     package_id: "DC",
//                     is_set: true,
//                 },
//                 name: "x",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Real",
//                 ),
//                 complex_type: None,
//                 datatype: Some(
//                     "Point",
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
//         "Point-y": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "Point-y",
//                     package_id: "DC",
//                     is_set: true,
//                 },
//                 name: "y",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Real",
//                 ),
//                 complex_type: None,
//                 datatype: Some(
//                     "Point",
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
//     },
//     owned_rule: {},
//     technical_name: "DC.cmof#Point",
//     table_name: "dc_point",
//     model_name: "Point",
//     full_name: "dc_datatype_point",
// }


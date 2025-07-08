//! bpmn_20_class_transaction

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_transaction")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SubProcess
    pub super_sub_process: i64,
    /// SIMPLE FIELD : Transaction-protocol
    pub protocol: Option<std::string::String>,
    /// SIMPLE FIELD : Transaction-method
    pub method: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Transaction need ONE SubProcess
    #[sea_orm(
        belongs_to = "super::bpmn_20_sub_process::Entity",
        from = "Column::SuperSubProcess",
        to = "super::bpmn_20_sub_process::Column::Id",
        on_delete = "Cascade"
    )]
    SubProcess,
}

// SUPER : ONE Transaction need ONE SubProcess
impl Related<super::bpmn_20_sub_process::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubProcess.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "Transaction",
//     name: "Transaction",
//     is_abstract: false,
//     super_class: [
//         "SubProcess",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "Transaction-protocol",
//                 name: "protocol",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#String",
//                         },
//                     ),
//                 ),
//                 datatype: None,
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
//                 xmi_id: "Transaction-method",
//                 name: "method",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#String",
//                         },
//                     ),
//                 ),
//                 datatype: None,
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


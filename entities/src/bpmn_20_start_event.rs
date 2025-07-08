//! bpmn_20_class_start_event

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_start_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : CatchEvent
    pub super_catch_event: i64,
    /// SIMPLE FIELD : StartEvent-isInterrupting
    #[sea_orm(default_value = "true")]
    pub is_interrupting: std::primitive::bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE StartEvent need ONE CatchEvent
    #[sea_orm(
        belongs_to = "super::bpmn_20_catch_event::Entity",
        from = "Column::SuperCatchEvent",
        to = "super::bpmn_20_catch_event::Column::Id",
        on_delete = "Cascade"
    )]
    CatchEvent,
}

// SUPER : ONE StartEvent need ONE CatchEvent
impl Related<super::bpmn_20_catch_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CatchEvent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "StartEvent",
//     name: "StartEvent",
//     is_abstract: false,
//     super_class: [
//         "CatchEvent",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "StartEvent-isInterrupting",
//                 name: "isInterrupting",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#Boolean",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: Some(
//                     "true",
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
//     ],
//     owned_rule: [],
// }


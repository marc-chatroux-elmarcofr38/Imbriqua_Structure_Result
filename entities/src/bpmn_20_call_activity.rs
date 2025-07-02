//! bpmn_20_class_call_activity

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_call_activity")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : Activity
    pub super_activity: i32,
    /// COMPLEX FIELD : CallActivity-calledElementRef
    pub called_element_ref: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_activity::Entity",
        from = "Column::SuperActivity",
        to = "super::bpmn_20_activity::Column::Id"
    )]
    Activity,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Activity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "CallActivity",
//     name: "CallActivity",
//     is_abstract: false,
//     super_class: [
//         "Activity",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "CallActivity-calledElementRef",
//                 name: "calledElementRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CallableElement",
//                 ),
//                 complex_type: None,
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
//                 association: Some(
//                     "A_calledElementRef_callActivity",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }


//! bpmn_20_class_ad_hoc_sub_process

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_ad_hoc_sub_process")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SubProcess
    pub super_sub_process: i64,
    /// COMPLEX FIELD : AdHocSubProcess-completionCondition
    pub completion_condition: i64,
    /// SIMPLE FIELD : AdHocSubProcess-ordering
    pub ordering: AdHocOrdering,
    /// SIMPLE FIELD : AdHocSubProcess-cancelRemainingInstances
    #[sea_orm(default_value = "true")]
    pub cancel_remaining_instances: std::primitive::bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE AdHocSubProcess need ONE SubProcess
    #[sea_orm(
        belongs_to = "super::bpmn_20_sub_process::Entity",
        from = "Column::SuperSubProcess",
        to = "super::bpmn_20_sub_process::Column::Id"
    )]
    SubProcess,
}

// SUPER : ONE AdHocSubProcess need ONE SubProcess
impl Related<super::bpmn_20_sub_process::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubProcess.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "AdHocSubProcess",
//     name: "AdHocSubProcess",
//     is_abstract: false,
//     super_class: [
//         "SubProcess",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "AdHocSubProcess-completionCondition",
//                 name: "completionCondition",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Expression",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_completionCondition_adHocSubProcess",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "AdHocSubProcess-ordering",
//                 name: "ordering",
//                 visibility: Public,
//                 simple_type: Some(
//                     "AdHocOrdering",
//                 ),
//                 complex_type: None,
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "AdHocSubProcess-cancelRemainingInstances",
//                 name: "cancelRemainingInstances",
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


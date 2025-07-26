//! bpmn_20_class_sub_process

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_sub_process")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperActivity
    pub super_activity: i64,
    /// SUPER FIELD : SuperFlowElementsContainer
    pub super_flow_elements_container: i64,
    /// SIMPLE FIELD : BPMN20-SubProcess-triggeredByEvent
    #[sea_orm(default_value = "false")]
    pub triggered_by_event: Boolean,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE SubProcess need ONE Activity
    #[sea_orm(
        belongs_to = "super::bpmn_20_activity::Entity",
        from = "Column::SuperActivity",
        to = "super::bpmn_20_activity::Column::Id",
        on_delete = "Cascade"
    )]
    Activity,
    // SUPER : ONE SubProcess need ONE FlowElementsContainer
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_elements_container::Entity",
        from = "Column::SuperFlowElementsContainer",
        to = "super::bpmn_20_flow_elements_container::Column::Id",
        on_delete = "Cascade"
    )]
    FlowElementsContainer,
    // SUPER : ONE AdHocSubProcess need ONE SubProcess
    #[sea_orm(has_one = "super::bpmn_20_ad_hoc_sub_process::Entity")]
    AdHocSubProcess,
    // SUPER : ONE Transaction need ONE SubProcess
    #[sea_orm(has_one = "super::bpmn_20_transaction::Entity")]
    Transaction,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-SubProcess',
//     name: "SubProcess",
//     is_abstract: false,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'BPMN20-Activity',
//         "Loaded XMIIdReference RefCell of 'BPMN20-FlowElementsContainer',
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "SubProcess-artifacts": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-SubProcess-artifacts',
//                 name: "artifacts",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-Artifact',
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: None,
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-A_artifacts_subProcess',
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "SubProcess-triggeredByEvent": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-SubProcess-triggeredByEvent',
//                 name: "triggeredByEvent",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "Loaded XMIIdReference RefCell of 'DC-Boolean',
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: Some(
//                     "false",
//                 ),
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: None,
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#SubProcess",
//     table_name: "bpmn_20_sub_process",
//     model_name: "SubProcess",
//     full_name: "bpmn_20_class_sub_process",
//     reverse_super: RefCell {
//         value: [
//             (Weak),
//             (Weak),
//         ],
//     },
// }


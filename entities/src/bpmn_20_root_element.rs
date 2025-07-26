//! bpmn_20_class_root_element

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_root_element")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperBaseElement
    pub super_base_element: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE RootElement need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
    // SUPER : ONE CallableElement need ONE RootElement
    #[sea_orm(has_one = "super::bpmn_20_callable_element::Entity")]
    CallableElement,
    // SUPER : ONE Category need ONE RootElement
    #[sea_orm(has_one = "super::bpmn_20_category::Entity")]
    Category,
    // SUPER : ONE Collaboration need ONE RootElement
    #[sea_orm(has_one = "super::bpmn_20_collaboration::Entity")]
    Collaboration,
    // SUPER : ONE CorrelationProperty need ONE RootElement
    #[sea_orm(has_one = "super::bpmn_20_correlation_property::Entity")]
    CorrelationProperty,
    // SUPER : ONE DataStore need ONE RootElement
    #[sea_orm(has_one = "super::bpmn_20_data_store::Entity")]
    DataStore,
    // SUPER : ONE EndPoint need ONE RootElement
    #[sea_orm(has_one = "super::bpmn_20_end_point::Entity")]
    EndPoint,
    // SUPER : ONE Error need ONE RootElement
    #[sea_orm(has_one = "super::bpmn_20_error::Entity")]
    Error,
    // SUPER : ONE EventDefinition need ONE RootElement
    #[sea_orm(has_one = "super::bpmn_20_event_definition::Entity")]
    EventDefinition,
    // SUPER : ONE Interface need ONE RootElement
    #[sea_orm(has_one = "super::bpmn_20_interface::Entity")]
    Interface,
    // SUPER : ONE ItemDefinition need ONE RootElement
    #[sea_orm(has_one = "super::bpmn_20_item_definition::Entity")]
    ItemDefinition,
    // SUPER : ONE Message need ONE RootElement
    #[sea_orm(has_one = "super::bpmn_20_message::Entity")]
    Message,
    // SUPER : ONE PartnerEntity need ONE RootElement
    #[sea_orm(has_one = "super::bpmn_20_partner_entity::Entity")]
    PartnerEntity,
    // SUPER : ONE PartnerRole need ONE RootElement
    #[sea_orm(has_one = "super::bpmn_20_partner_role::Entity")]
    PartnerRole,
    // SUPER : ONE Resource need ONE RootElement
    #[sea_orm(has_one = "super::bpmn_20_resource::Entity")]
    Resource,
    // SUPER : ONE Signal need ONE RootElement
    #[sea_orm(has_one = "super::bpmn_20_signal::Entity")]
    Signal,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-RootElement',
//     name: "RootElement",
//     is_abstract: true,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'BPMN20-BaseElement',
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#RootElement",
//     table_name: "bpmn_20_root_element",
//     model_name: "RootElement",
//     full_name: "bpmn_20_class_root_element",
//     reverse_super: RefCell {
//         value: [
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//         ],
//     },
// }


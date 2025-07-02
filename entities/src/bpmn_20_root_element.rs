//! bpmn_20_class_root_element

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_root_element")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE RootElement need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id"
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

// SUPER : ONE RootElement need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// SUPER : ONE CallableElement need ONE RootElement
impl Related<super::bpmn_20_callable_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CallableElement.def()
    }
}

// SUPER : ONE Category need ONE RootElement
impl Related<super::bpmn_20_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

// SUPER : ONE Collaboration need ONE RootElement
impl Related<super::bpmn_20_collaboration::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Collaboration.def()
    }
}

// SUPER : ONE CorrelationProperty need ONE RootElement
impl Related<super::bpmn_20_correlation_property::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CorrelationProperty.def()
    }
}

// SUPER : ONE DataStore need ONE RootElement
impl Related<super::bpmn_20_data_store::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataStore.def()
    }
}

// SUPER : ONE EndPoint need ONE RootElement
impl Related<super::bpmn_20_end_point::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EndPoint.def()
    }
}

// SUPER : ONE Error need ONE RootElement
impl Related<super::bpmn_20_error::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Error.def()
    }
}

// SUPER : ONE EventDefinition need ONE RootElement
impl Related<super::bpmn_20_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventDefinition.def()
    }
}

// SUPER : ONE Interface need ONE RootElement
impl Related<super::bpmn_20_interface::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Interface.def()
    }
}

// SUPER : ONE ItemDefinition need ONE RootElement
impl Related<super::bpmn_20_item_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemDefinition.def()
    }
}

// SUPER : ONE Message need ONE RootElement
impl Related<super::bpmn_20_message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}

// SUPER : ONE PartnerEntity need ONE RootElement
impl Related<super::bpmn_20_partner_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PartnerEntity.def()
    }
}

// SUPER : ONE PartnerRole need ONE RootElement
impl Related<super::bpmn_20_partner_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PartnerRole.def()
    }
}

// SUPER : ONE Resource need ONE RootElement
impl Related<super::bpmn_20_resource::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Resource.def()
    }
}

// SUPER : ONE Signal need ONE RootElement
impl Related<super::bpmn_20_signal::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Signal.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "RootElement",
//     name: "RootElement",
//     is_abstract: true,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }


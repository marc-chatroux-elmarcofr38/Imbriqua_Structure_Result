//! bpmn_20_class_root_element

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_root_element")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : BaseElement
    pub super_base_element: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id"
    )]
    BaseElement,
    #[sea_orm(has_one = "super::bpmn_20_callable_element::Entity")]
    CallableElement,
    #[sea_orm(has_one = "super::bpmn_20_category::Entity")]
    Category,
    #[sea_orm(has_one = "super::bpmn_20_collaboration::Entity")]
    Collaboration,
    #[sea_orm(has_one = "super::bpmn_20_correlation_property::Entity")]
    CorrelationProperty,
    #[sea_orm(has_one = "super::bpmn_20_data_store::Entity")]
    DataStore,
    #[sea_orm(has_one = "super::bpmn_20_end_point::Entity")]
    EndPoint,
    #[sea_orm(has_one = "super::bpmn_20_error::Entity")]
    Error,
    #[sea_orm(has_one = "super::bpmn_20_event_definition::Entity")]
    EventDefinition,
    #[sea_orm(has_one = "super::bpmn_20_interface::Entity")]
    Interface,
    #[sea_orm(has_one = "super::bpmn_20_item_definition::Entity")]
    ItemDefinition,
    #[sea_orm(has_one = "super::bpmn_20_message::Entity")]
    Message,
    #[sea_orm(has_one = "super::bpmn_20_partner_entity::Entity")]
    PartnerEntity,
    #[sea_orm(has_one = "super::bpmn_20_partner_role::Entity")]
    PartnerRole,
    #[sea_orm(has_one = "super::bpmn_20_resource::Entity")]
    Resource,
    #[sea_orm(has_one = "super::bpmn_20_signal::Entity")]
    Signal,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_callable_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CallableElement.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_collaboration::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Collaboration.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_correlation_property::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CorrelationProperty.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_data_store::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataStore.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_end_point::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EndPoint.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_error::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Error.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventDefinition.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_interface::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Interface.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_item_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemDefinition.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_partner_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PartnerEntity.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_partner_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PartnerRole.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_resource::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Resource.def()
    }
}
// `Related` trait has to be implemented by hand
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


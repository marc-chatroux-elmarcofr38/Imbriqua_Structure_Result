//! bpmn_20_class_root_element

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_root_element")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
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

impl ActiveModel {
    /// # Help document for "RootElement" (bpmn_20_class_root_element)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __Definitions__ (__DefinitionsModel__) from A_rootElements_definition
    ///   * one-to-many link : (0-1) __RootElement__ need (0-inf) __Definitions__)
    ///   * callable using find_with_related(__DefinitionsModel__) from __RootElement__
    ///   * named definition in BPMN
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __RootElement__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __RootElement__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __CallableElement__ (__CallableElementModel__)
    ///   * one-to-one link (reverse) : one __CallableElement__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __CallableElement__
    ///   * saved in __super_root_element__ field as foreing key in __CallableElementModel__
    /// * __Category__ (__CategoryModel__)
    ///   * one-to-one link (reverse) : one __Category__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __Category__
    ///   * saved in __super_root_element__ field as foreing key in __CategoryModel__
    /// * __Collaboration__ (__CollaborationModel__)
    ///   * one-to-one link (reverse) : one __Collaboration__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __Collaboration__
    ///   * saved in __super_root_element__ field as foreing key in __CollaborationModel__
    /// * __CorrelationProperty__ (__CorrelationPropertyModel__)
    ///   * one-to-one link (reverse) : one __CorrelationProperty__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __CorrelationProperty__
    ///   * saved in __super_root_element__ field as foreing key in __CorrelationPropertyModel__
    /// * __DataStore__ (__DataStoreModel__)
    ///   * one-to-one link (reverse) : one __DataStore__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __DataStore__
    ///   * saved in __super_root_element__ field as foreing key in __DataStoreModel__
    /// * __EndPoint__ (__EndPointModel__)
    ///   * one-to-one link (reverse) : one __EndPoint__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __EndPoint__
    ///   * saved in __super_root_element__ field as foreing key in __EndPointModel__
    /// * __Error__ (__ErrorModel__)
    ///   * one-to-one link (reverse) : one __Error__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __Error__
    ///   * saved in __super_root_element__ field as foreing key in __ErrorModel__
    /// * __EventDefinition__ (__EventDefinitionModel__)
    ///   * one-to-one link (reverse) : one __EventDefinition__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __EventDefinition__
    ///   * saved in __super_root_element__ field as foreing key in __EventDefinitionModel__
    /// * __Interface__ (__InterfaceModel__)
    ///   * one-to-one link (reverse) : one __Interface__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __Interface__
    ///   * saved in __super_root_element__ field as foreing key in __InterfaceModel__
    /// * __ItemDefinition__ (__ItemDefinitionModel__)
    ///   * one-to-one link (reverse) : one __ItemDefinition__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __ItemDefinition__
    ///   * saved in __super_root_element__ field as foreing key in __ItemDefinitionModel__
    /// * __Message__ (__MessageModel__)
    ///   * one-to-one link (reverse) : one __Message__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __Message__
    ///   * saved in __super_root_element__ field as foreing key in __MessageModel__
    /// * __PartnerEntity__ (__PartnerEntityModel__)
    ///   * one-to-one link (reverse) : one __PartnerEntity__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __PartnerEntity__
    ///   * saved in __super_root_element__ field as foreing key in __PartnerEntityModel__
    /// * __PartnerRole__ (__PartnerRoleModel__)
    ///   * one-to-one link (reverse) : one __PartnerRole__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __PartnerRole__
    ///   * saved in __super_root_element__ field as foreing key in __PartnerRoleModel__
    /// * __Resource__ (__ResourceModel__)
    ///   * one-to-one link (reverse) : one __Resource__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __Resource__
    ///   * saved in __super_root_element__ field as foreing key in __ResourceModel__
    /// * __Signal__ (__SignalModel__)
    ///   * one-to-one link (reverse) : one __Signal__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __Signal__
    ///   * saved in __super_root_element__ field as foreing key in __SignalModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "RootElement" (bpmn_20_class_root_element)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Relation : One To Many :
* __Definitions__ (__DefinitionsModel__) from A_rootElements_definition
  * one-to-many link : (0-1) __RootElement__ need (0-inf) __Definitions__)
  * callable using find_with_related(__DefinitionsModel__) from __RootElement__
  * named definition in BPMN

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __RootElement__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __RootElement__
  * saved in __super_base_element__ field as foreing key

## Reverse Super :
* __CallableElement__ (__CallableElementModel__)
  * one-to-one link (reverse) : one __CallableElement__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __CallableElement__
  * saved in __super_root_element__ field as foreing key in __CallableElementModel__
* __Category__ (__CategoryModel__)
  * one-to-one link (reverse) : one __Category__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __Category__
  * saved in __super_root_element__ field as foreing key in __CategoryModel__
* __Collaboration__ (__CollaborationModel__)
  * one-to-one link (reverse) : one __Collaboration__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __Collaboration__
  * saved in __super_root_element__ field as foreing key in __CollaborationModel__
* __CorrelationProperty__ (__CorrelationPropertyModel__)
  * one-to-one link (reverse) : one __CorrelationProperty__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __CorrelationProperty__
  * saved in __super_root_element__ field as foreing key in __CorrelationPropertyModel__
* __DataStore__ (__DataStoreModel__)
  * one-to-one link (reverse) : one __DataStore__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __DataStore__
  * saved in __super_root_element__ field as foreing key in __DataStoreModel__
* __EndPoint__ (__EndPointModel__)
  * one-to-one link (reverse) : one __EndPoint__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __EndPoint__
  * saved in __super_root_element__ field as foreing key in __EndPointModel__
* __Error__ (__ErrorModel__)
  * one-to-one link (reverse) : one __Error__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __Error__
  * saved in __super_root_element__ field as foreing key in __ErrorModel__
* __EventDefinition__ (__EventDefinitionModel__)
  * one-to-one link (reverse) : one __EventDefinition__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __EventDefinition__
  * saved in __super_root_element__ field as foreing key in __EventDefinitionModel__
* __Interface__ (__InterfaceModel__)
  * one-to-one link (reverse) : one __Interface__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __Interface__
  * saved in __super_root_element__ field as foreing key in __InterfaceModel__
* __ItemDefinition__ (__ItemDefinitionModel__)
  * one-to-one link (reverse) : one __ItemDefinition__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __ItemDefinition__
  * saved in __super_root_element__ field as foreing key in __ItemDefinitionModel__
* __Message__ (__MessageModel__)
  * one-to-one link (reverse) : one __Message__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __Message__
  * saved in __super_root_element__ field as foreing key in __MessageModel__
* __PartnerEntity__ (__PartnerEntityModel__)
  * one-to-one link (reverse) : one __PartnerEntity__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __PartnerEntity__
  * saved in __super_root_element__ field as foreing key in __PartnerEntityModel__
* __PartnerRole__ (__PartnerRoleModel__)
  * one-to-one link (reverse) : one __PartnerRole__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __PartnerRole__
  * saved in __super_root_element__ field as foreing key in __PartnerRoleModel__
* __Resource__ (__ResourceModel__)
  * one-to-one link (reverse) : one __Resource__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __Resource__
  * saved in __super_root_element__ field as foreing key in __ResourceModel__
* __Signal__ (__SignalModel__)
  * one-to-one link (reverse) : one __Signal__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __Signal__
  * saved in __super_root_element__ field as foreing key in __SignalModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "RootElement",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "RootElement",
//     is_abstract: true,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#RootElement",
//     table_name: "bpmn_20_root_element",
//     model_name: "RootElement",
//     full_name: "bpmn_20_class_root_element",
// }


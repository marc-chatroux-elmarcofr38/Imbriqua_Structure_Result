//! bpmn_20_class_item_aware_element

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_item_aware_element")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : BaseElement
    pub super_base_element: i32,
    /// COMPLEX FIELD : ItemAwareElement-itemSubjectRef
    pub item_subject_ref: Option<i32>,
    /// COMPLEX FIELD : ItemAwareElement-dataState
    pub data_state: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id"
    )]
    BaseElement,
    #[sea_orm(has_one = "super::bpmn_20_data_input::Entity")]
    DataInput,
    #[sea_orm(has_one = "super::bpmn_20_data_object::Entity")]
    DataObject,
    #[sea_orm(has_one = "super::bpmn_20_data_object_reference::Entity")]
    DataObjectReference,
    #[sea_orm(has_one = "super::bpmn_20_data_output::Entity")]
    DataOutput,
    #[sea_orm(has_one = "super::bpmn_20_data_store::Entity")]
    DataStore,
    #[sea_orm(has_one = "super::bpmn_20_data_store_reference::Entity")]
    DataStoreReference,
    #[sea_orm(has_one = "super::bpmn_20_property::Entity")]
    Property,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_data_input::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataInput.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_data_object::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataObject.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_data_object_reference::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataObjectReference.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_data_output::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataOutput.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_data_store::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataStore.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_data_store_reference::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataStoreReference.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_property::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Property.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "ItemAwareElement",
//     name: "ItemAwareElement",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "ItemAwareElement-itemSubjectRef",
//                 name: "itemSubjectRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ItemDefinition",
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
//                     "A_itemSubjectRef_itemAwareElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ItemAwareElement-dataState",
//                 name: "dataState",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataState",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
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
//                     "A_dataState_itemAwareElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }


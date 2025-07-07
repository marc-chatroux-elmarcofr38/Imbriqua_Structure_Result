//! bpmn_20_class_item_aware_element

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_item_aware_element")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : ItemAwareElement-itemSubjectRef
    pub item_subject_ref: Option<i64>,
    /// COMPLEX FIELD : ItemAwareElement-dataState
    pub data_state: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ItemAwareElement need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id"
    )]
    BaseElement,
    // SUPER : ONE DataInput need ONE ItemAwareElement
    #[sea_orm(has_one = "super::bpmn_20_data_input::Entity")]
    DataInput,
    // SUPER : ONE DataObject need ONE ItemAwareElement
    #[sea_orm(has_one = "super::bpmn_20_data_object::Entity")]
    DataObject,
    // SUPER : ONE DataObjectReference need ONE ItemAwareElement
    #[sea_orm(has_one = "super::bpmn_20_data_object_reference::Entity")]
    DataObjectReference,
    // SUPER : ONE DataOutput need ONE ItemAwareElement
    #[sea_orm(has_one = "super::bpmn_20_data_output::Entity")]
    DataOutput,
    // SUPER : ONE DataStore need ONE ItemAwareElement
    #[sea_orm(has_one = "super::bpmn_20_data_store::Entity")]
    DataStore,
    // SUPER : ONE DataStoreReference need ONE ItemAwareElement
    #[sea_orm(has_one = "super::bpmn_20_data_store_reference::Entity")]
    DataStoreReference,
    // SUPER : ONE Property need ONE ItemAwareElement
    #[sea_orm(has_one = "super::bpmn_20_property::Entity")]
    Property,
}

// SUPER : ONE ItemAwareElement need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// SUPER : ONE DataInput need ONE ItemAwareElement
impl Related<super::bpmn_20_data_input::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataInput.def()
    }
}

// SUPER : ONE DataObject need ONE ItemAwareElement
impl Related<super::bpmn_20_data_object::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataObject.def()
    }
}

// SUPER : ONE DataObjectReference need ONE ItemAwareElement
impl Related<super::bpmn_20_data_object_reference::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataObjectReference.def()
    }
}

// SUPER : ONE DataOutput need ONE ItemAwareElement
impl Related<super::bpmn_20_data_output::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataOutput.def()
    }
}

// SUPER : ONE DataStore need ONE ItemAwareElement
impl Related<super::bpmn_20_data_store::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataStore.def()
    }
}

// SUPER : ONE DataStoreReference need ONE ItemAwareElement
impl Related<super::bpmn_20_data_store_reference::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataStoreReference.def()
    }
}

// SUPER : ONE Property need ONE ItemAwareElement
impl Related<super::bpmn_20_property::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Property.def()
    }
}

// ManyToMany : with DataAssociation using A_sourceRef_dataAssociation
impl Related<super::bpmn_20_a_source_ref_data_association::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_source_ref_data_association::Relation::DataAssociation.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_source_ref_data_association::Relation::ItemAwareElement
                .def()
                .rev(),
        )
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


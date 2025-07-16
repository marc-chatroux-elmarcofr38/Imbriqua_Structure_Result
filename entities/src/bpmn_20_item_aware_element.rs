//! bpmn_20_class_item_aware_element

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_item_aware_element")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : ItemAwareElement-dataState
    pub data_state: Option<i64>,
    /// COMPLEX FIELD : ItemAwareElement-itemSubjectRef
    pub item_subject_ref: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ItemAwareElement need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
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

impl ActiveModel {
    /// # Help document for "ItemAwareElement" (bpmn_20_class_item_aware_element)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// ## Direct One To One :
    /// * __DataState__ (__DataStateModel__) from A_dataState_itemAwareElement
    ///   * one-to-one link : (0-1) __ItemAwareElement__ need (1-1) __DataState__)
    ///   * callable using find_also_related(__DataStateModel__) from __ItemAwareElement__
    ///   * saved in __data_state__ field as foreing key
    /// 
    /// ## Relation : One To Many :
    /// * __ItemDefinition__ (__ItemDefinitionModel__) from A_itemSubjectRef_itemAwareElement
    ///   * one-to-many link : (0-1) __ItemAwareElement__ need (0-inf) __ItemDefinition__)
    ///   * callable using find_with_related(__ItemDefinitionModel__) from __ItemAwareElement__
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __ItemAwareElement__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __ItemAwareElement__
    ///   * saved in __super_base_element__ field as foreing key
    /// ## Reverse One To One :
    /// * __MultiInstanceLoopCharacteristics__ (__MultiInstanceLoopCharacteristicsModel__) from A_loopDataInputRef_multiInstanceLoopCharacteristics
    ///   * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __ItemAwareElement__)
    ///   * callable using find_also_related(__ItemAwareElementModel__) from __MultiInstanceLoopCharacteristics__
    ///   * saved in __loop_data_input_ref__ field as foreing key
    /// * __MultiInstanceLoopCharacteristics__ (__MultiInstanceLoopCharacteristicsModel__) from A_loopDataOutputRef_multiInstanceLoopCharacteristics
    ///   * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __ItemAwareElement__)
    ///   * callable using find_also_related(__ItemAwareElementModel__) from __MultiInstanceLoopCharacteristics__
    ///   * saved in __loop_data_output_ref__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __DataInput__ (__DataInputModel__)
    ///   * one-to-one link (reverse) : one __DataInput__ need one __ItemAwareElement__)
    ///   * callable using find_also_related(__ItemAwareElementModel__) from __DataInput__
    ///   * saved in __super_item_aware_element__ field as foreing key in __DataInputModel__
    /// * __DataObject__ (__DataObjectModel__)
    ///   * one-to-one link (reverse) : one __DataObject__ need one __ItemAwareElement__)
    ///   * callable using find_also_related(__ItemAwareElementModel__) from __DataObject__
    ///   * saved in __super_item_aware_element__ field as foreing key in __DataObjectModel__
    /// * __DataObjectReference__ (__DataObjectReferenceModel__)
    ///   * one-to-one link (reverse) : one __DataObjectReference__ need one __ItemAwareElement__)
    ///   * callable using find_also_related(__ItemAwareElementModel__) from __DataObjectReference__
    ///   * saved in __super_item_aware_element__ field as foreing key in __DataObjectReferenceModel__
    /// * __DataOutput__ (__DataOutputModel__)
    ///   * one-to-one link (reverse) : one __DataOutput__ need one __ItemAwareElement__)
    ///   * callable using find_also_related(__ItemAwareElementModel__) from __DataOutput__
    ///   * saved in __super_item_aware_element__ field as foreing key in __DataOutputModel__
    /// * __DataStore__ (__DataStoreModel__)
    ///   * one-to-one link (reverse) : one __DataStore__ need one __ItemAwareElement__)
    ///   * callable using find_also_related(__ItemAwareElementModel__) from __DataStore__
    ///   * saved in __super_item_aware_element__ field as foreing key in __DataStoreModel__
    /// * __DataStoreReference__ (__DataStoreReferenceModel__)
    ///   * one-to-one link (reverse) : one __DataStoreReference__ need one __ItemAwareElement__)
    ///   * callable using find_also_related(__ItemAwareElementModel__) from __DataStoreReference__
    ///   * saved in __super_item_aware_element__ field as foreing key in __DataStoreReferenceModel__
    /// * __Property__ (__PropertyModel__)
    ///   * one-to-one link (reverse) : one __Property__ need one __ItemAwareElement__)
    ///   * callable using find_also_related(__ItemAwareElementModel__) from __Property__
    ///   * saved in __super_item_aware_element__ field as foreing key in __PropertyModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ItemAwareElement" (bpmn_20_class_item_aware_element)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__


## Direct One To One :
* __DataState__ (__DataStateModel__) from A_dataState_itemAwareElement
  * one-to-one link : (0-1) __ItemAwareElement__ need (1-1) __DataState__)
  * callable using find_also_related(__DataStateModel__) from __ItemAwareElement__
  * saved in __data_state__ field as foreing key

## Relation : One To Many :
* __ItemDefinition__ (__ItemDefinitionModel__) from A_itemSubjectRef_itemAwareElement
  * one-to-many link : (0-1) __ItemAwareElement__ need (0-inf) __ItemDefinition__)
  * callable using find_with_related(__ItemDefinitionModel__) from __ItemAwareElement__

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __ItemAwareElement__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __ItemAwareElement__
  * saved in __super_base_element__ field as foreing key
## Reverse One To One :
* __MultiInstanceLoopCharacteristics__ (__MultiInstanceLoopCharacteristicsModel__) from A_loopDataInputRef_multiInstanceLoopCharacteristics
  * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __ItemAwareElement__)
  * callable using find_also_related(__ItemAwareElementModel__) from __MultiInstanceLoopCharacteristics__
  * saved in __loop_data_input_ref__ field as foreing key
* __MultiInstanceLoopCharacteristics__ (__MultiInstanceLoopCharacteristicsModel__) from A_loopDataOutputRef_multiInstanceLoopCharacteristics
  * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __ItemAwareElement__)
  * callable using find_also_related(__ItemAwareElementModel__) from __MultiInstanceLoopCharacteristics__
  * saved in __loop_data_output_ref__ field as foreing key

## Reverse Super :
* __DataInput__ (__DataInputModel__)
  * one-to-one link (reverse) : one __DataInput__ need one __ItemAwareElement__)
  * callable using find_also_related(__ItemAwareElementModel__) from __DataInput__
  * saved in __super_item_aware_element__ field as foreing key in __DataInputModel__
* __DataObject__ (__DataObjectModel__)
  * one-to-one link (reverse) : one __DataObject__ need one __ItemAwareElement__)
  * callable using find_also_related(__ItemAwareElementModel__) from __DataObject__
  * saved in __super_item_aware_element__ field as foreing key in __DataObjectModel__
* __DataObjectReference__ (__DataObjectReferenceModel__)
  * one-to-one link (reverse) : one __DataObjectReference__ need one __ItemAwareElement__)
  * callable using find_also_related(__ItemAwareElementModel__) from __DataObjectReference__
  * saved in __super_item_aware_element__ field as foreing key in __DataObjectReferenceModel__
* __DataOutput__ (__DataOutputModel__)
  * one-to-one link (reverse) : one __DataOutput__ need one __ItemAwareElement__)
  * callable using find_also_related(__ItemAwareElementModel__) from __DataOutput__
  * saved in __super_item_aware_element__ field as foreing key in __DataOutputModel__
* __DataStore__ (__DataStoreModel__)
  * one-to-one link (reverse) : one __DataStore__ need one __ItemAwareElement__)
  * callable using find_also_related(__ItemAwareElementModel__) from __DataStore__
  * saved in __super_item_aware_element__ field as foreing key in __DataStoreModel__
* __DataStoreReference__ (__DataStoreReferenceModel__)
  * one-to-one link (reverse) : one __DataStoreReference__ need one __ItemAwareElement__)
  * callable using find_also_related(__ItemAwareElementModel__) from __DataStoreReference__
  * saved in __super_item_aware_element__ field as foreing key in __DataStoreReferenceModel__
* __Property__ (__PropertyModel__)
  * one-to-one link (reverse) : one __Property__ need one __ItemAwareElement__)
  * callable using find_also_related(__ItemAwareElementModel__) from __Property__
  * saved in __super_item_aware_element__ field as foreing key in __PropertyModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "ItemAwareElement",
//     name: "ItemAwareElement",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "ItemAwareElement-dataState": Property(
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
//         "ItemAwareElement-itemSubjectRef": Property(
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
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#ItemAwareElement",
//     table_name: "bpmn_20_item_aware_element",
//     model_name: "ItemAwareElement",
//     full_name: "bpmn_20_class_item_aware_element",
// }


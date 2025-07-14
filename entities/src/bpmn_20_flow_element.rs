//! bpmn_20_class_flow_element

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_flow_element")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : FlowElement-auditing
    pub auditing: Option<i64>,
    /// COMPLEX FIELD : FlowElement-monitoring
    pub monitoring: Option<i64>,
    /// SIMPLE FIELD : FlowElement-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE FlowElement need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
    // SUPER : ONE DataObject need ONE FlowElement
    #[sea_orm(has_one = "super::bpmn_20_data_object::Entity")]
    DataObject,
    // SUPER : ONE DataObjectReference need ONE FlowElement
    #[sea_orm(has_one = "super::bpmn_20_data_object_reference::Entity")]
    DataObjectReference,
    // SUPER : ONE DataStoreReference need ONE FlowElement
    #[sea_orm(has_one = "super::bpmn_20_data_store_reference::Entity")]
    DataStoreReference,
    // SUPER : ONE FlowNode need ONE FlowElement
    #[sea_orm(has_one = "super::bpmn_20_flow_node::Entity")]
    FlowNode,
    // SUPER : ONE SequenceFlow need ONE FlowElement
    #[sea_orm(has_one = "super::bpmn_20_sequence_flow::Entity")]
    SequenceFlow,
}

// SUPER : ONE FlowElement need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// SUPER : ONE DataObject need ONE FlowElement
impl Related<super::bpmn_20_data_object::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataObject.def()
    }
}

// SUPER : ONE DataObjectReference need ONE FlowElement
impl Related<super::bpmn_20_data_object_reference::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataObjectReference.def()
    }
}

// SUPER : ONE DataStoreReference need ONE FlowElement
impl Related<super::bpmn_20_data_store_reference::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataStoreReference.def()
    }
}

// SUPER : ONE FlowNode need ONE FlowElement
impl Related<super::bpmn_20_flow_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowNode.def()
    }
}

// SUPER : ONE SequenceFlow need ONE FlowElement
impl Related<super::bpmn_20_sequence_flow::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SequenceFlow.def()
    }
}

// ManyToMany : with CategoryValue using A_categorizedFlowElements_categoryValueRef
impl Related<super::bpmn_20_a_categorized_flow_elements_category_value_ref::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_categorized_flow_elements_category_value_ref::Relation::CategoryValue.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_categorized_flow_elements_category_value_ref::Relation::FlowElement
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "FlowElement" (bpmn_20_class_flow_element)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "FlowElement-name")
    ///   * type : __std::string::String__
    /// 
    /// ## Direct One To One :
    /// * __Monitoring__ (__MonitoringModel__) from A_monitoring_flowElement
    ///   * one-to-one link : (0-1) __FlowElement__ need (0-1) __Monitoring__)
    ///   * callable using find_also_related(__MonitoringModel__) from __FlowElement__
    ///   * saved in __monitoring__ field as foreing key
    /// * __Auditing__ (__AuditingModel__) from A_auditing_flowElement
    ///   * one-to-one link : (0-1) __FlowElement__ need (0-1) __Auditing__)
    ///   * callable using find_also_related(__AuditingModel__) from __FlowElement__
    ///   * saved in __auditing__ field as foreing key
    /// 
    /// ## Relation : One To Many :
    /// * __FlowElementsContainer__ (__FlowElementsContainerModel__) from A_flowElements_container
    ///   * one-to-many link : (1-1) __FlowElement__ need (0-inf) __FlowElementsContainer__)
    ///   * callable using find_with_related(__FlowElementsContainerModel__) from __FlowElement__
    ///   * named container in BPMN
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __FlowElement__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __FlowElement__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __DataObject__ (__DataObjectModel__)
    ///   * one-to-one link (reverse) : one __DataObject__ need one __FlowElement__)
    ///   * callable using find_also_related(__FlowElementModel__) from __DataObject__
    ///   * saved in __super_flow_element__ field as foreing key in __DataObjectModel__
    /// * __DataObjectReference__ (__DataObjectReferenceModel__)
    ///   * one-to-one link (reverse) : one __DataObjectReference__ need one __FlowElement__)
    ///   * callable using find_also_related(__FlowElementModel__) from __DataObjectReference__
    ///   * saved in __super_flow_element__ field as foreing key in __DataObjectReferenceModel__
    /// * __DataStoreReference__ (__DataStoreReferenceModel__)
    ///   * one-to-one link (reverse) : one __DataStoreReference__ need one __FlowElement__)
    ///   * callable using find_also_related(__FlowElementModel__) from __DataStoreReference__
    ///   * saved in __super_flow_element__ field as foreing key in __DataStoreReferenceModel__
    /// * __FlowNode__ (__FlowNodeModel__)
    ///   * one-to-one link (reverse) : one __FlowNode__ need one __FlowElement__)
    ///   * callable using find_also_related(__FlowElementModel__) from __FlowNode__
    ///   * saved in __super_flow_element__ field as foreing key in __FlowNodeModel__
    /// * __SequenceFlow__ (__SequenceFlowModel__)
    ///   * one-to-one link (reverse) : one __SequenceFlow__ need one __FlowElement__)
    ///   * callable using find_also_related(__FlowElementModel__) from __SequenceFlow__
    ///   * saved in __super_flow_element__ field as foreing key in __SequenceFlowModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "FlowElement" (bpmn_20_class_flow_element)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "FlowElement-name")
  * type : __std::string::String__

## Direct One To One :
* __Monitoring__ (__MonitoringModel__) from A_monitoring_flowElement
  * one-to-one link : (0-1) __FlowElement__ need (0-1) __Monitoring__)
  * callable using find_also_related(__MonitoringModel__) from __FlowElement__
  * saved in __monitoring__ field as foreing key
* __Auditing__ (__AuditingModel__) from A_auditing_flowElement
  * one-to-one link : (0-1) __FlowElement__ need (0-1) __Auditing__)
  * callable using find_also_related(__AuditingModel__) from __FlowElement__
  * saved in __auditing__ field as foreing key

## Relation : One To Many :
* __FlowElementsContainer__ (__FlowElementsContainerModel__) from A_flowElements_container
  * one-to-many link : (1-1) __FlowElement__ need (0-inf) __FlowElementsContainer__)
  * callable using find_with_related(__FlowElementsContainerModel__) from __FlowElement__
  * named container in BPMN

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __FlowElement__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __FlowElement__
  * saved in __super_base_element__ field as foreing key

## Reverse Super :
* __DataObject__ (__DataObjectModel__)
  * one-to-one link (reverse) : one __DataObject__ need one __FlowElement__)
  * callable using find_also_related(__FlowElementModel__) from __DataObject__
  * saved in __super_flow_element__ field as foreing key in __DataObjectModel__
* __DataObjectReference__ (__DataObjectReferenceModel__)
  * one-to-one link (reverse) : one __DataObjectReference__ need one __FlowElement__)
  * callable using find_also_related(__FlowElementModel__) from __DataObjectReference__
  * saved in __super_flow_element__ field as foreing key in __DataObjectReferenceModel__
* __DataStoreReference__ (__DataStoreReferenceModel__)
  * one-to-one link (reverse) : one __DataStoreReference__ need one __FlowElement__)
  * callable using find_also_related(__FlowElementModel__) from __DataStoreReference__
  * saved in __super_flow_element__ field as foreing key in __DataStoreReferenceModel__
* __FlowNode__ (__FlowNodeModel__)
  * one-to-one link (reverse) : one __FlowNode__ need one __FlowElement__)
  * callable using find_also_related(__FlowElementModel__) from __FlowNode__
  * saved in __super_flow_element__ field as foreing key in __FlowNodeModel__
* __SequenceFlow__ (__SequenceFlowModel__)
  * one-to-one link (reverse) : one __SequenceFlow__ need one __FlowElement__)
  * callable using find_also_related(__FlowElementModel__) from __SequenceFlow__
  * saved in __super_flow_element__ field as foreing key in __SequenceFlowModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "FlowElement",
//     name: "FlowElement",
//     is_abstract: true,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "FlowElement-name",
//                 name: "name",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#String",
//                         },
//                     ),
//                 ),
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
//                 xmi_id: "FlowElement-auditing",
//                 name: "auditing",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Auditing",
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
//                     "A_auditing_flowElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "FlowElement-monitoring",
//                 name: "monitoring",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Monitoring",
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
//                     "A_monitoring_flowElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "FlowElement-categoryValueRef",
//                 name: "categoryValueRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CategoryValue",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
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
//                     "A_categorizedFlowElements_categoryValueRef",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }


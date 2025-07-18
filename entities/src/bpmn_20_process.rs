//! bpmn_20_class_process

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_process")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : CallableElement
    pub super_callable_element: i64,
    /// SUPER FIELD : FlowElementsContainer
    pub super_flow_elements_container: i64,
    /// COMPLEX FIELD : BPMN20-Process-auditing
    pub auditing: Option<i64>,
    /// COMPLEX FIELD : BPMN20-Process-definitionalCollaborationRef
    pub definitional_collaboration_ref: Option<i64>,
    /// COMPLEX FIELD : BPMN20-Process-monitoring
    pub monitoring: Option<i64>,
    /// SIMPLE FIELD : BPMN20-Process-isClosed
    pub is_closed: std::primitive::bool,
    /// SIMPLE FIELD : BPMN20-Process-isExecutable
    pub is_executable: std::primitive::bool,
    /// SIMPLE FIELD : BPMN20-Process-processType
    pub process_type: ProcessType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Process need ONE CallableElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_callable_element::Entity",
        from = "Column::SuperCallableElement",
        to = "super::bpmn_20_callable_element::Column::Id",
        on_delete = "Cascade"
    )]
    CallableElement,
    // SUPER : ONE Process need ONE FlowElementsContainer
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_elements_container::Entity",
        from = "Column::SuperFlowElementsContainer",
        to = "super::bpmn_20_flow_elements_container::Column::Id",
        on_delete = "Cascade"
    )]
    FlowElementsContainer,
}

// SUPER : ONE Process need ONE CallableElement
impl Related<super::bpmn_20_callable_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CallableElement.def()
    }
}

// SUPER : ONE Process need ONE FlowElementsContainer
impl Related<super::bpmn_20_flow_elements_container::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowElementsContainer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Process" (bpmn_20_class_process)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __is_closed__ (xmi_id : "BPMN20-Process-isClosed")
    ///   * type : __std::primitive::bool__
    /// * __is_executable__ (xmi_id : "BPMN20-Process-isExecutable")
    ///   * type : __std::primitive::bool__
    /// * __process_type__ (xmi_id : "BPMN20-Process-processType")
    ///   * type : __ProcessType__
    /// 
    /// ## Direct One To One :
    /// * __Auditing__ (__AuditingModel__) from A_auditing_process
    ///   * one-to-one link : (0-1) __Process__ need (0-1) __Auditing__)
    ///   * callable using find_also_related(__AuditingModel__) from __Process__
    ///   * saved in __auditing__ field as foreing key
    /// * __Monitoring__ (__MonitoringModel__) from A_monitoring_process
    ///   * one-to-one link : (0-1) __Process__ need (0-1) __Monitoring__)
    ///   * callable using find_also_related(__MonitoringModel__) from __Process__
    ///   * saved in __monitoring__ field as foreing key
    /// 
    /// ## Relation : One To Many :
    /// * __Collaboration__ (__CollaborationModel__) from A_definitionalCollaborationRef_process
    ///   * one-to-many link : (0-1) __Process__ need (0-inf) __Collaboration__)
    ///   * callable using find_with_related(__CollaborationModel__) from __Process__
    /// 
    /// ## Direct Super :
    /// * __CallableElement__ (__CallableElementModel__)
    ///   * one-to-one link : one __Process__ need one __CallableElement__)
    ///   * callable using find_also_related(__CallableElementModel__) from __Process__
    ///   * saved in __super_callable_element__ field as foreing key
    /// * __FlowElementsContainer__ (__FlowElementsContainerModel__)
    ///   * one-to-one link : one __Process__ need one __FlowElementsContainer__)
    ///   * callable using find_also_related(__FlowElementsContainerModel__) from __Process__
    ///   * saved in __super_flow_elements_container__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Process" (bpmn_20_class_process)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __is_closed__ (xmi_id : "BPMN20-Process-isClosed")
  * type : __std::primitive::bool__
* __is_executable__ (xmi_id : "BPMN20-Process-isExecutable")
  * type : __std::primitive::bool__
* __process_type__ (xmi_id : "BPMN20-Process-processType")
  * type : __ProcessType__

## Direct One To One :
* __Auditing__ (__AuditingModel__) from A_auditing_process
  * one-to-one link : (0-1) __Process__ need (0-1) __Auditing__)
  * callable using find_also_related(__AuditingModel__) from __Process__
  * saved in __auditing__ field as foreing key
* __Monitoring__ (__MonitoringModel__) from A_monitoring_process
  * one-to-one link : (0-1) __Process__ need (0-1) __Monitoring__)
  * callable using find_also_related(__MonitoringModel__) from __Process__
  * saved in __monitoring__ field as foreing key

## Relation : One To Many :
* __Collaboration__ (__CollaborationModel__) from A_definitionalCollaborationRef_process
  * one-to-many link : (0-1) __Process__ need (0-inf) __Collaboration__)
  * callable using find_with_related(__CollaborationModel__) from __Process__

## Direct Super :
* __CallableElement__ (__CallableElementModel__)
  * one-to-one link : one __Process__ need one __CallableElement__)
  * callable using find_also_related(__CallableElementModel__) from __Process__
  * saved in __super_callable_element__ field as foreing key
* __FlowElementsContainer__ (__FlowElementsContainerModel__)
  * one-to-one link : one __Process__ need one __FlowElementsContainer__)
  * callable using find_also_related(__FlowElementsContainerModel__) from __Process__
  * saved in __super_flow_elements_container__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "Process",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "Process",
//     is_abstract: false,
//     super_class: [
//         "FlowElementsContainer",
//         "CallableElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-Process-artifacts": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Process-artifacts",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "artifacts",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Artifact",
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
//                 owning_association: "",
//                 association: Some(
//                     "A_artifacts_process",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Process-auditing": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Process-auditing",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
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
//                     "A_auditing_process",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Process-correlationSubscriptions": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Process-correlationSubscriptions",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "correlationSubscriptions",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CorrelationSubscription",
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
//                 owning_association: "",
//                 association: Some(
//                     "A_correlationSubscriptions_process",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Process-definitionalCollaborationRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Process-definitionalCollaborationRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "definitionalCollaborationRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Collaboration",
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
//                     "A_definitionalCollaborationRef_process",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Process-isClosed": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Process-isClosed",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "isClosed",
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
//         "-Process-isExecutable": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Process-isExecutable",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "isExecutable",
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
//         "-Process-monitoring": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Process-monitoring",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
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
//                     "A_monitoring_process",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Process-processType": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Process-processType",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "processType",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ProcessType",
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
//         "-Process-properties": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Process-properties",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "properties",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Property",
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
//                 owning_association: "",
//                 association: Some(
//                     "A_properties_process",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Process-resources": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Process-resources",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "resources",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ResourceRole",
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
//                 owning_association: "",
//                 association: Some(
//                     "A_resources_process",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Process-supports": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Process-supports",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "supports",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Process",
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
//                     "A_supports_process",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Process",
//     table_name: "bpmn_20_process",
//     model_name: "Process",
//     full_name: "bpmn_20_class_process",
// }


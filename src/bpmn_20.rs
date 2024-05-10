//! bpmn_20

// CMOFPackageImport {
//     xmi_type: "cmof:PackageImport",
//     xmi_id: "_packageImport.1",
//     importing_namespace: "_0",
//     imported_package: ImportedPackage {
//         type: "cmof:Package",
//         href: "DC.cmof#_0",
//     },
// }
use crate::dc;

// struct_level : A_errorRefs_operation

// struct_level : A_inMessageRef_operation

// struct_level : A_outMessageRef_operation

/// Conversion of Interface (Interface)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Interface",
///     name: "Interface",
///     is_abstract: None,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Interface-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Interface-operations",
///                     name: "operations",
///                     visibility: None,
///                     type: Some(
///                         "Operation",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_operations_interface",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Interface-implementationRef",
///                     name: "implementationRef",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:Class",
///                             href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Interface {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub root_element : RootElement,
}

/// Conversion of Operation (Operation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Operation",
///     name: "Operation",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Operation-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Operation-inMessageRef",
///                     name: "inMessageRef",
///                     visibility: None,
///                     type: Some(
///                         "Message",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_inMessageRef_operation",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Operation-outMessageRef",
///                     name: "outMessageRef",
///                     visibility: None,
///                     type: Some(
///                         "Message",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_outMessageRef_operation",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Operation-errorRefs",
///                     name: "errorRefs",
///                     visibility: None,
///                     type: Some(
///                         "Error",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_errorRefs_operation",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Operation-implementationRef",
///                     name: "implementationRef",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:Class",
///                             href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Operation {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

// struct_level : A_operations_interface

/// Conversion of EndPoint (EndPoint)
///
/// ```json
/// CMOFClass {
///     xmi_id: "EndPoint",
///     name: "EndPoint",
///     is_abstract: None,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct EndPoint {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub root_element : RootElement,
}

// struct_level : A_definitionalCollaborationRef_process

// struct_level : A_partitionElement_lane

// struct_level : A_flowNodeRefs_lanes

// struct_level : A_partitionElementRef_lane

// struct_level : A_auditing_process

// struct_level : A_monitoring_process

/// Conversion of Auditing (Auditing)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Auditing",
///     name: "Auditing",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct Auditing {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

/// Conversion of GlobalTask (GlobalTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "GlobalTask",
///     name: "GlobalTask",
///     is_abstract: None,
///     super_class: Some(
///         "CallableElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "GlobalTask-resources",
///                     name: "resources",
///                     visibility: None,
///                     type: Some(
///                         "ResourceRole",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_resources_globalTask",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct GlobalTask {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "CallableElement",
    // RAW | )
    // RAW | super_class_link : None
    pub callable_element : CallableElement,
}

/// Conversion of Monitoring (Monitoring)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Monitoring",
///     name: "Monitoring",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct Monitoring {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

/// Conversion of Performer (Performer)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Performer",
///     name: "Performer",
///     is_abstract: None,
///     super_class: Some(
///         "ResourceRole",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct Performer {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "ResourceRole",
    // RAW | )
    // RAW | super_class_link : None
    pub resource_role : ResourceRole,
}

/// Conversion of Process (Process)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Process",
///     name: "Process",
///     is_abstract: None,
///     super_class: Some(
///         "FlowElementsContainer CallableElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Process-processType",
///                     name: "processType",
///                     visibility: None,
///                     type: Some(
///                         "ProcessType",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Process-isClosed",
///                     name: "isClosed",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Process-auditing",
///                     name: "auditing",
///                     visibility: None,
///                     type: Some(
///                         "Auditing",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_auditing_process",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Process-monitoring",
///                     name: "monitoring",
///                     visibility: None,
///                     type: Some(
///                         "Monitoring",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_monitoring_process",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Process-properties",
///                     name: "properties",
///                     visibility: None,
///                     type: Some(
///                         "Property",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_properties_process",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Process-supports",
///                     name: "supports",
///                     visibility: None,
///                     type: Some(
///                         "Process",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_supports_process",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Process-definitionalCollaborationRef",
///                     name: "definitionalCollaborationRef",
///                     visibility: None,
///                     type: Some(
///                         "Collaboration",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_definitionalCollaborationRef_process",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Process-isExecutable",
///                     name: "isExecutable",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Process-resources",
///                     name: "resources",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "ResourceRole",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_resources_process",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Process-artifacts",
///                     name: "artifacts",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "Artifact",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_artifacts_process",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Process-correlationSubscriptions",
///                     name: "correlationSubscriptions",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "CorrelationSubscription",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_correlationSubscriptions_process",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Process {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "FlowElementsContainer CallableElement",
    // RAW | )
    // RAW | super_class_link : None
    pub flow_elements_container : FlowElementsContainer,
    pub callable_element : CallableElement,
}

/// struct_level : ProcessType
enum ProcessType {
    /// 'None' from (id : 'ProcessType-None', name : 'None')
    None, 
    /// 'Public' from (id : 'ProcessType-Public', name : 'Public')
    Public, 
    /// 'Private' from (id : 'ProcessType-Private', name : 'Private')
    Private, 
}

// struct_level : A_properties_process

/// Conversion of LaneSet (LaneSet)
///
/// ```json
/// CMOFClass {
///     xmi_id: "LaneSet",
///     name: "LaneSet",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "LaneSet-lanes",
///                     name: "lanes",
///                     visibility: None,
///                     type: Some(
///                         "Lane",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_lanes_laneSet",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "LaneSet-name",
///                     name: "name",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct LaneSet {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

/// Conversion of Lane (Lane)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Lane",
///     name: "Lane",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Lane-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Lane-childLaneSet",
///                     name: "childLaneSet",
///                     visibility: None,
///                     type: Some(
///                         "LaneSet",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_childLaneSet_parentLane",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Lane-partitionElementRef",
///                     name: "partitionElementRef",
///                     visibility: None,
///                     type: Some(
///                         "BaseElement",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_partitionElementRef_lane",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Lane-flowNodeRefs",
///                     name: "flowNodeRefs",
///                     visibility: None,
///                     type: Some(
///                         "FlowNode",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_flowNodeRefs_lanes",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Lane-partitionElement",
///                     name: "partitionElement",
///                     visibility: None,
///                     type: Some(
///                         "BaseElement",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_partitionElement_lane",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Lane {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

// struct_level : A_lanes_laneSet

// struct_level : A_childLaneSet_parentLane

// struct_level : A_resources_globalTask

// struct_level : A_supports_process

// struct_level : A_resources_process

// struct_level : A_artifacts_process

// struct_level : A_correlationSubscriptions_process

/// Conversion of GlobalManualTask (GlobalManualTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "GlobalManualTask",
///     name: "GlobalManualTask",
///     is_abstract: None,
///     super_class: Some(
///         "GlobalTask",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct GlobalManualTask {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "GlobalTask",
    // RAW | )
    // RAW | super_class_link : None
    pub global_task : GlobalTask,
}

/// Conversion of ManualTask (ManualTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ManualTask",
///     name: "ManualTask",
///     is_abstract: None,
///     super_class: Some(
///         "Task",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct ManualTask {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Task",
    // RAW | )
    // RAW | super_class_link : None
    pub task : Task,
}

/// Conversion of UserTask (UserTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "UserTask",
///     name: "UserTask",
///     is_abstract: None,
///     super_class: Some(
///         "Task",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "UserTask-renderings",
///                     name: "renderings",
///                     visibility: None,
///                     type: Some(
///                         "Rendering",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_renderings_usertask",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "UserTask-implementation",
///                     name: "implementation",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct UserTask {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Task",
    // RAW | )
    // RAW | super_class_link : None
    pub task : Task,
}

/// Conversion of Rendering (Rendering)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Rendering",
///     name: "Rendering",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct Rendering {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

// struct_level : A_renderings_usertask

/// Conversion of HumanPerformer (HumanPerformer)
///
/// ```json
/// CMOFClass {
///     xmi_id: "HumanPerformer",
///     name: "HumanPerformer",
///     is_abstract: None,
///     super_class: Some(
///         "Performer",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct HumanPerformer {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Performer",
    // RAW | )
    // RAW | super_class_link : None
    pub performer : Performer,
}

/// Conversion of PotentialOwner (PotentialOwner)
///
/// ```json
/// CMOFClass {
///     xmi_id: "PotentialOwner",
///     name: "PotentialOwner",
///     is_abstract: None,
///     super_class: Some(
///         "HumanPerformer",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct PotentialOwner {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "HumanPerformer",
    // RAW | )
    // RAW | super_class_link : None
    pub human_performer : HumanPerformer,
}

/// Conversion of GlobalUserTask (GlobalUserTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "GlobalUserTask",
///     name: "GlobalUserTask",
///     is_abstract: None,
///     super_class: Some(
///         "GlobalTask",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "GlobalUserTask-implementation",
///                     name: "implementation",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "GlobalUserTask-renderings",
///                     name: "renderings",
///                     visibility: None,
///                     type: Some(
///                         "Rendering",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_renderings_globalUserTask",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct GlobalUserTask {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "GlobalTask",
    // RAW | )
    // RAW | super_class_link : None
    pub global_task : GlobalTask,
}

// struct_level : A_renderings_globalUserTask

// struct_level : A_activationCondition_complexGateway

/// Conversion of Gateway (Gateway)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Gateway",
///     name: "Gateway",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "FlowNode",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Gateway-gatewayDirection",
///                     name: "gatewayDirection",
///                     visibility: None,
///                     type: Some(
///                         "GatewayDirection",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "unspecified",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Gateway {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "FlowNode",
    // RAW | )
    // RAW | super_class_link : None
    pub flow_node : FlowNode,
}

/// struct_level : GatewayDirection
enum GatewayDirection {
    /// 'Unspecified' from (id : 'GatewayDirection-Unspecified', name : 'Unspecified')
    Unspecified, 
    /// 'Converging' from (id : 'GatewayDirection-Converging', name : 'Converging')
    Converging, 
    /// 'Diverging' from (id : 'GatewayDirection-Diverging', name : 'Diverging')
    Diverging, 
    /// 'Mixed' from (id : 'GatewayDirection-Mixed', name : 'Mixed')
    Mixed, 
}

// struct_level : A_default_inclusiveGateway

// struct_level : A_default_exclusiveGateway

/// Conversion of EventBasedGateway (EventBasedGateway)
///
/// ```json
/// CMOFClass {
///     xmi_id: "EventBasedGateway",
///     name: "EventBasedGateway",
///     is_abstract: None,
///     super_class: Some(
///         "Gateway",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "EventBasedGateway-instantiate",
///                     name: "instantiate",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "false",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "EventBasedGateway-eventGatewayType",
///                     name: "eventGatewayType",
///                     visibility: None,
///                     type: Some(
///                         "EventBasedGatewayType",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct EventBasedGateway {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Gateway",
    // RAW | )
    // RAW | super_class_link : None
    pub gateway : Gateway,
}

/// Conversion of ComplexGateway (ComplexGateway)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ComplexGateway",
///     name: "ComplexGateway",
///     is_abstract: None,
///     super_class: Some(
///         "Gateway",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ComplexGateway-activationCondition",
///                     name: "activationCondition",
///                     visibility: None,
///                     type: Some(
///                         "Expression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_activationCondition_complexGateway",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ComplexGateway-default",
///                     name: "default",
///                     visibility: None,
///                     type: Some(
///                         "SequenceFlow",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_default_complexGateway",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ComplexGateway {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Gateway",
    // RAW | )
    // RAW | super_class_link : None
    pub gateway : Gateway,
}

/// Conversion of ExclusiveGateway (ExclusiveGateway)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ExclusiveGateway",
///     name: "ExclusiveGateway",
///     is_abstract: None,
///     super_class: Some(
///         "Gateway",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ExclusiveGateway-default",
///                     name: "default",
///                     visibility: None,
///                     type: Some(
///                         "SequenceFlow",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_default_exclusiveGateway",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ExclusiveGateway {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Gateway",
    // RAW | )
    // RAW | super_class_link : None
    pub gateway : Gateway,
}

/// Conversion of InclusiveGateway (InclusiveGateway)
///
/// ```json
/// CMOFClass {
///     xmi_id: "InclusiveGateway",
///     name: "InclusiveGateway",
///     is_abstract: None,
///     super_class: Some(
///         "Gateway",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "InclusiveGateway-default",
///                     name: "default",
///                     visibility: None,
///                     type: Some(
///                         "SequenceFlow",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_default_inclusiveGateway",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct InclusiveGateway {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Gateway",
    // RAW | )
    // RAW | super_class_link : None
    pub gateway : Gateway,
}

/// Conversion of ParallelGateway (ParallelGateway)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ParallelGateway",
///     name: "ParallelGateway",
///     is_abstract: None,
///     super_class: Some(
///         "Gateway",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct ParallelGateway {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Gateway",
    // RAW | )
    // RAW | super_class_link : None
    pub gateway : Gateway,
}

/// struct_level : EventBasedGatewayType
enum EventBasedGatewayType {
    /// 'Parallel' from (id : 'EventBasedGatewayType-Parallel', name : 'Parallel')
    Parallel, 
    /// 'Exclusive' from (id : 'EventBasedGatewayType-Exclusive', name : 'Exclusive')
    Exclusive, 
}

// struct_level : A_default_complexGateway

/// Conversion of RootElement (RootElement)
///
/// ```json
/// CMOFClass {
///     xmi_id: "RootElement",
///     name: "RootElement",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct RootElement {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

/// Conversion of Relationship (Relationship)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Relationship",
///     name: "Relationship",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Relationship-type",
///                     name: "type",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Relationship-direction",
///                     name: "direction",
///                     visibility: None,
///                     type: Some(
///                         "RelationshipDirection",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Relationship-sources",
///                     name: "sources",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:Class",
///                             href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_sources_relationship",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Relationship-targets",
///                     name: "targets",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:Class",
///                             href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_targets_relationship",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Relationship {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

// struct_level : A_valueRef_extensionAttributeValue

// struct_level : A_value_extensionAttributeValue

/// Conversion of BaseElement (BaseElement)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BaseElement",
///     name: "BaseElement",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BaseElement-id",
///                     name: "id",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BaseElement-extensionDefinitions",
///                     name: "extensionDefinitions",
///                     visibility: None,
///                     type: Some(
///                         "ExtensionDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_extensionDefinitions_baseElement",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BaseElement-extensionValues",
///                     name: "extensionValues",
///                     visibility: None,
///                     type: Some(
///                         "ExtensionAttributeValue",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_extensionValues_baseElement",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BaseElement-documentation",
///                     name: "documentation",
///                     visibility: None,
///                     type: Some(
///                         "Documentation",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_documentation_baseElement",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct BaseElement {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of Extension (Extension)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Extension",
///     name: "Extension",
///     is_abstract: None,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Extension-mustUnderstand",
///                     name: "mustUnderstand",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "false",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Extension-definition",
///                     name: "definition",
///                     visibility: None,
///                     type: Some(
///                         "ExtensionDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_definition_extension",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Extension {
    // RAW | is_abstract : None
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of ExtensionDefinition (ExtensionDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ExtensionDefinition",
///     name: "ExtensionDefinition",
///     is_abstract: None,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ExtensionDefinition-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ExtensionDefinition-extensionAttributeDefinitions",
///                     name: "extensionAttributeDefinitions",
///                     visibility: None,
///                     type: Some(
///                         "ExtensionAttributeDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_extensionAttributeDefinitions_extensionDefinition",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ExtensionDefinition {
    // RAW | is_abstract : None
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of ExtensionAttributeDefinition (ExtensionAttributeDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ExtensionAttributeDefinition",
///     name: "ExtensionAttributeDefinition",
///     is_abstract: None,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ExtensionAttributeDefinition-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ExtensionAttributeDefinition-type",
///                     name: "type",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ExtensionAttributeDefinition-isReference",
///                     name: "isReference",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "false",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ExtensionAttributeDefinition-extensionDefinition",
///                     name: "extensionDefinition",
///                     visibility: None,
///                     type: Some(
///                         "ExtensionDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_extensionAttributeDefinitions_extensionDefinition",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ExtensionAttributeDefinition {
    // RAW | is_abstract : None
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of ExtensionAttributeValue (ExtensionAttributeValue)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ExtensionAttributeValue",
///     name: "ExtensionAttributeValue",
///     is_abstract: None,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ExtensionAttributeValue-valueRef",
///                     name: "valueRef",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:Class",
///                             href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_valueRef_extensionAttributeValue",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ExtensionAttributeValue-value",
///                     name: "value",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:Class",
///                             href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_value_extensionAttributeValue",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ExtensionAttributeValue-extensionAttributeDefinition",
///                     name: "extensionAttributeDefinition",
///                     visibility: None,
///                     type: Some(
///                         "ExtensionAttributeDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_extensionAttributeDefinition_extensionAttributeValue",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ExtensionAttributeValue {
    // RAW | is_abstract : None
    // RAW | super_class : None
    // RAW | super_class_link : None
}

// struct_level : A_extensionDefinitions_baseElement

// struct_level : A_definition_extension

// struct_level : A_extensionAttributeDefinitions_extensionDefinition

// struct_level : A_extensionValues_baseElement

/// struct_level : RelationshipDirection
enum RelationshipDirection {
    /// 'None' from (id : 'RelationshipDirection-None', name : 'None')
    None, 
    /// 'Forward' from (id : 'RelationshipDirection-Forward', name : 'Forward')
    Forward, 
    /// 'Backward' from (id : 'RelationshipDirection-Backward', name : 'Backward')
    Backward, 
    /// 'Both' from (id : 'RelationshipDirection-Both', name : 'Both')
    Both, 
}

// struct_level : A_extensionAttributeDefinition_extensionAttributeValue

/// Conversion of Documentation (Documentation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Documentation",
///     name: "Documentation",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Documentation-text",
///                     name: "text",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Documentation-textFormat",
///                     name: "textFormat",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "text/plain",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Documentation {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

// struct_level : A_documentation_baseElement

// struct_level : A_sources_relationship

// struct_level : A_targets_relationship

// struct_level : A_dataInputAssociation_throwEvent

// struct_level : A_dataOutputAssociation_catchEvent

/// Conversion of Event (Event)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Event",
///     name: "Event",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "FlowNode InteractionNode",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Event-properties",
///                     name: "properties",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "Property",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_properties_event",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Event {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "FlowNode InteractionNode",
    // RAW | )
    // RAW | super_class_link : None
    pub flow_node : FlowNode,
    pub interaction_node : InteractionNode,
}

// struct_level : A_activityRef_compensateEventDefinition

// struct_level : A_inputSet_throwEvent

// struct_level : A_structureRef_signal

// struct_level : A_messageRef_messageEventDefinition

// struct_level : A_outputSet_catchEvent

// struct_level : A_structureRef_escalation

/// Conversion of IntermediateCatchEvent (IntermediateCatchEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "IntermediateCatchEvent",
///     name: "IntermediateCatchEvent",
///     is_abstract: None,
///     super_class: Some(
///         "CatchEvent",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct IntermediateCatchEvent {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "CatchEvent",
    // RAW | )
    // RAW | super_class_link : None
    pub catch_event : CatchEvent,
}

/// Conversion of IntermediateThrowEvent (IntermediateThrowEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "IntermediateThrowEvent",
///     name: "IntermediateThrowEvent",
///     is_abstract: None,
///     super_class: Some(
///         "ThrowEvent",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct IntermediateThrowEvent {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "ThrowEvent",
    // RAW | )
    // RAW | super_class_link : None
    pub throw_event : ThrowEvent,
}

/// Conversion of EndEvent (EndEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "EndEvent",
///     name: "EndEvent",
///     is_abstract: None,
///     super_class: Some(
///         "ThrowEvent",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct EndEvent {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "ThrowEvent",
    // RAW | )
    // RAW | super_class_link : None
    pub throw_event : ThrowEvent,
}

/// Conversion of StartEvent (StartEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "StartEvent",
///     name: "StartEvent",
///     is_abstract: None,
///     super_class: Some(
///         "CatchEvent",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "StartEvent-isInterrupting",
///                     name: "isInterrupting",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "true",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct StartEvent {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "CatchEvent",
    // RAW | )
    // RAW | super_class_link : None
    pub catch_event : CatchEvent,
}

/// Conversion of ThrowEvent (ThrowEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ThrowEvent",
///     name: "ThrowEvent",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "Event",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ThrowEvent-inputSet",
///                     name: "inputSet",
///                     visibility: None,
///                     type: Some(
///                         "InputSet",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_inputSet_throwEvent",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ThrowEvent-eventDefinitionRefs",
///                     name: "eventDefinitionRefs",
///                     visibility: None,
///                     type: Some(
///                         "EventDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_eventDefinitionRefs_throwEvent",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ThrowEvent-dataInputAssociation",
///                     name: "dataInputAssociation",
///                     visibility: None,
///                     type: Some(
///                         "DataInputAssociation",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_dataInputAssociation_throwEvent",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ThrowEvent-dataInputs",
///                     name: "dataInputs",
///                     visibility: None,
///                     type: Some(
///                         "DataInput",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_dataInputs_throwEvent",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ThrowEvent-eventDefinitions",
///                     name: "eventDefinitions",
///                     visibility: None,
///                     type: Some(
///                         "EventDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_eventDefinitions_throwEvent",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ThrowEvent {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "Event",
    // RAW | )
    // RAW | super_class_link : None
    pub event : Event,
}

/// Conversion of CatchEvent (CatchEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CatchEvent",
///     name: "CatchEvent",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "Event",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CatchEvent-parallelMultiple",
///                     name: "parallelMultiple",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CatchEvent-outputSet",
///                     name: "outputSet",
///                     visibility: None,
///                     type: Some(
///                         "OutputSet",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_outputSet_catchEvent",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CatchEvent-eventDefinitionRefs",
///                     name: "eventDefinitionRefs",
///                     visibility: None,
///                     type: Some(
///                         "EventDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_eventDefinitionRefs_catchEvent",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CatchEvent-dataOutputAssociation",
///                     name: "dataOutputAssociation",
///                     visibility: None,
///                     type: Some(
///                         "DataOutputAssociation",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_dataOutputAssociation_catchEvent",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CatchEvent-dataOutputs",
///                     name: "dataOutputs",
///                     visibility: None,
///                     type: Some(
///                         "DataOutput",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_dataOutputs_catchEvent",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CatchEvent-eventDefinitions",
///                     name: "eventDefinitions",
///                     visibility: None,
///                     type: Some(
///                         "EventDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_eventDefinitions_catchEvent",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct CatchEvent {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "Event",
    // RAW | )
    // RAW | super_class_link : None
    pub event : Event,
}

/// Conversion of BoundaryEvent (BoundaryEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BoundaryEvent",
///     name: "BoundaryEvent",
///     is_abstract: None,
///     super_class: Some(
///         "CatchEvent",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BoundaryEvent-cancelActivity",
///                     name: "cancelActivity",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "true",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BoundaryEvent-attachedToRef",
///                     name: "attachedToRef",
///                     visibility: None,
///                     type: Some(
///                         "Activity",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_boundaryEventRefs_attachedToRef",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct BoundaryEvent {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "CatchEvent",
    // RAW | )
    // RAW | super_class_link : None
    pub catch_event : CatchEvent,
}

/// Conversion of EventDefinition (EventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "EventDefinition",
///     name: "EventDefinition",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct EventDefinition {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub root_element : RootElement,
}

// struct_level : A_eventDefinitionRefs_throwEvent

// struct_level : A_eventDefinitionRefs_catchEvent

/// Conversion of CancelEventDefinition (CancelEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CancelEventDefinition",
///     name: "CancelEventDefinition",
///     is_abstract: None,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct CancelEventDefinition {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub event_definition : EventDefinition,
}

/// Conversion of ErrorEventDefinition (ErrorEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ErrorEventDefinition",
///     name: "ErrorEventDefinition",
///     is_abstract: None,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ErrorEventDefinition-errorRef",
///                     name: "errorRef",
///                     visibility: None,
///                     type: Some(
///                         "Error",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_errorRef_errorEventDefinition",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ErrorEventDefinition {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub event_definition : EventDefinition,
}

/// Conversion of TerminateEventDefinition (TerminateEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "TerminateEventDefinition",
///     name: "TerminateEventDefinition",
///     is_abstract: None,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct TerminateEventDefinition {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub event_definition : EventDefinition,
}

// struct_level : A_errorRef_errorEventDefinition

/// Conversion of EscalationEventDefinition (EscalationEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "EscalationEventDefinition",
///     name: "EscalationEventDefinition",
///     is_abstract: None,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "EscalationEventDefinition-escalationRef",
///                     name: "escalationRef",
///                     visibility: None,
///                     type: Some(
///                         "Escalation",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_escalationRef_escalationEventDefinition",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct EscalationEventDefinition {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub event_definition : EventDefinition,
}

/// Conversion of Escalation (Escalation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Escalation",
///     name: "Escalation",
///     is_abstract: None,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Escalation-structureRef",
///                     name: "structureRef",
///                     visibility: None,
///                     type: Some(
///                         "ItemDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_structureRef_escalation",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Escalation-name",
///                     name: "name",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Escalation-escalationCode",
///                     name: "escalationCode",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Escalation {
    // RAW | is_abstract : None
    // RAW | super_class : None
    // RAW | super_class_link : None
}

// struct_level : A_escalationRef_escalationEventDefinition

/// Conversion of CompensateEventDefinition (CompensateEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CompensateEventDefinition",
///     name: "CompensateEventDefinition",
///     is_abstract: None,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CompensateEventDefinition-waitForCompletion",
///                     name: "waitForCompletion",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CompensateEventDefinition-activityRef",
///                     name: "activityRef",
///                     visibility: None,
///                     type: Some(
///                         "Activity",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_activityRef_compensateEventDefinition",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct CompensateEventDefinition {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub event_definition : EventDefinition,
}

/// Conversion of TimerEventDefinition (TimerEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "TimerEventDefinition",
///     name: "TimerEventDefinition",
///     is_abstract: None,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "TimerEventDefinition-timeDate",
///                     name: "timeDate",
///                     visibility: None,
///                     type: Some(
///                         "Expression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_timeDate_timerEventDefinition",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "TimerEventDefinition-timeCycle",
///                     name: "timeCycle",
///                     visibility: None,
///                     type: Some(
///                         "Expression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_timeCycle_timerEventDefinition",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "TimerEventDefinition-timeDuration",
///                     name: "timeDuration",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "Expression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_timeDuration_timerEventDefinition",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct TimerEventDefinition {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub event_definition : EventDefinition,
}

/// Conversion of LinkEventDefinition (LinkEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "LinkEventDefinition",
///     name: "LinkEventDefinition",
///     is_abstract: None,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "LinkEventDefinition-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "LinkEventDefinition-target",
///                     name: "target",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "LinkEventDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_target_source",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "LinkEventDefinition-source",
///                     name: "source",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "LinkEventDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_target_source",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct LinkEventDefinition {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub event_definition : EventDefinition,
}

/// Conversion of MessageEventDefinition (MessageEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "MessageEventDefinition",
///     name: "MessageEventDefinition",
///     is_abstract: None,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "MessageEventDefinition-messageRef",
///                     name: "messageRef",
///                     visibility: None,
///                     type: Some(
///                         "Message",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_messageRef_messageEventDefinition",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "MessageEventDefinition-operationRef",
///                     name: "operationRef",
///                     visibility: None,
///                     type: Some(
///                         "Operation",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_operationRef_messageEventDefinition",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct MessageEventDefinition {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub event_definition : EventDefinition,
}

/// Conversion of ConditionalEventDefinition (ConditionalEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ConditionalEventDefinition",
///     name: "ConditionalEventDefinition",
///     is_abstract: None,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ConditionalEventDefinition-condition",
///                     name: "condition",
///                     visibility: None,
///                     type: Some(
///                         "Expression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_condition_conditionalEventDefinition",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ConditionalEventDefinition {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub event_definition : EventDefinition,
}

/// Conversion of SignalEventDefinition (SignalEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "SignalEventDefinition",
///     name: "SignalEventDefinition",
///     is_abstract: None,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "SignalEventDefinition-signalRef",
///                     name: "signalRef",
///                     visibility: None,
///                     type: Some(
///                         "Signal",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_signalRef_signalEventDefinition",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct SignalEventDefinition {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub event_definition : EventDefinition,
}

/// Conversion of Signal (Signal)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Signal",
///     name: "Signal",
///     is_abstract: None,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Signal-structureRef",
///                     name: "structureRef",
///                     visibility: None,
///                     type: Some(
///                         "ItemDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_structureRef_signal",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Signal-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Signal {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub root_element : RootElement,
}

// struct_level : A_signalRef_signalEventDefinition

/// Conversion of ImplicitThrowEvent (ImplicitThrowEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ImplicitThrowEvent",
///     name: "ImplicitThrowEvent",
///     is_abstract: None,
///     super_class: Some(
///         "ThrowEvent",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct ImplicitThrowEvent {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "ThrowEvent",
    // RAW | )
    // RAW | super_class_link : None
    pub throw_event : ThrowEvent,
}

// struct_level : A_eventDefinitions_throwEvent

// struct_level : A_eventDefinitions_catchEvent

// struct_level : A_dataInputs_throwEvent

// struct_level : A_dataOutputs_catchEvent

// struct_level : A_operationRef_messageEventDefinition

// struct_level : A_condition_conditionalEventDefinition

// struct_level : A_timeDate_timerEventDefinition

// struct_level : A_timeCycle_timerEventDefinition

// struct_level : A_target_source

// struct_level : A_properties_event

// struct_level : A_timeDuration_timerEventDefinition

// struct_level : A_dataState_itemAwareElement

/// Conversion of DataState (DataState)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataState",
///     name: "DataState",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataState-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct DataState {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

/// Conversion of ItemAwareElement (ItemAwareElement)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ItemAwareElement",
///     name: "ItemAwareElement",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ItemAwareElement-itemSubjectRef",
///                     name: "itemSubjectRef",
///                     visibility: None,
///                     type: Some(
///                         "ItemDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_itemSubjectRef_itemAwareElement",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ItemAwareElement-dataState",
///                     name: "dataState",
///                     visibility: None,
///                     type: Some(
///                         "DataState",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_dataState_itemAwareElement",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ItemAwareElement {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

// struct_level : A_operationRef_ioBinding

// struct_level : A_sourceRef_dataAssociation

// struct_level : A_targetRef_dataAssociation

/// Conversion of DataAssociation (DataAssociation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataAssociation",
///     name: "DataAssociation",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataAssociation-transformation",
///                     name: "transformation",
///                     visibility: None,
///                     type: Some(
///                         "FormalExpression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_transformation_dataAssociation",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataAssociation-assignment",
///                     name: "assignment",
///                     visibility: None,
///                     type: Some(
///                         "Assignment",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_assignment_dataAssociation",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataAssociation-targetRef",
///                     name: "targetRef",
///                     visibility: None,
///                     type: Some(
///                         "ItemAwareElement",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_targetRef_dataAssociation",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataAssociation-sourceRef",
///                     name: "sourceRef",
///                     visibility: None,
///                     type: Some(
///                         "ItemAwareElement",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_sourceRef_dataAssociation",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct DataAssociation {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

// struct_level : A_transformation_dataAssociation

/// Conversion of DataInput (DataInput)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataInput",
///     name: "DataInput",
///     is_abstract: None,
///     super_class: Some(
///         "ItemAwareElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataInput-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataInput-isCollection",
///                     name: "isCollection",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "false",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataInput-inputSetRefs",
///                     name: "inputSetRefs",
///                     visibility: None,
///                     type: Some(
///                         "InputSet",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_dataInputRefs_inputSetRefs",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataInput-inputSetWithOptional",
///                     name: "inputSetWithOptional",
///                     visibility: None,
///                     type: Some(
///                         "InputSet",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_optionalInputRefs_inputSetWithOptional",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataInput-inputSetWithWhileExecuting",
///                     name: "inputSetWithWhileExecuting",
///                     visibility: None,
///                     type: Some(
///                         "InputSet",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_whileExecutingInputRefs_inputSetWithWhileExecuting",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct DataInput {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "ItemAwareElement",
    // RAW | )
    // RAW | super_class_link : None
    pub item_aware_element : ItemAwareElement,
}

/// Conversion of DataOutput (DataOutput)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataOutput",
///     name: "DataOutput",
///     is_abstract: None,
///     super_class: Some(
///         "ItemAwareElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataOutput-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataOutput-isCollection",
///                     name: "isCollection",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "false",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataOutput-outputSetRefs",
///                     name: "outputSetRefs",
///                     visibility: None,
///                     type: Some(
///                         "OutputSet",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_dataOutputRefs_outputSetRefs",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataOutput-outputSetWithOptional",
///                     name: "outputSetWithOptional",
///                     visibility: None,
///                     type: Some(
///                         "OutputSet",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_outputSetWithOptional_optionalOutputRefs",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataOutput-outputSetWithWhileExecuting",
///                     name: "outputSetWithWhileExecuting",
///                     visibility: None,
///                     type: Some(
///                         "OutputSet",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_outputSetWithWhileExecuting_whileExecutingOutputRefs",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct DataOutput {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "ItemAwareElement",
    // RAW | )
    // RAW | super_class_link : None
    pub item_aware_element : ItemAwareElement,
}

/// Conversion of InputSet (InputSet)
///
/// ```json
/// CMOFClass {
///     xmi_id: "InputSet",
///     name: "InputSet",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "InputSet-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "InputSet-dataInputRefs",
///                     name: "dataInputRefs",
///                     visibility: None,
///                     type: Some(
///                         "DataInput",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_dataInputRefs_inputSetRefs",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "InputSet-optionalInputRefs",
///                     name: "optionalInputRefs",
///                     visibility: None,
///                     type: Some(
///                         "DataInput",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_optionalInputRefs_inputSetWithOptional",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "InputSet-whileExecutingInputRefs",
///                     name: "whileExecutingInputRefs",
///                     visibility: None,
///                     type: Some(
///                         "DataInput",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_whileExecutingInputRefs_inputSetWithWhileExecuting",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "InputSet-outputSetRefs",
///                     name: "outputSetRefs",
///                     visibility: None,
///                     type: Some(
///                         "OutputSet",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_inputSetRefs_outputSetRefs",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct InputSet {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

/// Conversion of OutputSet (OutputSet)
///
/// ```json
/// CMOFClass {
///     xmi_id: "OutputSet",
///     name: "OutputSet",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "OutputSet-dataOutputRefs",
///                     name: "dataOutputRefs",
///                     visibility: None,
///                     type: Some(
///                         "DataOutput",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_dataOutputRefs_outputSetRefs",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "OutputSet-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "OutputSet-inputSetRefs",
///                     name: "inputSetRefs",
///                     visibility: None,
///                     type: Some(
///                         "InputSet",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_inputSetRefs_outputSetRefs",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "OutputSet-optionalOutputRefs",
///                     name: "optionalOutputRefs",
///                     visibility: None,
///                     type: Some(
///                         "DataOutput",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_outputSetWithOptional_optionalOutputRefs",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "OutputSet-whileExecutingOutputRefs",
///                     name: "whileExecutingOutputRefs",
///                     visibility: None,
///                     type: Some(
///                         "DataOutput",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_outputSetWithWhileExecuting_whileExecutingOutputRefs",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct OutputSet {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

// struct_level : A_dataInputRefs_inputSetRefs

// struct_level : A_dataOutputRefs_outputSetRefs

/// Conversion of Property (Property)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Property",
///     name: "Property",
///     is_abstract: None,
///     super_class: Some(
///         "ItemAwareElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Property-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Property {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "ItemAwareElement",
    // RAW | )
    // RAW | super_class_link : None
    pub item_aware_element : ItemAwareElement,
}

/// Conversion of DataInputAssociation (DataInputAssociation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataInputAssociation",
///     name: "DataInputAssociation",
///     is_abstract: None,
///     super_class: Some(
///         "DataAssociation",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct DataInputAssociation {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "DataAssociation",
    // RAW | )
    // RAW | super_class_link : None
    pub data_association : DataAssociation,
}

/// Conversion of DataOutputAssociation (DataOutputAssociation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataOutputAssociation",
///     name: "DataOutputAssociation",
///     is_abstract: None,
///     super_class: Some(
///         "DataAssociation",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct DataOutputAssociation {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "DataAssociation",
    // RAW | )
    // RAW | super_class_link : None
    pub data_association : DataAssociation,
}

/// Conversion of InputOutputSpecification (InputOutputSpecification)
///
/// ```json
/// CMOFClass {
///     xmi_id: "InputOutputSpecification",
///     name: "InputOutputSpecification",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "InputOutputSpecification-inputSets",
///                     name: "inputSets",
///                     visibility: None,
///                     type: Some(
///                         "InputSet",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_inputSets_inputOutputSpecification",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "InputOutputSpecification-outputSets",
///                     name: "outputSets",
///                     visibility: None,
///                     type: Some(
///                         "OutputSet",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_outputSets_inputOutputSpecification",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "InputOutputSpecification-dataInputs",
///                     name: "dataInputs",
///                     visibility: None,
///                     type: Some(
///                         "DataInput",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_dataInputs_inputOutputSpecification",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "InputOutputSpecification-dataOutputs",
///                     name: "dataOutputs",
///                     visibility: None,
///                     type: Some(
///                         "DataOutput",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_dataOutputs_inputOutputSpecification",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct InputOutputSpecification {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

// struct_level : A_inputSets_inputOutputSpecification

// struct_level : A_outputSets_inputOutputSpecification

// struct_level : A_dataInputs_inputOutputSpecification

// struct_level : A_dataOutputs_inputOutputSpecification

/// Conversion of DataObject (DataObject)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataObject",
///     name: "DataObject",
///     is_abstract: None,
///     super_class: Some(
///         "FlowElement ItemAwareElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataObject-isCollection",
///                     name: "isCollection",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "false",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct DataObject {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "FlowElement ItemAwareElement",
    // RAW | )
    // RAW | super_class_link : None
    pub flow_element : FlowElement,
    pub item_aware_element : ItemAwareElement,
}

// struct_level : A_inputSetRefs_outputSetRefs

/// Conversion of InputOutputBinding (InputOutputBinding)
///
/// ```json
/// CMOFClass {
///     xmi_id: "InputOutputBinding",
///     name: "InputOutputBinding",
///     is_abstract: None,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "InputOutputBinding-inputDataRef",
///                     name: "inputDataRef",
///                     visibility: None,
///                     type: Some(
///                         "InputSet",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_inputDataRef_inputOutputBinding",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "InputOutputBinding-outputDataRef",
///                     name: "outputDataRef",
///                     visibility: None,
///                     type: Some(
///                         "OutputSet",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_outputDataRef_inputOutputBinding",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "InputOutputBinding-operationRef",
///                     name: "operationRef",
///                     visibility: None,
///                     type: Some(
///                         "Operation",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_operationRef_ioBinding",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct InputOutputBinding {
    // RAW | is_abstract : None
    // RAW | super_class : None
    // RAW | super_class_link : None
}

// struct_level : A_inputDataRef_inputOutputBinding

// struct_level : A_outputDataRef_inputOutputBinding

// struct_level : A_whileExecutingInputRefs_inputSetWithWhileExecuting

// struct_level : A_optionalInputRefs_inputSetWithOptional

// struct_level : A_outputSetWithOptional_optionalOutputRefs

// struct_level : A_outputSetWithWhileExecuting_whileExecutingOutputRefs

/// Conversion of Assignment (Assignment)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Assignment",
///     name: "Assignment",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Assignment-from",
///                     name: "from",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "Expression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_from_assignment",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Assignment-to",
///                     name: "to",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "Expression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_to_assignment",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Assignment {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

// struct_level : A_assignment_dataAssociation

/// Conversion of DataStore (DataStore)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataStore",
///     name: "DataStore",
///     is_abstract: None,
///     super_class: Some(
///         "RootElement ItemAwareElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataStore-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataStore-capacity",
///                     name: "capacity",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Integer",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataStore-isUnlimited",
///                     name: "isUnlimited",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "true",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct DataStore {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "RootElement ItemAwareElement",
    // RAW | )
    // RAW | super_class_link : None
    pub root_element : RootElement,
    pub item_aware_element : ItemAwareElement,
}

/// Conversion of DataStoreReference (DataStoreReference)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataStoreReference",
///     name: "DataStoreReference",
///     is_abstract: None,
///     super_class: Some(
///         "ItemAwareElement FlowElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataStoreReference-dataStoreRef",
///                     name: "dataStoreRef",
///                     visibility: None,
///                     type: Some(
///                         "DataStore",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_dataStoreRef_dataStoreReference",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct DataStoreReference {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "ItemAwareElement FlowElement",
    // RAW | )
    // RAW | super_class_link : None
    pub item_aware_element : ItemAwareElement,
    pub flow_element : FlowElement,
}

// struct_level : A_dataStoreRef_dataStoreReference

// struct_level : A_itemSubjectRef_itemAwareElement

// struct_level : A_from_assignment

// struct_level : A_to_assignment

/// Conversion of DataObjectReference (DataObjectReference)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataObjectReference",
///     name: "DataObjectReference",
///     is_abstract: None,
///     super_class: Some(
///         "ItemAwareElement FlowElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DataObjectReference-dataObjectRef",
///                     name: "dataObjectRef",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "DataObject",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_dataObjectRef_dataObject",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct DataObjectReference {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "ItemAwareElement FlowElement",
    // RAW | )
    // RAW | super_class_link : None
    pub item_aware_element : ItemAwareElement,
    pub flow_element : FlowElement,
}

// struct_level : A_dataObjectRef_dataObject

/// Conversion of ConversationLink (ConversationLink)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ConversationLink",
///     name: "ConversationLink",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ConversationLink-sourceRef",
///                     name: "sourceRef",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "InteractionNode",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_sourceRef_outgoingConversationLinks",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ConversationLink-targetRef",
///                     name: "targetRef",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "InteractionNode",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_targetRef_incomingConversationLinks",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ConversationLink-name",
///                     name: "name",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ConversationLink {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

/// Conversion of ConversationAssociation (ConversationAssociation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ConversationAssociation",
///     name: "ConversationAssociation",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ConversationAssociation-innerConversationNodeRef",
///                     name: "innerConversationNodeRef",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "ConversationNode",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_innerConversationNodeRef_conversationAssociation",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ConversationAssociation-outerConversationNodeRef",
///                     name: "outerConversationNodeRef",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "ConversationNode",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_outerConversationNodeRef_conversationAssociation",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ConversationAssociation {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

// struct_level : A_calledCollaborationRef_callConversation

// struct_level : A_participantRefs_conversationNode

// struct_level : A_messageFlowRefs_communication

// struct_level : A_participantAssociations_callConversation

/// Conversion of CallConversation (CallConversation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CallConversation",
///     name: "CallConversation",
///     is_abstract: None,
///     super_class: Some(
///         "ConversationNode",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CallConversation-calledCollaborationRef",
///                     name: "calledCollaborationRef",
///                     visibility: None,
///                     type: Some(
///                         "Collaboration",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_calledCollaborationRef_callConversation",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CallConversation-participantAssociations",
///                     name: "participantAssociations",
///                     visibility: None,
///                     type: Some(
///                         "ParticipantAssociation",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_participantAssociations_callConversation",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct CallConversation {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "ConversationNode",
    // RAW | )
    // RAW | super_class_link : None
    pub conversation_node : ConversationNode,
}

/// Conversion of Conversation (Conversation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Conversation",
///     name: "Conversation",
///     is_abstract: None,
///     super_class: Some(
///         "ConversationNode",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct Conversation {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "ConversationNode",
    // RAW | )
    // RAW | super_class_link : None
    pub conversation_node : ConversationNode,
}

/// Conversion of SubConversation (SubConversation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "SubConversation",
///     name: "SubConversation",
///     is_abstract: None,
///     super_class: Some(
///         "ConversationNode",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "SubConversation-conversationNodes",
///                     name: "conversationNodes",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "ConversationNode",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_conversationNodes_subConversation",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct SubConversation {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "ConversationNode",
    // RAW | )
    // RAW | super_class_link : None
    pub conversation_node : ConversationNode,
}

/// Conversion of ConversationNode (ConversationNode)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ConversationNode",
///     name: "ConversationNode",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "InteractionNode BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ConversationNode-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ConversationNode-participantRefs",
///                     name: "participantRefs",
///                     visibility: None,
///                     type: Some(
///                         "Participant",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "2",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_participantRefs_conversationNode",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ConversationNode-messageFlowRefs",
///                     name: "messageFlowRefs",
///                     visibility: None,
///                     type: Some(
///                         "MessageFlow",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_messageFlowRefs_communication",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ConversationNode-correlationKeys",
///                     name: "correlationKeys",
///                     visibility: None,
///                     type: Some(
///                         "CorrelationKey",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_correlationKeys_conversationNode",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ConversationNode {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "InteractionNode BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub interaction_node : InteractionNode,
    pub base_element : BaseElement,
}

/// Conversion of GlobalConversation (GlobalConversation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "GlobalConversation",
///     name: "GlobalConversation",
///     is_abstract: None,
///     super_class: Some(
///         "Collaboration",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct GlobalConversation {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Collaboration",
    // RAW | )
    // RAW | super_class_link : None
    pub collaboration : Collaboration,
}

// struct_level : A_correlationKeys_collaboration

// struct_level : A_correlationKeys_conversationNode

// struct_level : A_innerConversationNodeRef_conversationAssociation

// struct_level : A_outerConversationNodeRef_conversationAssociation

// struct_level : A_conversationNodes_subConversation

// struct_level : A_sourceRef_outgoingConversationLinks

// struct_level : A_targetRef_incomingConversationLinks

/// Conversion of PartnerEntity (PartnerEntity)
///
/// ```json
/// CMOFClass {
///     xmi_id: "PartnerEntity",
///     name: "PartnerEntity",
///     is_abstract: None,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "PartnerEntity-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "PartnerEntity-participantRef",
///                     name: "participantRef",
///                     visibility: None,
///                     type: Some(
///                         "Participant",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_partnerEntityRef_participantRef",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct PartnerEntity {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub root_element : RootElement,
}

/// Conversion of PartnerRole (PartnerRole)
///
/// ```json
/// CMOFClass {
///     xmi_id: "PartnerRole",
///     name: "PartnerRole",
///     is_abstract: None,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "PartnerRole-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "PartnerRole-participantRef",
///                     name: "participantRef",
///                     visibility: None,
///                     type: Some(
///                         "Participant",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_partnerRoleRef_participantRef",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct PartnerRole {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub root_element : RootElement,
}

// struct_level : A_correlationPropertyRef_correlationKey

/// Conversion of CorrelationProperty (CorrelationProperty)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CorrelationProperty",
///     name: "CorrelationProperty",
///     is_abstract: None,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CorrelationProperty-correlationPropertyRetrievalExpression",
///                     name: "correlationPropertyRetrievalExpression",
///                     visibility: None,
///                     type: Some(
///                         "CorrelationPropertyRetrievalExpression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_correlationPropertyRetrievalExpression_correlationproperty",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CorrelationProperty-name",
///                     name: "name",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CorrelationProperty-type",
///                     name: "type",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "ItemDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_type_correlationProperty",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct CorrelationProperty {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub root_element : RootElement,
}

/// struct_level : ItemKind
enum ItemKind {
    /// 'Physical' from (id : 'ItemKind-Physical', name : 'Physical')
    Physical, 
    /// 'Information' from (id : 'ItemKind-Information', name : 'Information')
    Information, 
}

// struct_level : A_supportedInterfaceRefs_callableElements

// struct_level : A_ioBinding_callableElement

// struct_level : A_ioSpecification_callableElement

// struct_level : A_messagePath_correlationset

// struct_level : A_structureRef_error

/// Conversion of Error (Error)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Error",
///     name: "Error",
///     is_abstract: None,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Error-structureRef",
///                     name: "structureRef",
///                     visibility: None,
///                     type: Some(
///                         "ItemDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_structureRef_error",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Error-name",
///                     name: "name",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Error-errorCode",
///                     name: "errorCode",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Error {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub root_element : RootElement,
}

// struct_level : A_evaluatesToTypeRef_formalExpression

/// Conversion of CorrelationKey (CorrelationKey)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CorrelationKey",
///     name: "CorrelationKey",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CorrelationKey-correlationPropertyRef",
///                     name: "correlationPropertyRef",
///                     visibility: None,
///                     type: Some(
///                         "CorrelationProperty",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_correlationPropertyRef_correlationKey",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CorrelationKey-name",
///                     name: "name",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct CorrelationKey {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

/// Conversion of Expression (Expression)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Expression",
///     name: "Expression",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct Expression {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

/// Conversion of FormalExpression (FormalExpression)
///
/// ```json
/// CMOFClass {
///     xmi_id: "FormalExpression",
///     name: "FormalExpression",
///     is_abstract: None,
///     super_class: Some(
///         "Expression",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "FormalExpression-language",
///                     name: "language",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "FormalExpression-body",
///                     name: "body",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:Class",
///                             href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "FormalExpression-evaluatesToTypeRef",
///                     name: "evaluatesToTypeRef",
///                     visibility: None,
///                     type: Some(
///                         "ItemDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_evaluatesToTypeRef_formalExpression",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct FormalExpression {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Expression",
    // RAW | )
    // RAW | super_class_link : None
    pub expression : Expression,
}

/// Conversion of Message (Message)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Message",
///     name: "Message",
///     is_abstract: None,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Message-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Message-itemRef",
///                     name: "itemRef",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "ItemDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_itemRef_message",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Message {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub root_element : RootElement,
}

/// Conversion of ItemDefinition (ItemDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ItemDefinition",
///     name: "ItemDefinition",
///     is_abstract: None,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ItemDefinition-itemKind",
///                     name: "itemKind",
///                     visibility: None,
///                     type: Some(
///                         "ItemKind",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ItemDefinition-structureRef",
///                     name: "structureRef",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:Class",
///                             href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ItemDefinition-isCollection",
///                     name: "isCollection",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "false",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ItemDefinition-import",
///                     name: "import",
///                     visibility: None,
///                     type: Some(
///                         "Import",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_import_itemDefinition",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ItemDefinition {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub root_element : RootElement,
}

// struct_level : A_conditionExpression_sequenceFlow

/// Conversion of FlowElement (FlowElement)
///
/// ```json
/// CMOFClass {
///     xmi_id: "FlowElement",
///     name: "FlowElement",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "FlowElement-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "FlowElement-auditing",
///                     name: "auditing",
///                     visibility: None,
///                     type: Some(
///                         "Auditing",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_auditing_flowElement",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "FlowElement-monitoring",
///                     name: "monitoring",
///                     visibility: None,
///                     type: Some(
///                         "Monitoring",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_monitoring_flowElement",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "FlowElement-categoryValueRef",
///                     name: "categoryValueRef",
///                     visibility: None,
///                     type: Some(
///                         "CategoryValue",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_categorizedFlowElements_categoryValueRef",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct FlowElement {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

/// Conversion of SequenceFlow (SequenceFlow)
///
/// ```json
/// CMOFClass {
///     xmi_id: "SequenceFlow",
///     name: "SequenceFlow",
///     is_abstract: None,
///     super_class: Some(
///         "FlowElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "SequenceFlow-isImmediate",
///                     name: "isImmediate",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "SequenceFlow-conditionExpression",
///                     name: "conditionExpression",
///                     visibility: None,
///                     type: Some(
///                         "Expression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_conditionExpression_sequenceFlow",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "SequenceFlow-sourceRef",
///                     name: "sourceRef",
///                     visibility: None,
///                     type: Some(
///                         "FlowNode",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_sourceRef_outgoing_flow",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "SequenceFlow-targetRef",
///                     name: "targetRef",
///                     visibility: None,
///                     type: Some(
///                         "FlowNode",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_targetRef_incoming_flow",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct SequenceFlow {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "FlowElement",
    // RAW | )
    // RAW | super_class_link : None
    pub flow_element : FlowElement,
}

/// Conversion of FlowElementsContainer (FlowElementsContainer)
///
/// ```json
/// CMOFClass {
///     xmi_id: "FlowElementsContainer",
///     name: "FlowElementsContainer",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "FlowElementsContainer-flowElements",
///                     name: "flowElements",
///                     visibility: None,
///                     type: Some(
///                         "FlowElement",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_flowElements_container",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "FlowElementsContainer-laneSets",
///                     name: "laneSets",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "LaneSet",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_laneSets_flowElementsContainer",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct FlowElementsContainer {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

// struct_level : A_flowElements_container

/// Conversion of CallableElement (CallableElement)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CallableElement",
///     name: "CallableElement",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CallableElement-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CallableElement-ioSpecification",
///                     name: "ioSpecification",
///                     visibility: None,
///                     type: Some(
///                         "InputOutputSpecification",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_ioSpecification_callableElement",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CallableElement-supportedInterfaceRefs",
///                     name: "supportedInterfaceRefs",
///                     visibility: None,
///                     type: Some(
///                         "Interface",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_supportedInterfaceRefs_callableElements",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CallableElement-ioBinding",
///                     name: "ioBinding",
///                     visibility: None,
///                     type: Some(
///                         "InputOutputBinding",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_ioBinding_callableElement",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct CallableElement {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub root_element : RootElement,
}

/// Conversion of FlowNode (FlowNode)
///
/// ```json
/// CMOFClass {
///     xmi_id: "FlowNode",
///     name: "FlowNode",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "FlowElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "FlowNode-outgoing",
///                     name: "outgoing",
///                     visibility: None,
///                     type: Some(
///                         "SequenceFlow",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: Some(
///                         "true",
///                     ),
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_sourceRef_outgoing_flow",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "FlowNode-incoming",
///                     name: "incoming",
///                     visibility: None,
///                     type: Some(
///                         "SequenceFlow",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_targetRef_incoming_flow",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "FlowNode-lanes",
///                     name: "lanes",
///                     visibility: None,
///                     type: Some(
///                         "Lane",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_flowNodeRefs_lanes",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct FlowNode {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "FlowElement",
    // RAW | )
    // RAW | super_class_link : None
    pub flow_element : FlowElement,
}

// struct_level : A_sourceRef_outgoing_flow

// struct_level : A_targetRef_incoming_flow

/// Conversion of CorrelationPropertyRetrievalExpression (CorrelationPropertyRetrievalExpression)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CorrelationPropertyRetrievalExpression",
///     name: "CorrelationPropertyRetrievalExpression",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CorrelationPropertyRetrievalExpression-messagePath",
///                     name: "messagePath",
///                     visibility: None,
///                     type: Some(
///                         "FormalExpression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_messagePath_correlationset",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CorrelationPropertyRetrievalExpression-messageRef",
///                     name: "messageRef",
///                     visibility: None,
///                     type: Some(
///                         "Message",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_messageRef_correlationPropertyRetrievalExpression",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct CorrelationPropertyRetrievalExpression {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

/// Conversion of CorrelationPropertyBinding (CorrelationPropertyBinding)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CorrelationPropertyBinding",
///     name: "CorrelationPropertyBinding",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CorrelationPropertyBinding-dataPath",
///                     name: "dataPath",
///                     visibility: None,
///                     type: Some(
///                         "FormalExpression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_dataPath_correlationPropertyBinding",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CorrelationPropertyBinding-correlationPropertyRef",
///                     name: "correlationPropertyRef",
///                     visibility: None,
///                     type: Some(
///                         "CorrelationProperty",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_correlationPropertyRef_correlationPropertyBinding",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct CorrelationPropertyBinding {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

// struct_level : A_correlationPropertyRetrievalExpression_correlationproperty

// struct_level : A_messageRef_correlationPropertyRetrievalExpression

// struct_level : A_dataPath_correlationPropertyBinding

// struct_level : A_correlationPropertyRef_correlationPropertyBinding

/// Conversion of Resource (Resource)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Resource",
///     name: "Resource",
///     is_abstract: None,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Resource-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Resource-resourceParameters",
///                     name: "resourceParameters",
///                     visibility: None,
///                     type: Some(
///                         "ResourceParameter",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_resourceParameters_resource",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Resource {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub root_element : RootElement,
}

/// Conversion of ResourceParameter (ResourceParameter)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ResourceParameter",
///     name: "ResourceParameter",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ResourceParameter-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ResourceParameter-isRequired",
///                     name: "isRequired",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ResourceParameter-type",
///                     name: "type",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "ItemDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_type_resourceParameter",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ResourceParameter {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

// struct_level : A_resourceParameters_resource

// struct_level : A_import_itemDefinition

/// Conversion of CorrelationSubscription (CorrelationSubscription)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CorrelationSubscription",
///     name: "CorrelationSubscription",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CorrelationSubscription-correlationKeyRef",
///                     name: "correlationKeyRef",
///                     visibility: None,
///                     type: Some(
///                         "CorrelationKey",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_correlationKeyRef_correlationSubscription",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CorrelationSubscription-correlationPropertyBinding",
///                     name: "correlationPropertyBinding",
///                     visibility: None,
///                     type: Some(
///                         "CorrelationPropertyBinding",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_correlationPropertyBinding_correlationSubscription",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct CorrelationSubscription {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

// struct_level : A_correlationKeyRef_correlationSubscription

// struct_level : A_correlationPropertyBinding_correlationSubscription

// struct_level : A_auditing_flowElement

// struct_level : A_monitoring_flowElement

// struct_level : A_type_correlationProperty

// struct_level : A_type_resourceParameter

// struct_level : A_itemRef_message

// struct_level : A_laneSets_flowElementsContainer

/// Conversion of MessageFlow (MessageFlow)
///
/// ```json
/// CMOFClass {
///     xmi_id: "MessageFlow",
///     name: "MessageFlow",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "MessageFlow-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "MessageFlow-sourceRef",
///                     name: "sourceRef",
///                     visibility: None,
///                     type: Some(
///                         "InteractionNode",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_sourceRef_messageFlow",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "MessageFlow-targetRef",
///                     name: "targetRef",
///                     visibility: None,
///                     type: Some(
///                         "InteractionNode",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_targetRef_messageFlow",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "MessageFlow-messageRef",
///                     name: "messageRef",
///                     visibility: None,
///                     type: Some(
///                         "Message",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_messageRef_messageFlow",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct MessageFlow {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

/// Conversion of MessageFlowAssociation (MessageFlowAssociation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "MessageFlowAssociation",
///     name: "MessageFlowAssociation",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "MessageFlowAssociation-innerMessageFlowRef",
///                     name: "innerMessageFlowRef",
///                     visibility: None,
///                     type: Some(
///                         "MessageFlow",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_innerMessageFlowRef_messageFlowAssociation",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "MessageFlowAssociation-outerMessageFlowRef",
///                     name: "outerMessageFlowRef",
///                     visibility: None,
///                     type: Some(
///                         "MessageFlow",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_outerMessageFlowRef_messageFlowAssociation",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct MessageFlowAssociation {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

/// Conversion of InteractionNode (InteractionNode)
///
/// ```json
/// CMOFClass {
///     xmi_id: "InteractionNode",
///     name: "InteractionNode",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "InteractionNode-incomingConversationLinks",
///                     name: "incomingConversationLinks",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "ConversationLink",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_targetRef_incomingConversationLinks",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "InteractionNode-outgoingConversationLinks",
///                     name: "outgoingConversationLinks",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "ConversationLink",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_sourceRef_outgoingConversationLinks",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct InteractionNode {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of Participant (Participant)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Participant",
///     name: "Participant",
///     is_abstract: None,
///     super_class: Some(
///         "InteractionNode BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Participant-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Participant-interfaceRefs",
///                     name: "interfaceRefs",
///                     visibility: None,
///                     type: Some(
///                         "Interface",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_interfaceRefs_participant",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Participant-participantMultiplicity",
///                     name: "participantMultiplicity",
///                     visibility: None,
///                     type: Some(
///                         "ParticipantMultiplicity",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_participantMultiplicity_participant",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Participant-endPointRefs",
///                     name: "endPointRefs",
///                     visibility: None,
///                     type: Some(
///                         "EndPoint",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_endPointRefs_participant",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Participant-processRef",
///                     name: "processRef",
///                     visibility: None,
///                     type: Some(
///                         "Process",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_processRef_participant",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Participant {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "InteractionNode BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub interaction_node : InteractionNode,
    pub base_element : BaseElement,
}

/// Conversion of ParticipantAssociation (ParticipantAssociation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ParticipantAssociation",
///     name: "ParticipantAssociation",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ParticipantAssociation-innerParticipantRef",
///                     name: "innerParticipantRef",
///                     visibility: None,
///                     type: Some(
///                         "Participant",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_innerParticipantRef_participantAssociation",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ParticipantAssociation-outerParticipantRef",
///                     name: "outerParticipantRef",
///                     visibility: None,
///                     type: Some(
///                         "Participant",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_outerParticipantRef_participantAssociation",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ParticipantAssociation {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

/// Conversion of ParticipantMultiplicity (ParticipantMultiplicity)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ParticipantMultiplicity",
///     name: "ParticipantMultiplicity",
///     is_abstract: None,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ParticipantMultiplicity-minimum",
///                     name: "minimum",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Integer",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "0",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ParticipantMultiplicity-maximum",
///                     name: "maximum",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Integer",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: Some(
///                         "1",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ParticipantMultiplicity {
    // RAW | is_abstract : None
    // RAW | super_class : None
    // RAW | super_class_link : None
}

// struct_level : A_messageFlowAssociations_collaboration

// struct_level : A_participantAssociations_collaboration

// struct_level : A_artifacts_collaboration

/// Conversion of Collaboration (Collaboration)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Collaboration",
///     name: "Collaboration",
///     is_abstract: None,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Collaboration-name",
///                     name: "name",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Collaboration-isClosed",
///                     name: "isClosed",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Collaboration-choreographyRef",
///                     name: "choreographyRef",
///                     visibility: None,
///                     type: Some(
///                         "Choreography",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_choreographyRef_collaboration",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Collaboration-artifacts",
///                     name: "artifacts",
///                     visibility: None,
///                     type: Some(
///                         "Artifact",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_artifacts_collaboration",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Collaboration-participantAssociations",
///                     name: "participantAssociations",
///                     visibility: None,
///                     type: Some(
///                         "ParticipantAssociation",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_participantAssociations_collaboration",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Collaboration-messageFlowAssociations",
///                     name: "messageFlowAssociations",
///                     visibility: None,
///                     type: Some(
///                         "MessageFlowAssociation",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_messageFlowAssociations_collaboration",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Collaboration-conversationAssociations",
///                     name: "conversationAssociations",
///                     visibility: None,
///                     type: Some(
///                         "ConversationAssociation",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_conversationAssociations_converstaionAssociations",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Collaboration-participants",
///                     name: "participants",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "Participant",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_participants_collaboration",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Collaboration-messageFlows",
///                     name: "messageFlows",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "MessageFlow",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_messageFlows_collaboration",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Collaboration-correlationKeys",
///                     name: "correlationKeys",
///                     visibility: None,
///                     type: Some(
///                         "CorrelationKey",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_correlationKeys_collaboration",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Collaboration-conversations",
///                     name: "conversations",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "ConversationNode",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_conversations_collaboration",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Collaboration-conversationLinks",
///                     name: "conversationLinks",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "ConversationLink",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_conversationLinks_collaboration",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Collaboration {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub root_element : RootElement,
}

// struct_level : A_conversationAssociations_converstaionAssociations

// struct_level : A_choreographyRef_collaboration

// struct_level : A_innerParticipantRef_participantAssociation

// struct_level : A_outerParticipantRef_participantAssociation

// struct_level : A_endPointRefs_participant

// struct_level : A_participantMultiplicity_participant

// struct_level : A_interfaceRefs_participant

// struct_level : A_partnerEntityRef_participantRef

// struct_level : A_partnerRoleRef_participantRef

// struct_level : A_processRef_participant

// struct_level : A_innerMessageFlowRef_messageFlowAssociation

// struct_level : A_outerMessageFlowRef_messageFlowAssociation

// struct_level : A_targetRef_messageFlow

// struct_level : A_messageRef_messageFlow

// struct_level : A_sourceRef_messageFlow

// struct_level : A_participants_collaboration

// struct_level : A_messageFlows_collaboration

// struct_level : A_conversations_collaboration

// struct_level : A_conversationLinks_collaboration

// struct_level : A_participantAssociations_callChoreographyActivity

/// Conversion of ChoreographyActivity (ChoreographyActivity)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ChoreographyActivity",
///     name: "ChoreographyActivity",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "FlowNode",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ChoreographyActivity-participantRefs",
///                     name: "participantRefs",
///                     visibility: None,
///                     type: Some(
///                         "Participant",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "2",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_participantRefs_choreographyActivity",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ChoreographyActivity-initiatingParticipantRef",
///                     name: "initiatingParticipantRef",
///                     visibility: None,
///                     type: Some(
///                         "Participant",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_initiatingParticipantRef_choreographyActivity",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ChoreographyActivity-correlationKeys",
///                     name: "correlationKeys",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "CorrelationKey",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_correlationKeys_choreographyActivity",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ChoreographyActivity-loopType",
///                     name: "loopType",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "ChoreographyLoopType",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "None",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ChoreographyActivity {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "FlowNode",
    // RAW | )
    // RAW | super_class_link : None
    pub flow_node : FlowNode,
}

/// Conversion of CallChoreography (CallChoreography)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CallChoreography",
///     name: "CallChoreography",
///     is_abstract: None,
///     super_class: Some(
///         "ChoreographyActivity",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CallChoreography-calledChoreographyRef",
///                     name: "calledChoreographyRef",
///                     visibility: None,
///                     type: Some(
///                         "Choreography",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_calledChoreographyRef_callChoreographyActivity",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CallChoreography-participantAssociations",
///                     name: "participantAssociations",
///                     visibility: None,
///                     type: Some(
///                         "ParticipantAssociation",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_participantAssociations_callChoreographyActivity",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct CallChoreography {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "ChoreographyActivity",
    // RAW | )
    // RAW | super_class_link : None
    pub choreography_activity : ChoreographyActivity,
}

/// Conversion of SubChoreography (SubChoreography)
///
/// ```json
/// CMOFClass {
///     xmi_id: "SubChoreography",
///     name: "SubChoreography",
///     is_abstract: None,
///     super_class: Some(
///         "ChoreographyActivity FlowElementsContainer",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "SubChoreography-artifacts",
///                     name: "artifacts",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "Artifact",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_artifacts_subChoreography",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct SubChoreography {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "ChoreographyActivity FlowElementsContainer",
    // RAW | )
    // RAW | super_class_link : None
    pub choreography_activity : ChoreographyActivity,
    pub flow_elements_container : FlowElementsContainer,
}

/// Conversion of ChoreographyTask (ChoreographyTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ChoreographyTask",
///     name: "ChoreographyTask",
///     is_abstract: None,
///     super_class: Some(
///         "ChoreographyActivity",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ChoreographyTask-messageFlowRef",
///                     name: "messageFlowRef",
///                     visibility: None,
///                     type: Some(
///                         "MessageFlow",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: Some(
///                         "2",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_messageFlowRef_choreographyTask",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ChoreographyTask {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "ChoreographyActivity",
    // RAW | )
    // RAW | super_class_link : None
    pub choreography_activity : ChoreographyActivity,
}

// struct_level : A_calledChoreographyRef_callChoreographyActivity

// struct_level : A_messageFlowRef_choreographyTask

// struct_level : A_initiatingParticipantRef_choreographyActivity

// struct_level : A_participantRefs_choreographyActivity

// struct_level : A_artifacts_subChoreography

/// struct_level : ChoreographyLoopType
enum ChoreographyLoopType {
    /// 'None' from (id : 'ChoreographyLoopType-None', name : 'None')
    None, 
    /// 'Standard' from (id : 'ChoreographyLoopType-Standard', name : 'Standard')
    Standard, 
    /// 'MultiInstanceSequential' from (id : 'ChoreographyLoopType-MultiInstanceSequential', name : 'MultiInstanceSequential')
    MultiInstanceSequential, 
    /// 'MultiInstanceParallel' from (id : 'ChoreographyLoopType-MultiInstanceParallel', name : 'MultiInstanceParallel')
    MultiInstanceParallel, 
}

// struct_level : A_correlationKeys_choreographyActivity

// struct_level : A_initiatingParticipantRef_globalChoreographyTask

/// Conversion of Choreography (Choreography)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Choreography",
///     name: "Choreography",
///     is_abstract: None,
///     super_class: Some(
///         "FlowElementsContainer Collaboration",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct Choreography {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "FlowElementsContainer Collaboration",
    // RAW | )
    // RAW | super_class_link : None
    pub flow_elements_container : FlowElementsContainer,
    pub collaboration : Collaboration,
}

/// Conversion of GlobalChoreographyTask (GlobalChoreographyTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "GlobalChoreographyTask",
///     name: "GlobalChoreographyTask",
///     is_abstract: None,
///     super_class: Some(
///         "Choreography",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "GlobalChoreographyTask-initiatingParticipantRef",
///                     name: "initiatingParticipantRef",
///                     visibility: None,
///                     type: Some(
///                         "Participant",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_initiatingParticipantRef_globalChoreographyTask",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct GlobalChoreographyTask {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Choreography",
    // RAW | )
    // RAW | super_class_link : None
    pub choreography : Choreography,
}

// struct_level : A_sourceRef_outgoing_association

// struct_level : A_targetRef_incoming_association

/// Conversion of TextAnnotation (TextAnnotation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "TextAnnotation",
///     name: "TextAnnotation",
///     is_abstract: None,
///     super_class: Some(
///         "Artifact",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "TextAnnotation-text",
///                     name: "text",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "TextAnnotation-textFormat",
///                     name: "textFormat",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "text/plain",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct TextAnnotation {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Artifact",
    // RAW | )
    // RAW | super_class_link : None
    pub artifact : Artifact,
}

/// Conversion of Group (Group)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Group",
///     name: "Group",
///     is_abstract: None,
///     super_class: Some(
///         "Artifact",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Group-categoryValueRef",
///                     name: "categoryValueRef",
///                     visibility: None,
///                     type: Some(
///                         "CategoryValue",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_categoryValueRef_categoryValueRef",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Group {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Artifact",
    // RAW | )
    // RAW | super_class_link : None
    pub artifact : Artifact,
}

/// Conversion of Association (Association)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Association",
///     name: "Association",
///     is_abstract: None,
///     super_class: Some(
///         "Artifact",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Association-associationDirection",
///                     name: "associationDirection",
///                     visibility: None,
///                     type: Some(
///                         "AssociationDirection",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Association-sourceRef",
///                     name: "sourceRef",
///                     visibility: None,
///                     type: Some(
///                         "BaseElement",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_sourceRef_outgoing_association",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Association-targetRef",
///                     name: "targetRef",
///                     visibility: None,
///                     type: Some(
///                         "BaseElement",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_targetRef_incoming_association",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Association {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Artifact",
    // RAW | )
    // RAW | super_class_link : None
    pub artifact : Artifact,
}

/// Conversion of Category (Category)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Category",
///     name: "Category",
///     is_abstract: None,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Category-categoryValue",
///                     name: "categoryValue",
///                     visibility: None,
///                     type: Some(
///                         "CategoryValue",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_categoryValue_category",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Category-name",
///                     name: "name",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Category {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub root_element : RootElement,
}

/// Conversion of Artifact (Artifact)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Artifact",
///     name: "Artifact",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct Artifact {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

// struct_level : A_categoryValueRef_categoryValueRef

/// struct_level : AssociationDirection
enum AssociationDirection {
    /// 'None' from (id : 'AssociationDirection-None', name : 'None')
    None, 
    /// 'One' from (id : 'AssociationDirection-One', name : 'One')
    One, 
    /// 'Both' from (id : 'AssociationDirection-Both', name : 'Both')
    Both, 
}

/// Conversion of CategoryValue (CategoryValue)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CategoryValue",
///     name: "CategoryValue",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CategoryValue-categorizedFlowElements",
///                     name: "categorizedFlowElements",
///                     visibility: None,
///                     type: Some(
///                         "FlowElement",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_categorizedFlowElements_categoryValueRef",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CategoryValue-value",
///                     name: "value",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct CategoryValue {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

// struct_level : A_categoryValue_category

// struct_level : A_categorizedFlowElements_categoryValueRef

// struct_level : A_event_complexBehaviorDefinition

// struct_level : A_expression_resourceAssignmentExpression

// struct_level : A_expression_resourceParameterBinding

// struct_level : A_noneBehaviorEventRef_multiInstanceLoopCharacteristics

// struct_level : A_oneBehaviorEventRef_multiInstanceLoopCharacteristics

// struct_level : A_completionCondition_multiInstanceLoopCharacteristics

// struct_level : A_condition_complexBehaviorDefinition

// struct_level : A_resourceRef_activityResource

// struct_level : A_messageRef_sendTask

// struct_level : A_messageRef_receiveTask

/// Conversion of Activity (Activity)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Activity",
///     name: "Activity",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "FlowNode",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Activity-isForCompensation",
///                     name: "isForCompensation",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "false",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Activity-loopCharacteristics",
///                     name: "loopCharacteristics",
///                     visibility: None,
///                     type: Some(
///                         "LoopCharacteristics",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_loopCharacteristics_activity",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Activity-resources",
///                     name: "resources",
///                     visibility: None,
///                     type: Some(
///                         "ResourceRole",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_resources_activity",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Activity-default",
///                     name: "default",
///                     visibility: None,
///                     type: Some(
///                         "SequenceFlow",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_default_activity",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Activity-properties",
///                     name: "properties",
///                     visibility: None,
///                     type: Some(
///                         "Property",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_properties_activity",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Activity-ioSpecification",
///                     name: "ioSpecification",
///                     visibility: None,
///                     type: Some(
///                         "InputOutputSpecification",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_ioSpecification_activity",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Activity-boundaryEventRefs",
///                     name: "boundaryEventRefs",
///                     visibility: None,
///                     type: Some(
///                         "BoundaryEvent",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_boundaryEventRefs_attachedToRef",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Activity-dataInputAssociations",
///                     name: "dataInputAssociations",
///                     visibility: None,
///                     type: Some(
///                         "DataInputAssociation",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_dataInputAssociations_activity",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Activity-dataOutputAssociations",
///                     name: "dataOutputAssociations",
///                     visibility: None,
///                     type: Some(
///                         "DataOutputAssociation",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_dataOutputAssociations_activity",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Activity-startQuantity",
///                     name: "startQuantity",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Integer",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "1",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Activity-completionQuantity",
///                     name: "completionQuantity",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Integer",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "1",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Activity {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "FlowNode",
    // RAW | )
    // RAW | super_class_link : None
    pub flow_node : FlowNode,
}

// struct_level : A_operationRef_serviceTask

// struct_level : A_calledElementRef_callActivity

// struct_level : A_loopCardinality_multiInstanceLoopCharacteristics

// struct_level : A_properties_activity

// struct_level : A_resources_activity

// struct_level : A_loopCondition_standardLoopCharacteristics

// struct_level : A_loopCharacteristics_activity

// struct_level : A_ioSpecification_activity

// struct_level : A_completionCondition_adHocSubProcess

/// Conversion of ServiceTask (ServiceTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ServiceTask",
///     name: "ServiceTask",
///     is_abstract: None,
///     super_class: Some(
///         "Task",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ServiceTask-implementation",
///                     name: "implementation",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ServiceTask-operationRef",
///                     name: "operationRef",
///                     visibility: None,
///                     type: Some(
///                         "Operation",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_operationRef_serviceTask",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ServiceTask {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Task",
    // RAW | )
    // RAW | super_class_link : None
    pub task : Task,
}

/// Conversion of SubProcess (SubProcess)
///
/// ```json
/// CMOFClass {
///     xmi_id: "SubProcess",
///     name: "SubProcess",
///     is_abstract: None,
///     super_class: Some(
///         "Activity FlowElementsContainer",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "SubProcess-triggeredByEvent",
///                     name: "triggeredByEvent",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "false",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "SubProcess-artifacts",
///                     name: "artifacts",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "Artifact",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_artifacts_subProcess",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct SubProcess {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Activity FlowElementsContainer",
    // RAW | )
    // RAW | super_class_link : None
    pub activity : Activity,
    pub flow_elements_container : FlowElementsContainer,
}

// struct_level : A_operationRef_receiveTask

// struct_level : A_operationRef_sendTask

/// Conversion of LoopCharacteristics (LoopCharacteristics)
///
/// ```json
/// CMOFClass {
///     xmi_id: "LoopCharacteristics",
///     name: "LoopCharacteristics",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct LoopCharacteristics {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

/// struct_level : MultiInstanceBehavior
enum MultiInstanceBehavior {
    /// 'None' from (id : 'MultiInstanceBehavior-None', name : 'None')
    None, 
    /// 'One' from (id : 'MultiInstanceBehavior-One', name : 'One')
    One, 
    /// 'All' from (id : 'MultiInstanceBehavior-All', name : 'All')
    All, 
    /// 'Complex' from (id : 'MultiInstanceBehavior-Complex', name : 'Complex')
    Complex, 
}

/// Conversion of MultiInstanceLoopCharacteristics (MultiInstanceLoopCharacteristics)
///
/// ```json
/// CMOFClass {
///     xmi_id: "MultiInstanceLoopCharacteristics",
///     name: "MultiInstanceLoopCharacteristics",
///     is_abstract: None,
///     super_class: Some(
///         "LoopCharacteristics",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "MultiInstanceLoopCharacteristics-isSequential",
///                     name: "isSequential",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "false",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "MultiInstanceLoopCharacteristics-behavior",
///                     name: "behavior",
///                     visibility: None,
///                     type: Some(
///                         "MultiInstanceBehavior",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "All",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "MultiInstanceLoopCharacteristics-loopCardinality",
///                     name: "loopCardinality",
///                     visibility: None,
///                     type: Some(
///                         "Expression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_loopCardinality_multiInstanceLoopCharacteristics",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "MultiInstanceLoopCharacteristics-loopDataInputRef",
///                     name: "loopDataInputRef",
///                     visibility: None,
///                     type: Some(
///                         "ItemAwareElement",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_loopDataInputRef_multiInstanceLoopCharacteristics",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "MultiInstanceLoopCharacteristics-loopDataOutputRef",
///                     name: "loopDataOutputRef",
///                     visibility: None,
///                     type: Some(
///                         "ItemAwareElement",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_loopDataOutputRef_multiInstanceLoopCharacteristics",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "MultiInstanceLoopCharacteristics-inputDataItem",
///                     name: "inputDataItem",
///                     visibility: None,
///                     type: Some(
///                         "DataInput",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_inputDataItem_multiInstanceLoopCharacteristics",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "MultiInstanceLoopCharacteristics-outputDataItem",
///                     name: "outputDataItem",
///                     visibility: None,
///                     type: Some(
///                         "DataOutput",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_outputDataItem_multiInstanceLoopCharacteristics",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "MultiInstanceLoopCharacteristics-completionCondition",
///                     name: "completionCondition",
///                     visibility: None,
///                     type: Some(
///                         "Expression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_completionCondition_multiInstanceLoopCharacteristics",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "MultiInstanceLoopCharacteristics-complexBehaviorDefinition",
///                     name: "complexBehaviorDefinition",
///                     visibility: None,
///                     type: Some(
///                         "ComplexBehaviorDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_complexBehaviorDefinition_multiInstanceLoopCharacteristics",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "MultiInstanceLoopCharacteristics-oneBehaviorEventRef",
///                     name: "oneBehaviorEventRef",
///                     visibility: None,
///                     type: Some(
///                         "EventDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_oneBehaviorEventRef_multiInstanceLoopCharacteristics",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "MultiInstanceLoopCharacteristics-noneBehaviorEventRef",
///                     name: "noneBehaviorEventRef",
///                     visibility: None,
///                     type: Some(
///                         "EventDefinition",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_noneBehaviorEventRef_multiInstanceLoopCharacteristics",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct MultiInstanceLoopCharacteristics {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "LoopCharacteristics",
    // RAW | )
    // RAW | super_class_link : None
    pub loop_characteristics : LoopCharacteristics,
}

/// Conversion of StandardLoopCharacteristics (StandardLoopCharacteristics)
///
/// ```json
/// CMOFClass {
///     xmi_id: "StandardLoopCharacteristics",
///     name: "StandardLoopCharacteristics",
///     is_abstract: None,
///     super_class: Some(
///         "LoopCharacteristics",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "StandardLoopCharacteristics-testBefore",
///                     name: "testBefore",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "false",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "StandardLoopCharacteristics-loopCondition",
///                     name: "loopCondition",
///                     visibility: None,
///                     type: Some(
///                         "Expression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_loopCondition_standardLoopCharacteristics",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "StandardLoopCharacteristics-loopMaximum",
///                     name: "loopMaximum",
///                     visibility: None,
///                     type: Some(
///                         "Expression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_loopMaximum_standardLoopCharacteristics",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct StandardLoopCharacteristics {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "LoopCharacteristics",
    // RAW | )
    // RAW | super_class_link : None
    pub loop_characteristics : LoopCharacteristics,
}

/// Conversion of CallActivity (CallActivity)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CallActivity",
///     name: "CallActivity",
///     is_abstract: None,
///     super_class: Some(
///         "Activity",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "CallActivity-calledElementRef",
///                     name: "calledElementRef",
///                     visibility: None,
///                     type: Some(
///                         "CallableElement",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_calledElementRef_callActivity",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct CallActivity {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Activity",
    // RAW | )
    // RAW | super_class_link : None
    pub activity : Activity,
}

/// Conversion of Task (Task)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Task",
///     name: "Task",
///     is_abstract: None,
///     super_class: Some(
///         "Activity InteractionNode",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct Task {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Activity InteractionNode",
    // RAW | )
    // RAW | super_class_link : None
    pub activity : Activity,
    pub interaction_node : InteractionNode,
}

/// Conversion of SendTask (SendTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "SendTask",
///     name: "SendTask",
///     is_abstract: None,
///     super_class: Some(
///         "Task",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "SendTask-implementation",
///                     name: "implementation",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "SendTask-operationRef",
///                     name: "operationRef",
///                     visibility: None,
///                     type: Some(
///                         "Operation",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_operationRef_sendTask",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "SendTask-messageRef",
///                     name: "messageRef",
///                     visibility: None,
///                     type: Some(
///                         "Message",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_messageRef_sendTask",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct SendTask {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Task",
    // RAW | )
    // RAW | super_class_link : None
    pub task : Task,
}

/// Conversion of ReceiveTask (ReceiveTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ReceiveTask",
///     name: "ReceiveTask",
///     is_abstract: None,
///     super_class: Some(
///         "Task",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ReceiveTask-implementation",
///                     name: "implementation",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ReceiveTask-instantiate",
///                     name: "instantiate",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "false",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ReceiveTask-operationRef",
///                     name: "operationRef",
///                     visibility: None,
///                     type: Some(
///                         "Operation",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_operationRef_receiveTask",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ReceiveTask-messageRef",
///                     name: "messageRef",
///                     visibility: None,
///                     type: Some(
///                         "Message",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_messageRef_receiveTask",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ReceiveTask {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Task",
    // RAW | )
    // RAW | super_class_link : None
    pub task : Task,
}

/// Conversion of ScriptTask (ScriptTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ScriptTask",
///     name: "ScriptTask",
///     is_abstract: None,
///     super_class: Some(
///         "Task",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ScriptTask-scriptFormat",
///                     name: "scriptFormat",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ScriptTask-script",
///                     name: "script",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ScriptTask {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Task",
    // RAW | )
    // RAW | super_class_link : None
    pub task : Task,
}

/// Conversion of BusinessRuleTask (BusinessRuleTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BusinessRuleTask",
///     name: "BusinessRuleTask",
///     is_abstract: None,
///     super_class: Some(
///         "Task",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BusinessRuleTask-implementation",
///                     name: "implementation",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct BusinessRuleTask {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "Task",
    // RAW | )
    // RAW | super_class_link : None
    pub task : Task,
}

/// Conversion of AdHocSubProcess (AdHocSubProcess)
///
/// ```json
/// CMOFClass {
///     xmi_id: "AdHocSubProcess",
///     name: "AdHocSubProcess",
///     is_abstract: None,
///     super_class: Some(
///         "SubProcess",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "AdHocSubProcess-completionCondition",
///                     name: "completionCondition",
///                     visibility: None,
///                     type: Some(
///                         "Expression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_completionCondition_adHocSubProcess",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "AdHocSubProcess-ordering",
///                     name: "ordering",
///                     visibility: None,
///                     type: Some(
///                         "AdHocOrdering",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "AdHocSubProcess-cancelRemainingInstances",
///                     name: "cancelRemainingInstances",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "true",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct AdHocSubProcess {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "SubProcess",
    // RAW | )
    // RAW | super_class_link : None
    pub sub_process : SubProcess,
}

/// struct_level : AdHocOrdering
enum AdHocOrdering {
    /// 'Parallel' from (id : 'AdHocOrdering-Parallel', name : 'Parallel')
    Parallel, 
    /// 'Sequential' from (id : 'AdHocOrdering-Sequential', name : 'Sequential')
    Sequential, 
}

/// Conversion of Transaction (Transaction)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Transaction",
///     name: "Transaction",
///     is_abstract: None,
///     super_class: Some(
///         "SubProcess",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Transaction-protocol",
///                     name: "protocol",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Transaction-method",
///                     name: "method",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Transaction {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "SubProcess",
    // RAW | )
    // RAW | super_class_link : None
    pub sub_process : SubProcess,
}

/// Conversion of GlobalScriptTask (GlobalScriptTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "GlobalScriptTask",
///     name: "GlobalScriptTask",
///     is_abstract: None,
///     super_class: Some(
///         "GlobalTask",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "GlobalScriptTask-scriptLanguage",
///                     name: "scriptLanguage",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "GlobalScriptTask-script",
///                     name: "script",
///                     visibility: None,
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct GlobalScriptTask {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "GlobalTask",
    // RAW | )
    // RAW | super_class_link : None
    pub global_task : GlobalTask,
}

/// Conversion of GlobalBusinessRuleTask (GlobalBusinessRuleTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "GlobalBusinessRuleTask",
///     name: "GlobalBusinessRuleTask",
///     is_abstract: None,
///     super_class: Some(
///         "GlobalTask",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "GlobalBusinessRuleTask-implementation",
///                     name: "implementation",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct GlobalBusinessRuleTask {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "GlobalTask",
    // RAW | )
    // RAW | super_class_link : None
    pub global_task : GlobalTask,
}

/// Conversion of ComplexBehaviorDefinition (ComplexBehaviorDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ComplexBehaviorDefinition",
///     name: "ComplexBehaviorDefinition",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ComplexBehaviorDefinition-condition",
///                     name: "condition",
///                     visibility: None,
///                     type: Some(
///                         "FormalExpression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_condition_complexBehaviorDefinition",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ComplexBehaviorDefinition-event",
///                     name: "event",
///                     visibility: None,
///                     type: Some(
///                         "ImplicitThrowEvent",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_event_complexBehaviorDefinition",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ComplexBehaviorDefinition {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

// struct_level : A_complexBehaviorDefinition_multiInstanceLoopCharacteristics

/// Conversion of ResourceRole (ResourceRole)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ResourceRole",
///     name: "ResourceRole",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ResourceRole-resourceRef",
///                     name: "resourceRef",
///                     visibility: None,
///                     type: Some(
///                         "Resource",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_resourceRef_activityResource",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ResourceRole-resourceParameterBindings",
///                     name: "resourceParameterBindings",
///                     visibility: None,
///                     type: Some(
///                         "ResourceParameterBinding",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_resourceParameterBindings_activityResource",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ResourceRole-resourceAssignmentExpression",
///                     name: "resourceAssignmentExpression",
///                     visibility: None,
///                     type: Some(
///                         "ResourceAssignmentExpression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_resourceAssignmentExpression_activityResource",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ResourceRole-name",
///                     name: "name",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ResourceRole {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

/// Conversion of ResourceParameterBinding (ResourceParameterBinding)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ResourceParameterBinding",
///     name: "ResourceParameterBinding",
///     is_abstract: None,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ResourceParameterBinding-expression",
///                     name: "expression",
///                     visibility: None,
///                     type: Some(
///                         "Expression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_expression_resourceParameterBinding",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ResourceParameterBinding-parameterRef",
///                     name: "parameterRef",
///                     visibility: None,
///                     type: Some(
///                         "ResourceParameter",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_parameterRef_resourceParameterBinding",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ResourceParameterBinding {
    // RAW | is_abstract : None
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of ResourceAssignmentExpression (ResourceAssignmentExpression)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ResourceAssignmentExpression",
///     name: "ResourceAssignmentExpression",
///     is_abstract: None,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "ResourceAssignmentExpression-expression",
///                     name: "expression",
///                     visibility: None,
///                     type: Some(
///                         "Expression",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_expression_resourceAssignmentExpression",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct ResourceAssignmentExpression {
    // RAW | is_abstract : None
    // RAW | super_class : None
    // RAW | super_class_link : None
}

// struct_level : A_resourceParameterBindings_activityResource

// struct_level : A_resourceAssignmentExpression_activityResource

// struct_level : A_loopMaximum_standardLoopCharacteristics

// struct_level : A_dataInputAssociations_activity

// struct_level : A_dataOutputAssociations_activity

// struct_level : A_parameterRef_resourceParameterBinding

// struct_level : A_loopDataInputRef_multiInstanceLoopCharacteristics

// struct_level : A_loopDataOutputRef_multiInstanceLoopCharacteristics

// struct_level : A_inputDataItem_multiInstanceLoopCharacteristics

// struct_level : A_outputDataItem_multiInstanceLoopCharacteristics

// struct_level : A_boundaryEventRefs_attachedToRef

// struct_level : A_default_activity

// struct_level : A_artifacts_subProcess

/// Conversion of Import (Import)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Import",
///     name: "Import",
///     is_abstract: None,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Import-importType",
///                     name: "importType",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Import-location",
///                     name: "location",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Import-namespace",
///                     name: "namespace",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Import {
    // RAW | is_abstract : None
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of Definitions (Definitions)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Definitions",
///     name: "Definitions",
///     is_abstract: None,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Definitions-name",
///                     name: "name",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Definitions-targetNamespace",
///                     name: "targetNamespace",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Definitions-expressionLanguage",
///                     name: "expressionLanguage",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "http://www.w3.org/1999/XPath",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Definitions-typeLanguage",
///                     name: "typeLanguage",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: Some(
///                         "http://www.w3.org/2001/XMLSchema",
///                     ),
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Definitions-imports",
///                     name: "imports",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "Import",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_imports_definition",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Definitions-extensions",
///                     name: "extensions",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "Extension",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_extensions_definitions",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Definitions-relationships",
///                     name: "relationships",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "Relationship",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_relationships_definition",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Definitions-rootElements",
///                     name: "rootElements",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "RootElement",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_rootElements_definition",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Definitions-diagrams",
///                     name: "diagrams",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:Class",
///                             href: "BPMNDI.cmof#BPMNDiagram",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_diagrams_definitions",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Definitions-exporter",
///                     name: "exporter",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Definitions-exporterVersion",
///                     name: "exporterVersion",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Definitions {
    // RAW | is_abstract : None
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub base_element : BaseElement,
}

// struct_level : A_diagrams_definitions

// struct_level : A_imports_definition

// struct_level : A_extensions_definitions

// struct_level : A_relationships_definition

// struct_level : A_rootElements_definition

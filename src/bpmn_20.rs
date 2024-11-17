//! bpmn_20
use derive_builder::Builder;

/// Conversion of _packageImport.1 (PackageImport)
use crate::dc;

/// Conversion of Interface (Class : Interface)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Interface",
///     name: "Interface",
///     is_abstract: false,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Interface-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Interface-operations",
///                 name: "operations",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Operation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_operations_interface",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Interface-implementationRef",
///                 name: "implementationRef",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:Class",
///                         href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Interface {
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_root_element : RootElement,
}

/// Conversion of Operation (Class : Operation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Operation",
///     name: "Operation",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Operation-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Operation-inMessageRef",
///                 name: "inMessageRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Message",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_inMessageRef_operation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Operation-outMessageRef",
///                 name: "outMessageRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Message",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_outMessageRef_operation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Operation-errorRefs",
///                 name: "errorRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Error",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_errorRefs_operation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Operation-implementationRef",
///                 name: "implementationRef",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:Class",
///                         href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Operation {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of EndPoint (Class : EndPoint)
///
/// ```json
/// CMOFClass {
///     xmi_id: "EndPoint",
///     name: "EndPoint",
///     is_abstract: false,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct EndPoint {
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_root_element : RootElement,
}

/// Conversion of Auditing (Class : Auditing)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Auditing",
///     name: "Auditing",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct Auditing {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of GlobalTask (Class : GlobalTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "GlobalTask",
///     name: "GlobalTask",
///     is_abstract: false,
///     super_class: Some(
///         "CallableElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "GlobalTask-resources",
///                 name: "resources",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ResourceRole",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_resources_globalTask",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct GlobalTask {
    // RAW | super_class : Some(
    // RAW |     "CallableElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_callable_element : CallableElement,
}

/// Conversion of Monitoring (Class : Monitoring)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Monitoring",
///     name: "Monitoring",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct Monitoring {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of Performer (Class : Performer)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Performer",
///     name: "Performer",
///     is_abstract: false,
///     super_class: Some(
///         "ResourceRole",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct Performer {
    // RAW | super_class : Some(
    // RAW |     "ResourceRole",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_resource_role : ResourceRole,
}

/// Conversion of Process (Class : Process)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Process",
///     name: "Process",
///     is_abstract: false,
///     super_class: Some(
///         "FlowElementsContainer CallableElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Process-processType",
///                 name: "processType",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ProcessType",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Process-isClosed",
///                 name: "isClosed",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Process-auditing",
///                 name: "auditing",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Auditing",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_auditing_process",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Process-monitoring",
///                 name: "monitoring",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Monitoring",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_monitoring_process",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Process-properties",
///                 name: "properties",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Property",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_properties_process",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Process-supports",
///                 name: "supports",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Process",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_supports_process",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Process-definitionalCollaborationRef",
///                 name: "definitionalCollaborationRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Collaboration",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_definitionalCollaborationRef_process",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Process-isExecutable",
///                 name: "isExecutable",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Process-resources",
///                 name: "resources",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ResourceRole",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_resources_process",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Process-artifacts",
///                 name: "artifacts",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Artifact",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_artifacts_process",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Process-correlationSubscriptions",
///                 name: "correlationSubscriptions",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CorrelationSubscription",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_correlationSubscriptions_process",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Process {
    // RAW | super_class : Some(
    // RAW |     "FlowElementsContainer CallableElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_flow_elements_container : FlowElementsContainer,
    pub heritage_callable_element : CallableElement,
}

/// Conversion of ProcessType (Enumeration : ProcessType)
pub enum ProcessType {
    /// 'None' from (id : 'ProcessType-None', name : 'None')
    None, 
    /// 'Public' from (id : 'ProcessType-Public', name : 'Public')
    Public, 
    /// 'Private' from (id : 'ProcessType-Private', name : 'Private')
    Private, 
}

/// Conversion of LaneSet (Class : LaneSet)
///
/// ```json
/// CMOFClass {
///     xmi_id: "LaneSet",
///     name: "LaneSet",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "LaneSet-lanes",
///                 name: "lanes",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Lane",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_lanes_laneSet",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "LaneSet-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct LaneSet {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of Lane (Class : Lane)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Lane",
///     name: "Lane",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Lane-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Lane-childLaneSet",
///                 name: "childLaneSet",
///                 visibility: Public,
///                 simple_type: Some(
///                     "LaneSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_childLaneSet_parentLane",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Lane-partitionElementRef",
///                 name: "partitionElementRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "BaseElement",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_partitionElementRef_lane",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Lane-flowNodeRefs",
///                 name: "flowNodeRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "FlowNode",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_flowNodeRefs_lanes",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Lane-partitionElement",
///                 name: "partitionElement",
///                 visibility: Public,
///                 simple_type: Some(
///                     "BaseElement",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_partitionElement_lane",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Lane {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of GlobalManualTask (Class : GlobalManualTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "GlobalManualTask",
///     name: "GlobalManualTask",
///     is_abstract: false,
///     super_class: Some(
///         "GlobalTask",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct GlobalManualTask {
    // RAW | super_class : Some(
    // RAW |     "GlobalTask",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_global_task : GlobalTask,
}

/// Conversion of ManualTask (Class : ManualTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ManualTask",
///     name: "ManualTask",
///     is_abstract: false,
///     super_class: Some(
///         "Task",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct ManualTask {
    // RAW | super_class : Some(
    // RAW |     "Task",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_task : Task,
}

/// Conversion of UserTask (Class : UserTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "UserTask",
///     name: "UserTask",
///     is_abstract: false,
///     super_class: Some(
///         "Task",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "UserTask-renderings",
///                 name: "renderings",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Rendering",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_renderings_usertask",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "UserTask-implementation",
///                 name: "implementation",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct UserTask {
    // RAW | super_class : Some(
    // RAW |     "Task",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_task : Task,
}

/// Conversion of Rendering (Class : Rendering)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Rendering",
///     name: "Rendering",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct Rendering {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of HumanPerformer (Class : HumanPerformer)
///
/// ```json
/// CMOFClass {
///     xmi_id: "HumanPerformer",
///     name: "HumanPerformer",
///     is_abstract: false,
///     super_class: Some(
///         "Performer",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct HumanPerformer {
    // RAW | super_class : Some(
    // RAW |     "Performer",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_performer : Performer,
}

/// Conversion of PotentialOwner (Class : PotentialOwner)
///
/// ```json
/// CMOFClass {
///     xmi_id: "PotentialOwner",
///     name: "PotentialOwner",
///     is_abstract: false,
///     super_class: Some(
///         "HumanPerformer",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct PotentialOwner {
    // RAW | super_class : Some(
    // RAW |     "HumanPerformer",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_human_performer : HumanPerformer,
}

/// Conversion of GlobalUserTask (Class : GlobalUserTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "GlobalUserTask",
///     name: "GlobalUserTask",
///     is_abstract: false,
///     super_class: Some(
///         "GlobalTask",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "GlobalUserTask-implementation",
///                 name: "implementation",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "GlobalUserTask-renderings",
///                 name: "renderings",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Rendering",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_renderings_globalUserTask",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct GlobalUserTask {
    // RAW | super_class : Some(
    // RAW |     "GlobalTask",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_global_task : GlobalTask,
}

/// Conversion of Gateway (Class : Gateway)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Gateway",
///     name: "Gateway",
///     is_abstract: false,
///     super_class: Some(
///         "FlowNode",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Gateway-gatewayDirection",
///                 name: "gatewayDirection",
///                 visibility: Public,
///                 simple_type: Some(
///                     "GatewayDirection",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "unspecified",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Gateway {
    // RAW | super_class : Some(
    // RAW |     "FlowNode",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_flow_node : FlowNode,
}

/// Conversion of GatewayDirection (Enumeration : GatewayDirection)
pub enum GatewayDirection {
    /// 'Unspecified' from (id : 'GatewayDirection-Unspecified', name : 'Unspecified')
    Unspecified, 
    /// 'Converging' from (id : 'GatewayDirection-Converging', name : 'Converging')
    Converging, 
    /// 'Diverging' from (id : 'GatewayDirection-Diverging', name : 'Diverging')
    Diverging, 
    /// 'Mixed' from (id : 'GatewayDirection-Mixed', name : 'Mixed')
    Mixed, 
}

/// Conversion of EventBasedGateway (Class : EventBasedGateway)
///
/// ```json
/// CMOFClass {
///     xmi_id: "EventBasedGateway",
///     name: "EventBasedGateway",
///     is_abstract: false,
///     super_class: Some(
///         "Gateway",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "EventBasedGateway-instantiate",
///                 name: "instantiate",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "false",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "EventBasedGateway-eventGatewayType",
///                 name: "eventGatewayType",
///                 visibility: Public,
///                 simple_type: Some(
///                     "EventBasedGatewayType",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct EventBasedGateway {
    // RAW | super_class : Some(
    // RAW |     "Gateway",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_gateway : Gateway,
}

/// Conversion of ComplexGateway (Class : ComplexGateway)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ComplexGateway",
///     name: "ComplexGateway",
///     is_abstract: false,
///     super_class: Some(
///         "Gateway",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ComplexGateway-activationCondition",
///                 name: "activationCondition",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Expression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_activationCondition_complexGateway",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ComplexGateway-default",
///                 name: "default",
///                 visibility: Public,
///                 simple_type: Some(
///                     "SequenceFlow",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_default_complexGateway",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ComplexGateway {
    // RAW | super_class : Some(
    // RAW |     "Gateway",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_gateway : Gateway,
}

/// Conversion of ExclusiveGateway (Class : ExclusiveGateway)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ExclusiveGateway",
///     name: "ExclusiveGateway",
///     is_abstract: false,
///     super_class: Some(
///         "Gateway",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ExclusiveGateway-default",
///                 name: "default",
///                 visibility: Public,
///                 simple_type: Some(
///                     "SequenceFlow",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_default_exclusiveGateway",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ExclusiveGateway {
    // RAW | super_class : Some(
    // RAW |     "Gateway",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_gateway : Gateway,
}

/// Conversion of InclusiveGateway (Class : InclusiveGateway)
///
/// ```json
/// CMOFClass {
///     xmi_id: "InclusiveGateway",
///     name: "InclusiveGateway",
///     is_abstract: false,
///     super_class: Some(
///         "Gateway",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "InclusiveGateway-default",
///                 name: "default",
///                 visibility: Public,
///                 simple_type: Some(
///                     "SequenceFlow",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_default_inclusiveGateway",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct InclusiveGateway {
    // RAW | super_class : Some(
    // RAW |     "Gateway",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_gateway : Gateway,
}

/// Conversion of ParallelGateway (Class : ParallelGateway)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ParallelGateway",
///     name: "ParallelGateway",
///     is_abstract: false,
///     super_class: Some(
///         "Gateway",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct ParallelGateway {
    // RAW | super_class : Some(
    // RAW |     "Gateway",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_gateway : Gateway,
}

/// Conversion of EventBasedGatewayType (Enumeration : EventBasedGatewayType)
pub enum EventBasedGatewayType {
    /// 'Parallel' from (id : 'EventBasedGatewayType-Parallel', name : 'Parallel')
    Parallel, 
    /// 'Exclusive' from (id : 'EventBasedGatewayType-Exclusive', name : 'Exclusive')
    Exclusive, 
}

/// Conversion of RootElement (Class : RootElement)
///
/// ```json
/// CMOFClass {
///     xmi_id: "RootElement",
///     name: "RootElement",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct RootElement {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of Relationship (Class : Relationship)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Relationship",
///     name: "Relationship",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Relationship-type",
///                 name: "type",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Relationship-direction",
///                 name: "direction",
///                 visibility: Public,
///                 simple_type: Some(
///                     "RelationshipDirection",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Relationship-sources",
///                 name: "sources",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:Class",
///                         href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_sources_relationship",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Relationship-targets",
///                 name: "targets",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:Class",
///                         href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_targets_relationship",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Relationship {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of BaseElement (Class : BaseElement)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BaseElement",
///     name: "BaseElement",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "BaseElement-id",
///                 name: "id",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "BaseElement-extensionDefinitions",
///                 name: "extensionDefinitions",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ExtensionDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_extensionDefinitions_baseElement",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "BaseElement-extensionValues",
///                 name: "extensionValues",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ExtensionAttributeValue",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_extensionValues_baseElement",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "BaseElement-documentation",
///                 name: "documentation",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Documentation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_documentation_baseElement",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct BaseElement {
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of Extension (Class : Extension)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Extension",
///     name: "Extension",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Extension-mustUnderstand",
///                 name: "mustUnderstand",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "false",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Extension-definition",
///                 name: "definition",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ExtensionDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_definition_extension",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Extension {
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of ExtensionDefinition (Class : ExtensionDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ExtensionDefinition",
///     name: "ExtensionDefinition",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ExtensionDefinition-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ExtensionDefinition-extensionAttributeDefinitions",
///                 name: "extensionAttributeDefinitions",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ExtensionAttributeDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_extensionAttributeDefinitions_extensionDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ExtensionDefinition {
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of ExtensionAttributeDefinition (Class : ExtensionAttributeDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ExtensionAttributeDefinition",
///     name: "ExtensionAttributeDefinition",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ExtensionAttributeDefinition-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ExtensionAttributeDefinition-type",
///                 name: "type",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ExtensionAttributeDefinition-isReference",
///                 name: "isReference",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "false",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ExtensionAttributeDefinition-extensionDefinition",
///                 name: "extensionDefinition",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ExtensionDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_extensionAttributeDefinitions_extensionDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ExtensionAttributeDefinition {
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of ExtensionAttributeValue (Class : ExtensionAttributeValue)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ExtensionAttributeValue",
///     name: "ExtensionAttributeValue",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ExtensionAttributeValue-valueRef",
///                 name: "valueRef",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:Class",
///                         href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_valueRef_extensionAttributeValue",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ExtensionAttributeValue-value",
///                 name: "value",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:Class",
///                         href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_value_extensionAttributeValue",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ExtensionAttributeValue-extensionAttributeDefinition",
///                 name: "extensionAttributeDefinition",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ExtensionAttributeDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_extensionAttributeDefinition_extensionAttributeValue",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ExtensionAttributeValue {
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of RelationshipDirection (Enumeration : RelationshipDirection)
pub enum RelationshipDirection {
    /// 'None' from (id : 'RelationshipDirection-None', name : 'None')
    None, 
    /// 'Forward' from (id : 'RelationshipDirection-Forward', name : 'Forward')
    Forward, 
    /// 'Backward' from (id : 'RelationshipDirection-Backward', name : 'Backward')
    Backward, 
    /// 'Both' from (id : 'RelationshipDirection-Both', name : 'Both')
    Both, 
}

/// Conversion of Documentation (Class : Documentation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Documentation",
///     name: "Documentation",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Documentation-text",
///                 name: "text",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Documentation-textFormat",
///                 name: "textFormat",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "text/plain",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Documentation {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of Event (Class : Event)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Event",
///     name: "Event",
///     is_abstract: false,
///     super_class: Some(
///         "FlowNode InteractionNode",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Event-properties",
///                 name: "properties",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Property",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_properties_event",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Event {
    // RAW | super_class : Some(
    // RAW |     "FlowNode InteractionNode",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_flow_node : FlowNode,
    pub heritage_interaction_node : InteractionNode,
}

/// Conversion of IntermediateCatchEvent (Class : IntermediateCatchEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "IntermediateCatchEvent",
///     name: "IntermediateCatchEvent",
///     is_abstract: false,
///     super_class: Some(
///         "CatchEvent",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct IntermediateCatchEvent {
    // RAW | super_class : Some(
    // RAW |     "CatchEvent",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_catch_event : CatchEvent,
}

/// Conversion of IntermediateThrowEvent (Class : IntermediateThrowEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "IntermediateThrowEvent",
///     name: "IntermediateThrowEvent",
///     is_abstract: false,
///     super_class: Some(
///         "ThrowEvent",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct IntermediateThrowEvent {
    // RAW | super_class : Some(
    // RAW |     "ThrowEvent",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_throw_event : ThrowEvent,
}

/// Conversion of EndEvent (Class : EndEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "EndEvent",
///     name: "EndEvent",
///     is_abstract: false,
///     super_class: Some(
///         "ThrowEvent",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct EndEvent {
    // RAW | super_class : Some(
    // RAW |     "ThrowEvent",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_throw_event : ThrowEvent,
}

/// Conversion of StartEvent (Class : StartEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "StartEvent",
///     name: "StartEvent",
///     is_abstract: false,
///     super_class: Some(
///         "CatchEvent",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "StartEvent-isInterrupting",
///                 name: "isInterrupting",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "true",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct StartEvent {
    // RAW | super_class : Some(
    // RAW |     "CatchEvent",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_catch_event : CatchEvent,
}

/// Conversion of ThrowEvent (Class : ThrowEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ThrowEvent",
///     name: "ThrowEvent",
///     is_abstract: false,
///     super_class: Some(
///         "Event",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ThrowEvent-inputSet",
///                 name: "inputSet",
///                 visibility: Public,
///                 simple_type: Some(
///                     "InputSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_inputSet_throwEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ThrowEvent-eventDefinitionRefs",
///                 name: "eventDefinitionRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "EventDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_eventDefinitionRefs_throwEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ThrowEvent-dataInputAssociation",
///                 name: "dataInputAssociation",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataInputAssociation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataInputAssociation_throwEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ThrowEvent-dataInputs",
///                 name: "dataInputs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataInput",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataInputs_throwEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ThrowEvent-eventDefinitions",
///                 name: "eventDefinitions",
///                 visibility: Public,
///                 simple_type: Some(
///                     "EventDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_eventDefinitions_throwEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ThrowEvent {
    // RAW | super_class : Some(
    // RAW |     "Event",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_event : Event,
}

/// Conversion of CatchEvent (Class : CatchEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CatchEvent",
///     name: "CatchEvent",
///     is_abstract: false,
///     super_class: Some(
///         "Event",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CatchEvent-parallelMultiple",
///                 name: "parallelMultiple",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CatchEvent-outputSet",
///                 name: "outputSet",
///                 visibility: Public,
///                 simple_type: Some(
///                     "OutputSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_outputSet_catchEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CatchEvent-eventDefinitionRefs",
///                 name: "eventDefinitionRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "EventDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_eventDefinitionRefs_catchEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CatchEvent-dataOutputAssociation",
///                 name: "dataOutputAssociation",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataOutputAssociation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataOutputAssociation_catchEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CatchEvent-dataOutputs",
///                 name: "dataOutputs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataOutput",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataOutputs_catchEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CatchEvent-eventDefinitions",
///                 name: "eventDefinitions",
///                 visibility: Public,
///                 simple_type: Some(
///                     "EventDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_eventDefinitions_catchEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct CatchEvent {
    // RAW | super_class : Some(
    // RAW |     "Event",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_event : Event,
}

/// Conversion of BoundaryEvent (Class : BoundaryEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BoundaryEvent",
///     name: "BoundaryEvent",
///     is_abstract: false,
///     super_class: Some(
///         "CatchEvent",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "BoundaryEvent-cancelActivity",
///                 name: "cancelActivity",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "true",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "BoundaryEvent-attachedToRef",
///                 name: "attachedToRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Activity",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_boundaryEventRefs_attachedToRef",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct BoundaryEvent {
    // RAW | super_class : Some(
    // RAW |     "CatchEvent",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_catch_event : CatchEvent,
}

/// Conversion of EventDefinition (Class : EventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "EventDefinition",
///     name: "EventDefinition",
///     is_abstract: false,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct EventDefinition {
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_root_element : RootElement,
}

/// Conversion of CancelEventDefinition (Class : CancelEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CancelEventDefinition",
///     name: "CancelEventDefinition",
///     is_abstract: false,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct CancelEventDefinition {
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_event_definition : EventDefinition,
}

/// Conversion of ErrorEventDefinition (Class : ErrorEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ErrorEventDefinition",
///     name: "ErrorEventDefinition",
///     is_abstract: false,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ErrorEventDefinition-errorRef",
///                 name: "errorRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Error",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_errorRef_errorEventDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ErrorEventDefinition {
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_event_definition : EventDefinition,
}

/// Conversion of TerminateEventDefinition (Class : TerminateEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "TerminateEventDefinition",
///     name: "TerminateEventDefinition",
///     is_abstract: false,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct TerminateEventDefinition {
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_event_definition : EventDefinition,
}

/// Conversion of EscalationEventDefinition (Class : EscalationEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "EscalationEventDefinition",
///     name: "EscalationEventDefinition",
///     is_abstract: false,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "EscalationEventDefinition-escalationRef",
///                 name: "escalationRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Escalation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_escalationRef_escalationEventDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct EscalationEventDefinition {
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_event_definition : EventDefinition,
}

/// Conversion of Escalation (Class : Escalation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Escalation",
///     name: "Escalation",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Escalation-structureRef",
///                 name: "structureRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ItemDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_structureRef_escalation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Escalation-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Escalation-escalationCode",
///                 name: "escalationCode",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Escalation {
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of CompensateEventDefinition (Class : CompensateEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CompensateEventDefinition",
///     name: "CompensateEventDefinition",
///     is_abstract: false,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CompensateEventDefinition-waitForCompletion",
///                 name: "waitForCompletion",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CompensateEventDefinition-activityRef",
///                 name: "activityRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Activity",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_activityRef_compensateEventDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct CompensateEventDefinition {
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_event_definition : EventDefinition,
}

/// Conversion of TimerEventDefinition (Class : TimerEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "TimerEventDefinition",
///     name: "TimerEventDefinition",
///     is_abstract: false,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "TimerEventDefinition-timeDate",
///                 name: "timeDate",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Expression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_timeDate_timerEventDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "TimerEventDefinition-timeCycle",
///                 name: "timeCycle",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Expression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_timeCycle_timerEventDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "TimerEventDefinition-timeDuration",
///                 name: "timeDuration",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Expression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_timeDuration_timerEventDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct TimerEventDefinition {
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_event_definition : EventDefinition,
}

/// Conversion of LinkEventDefinition (Class : LinkEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "LinkEventDefinition",
///     name: "LinkEventDefinition",
///     is_abstract: false,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "LinkEventDefinition-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "LinkEventDefinition-target",
///                 name: "target",
///                 visibility: Public,
///                 simple_type: Some(
///                     "LinkEventDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_target_source",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "LinkEventDefinition-source",
///                 name: "source",
///                 visibility: Public,
///                 simple_type: Some(
///                     "LinkEventDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_target_source",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct LinkEventDefinition {
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_event_definition : EventDefinition,
}

/// Conversion of MessageEventDefinition (Class : MessageEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "MessageEventDefinition",
///     name: "MessageEventDefinition",
///     is_abstract: false,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "MessageEventDefinition-messageRef",
///                 name: "messageRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Message",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_messageRef_messageEventDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "MessageEventDefinition-operationRef",
///                 name: "operationRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Operation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_operationRef_messageEventDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct MessageEventDefinition {
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_event_definition : EventDefinition,
}

/// Conversion of ConditionalEventDefinition (Class : ConditionalEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ConditionalEventDefinition",
///     name: "ConditionalEventDefinition",
///     is_abstract: false,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ConditionalEventDefinition-condition",
///                 name: "condition",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Expression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_condition_conditionalEventDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ConditionalEventDefinition {
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_event_definition : EventDefinition,
}

/// Conversion of SignalEventDefinition (Class : SignalEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "SignalEventDefinition",
///     name: "SignalEventDefinition",
///     is_abstract: false,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "SignalEventDefinition-signalRef",
///                 name: "signalRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Signal",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_signalRef_signalEventDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct SignalEventDefinition {
    // RAW | super_class : Some(
    // RAW |     "EventDefinition",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_event_definition : EventDefinition,
}

/// Conversion of Signal (Class : Signal)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Signal",
///     name: "Signal",
///     is_abstract: false,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Signal-structureRef",
///                 name: "structureRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ItemDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_structureRef_signal",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Signal-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Signal {
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_root_element : RootElement,
}

/// Conversion of ImplicitThrowEvent (Class : ImplicitThrowEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ImplicitThrowEvent",
///     name: "ImplicitThrowEvent",
///     is_abstract: false,
///     super_class: Some(
///         "ThrowEvent",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct ImplicitThrowEvent {
    // RAW | super_class : Some(
    // RAW |     "ThrowEvent",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_throw_event : ThrowEvent,
}

/// Conversion of DataState (Class : DataState)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataState",
///     name: "DataState",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataState-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct DataState {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of ItemAwareElement (Class : ItemAwareElement)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ItemAwareElement",
///     name: "ItemAwareElement",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ItemAwareElement-itemSubjectRef",
///                 name: "itemSubjectRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ItemDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_itemSubjectRef_itemAwareElement",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ItemAwareElement-dataState",
///                 name: "dataState",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataState",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataState_itemAwareElement",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ItemAwareElement {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of DataAssociation (Class : DataAssociation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataAssociation",
///     name: "DataAssociation",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataAssociation-transformation",
///                 name: "transformation",
///                 visibility: Public,
///                 simple_type: Some(
///                     "FormalExpression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_transformation_dataAssociation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataAssociation-assignment",
///                 name: "assignment",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Assignment",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_assignment_dataAssociation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataAssociation-targetRef",
///                 name: "targetRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ItemAwareElement",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_targetRef_dataAssociation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataAssociation-sourceRef",
///                 name: "sourceRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ItemAwareElement",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_sourceRef_dataAssociation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct DataAssociation {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of DataInput (Class : DataInput)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataInput",
///     name: "DataInput",
///     is_abstract: false,
///     super_class: Some(
///         "ItemAwareElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataInput-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataInput-isCollection",
///                 name: "isCollection",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "false",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataInput-inputSetRefs",
///                 name: "inputSetRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "InputSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataInputRefs_inputSetRefs",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataInput-inputSetWithOptional",
///                 name: "inputSetWithOptional",
///                 visibility: Public,
///                 simple_type: Some(
///                     "InputSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_optionalInputRefs_inputSetWithOptional",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataInput-inputSetWithWhileExecuting",
///                 name: "inputSetWithWhileExecuting",
///                 visibility: Public,
///                 simple_type: Some(
///                     "InputSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_whileExecutingInputRefs_inputSetWithWhileExecuting",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct DataInput {
    // RAW | super_class : Some(
    // RAW |     "ItemAwareElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_item_aware_element : ItemAwareElement,
}

/// Conversion of DataOutput (Class : DataOutput)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataOutput",
///     name: "DataOutput",
///     is_abstract: false,
///     super_class: Some(
///         "ItemAwareElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataOutput-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataOutput-isCollection",
///                 name: "isCollection",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "false",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataOutput-outputSetRefs",
///                 name: "outputSetRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "OutputSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataOutputRefs_outputSetRefs",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataOutput-outputSetWithOptional",
///                 name: "outputSetWithOptional",
///                 visibility: Public,
///                 simple_type: Some(
///                     "OutputSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_outputSetWithOptional_optionalOutputRefs",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataOutput-outputSetWithWhileExecuting",
///                 name: "outputSetWithWhileExecuting",
///                 visibility: Public,
///                 simple_type: Some(
///                     "OutputSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_outputSetWithWhileExecuting_whileExecutingOutputRefs",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct DataOutput {
    // RAW | super_class : Some(
    // RAW |     "ItemAwareElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_item_aware_element : ItemAwareElement,
}

/// Conversion of InputSet (Class : InputSet)
///
/// ```json
/// CMOFClass {
///     xmi_id: "InputSet",
///     name: "InputSet",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "InputSet-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "InputSet-dataInputRefs",
///                 name: "dataInputRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataInput",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataInputRefs_inputSetRefs",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "InputSet-optionalInputRefs",
///                 name: "optionalInputRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataInput",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_optionalInputRefs_inputSetWithOptional",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "InputSet-whileExecutingInputRefs",
///                 name: "whileExecutingInputRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataInput",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_whileExecutingInputRefs_inputSetWithWhileExecuting",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "InputSet-outputSetRefs",
///                 name: "outputSetRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "OutputSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_inputSetRefs_outputSetRefs",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct InputSet {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of OutputSet (Class : OutputSet)
///
/// ```json
/// CMOFClass {
///     xmi_id: "OutputSet",
///     name: "OutputSet",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "OutputSet-dataOutputRefs",
///                 name: "dataOutputRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataOutput",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataOutputRefs_outputSetRefs",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "OutputSet-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "OutputSet-inputSetRefs",
///                 name: "inputSetRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "InputSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_inputSetRefs_outputSetRefs",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "OutputSet-optionalOutputRefs",
///                 name: "optionalOutputRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataOutput",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_outputSetWithOptional_optionalOutputRefs",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "OutputSet-whileExecutingOutputRefs",
///                 name: "whileExecutingOutputRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataOutput",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_outputSetWithWhileExecuting_whileExecutingOutputRefs",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct OutputSet {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of Property (Class : Property)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Property",
///     name: "Property",
///     is_abstract: false,
///     super_class: Some(
///         "ItemAwareElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Property-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Property {
    // RAW | super_class : Some(
    // RAW |     "ItemAwareElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_item_aware_element : ItemAwareElement,
}

/// Conversion of DataInputAssociation (Class : DataInputAssociation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataInputAssociation",
///     name: "DataInputAssociation",
///     is_abstract: false,
///     super_class: Some(
///         "DataAssociation",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct DataInputAssociation {
    // RAW | super_class : Some(
    // RAW |     "DataAssociation",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_data_association : DataAssociation,
}

/// Conversion of DataOutputAssociation (Class : DataOutputAssociation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataOutputAssociation",
///     name: "DataOutputAssociation",
///     is_abstract: false,
///     super_class: Some(
///         "DataAssociation",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct DataOutputAssociation {
    // RAW | super_class : Some(
    // RAW |     "DataAssociation",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_data_association : DataAssociation,
}

/// Conversion of InputOutputSpecification (Class : InputOutputSpecification)
///
/// ```json
/// CMOFClass {
///     xmi_id: "InputOutputSpecification",
///     name: "InputOutputSpecification",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "InputOutputSpecification-inputSets",
///                 name: "inputSets",
///                 visibility: Public,
///                 simple_type: Some(
///                     "InputSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_inputSets_inputOutputSpecification",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "InputOutputSpecification-outputSets",
///                 name: "outputSets",
///                 visibility: Public,
///                 simple_type: Some(
///                     "OutputSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_outputSets_inputOutputSpecification",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "InputOutputSpecification-dataInputs",
///                 name: "dataInputs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataInput",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataInputs_inputOutputSpecification",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "InputOutputSpecification-dataOutputs",
///                 name: "dataOutputs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataOutput",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataOutputs_inputOutputSpecification",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct InputOutputSpecification {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of DataObject (Class : DataObject)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataObject",
///     name: "DataObject",
///     is_abstract: false,
///     super_class: Some(
///         "FlowElement ItemAwareElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataObject-isCollection",
///                 name: "isCollection",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "false",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct DataObject {
    // RAW | super_class : Some(
    // RAW |     "FlowElement ItemAwareElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_flow_element : FlowElement,
    pub heritage_item_aware_element : ItemAwareElement,
}

/// Conversion of InputOutputBinding (Class : InputOutputBinding)
///
/// ```json
/// CMOFClass {
///     xmi_id: "InputOutputBinding",
///     name: "InputOutputBinding",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "InputOutputBinding-inputDataRef",
///                 name: "inputDataRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "InputSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_inputDataRef_inputOutputBinding",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "InputOutputBinding-outputDataRef",
///                 name: "outputDataRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "OutputSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_outputDataRef_inputOutputBinding",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "InputOutputBinding-operationRef",
///                 name: "operationRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Operation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_operationRef_ioBinding",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct InputOutputBinding {
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of Assignment (Class : Assignment)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Assignment",
///     name: "Assignment",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Assignment-from",
///                 name: "from",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Expression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_from_assignment",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Assignment-to",
///                 name: "to",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Expression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_to_assignment",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Assignment {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of DataStore (Class : DataStore)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataStore",
///     name: "DataStore",
///     is_abstract: false,
///     super_class: Some(
///         "RootElement ItemAwareElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataStore-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataStore-capacity",
///                 name: "capacity",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Integer",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataStore-isUnlimited",
///                 name: "isUnlimited",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "true",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct DataStore {
    // RAW | super_class : Some(
    // RAW |     "RootElement ItemAwareElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_root_element : RootElement,
    pub heritage_item_aware_element : ItemAwareElement,
}

/// Conversion of DataStoreReference (Class : DataStoreReference)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataStoreReference",
///     name: "DataStoreReference",
///     is_abstract: false,
///     super_class: Some(
///         "ItemAwareElement FlowElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataStoreReference-dataStoreRef",
///                 name: "dataStoreRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataStore",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataStoreRef_dataStoreReference",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct DataStoreReference {
    // RAW | super_class : Some(
    // RAW |     "ItemAwareElement FlowElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_item_aware_element : ItemAwareElement,
    pub heritage_flow_element : FlowElement,
}

/// Conversion of DataObjectReference (Class : DataObjectReference)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataObjectReference",
///     name: "DataObjectReference",
///     is_abstract: false,
///     super_class: Some(
///         "ItemAwareElement FlowElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataObjectReference-dataObjectRef",
///                 name: "dataObjectRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataObject",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataObjectRef_dataObject",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct DataObjectReference {
    // RAW | super_class : Some(
    // RAW |     "ItemAwareElement FlowElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_item_aware_element : ItemAwareElement,
    pub heritage_flow_element : FlowElement,
}

/// Conversion of ConversationLink (Class : ConversationLink)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ConversationLink",
///     name: "ConversationLink",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ConversationLink-sourceRef",
///                 name: "sourceRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "InteractionNode",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_sourceRef_outgoingConversationLinks",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ConversationLink-targetRef",
///                 name: "targetRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "InteractionNode",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_targetRef_incomingConversationLinks",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ConversationLink-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ConversationLink {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of ConversationAssociation (Class : ConversationAssociation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ConversationAssociation",
///     name: "ConversationAssociation",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ConversationAssociation-innerConversationNodeRef",
///                 name: "innerConversationNodeRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ConversationNode",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_innerConversationNodeRef_conversationAssociation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ConversationAssociation-outerConversationNodeRef",
///                 name: "outerConversationNodeRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ConversationNode",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_outerConversationNodeRef_conversationAssociation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ConversationAssociation {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of CallConversation (Class : CallConversation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CallConversation",
///     name: "CallConversation",
///     is_abstract: false,
///     super_class: Some(
///         "ConversationNode",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CallConversation-calledCollaborationRef",
///                 name: "calledCollaborationRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Collaboration",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_calledCollaborationRef_callConversation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CallConversation-participantAssociations",
///                 name: "participantAssociations",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ParticipantAssociation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_participantAssociations_callConversation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct CallConversation {
    // RAW | super_class : Some(
    // RAW |     "ConversationNode",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_conversation_node : ConversationNode,
}

/// Conversion of Conversation (Class : Conversation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Conversation",
///     name: "Conversation",
///     is_abstract: false,
///     super_class: Some(
///         "ConversationNode",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct Conversation {
    // RAW | super_class : Some(
    // RAW |     "ConversationNode",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_conversation_node : ConversationNode,
}

/// Conversion of SubConversation (Class : SubConversation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "SubConversation",
///     name: "SubConversation",
///     is_abstract: false,
///     super_class: Some(
///         "ConversationNode",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "SubConversation-conversationNodes",
///                 name: "conversationNodes",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ConversationNode",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_conversationNodes_subConversation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct SubConversation {
    // RAW | super_class : Some(
    // RAW |     "ConversationNode",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_conversation_node : ConversationNode,
}

/// Conversion of ConversationNode (Class : ConversationNode)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ConversationNode",
///     name: "ConversationNode",
///     is_abstract: false,
///     super_class: Some(
///         "InteractionNode BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ConversationNode-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ConversationNode-participantRefs",
///                 name: "participantRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Participant",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 2,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_participantRefs_conversationNode",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ConversationNode-messageFlowRefs",
///                 name: "messageFlowRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "MessageFlow",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_messageFlowRefs_communication",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ConversationNode-correlationKeys",
///                 name: "correlationKeys",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CorrelationKey",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_correlationKeys_conversationNode",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ConversationNode {
    // RAW | super_class : Some(
    // RAW |     "InteractionNode BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_interaction_node : InteractionNode,
    pub heritage_base_element : BaseElement,
}

/// Conversion of GlobalConversation (Class : GlobalConversation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "GlobalConversation",
///     name: "GlobalConversation",
///     is_abstract: false,
///     super_class: Some(
///         "Collaboration",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct GlobalConversation {
    // RAW | super_class : Some(
    // RAW |     "Collaboration",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_collaboration : Collaboration,
}

/// Conversion of PartnerEntity (Class : PartnerEntity)
///
/// ```json
/// CMOFClass {
///     xmi_id: "PartnerEntity",
///     name: "PartnerEntity",
///     is_abstract: false,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "PartnerEntity-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "PartnerEntity-participantRef",
///                 name: "participantRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Participant",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_partnerEntityRef_participantRef",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct PartnerEntity {
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_root_element : RootElement,
}

/// Conversion of PartnerRole (Class : PartnerRole)
///
/// ```json
/// CMOFClass {
///     xmi_id: "PartnerRole",
///     name: "PartnerRole",
///     is_abstract: false,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "PartnerRole-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "PartnerRole-participantRef",
///                 name: "participantRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Participant",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_partnerRoleRef_participantRef",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct PartnerRole {
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_root_element : RootElement,
}

/// Conversion of CorrelationProperty (Class : CorrelationProperty)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CorrelationProperty",
///     name: "CorrelationProperty",
///     is_abstract: false,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CorrelationProperty-correlationPropertyRetrievalExpression",
///                 name: "correlationPropertyRetrievalExpression",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CorrelationPropertyRetrievalExpression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_correlationPropertyRetrievalExpression_correlationproperty",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CorrelationProperty-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CorrelationProperty-type",
///                 name: "type",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ItemDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_type_correlationProperty",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct CorrelationProperty {
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_root_element : RootElement,
}

/// Conversion of ItemKind (Enumeration : ItemKind)
pub enum ItemKind {
    /// 'Physical' from (id : 'ItemKind-Physical', name : 'Physical')
    Physical, 
    /// 'Information' from (id : 'ItemKind-Information', name : 'Information')
    Information, 
}

/// Conversion of Error (Class : Error)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Error",
///     name: "Error",
///     is_abstract: false,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Error-structureRef",
///                 name: "structureRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ItemDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_structureRef_error",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Error-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Error-errorCode",
///                 name: "errorCode",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Error {
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_root_element : RootElement,
}

/// Conversion of CorrelationKey (Class : CorrelationKey)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CorrelationKey",
///     name: "CorrelationKey",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CorrelationKey-correlationPropertyRef",
///                 name: "correlationPropertyRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CorrelationProperty",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_correlationPropertyRef_correlationKey",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CorrelationKey-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct CorrelationKey {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of Expression (Class : Expression)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Expression",
///     name: "Expression",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct Expression {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of FormalExpression (Class : FormalExpression)
///
/// ```json
/// CMOFClass {
///     xmi_id: "FormalExpression",
///     name: "FormalExpression",
///     is_abstract: false,
///     super_class: Some(
///         "Expression",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "FormalExpression-language",
///                 name: "language",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "FormalExpression-body",
///                 name: "body",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:Class",
///                         href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "FormalExpression-evaluatesToTypeRef",
///                 name: "evaluatesToTypeRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ItemDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_evaluatesToTypeRef_formalExpression",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct FormalExpression {
    // RAW | super_class : Some(
    // RAW |     "Expression",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_expression : Expression,
}

/// Conversion of Message (Class : Message)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Message",
///     name: "Message",
///     is_abstract: false,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Message-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Message-itemRef",
///                 name: "itemRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ItemDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_itemRef_message",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Message {
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_root_element : RootElement,
}

/// Conversion of ItemDefinition (Class : ItemDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ItemDefinition",
///     name: "ItemDefinition",
///     is_abstract: false,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ItemDefinition-itemKind",
///                 name: "itemKind",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ItemKind",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ItemDefinition-structureRef",
///                 name: "structureRef",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:Class",
///                         href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ItemDefinition-isCollection",
///                 name: "isCollection",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "false",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ItemDefinition-import",
///                 name: "import",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Import",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_import_itemDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ItemDefinition {
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_root_element : RootElement,
}

/// Conversion of FlowElement (Class : FlowElement)
///
/// ```json
/// CMOFClass {
///     xmi_id: "FlowElement",
///     name: "FlowElement",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "FlowElement-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "FlowElement-auditing",
///                 name: "auditing",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Auditing",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_auditing_flowElement",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "FlowElement-monitoring",
///                 name: "monitoring",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Monitoring",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_monitoring_flowElement",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "FlowElement-categoryValueRef",
///                 name: "categoryValueRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CategoryValue",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_categorizedFlowElements_categoryValueRef",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct FlowElement {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of SequenceFlow (Class : SequenceFlow)
///
/// ```json
/// CMOFClass {
///     xmi_id: "SequenceFlow",
///     name: "SequenceFlow",
///     is_abstract: false,
///     super_class: Some(
///         "FlowElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "SequenceFlow-isImmediate",
///                 name: "isImmediate",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "SequenceFlow-conditionExpression",
///                 name: "conditionExpression",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Expression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_conditionExpression_sequenceFlow",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "SequenceFlow-sourceRef",
///                 name: "sourceRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "FlowNode",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_sourceRef_outgoing_flow",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "SequenceFlow-targetRef",
///                 name: "targetRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "FlowNode",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_targetRef_incoming_flow",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct SequenceFlow {
    // RAW | super_class : Some(
    // RAW |     "FlowElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_flow_element : FlowElement,
}

/// Conversion of FlowElementsContainer (Class : FlowElementsContainer)
///
/// ```json
/// CMOFClass {
///     xmi_id: "FlowElementsContainer",
///     name: "FlowElementsContainer",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "FlowElementsContainer-flowElements",
///                 name: "flowElements",
///                 visibility: Public,
///                 simple_type: Some(
///                     "FlowElement",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_flowElements_container",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "FlowElementsContainer-laneSets",
///                 name: "laneSets",
///                 visibility: Public,
///                 simple_type: Some(
///                     "LaneSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_laneSets_flowElementsContainer",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct FlowElementsContainer {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of CallableElement (Class : CallableElement)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CallableElement",
///     name: "CallableElement",
///     is_abstract: false,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CallableElement-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CallableElement-ioSpecification",
///                 name: "ioSpecification",
///                 visibility: Public,
///                 simple_type: Some(
///                     "InputOutputSpecification",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_ioSpecification_callableElement",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CallableElement-supportedInterfaceRefs",
///                 name: "supportedInterfaceRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Interface",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_supportedInterfaceRefs_callableElements",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CallableElement-ioBinding",
///                 name: "ioBinding",
///                 visibility: Public,
///                 simple_type: Some(
///                     "InputOutputBinding",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_ioBinding_callableElement",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct CallableElement {
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_root_element : RootElement,
}

/// Conversion of FlowNode (Class : FlowNode)
///
/// ```json
/// CMOFClass {
///     xmi_id: "FlowNode",
///     name: "FlowNode",
///     is_abstract: false,
///     super_class: Some(
///         "FlowElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "FlowNode-outgoing",
///                 name: "outgoing",
///                 visibility: Public,
///                 simple_type: Some(
///                     "SequenceFlow",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_sourceRef_outgoing_flow",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "FlowNode-incoming",
///                 name: "incoming",
///                 visibility: Public,
///                 simple_type: Some(
///                     "SequenceFlow",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_targetRef_incoming_flow",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "FlowNode-lanes",
///                 name: "lanes",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Lane",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_flowNodeRefs_lanes",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct FlowNode {
    // RAW | super_class : Some(
    // RAW |     "FlowElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_flow_element : FlowElement,
}

/// Conversion of CorrelationPropertyRetrievalExpression (Class : CorrelationPropertyRetrievalExpression)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CorrelationPropertyRetrievalExpression",
///     name: "CorrelationPropertyRetrievalExpression",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CorrelationPropertyRetrievalExpression-messagePath",
///                 name: "messagePath",
///                 visibility: Public,
///                 simple_type: Some(
///                     "FormalExpression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_messagePath_correlationset",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CorrelationPropertyRetrievalExpression-messageRef",
///                 name: "messageRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Message",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_messageRef_correlationPropertyRetrievalExpression",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct CorrelationPropertyRetrievalExpression {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of CorrelationPropertyBinding (Class : CorrelationPropertyBinding)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CorrelationPropertyBinding",
///     name: "CorrelationPropertyBinding",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CorrelationPropertyBinding-dataPath",
///                 name: "dataPath",
///                 visibility: Public,
///                 simple_type: Some(
///                     "FormalExpression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataPath_correlationPropertyBinding",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CorrelationPropertyBinding-correlationPropertyRef",
///                 name: "correlationPropertyRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CorrelationProperty",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_correlationPropertyRef_correlationPropertyBinding",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct CorrelationPropertyBinding {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of Resource (Class : Resource)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Resource",
///     name: "Resource",
///     is_abstract: false,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Resource-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Resource-resourceParameters",
///                 name: "resourceParameters",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ResourceParameter",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_resourceParameters_resource",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Resource {
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_root_element : RootElement,
}

/// Conversion of ResourceParameter (Class : ResourceParameter)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ResourceParameter",
///     name: "ResourceParameter",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ResourceParameter-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ResourceParameter-isRequired",
///                 name: "isRequired",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ResourceParameter-type",
///                 name: "type",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ItemDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_type_resourceParameter",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ResourceParameter {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of CorrelationSubscription (Class : CorrelationSubscription)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CorrelationSubscription",
///     name: "CorrelationSubscription",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CorrelationSubscription-correlationKeyRef",
///                 name: "correlationKeyRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CorrelationKey",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_correlationKeyRef_correlationSubscription",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CorrelationSubscription-correlationPropertyBinding",
///                 name: "correlationPropertyBinding",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CorrelationPropertyBinding",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_correlationPropertyBinding_correlationSubscription",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct CorrelationSubscription {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of MessageFlow (Class : MessageFlow)
///
/// ```json
/// CMOFClass {
///     xmi_id: "MessageFlow",
///     name: "MessageFlow",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "MessageFlow-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "MessageFlow-sourceRef",
///                 name: "sourceRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "InteractionNode",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_sourceRef_messageFlow",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "MessageFlow-targetRef",
///                 name: "targetRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "InteractionNode",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_targetRef_messageFlow",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "MessageFlow-messageRef",
///                 name: "messageRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Message",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_messageRef_messageFlow",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct MessageFlow {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of MessageFlowAssociation (Class : MessageFlowAssociation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "MessageFlowAssociation",
///     name: "MessageFlowAssociation",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "MessageFlowAssociation-innerMessageFlowRef",
///                 name: "innerMessageFlowRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "MessageFlow",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_innerMessageFlowRef_messageFlowAssociation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "MessageFlowAssociation-outerMessageFlowRef",
///                 name: "outerMessageFlowRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "MessageFlow",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_outerMessageFlowRef_messageFlowAssociation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct MessageFlowAssociation {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of InteractionNode (Class : InteractionNode)
///
/// ```json
/// CMOFClass {
///     xmi_id: "InteractionNode",
///     name: "InteractionNode",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "InteractionNode-incomingConversationLinks",
///                 name: "incomingConversationLinks",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ConversationLink",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_targetRef_incomingConversationLinks",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "InteractionNode-outgoingConversationLinks",
///                 name: "outgoingConversationLinks",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ConversationLink",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_sourceRef_outgoingConversationLinks",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct InteractionNode {
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of Participant (Class : Participant)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Participant",
///     name: "Participant",
///     is_abstract: false,
///     super_class: Some(
///         "InteractionNode BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Participant-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Participant-interfaceRefs",
///                 name: "interfaceRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Interface",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_interfaceRefs_participant",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Participant-participantMultiplicity",
///                 name: "participantMultiplicity",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ParticipantMultiplicity",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_participantMultiplicity_participant",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Participant-endPointRefs",
///                 name: "endPointRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "EndPoint",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_endPointRefs_participant",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Participant-processRef",
///                 name: "processRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Process",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_processRef_participant",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Participant {
    // RAW | super_class : Some(
    // RAW |     "InteractionNode BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_interaction_node : InteractionNode,
    pub heritage_base_element : BaseElement,
}

/// Conversion of ParticipantAssociation (Class : ParticipantAssociation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ParticipantAssociation",
///     name: "ParticipantAssociation",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ParticipantAssociation-innerParticipantRef",
///                 name: "innerParticipantRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Participant",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_innerParticipantRef_participantAssociation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ParticipantAssociation-outerParticipantRef",
///                 name: "outerParticipantRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Participant",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_outerParticipantRef_participantAssociation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ParticipantAssociation {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of ParticipantMultiplicity (Class : ParticipantMultiplicity)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ParticipantMultiplicity",
///     name: "ParticipantMultiplicity",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ParticipantMultiplicity-minimum",
///                 name: "minimum",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Integer",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "0",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ParticipantMultiplicity-maximum",
///                 name: "maximum",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Integer",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "1",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ParticipantMultiplicity {
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of Collaboration (Class : Collaboration)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Collaboration",
///     name: "Collaboration",
///     is_abstract: false,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Collaboration-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Collaboration-isClosed",
///                 name: "isClosed",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Collaboration-choreographyRef",
///                 name: "choreographyRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Choreography",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_choreographyRef_collaboration",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Collaboration-artifacts",
///                 name: "artifacts",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Artifact",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_artifacts_collaboration",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Collaboration-participantAssociations",
///                 name: "participantAssociations",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ParticipantAssociation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_participantAssociations_collaboration",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Collaboration-messageFlowAssociations",
///                 name: "messageFlowAssociations",
///                 visibility: Public,
///                 simple_type: Some(
///                     "MessageFlowAssociation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_messageFlowAssociations_collaboration",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Collaboration-conversationAssociations",
///                 name: "conversationAssociations",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ConversationAssociation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_conversationAssociations_converstaionAssociations",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Collaboration-participants",
///                 name: "participants",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Participant",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_participants_collaboration",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Collaboration-messageFlows",
///                 name: "messageFlows",
///                 visibility: Public,
///                 simple_type: Some(
///                     "MessageFlow",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_messageFlows_collaboration",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Collaboration-correlationKeys",
///                 name: "correlationKeys",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CorrelationKey",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_correlationKeys_collaboration",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Collaboration-conversations",
///                 name: "conversations",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ConversationNode",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_conversations_collaboration",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Collaboration-conversationLinks",
///                 name: "conversationLinks",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ConversationLink",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_conversationLinks_collaboration",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Collaboration {
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_root_element : RootElement,
}

/// Conversion of ChoreographyActivity (Class : ChoreographyActivity)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ChoreographyActivity",
///     name: "ChoreographyActivity",
///     is_abstract: false,
///     super_class: Some(
///         "FlowNode",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ChoreographyActivity-participantRefs",
///                 name: "participantRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Participant",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 2,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_participantRefs_choreographyActivity",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ChoreographyActivity-initiatingParticipantRef",
///                 name: "initiatingParticipantRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Participant",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_initiatingParticipantRef_choreographyActivity",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ChoreographyActivity-correlationKeys",
///                 name: "correlationKeys",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CorrelationKey",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_correlationKeys_choreographyActivity",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ChoreographyActivity-loopType",
///                 name: "loopType",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ChoreographyLoopType",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "None",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ChoreographyActivity {
    // RAW | super_class : Some(
    // RAW |     "FlowNode",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_flow_node : FlowNode,
}

/// Conversion of CallChoreography (Class : CallChoreography)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CallChoreography",
///     name: "CallChoreography",
///     is_abstract: false,
///     super_class: Some(
///         "ChoreographyActivity",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CallChoreography-calledChoreographyRef",
///                 name: "calledChoreographyRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Choreography",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_calledChoreographyRef_callChoreographyActivity",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CallChoreography-participantAssociations",
///                 name: "participantAssociations",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ParticipantAssociation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_participantAssociations_callChoreographyActivity",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct CallChoreography {
    // RAW | super_class : Some(
    // RAW |     "ChoreographyActivity",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_choreography_activity : ChoreographyActivity,
}

/// Conversion of SubChoreography (Class : SubChoreography)
///
/// ```json
/// CMOFClass {
///     xmi_id: "SubChoreography",
///     name: "SubChoreography",
///     is_abstract: false,
///     super_class: Some(
///         "ChoreographyActivity FlowElementsContainer",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "SubChoreography-artifacts",
///                 name: "artifacts",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Artifact",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_artifacts_subChoreography",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct SubChoreography {
    // RAW | super_class : Some(
    // RAW |     "ChoreographyActivity FlowElementsContainer",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_choreography_activity : ChoreographyActivity,
    pub heritage_flow_elements_container : FlowElementsContainer,
}

/// Conversion of ChoreographyTask (Class : ChoreographyTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ChoreographyTask",
///     name: "ChoreographyTask",
///     is_abstract: false,
///     super_class: Some(
///         "ChoreographyActivity",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ChoreographyTask-messageFlowRef",
///                 name: "messageFlowRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "MessageFlow",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     2,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_messageFlowRef_choreographyTask",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ChoreographyTask {
    // RAW | super_class : Some(
    // RAW |     "ChoreographyActivity",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_choreography_activity : ChoreographyActivity,
}

/// Conversion of ChoreographyLoopType (Enumeration : ChoreographyLoopType)
pub enum ChoreographyLoopType {
    /// 'None' from (id : 'ChoreographyLoopType-None', name : 'None')
    None, 
    /// 'Standard' from (id : 'ChoreographyLoopType-Standard', name : 'Standard')
    Standard, 
    /// 'MultiInstanceSequential' from (id : 'ChoreographyLoopType-MultiInstanceSequential', name : 'MultiInstanceSequential')
    MultiInstanceSequential, 
    /// 'MultiInstanceParallel' from (id : 'ChoreographyLoopType-MultiInstanceParallel', name : 'MultiInstanceParallel')
    MultiInstanceParallel, 
}

/// Conversion of Choreography (Class : Choreography)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Choreography",
///     name: "Choreography",
///     is_abstract: false,
///     super_class: Some(
///         "FlowElementsContainer Collaboration",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct Choreography {
    // RAW | super_class : Some(
    // RAW |     "FlowElementsContainer Collaboration",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_flow_elements_container : FlowElementsContainer,
    pub heritage_collaboration : Collaboration,
}

/// Conversion of GlobalChoreographyTask (Class : GlobalChoreographyTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "GlobalChoreographyTask",
///     name: "GlobalChoreographyTask",
///     is_abstract: false,
///     super_class: Some(
///         "Choreography",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "GlobalChoreographyTask-initiatingParticipantRef",
///                 name: "initiatingParticipantRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Participant",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_initiatingParticipantRef_globalChoreographyTask",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct GlobalChoreographyTask {
    // RAW | super_class : Some(
    // RAW |     "Choreography",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_choreography : Choreography,
}

/// Conversion of TextAnnotation (Class : TextAnnotation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "TextAnnotation",
///     name: "TextAnnotation",
///     is_abstract: false,
///     super_class: Some(
///         "Artifact",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "TextAnnotation-text",
///                 name: "text",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "TextAnnotation-textFormat",
///                 name: "textFormat",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "text/plain",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct TextAnnotation {
    // RAW | super_class : Some(
    // RAW |     "Artifact",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_artifact : Artifact,
}

/// Conversion of Group (Class : Group)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Group",
///     name: "Group",
///     is_abstract: false,
///     super_class: Some(
///         "Artifact",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Group-categoryValueRef",
///                 name: "categoryValueRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CategoryValue",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_categoryValueRef_categoryValueRef",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Group {
    // RAW | super_class : Some(
    // RAW |     "Artifact",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_artifact : Artifact,
}

/// Conversion of Association (Class : Association)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Association",
///     name: "Association",
///     is_abstract: false,
///     super_class: Some(
///         "Artifact",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Association-associationDirection",
///                 name: "associationDirection",
///                 visibility: Public,
///                 simple_type: Some(
///                     "AssociationDirection",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Association-sourceRef",
///                 name: "sourceRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "BaseElement",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_sourceRef_outgoing_association",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Association-targetRef",
///                 name: "targetRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "BaseElement",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_targetRef_incoming_association",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Association {
    // RAW | super_class : Some(
    // RAW |     "Artifact",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_artifact : Artifact,
}

/// Conversion of Category (Class : Category)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Category",
///     name: "Category",
///     is_abstract: false,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Category-categoryValue",
///                 name: "categoryValue",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CategoryValue",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_categoryValue_category",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Category-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Category {
    // RAW | super_class : Some(
    // RAW |     "RootElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_root_element : RootElement,
}

/// Conversion of Artifact (Class : Artifact)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Artifact",
///     name: "Artifact",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct Artifact {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of AssociationDirection (Enumeration : AssociationDirection)
pub enum AssociationDirection {
    /// 'None' from (id : 'AssociationDirection-None', name : 'None')
    None, 
    /// 'One' from (id : 'AssociationDirection-One', name : 'One')
    One, 
    /// 'Both' from (id : 'AssociationDirection-Both', name : 'Both')
    Both, 
}

/// Conversion of CategoryValue (Class : CategoryValue)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CategoryValue",
///     name: "CategoryValue",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CategoryValue-categorizedFlowElements",
///                 name: "categorizedFlowElements",
///                 visibility: Public,
///                 simple_type: Some(
///                     "FlowElement",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_categorizedFlowElements_categoryValueRef",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CategoryValue-value",
///                 name: "value",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct CategoryValue {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of Activity (Class : Activity)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Activity",
///     name: "Activity",
///     is_abstract: false,
///     super_class: Some(
///         "FlowNode",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Activity-isForCompensation",
///                 name: "isForCompensation",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "false",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Activity-loopCharacteristics",
///                 name: "loopCharacteristics",
///                 visibility: Public,
///                 simple_type: Some(
///                     "LoopCharacteristics",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_loopCharacteristics_activity",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Activity-resources",
///                 name: "resources",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ResourceRole",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_resources_activity",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Activity-default",
///                 name: "default",
///                 visibility: Public,
///                 simple_type: Some(
///                     "SequenceFlow",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_default_activity",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Activity-properties",
///                 name: "properties",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Property",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_properties_activity",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Activity-ioSpecification",
///                 name: "ioSpecification",
///                 visibility: Public,
///                 simple_type: Some(
///                     "InputOutputSpecification",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_ioSpecification_activity",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Activity-boundaryEventRefs",
///                 name: "boundaryEventRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "BoundaryEvent",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_boundaryEventRefs_attachedToRef",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Activity-dataInputAssociations",
///                 name: "dataInputAssociations",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataInputAssociation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataInputAssociations_activity",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Activity-dataOutputAssociations",
///                 name: "dataOutputAssociations",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataOutputAssociation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataOutputAssociations_activity",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Activity-startQuantity",
///                 name: "startQuantity",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Integer",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "1",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Activity-completionQuantity",
///                 name: "completionQuantity",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Integer",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "1",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Activity {
    // RAW | super_class : Some(
    // RAW |     "FlowNode",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_flow_node : FlowNode,
}

/// Conversion of ServiceTask (Class : ServiceTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ServiceTask",
///     name: "ServiceTask",
///     is_abstract: false,
///     super_class: Some(
///         "Task",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ServiceTask-implementation",
///                 name: "implementation",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ServiceTask-operationRef",
///                 name: "operationRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Operation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_operationRef_serviceTask",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ServiceTask {
    // RAW | super_class : Some(
    // RAW |     "Task",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_task : Task,
}

/// Conversion of SubProcess (Class : SubProcess)
///
/// ```json
/// CMOFClass {
///     xmi_id: "SubProcess",
///     name: "SubProcess",
///     is_abstract: false,
///     super_class: Some(
///         "Activity FlowElementsContainer",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "SubProcess-triggeredByEvent",
///                 name: "triggeredByEvent",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "false",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "SubProcess-artifacts",
///                 name: "artifacts",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Artifact",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_artifacts_subProcess",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct SubProcess {
    // RAW | super_class : Some(
    // RAW |     "Activity FlowElementsContainer",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_activity : Activity,
    pub heritage_flow_elements_container : FlowElementsContainer,
}

/// Conversion of LoopCharacteristics (Class : LoopCharacteristics)
///
/// ```json
/// CMOFClass {
///     xmi_id: "LoopCharacteristics",
///     name: "LoopCharacteristics",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct LoopCharacteristics {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of MultiInstanceBehavior (Enumeration : MultiInstanceBehavior)
pub enum MultiInstanceBehavior {
    /// 'None' from (id : 'MultiInstanceBehavior-None', name : 'None')
    None, 
    /// 'One' from (id : 'MultiInstanceBehavior-One', name : 'One')
    One, 
    /// 'All' from (id : 'MultiInstanceBehavior-All', name : 'All')
    All, 
    /// 'Complex' from (id : 'MultiInstanceBehavior-Complex', name : 'Complex')
    Complex, 
}

/// Conversion of MultiInstanceLoopCharacteristics (Class : MultiInstanceLoopCharacteristics)
///
/// ```json
/// CMOFClass {
///     xmi_id: "MultiInstanceLoopCharacteristics",
///     name: "MultiInstanceLoopCharacteristics",
///     is_abstract: false,
///     super_class: Some(
///         "LoopCharacteristics",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "MultiInstanceLoopCharacteristics-isSequential",
///                 name: "isSequential",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "false",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "MultiInstanceLoopCharacteristics-behavior",
///                 name: "behavior",
///                 visibility: Public,
///                 simple_type: Some(
///                     "MultiInstanceBehavior",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "All",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "MultiInstanceLoopCharacteristics-loopCardinality",
///                 name: "loopCardinality",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Expression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_loopCardinality_multiInstanceLoopCharacteristics",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "MultiInstanceLoopCharacteristics-loopDataInputRef",
///                 name: "loopDataInputRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ItemAwareElement",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_loopDataInputRef_multiInstanceLoopCharacteristics",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "MultiInstanceLoopCharacteristics-loopDataOutputRef",
///                 name: "loopDataOutputRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ItemAwareElement",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_loopDataOutputRef_multiInstanceLoopCharacteristics",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "MultiInstanceLoopCharacteristics-inputDataItem",
///                 name: "inputDataItem",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataInput",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_inputDataItem_multiInstanceLoopCharacteristics",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "MultiInstanceLoopCharacteristics-outputDataItem",
///                 name: "outputDataItem",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataOutput",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_outputDataItem_multiInstanceLoopCharacteristics",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "MultiInstanceLoopCharacteristics-completionCondition",
///                 name: "completionCondition",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Expression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_completionCondition_multiInstanceLoopCharacteristics",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "MultiInstanceLoopCharacteristics-complexBehaviorDefinition",
///                 name: "complexBehaviorDefinition",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ComplexBehaviorDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_complexBehaviorDefinition_multiInstanceLoopCharacteristics",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "MultiInstanceLoopCharacteristics-oneBehaviorEventRef",
///                 name: "oneBehaviorEventRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "EventDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_oneBehaviorEventRef_multiInstanceLoopCharacteristics",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "MultiInstanceLoopCharacteristics-noneBehaviorEventRef",
///                 name: "noneBehaviorEventRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "EventDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_noneBehaviorEventRef_multiInstanceLoopCharacteristics",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct MultiInstanceLoopCharacteristics {
    // RAW | super_class : Some(
    // RAW |     "LoopCharacteristics",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_loop_characteristics : LoopCharacteristics,
}

/// Conversion of StandardLoopCharacteristics (Class : StandardLoopCharacteristics)
///
/// ```json
/// CMOFClass {
///     xmi_id: "StandardLoopCharacteristics",
///     name: "StandardLoopCharacteristics",
///     is_abstract: false,
///     super_class: Some(
///         "LoopCharacteristics",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "StandardLoopCharacteristics-testBefore",
///                 name: "testBefore",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "false",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "StandardLoopCharacteristics-loopCondition",
///                 name: "loopCondition",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Expression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_loopCondition_standardLoopCharacteristics",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "StandardLoopCharacteristics-loopMaximum",
///                 name: "loopMaximum",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Expression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_loopMaximum_standardLoopCharacteristics",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct StandardLoopCharacteristics {
    // RAW | super_class : Some(
    // RAW |     "LoopCharacteristics",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_loop_characteristics : LoopCharacteristics,
}

/// Conversion of CallActivity (Class : CallActivity)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CallActivity",
///     name: "CallActivity",
///     is_abstract: false,
///     super_class: Some(
///         "Activity",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CallActivity-calledElementRef",
///                 name: "calledElementRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CallableElement",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_calledElementRef_callActivity",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct CallActivity {
    // RAW | super_class : Some(
    // RAW |     "Activity",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_activity : Activity,
}

/// Conversion of Task (Class : Task)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Task",
///     name: "Task",
///     is_abstract: false,
///     super_class: Some(
///         "Activity InteractionNode",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct Task {
    // RAW | super_class : Some(
    // RAW |     "Activity InteractionNode",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_activity : Activity,
    pub heritage_interaction_node : InteractionNode,
}

/// Conversion of SendTask (Class : SendTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "SendTask",
///     name: "SendTask",
///     is_abstract: false,
///     super_class: Some(
///         "Task",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "SendTask-implementation",
///                 name: "implementation",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "SendTask-operationRef",
///                 name: "operationRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Operation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_operationRef_sendTask",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "SendTask-messageRef",
///                 name: "messageRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Message",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_messageRef_sendTask",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct SendTask {
    // RAW | super_class : Some(
    // RAW |     "Task",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_task : Task,
}

/// Conversion of ReceiveTask (Class : ReceiveTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ReceiveTask",
///     name: "ReceiveTask",
///     is_abstract: false,
///     super_class: Some(
///         "Task",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ReceiveTask-implementation",
///                 name: "implementation",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ReceiveTask-instantiate",
///                 name: "instantiate",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "false",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ReceiveTask-operationRef",
///                 name: "operationRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Operation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_operationRef_receiveTask",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ReceiveTask-messageRef",
///                 name: "messageRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Message",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_messageRef_receiveTask",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ReceiveTask {
    // RAW | super_class : Some(
    // RAW |     "Task",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_task : Task,
}

/// Conversion of ScriptTask (Class : ScriptTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ScriptTask",
///     name: "ScriptTask",
///     is_abstract: false,
///     super_class: Some(
///         "Task",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ScriptTask-scriptFormat",
///                 name: "scriptFormat",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ScriptTask-script",
///                 name: "script",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ScriptTask {
    // RAW | super_class : Some(
    // RAW |     "Task",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_task : Task,
}

/// Conversion of BusinessRuleTask (Class : BusinessRuleTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BusinessRuleTask",
///     name: "BusinessRuleTask",
///     is_abstract: false,
///     super_class: Some(
///         "Task",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "BusinessRuleTask-implementation",
///                 name: "implementation",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct BusinessRuleTask {
    // RAW | super_class : Some(
    // RAW |     "Task",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_task : Task,
}

/// Conversion of AdHocSubProcess (Class : AdHocSubProcess)
///
/// ```json
/// CMOFClass {
///     xmi_id: "AdHocSubProcess",
///     name: "AdHocSubProcess",
///     is_abstract: false,
///     super_class: Some(
///         "SubProcess",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "AdHocSubProcess-completionCondition",
///                 name: "completionCondition",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Expression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_completionCondition_adHocSubProcess",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "AdHocSubProcess-ordering",
///                 name: "ordering",
///                 visibility: Public,
///                 simple_type: Some(
///                     "AdHocOrdering",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "AdHocSubProcess-cancelRemainingInstances",
///                 name: "cancelRemainingInstances",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "true",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct AdHocSubProcess {
    // RAW | super_class : Some(
    // RAW |     "SubProcess",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_sub_process : SubProcess,
}

/// Conversion of AdHocOrdering (Enumeration : AdHocOrdering)
pub enum AdHocOrdering {
    /// 'Parallel' from (id : 'AdHocOrdering-Parallel', name : 'Parallel')
    Parallel, 
    /// 'Sequential' from (id : 'AdHocOrdering-Sequential', name : 'Sequential')
    Sequential, 
}

/// Conversion of Transaction (Class : Transaction)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Transaction",
///     name: "Transaction",
///     is_abstract: false,
///     super_class: Some(
///         "SubProcess",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Transaction-protocol",
///                 name: "protocol",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Transaction-method",
///                 name: "method",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Transaction {
    // RAW | super_class : Some(
    // RAW |     "SubProcess",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_sub_process : SubProcess,
}

/// Conversion of GlobalScriptTask (Class : GlobalScriptTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "GlobalScriptTask",
///     name: "GlobalScriptTask",
///     is_abstract: false,
///     super_class: Some(
///         "GlobalTask",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "GlobalScriptTask-scriptLanguage",
///                 name: "scriptLanguage",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "GlobalScriptTask-script",
///                 name: "script",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct GlobalScriptTask {
    // RAW | super_class : Some(
    // RAW |     "GlobalTask",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_global_task : GlobalTask,
}

/// Conversion of GlobalBusinessRuleTask (Class : GlobalBusinessRuleTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "GlobalBusinessRuleTask",
///     name: "GlobalBusinessRuleTask",
///     is_abstract: false,
///     super_class: Some(
///         "GlobalTask",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "GlobalBusinessRuleTask-implementation",
///                 name: "implementation",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct GlobalBusinessRuleTask {
    // RAW | super_class : Some(
    // RAW |     "GlobalTask",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_global_task : GlobalTask,
}

/// Conversion of ComplexBehaviorDefinition (Class : ComplexBehaviorDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ComplexBehaviorDefinition",
///     name: "ComplexBehaviorDefinition",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ComplexBehaviorDefinition-condition",
///                 name: "condition",
///                 visibility: Public,
///                 simple_type: Some(
///                     "FormalExpression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_condition_complexBehaviorDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ComplexBehaviorDefinition-event",
///                 name: "event",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ImplicitThrowEvent",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_event_complexBehaviorDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ComplexBehaviorDefinition {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of ResourceRole (Class : ResourceRole)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ResourceRole",
///     name: "ResourceRole",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ResourceRole-resourceRef",
///                 name: "resourceRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Resource",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_resourceRef_activityResource",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ResourceRole-resourceParameterBindings",
///                 name: "resourceParameterBindings",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ResourceParameterBinding",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_resourceParameterBindings_activityResource",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ResourceRole-resourceAssignmentExpression",
///                 name: "resourceAssignmentExpression",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ResourceAssignmentExpression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_resourceAssignmentExpression_activityResource",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ResourceRole-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ResourceRole {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

/// Conversion of ResourceParameterBinding (Class : ResourceParameterBinding)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ResourceParameterBinding",
///     name: "ResourceParameterBinding",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ResourceParameterBinding-expression",
///                 name: "expression",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Expression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_expression_resourceParameterBinding",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ResourceParameterBinding-parameterRef",
///                 name: "parameterRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ResourceParameter",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_parameterRef_resourceParameterBinding",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ResourceParameterBinding {
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of ResourceAssignmentExpression (Class : ResourceAssignmentExpression)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ResourceAssignmentExpression",
///     name: "ResourceAssignmentExpression",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ResourceAssignmentExpression-expression",
///                 name: "expression",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Expression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_expression_resourceAssignmentExpression",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct ResourceAssignmentExpression {
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of Import (Class : Import)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Import",
///     name: "Import",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Import-importType",
///                 name: "importType",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Import-location",
///                 name: "location",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Import-namespace",
///                 name: "namespace",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Import {
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of Definitions (Class : Definitions)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Definitions",
///     name: "Definitions",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Definitions-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Definitions-targetNamespace",
///                 name: "targetNamespace",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Definitions-expressionLanguage",
///                 name: "expressionLanguage",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "http://www.w3.org/1999/XPath",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Definitions-typeLanguage",
///                 name: "typeLanguage",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "http://www.w3.org/2001/XMLSchema",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Definitions-imports",
///                 name: "imports",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Import",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_imports_definition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Definitions-extensions",
///                 name: "extensions",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Extension",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_extensions_definitions",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Definitions-relationships",
///                 name: "relationships",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Relationship",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_relationships_definition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Definitions-rootElements",
///                 name: "rootElements",
///                 visibility: Public,
///                 simple_type: Some(
///                     "RootElement",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_rootElements_definition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Definitions-diagrams",
///                 name: "diagrams",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:Class",
///                         href: "BPMNDI.cmof#BPMNDiagram",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_diagrams_definitions",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Definitions-exporter",
///                 name: "exporter",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Definitions-exporterVersion",
///                 name: "exporterVersion",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Definitions {
    // RAW | super_class : Some(
    // RAW |     "BaseElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_base_element : BaseElement,
}

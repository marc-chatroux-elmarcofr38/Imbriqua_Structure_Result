
/// Conversion of Gateway (Class : Gateway)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Gateway",
///     name: "Gateway",
///     is_abstract: true,
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

#[derive(Builder, Debug, Clone)]
pub struct Gateway<'a> {
    #[builder(setter(into), default = "GatewayDirection::Unspecified")]
    pub gateway_direction: &'a GatewayDirection<'a>,
}


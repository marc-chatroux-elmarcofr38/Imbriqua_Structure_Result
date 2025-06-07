//! bpmndi

use derive_builder::Builder;
/// Link from _packageImport.0 (PackageImport)
use crate::di;
/// Link from _packageImport.1 (PackageImport)
use crate::dc;
/// Link from _packageImport.2 (PackageImport)
use crate::bpmn_20;

/// Class : BPMNDiagram
pub mod BPMNDiagram;

/// Class : BPMNPlane
pub mod BPMNPlane;

/// Class : BPMNShape
pub mod BPMNShape;

/// Class : BPMNEdge
pub mod BPMNEdge;

/// Class : BPMNLabel
pub mod BPMNLabel;

/// Class : BPMNLabelStyle
pub mod BPMNLabelStyle;

/// Enumeration : ParticipantBandKind
pub mod ParticipantBandKind;

/// Enumeration : MessageVisibleKind
pub mod MessageVisibleKind;

/// Association : A_plane_diagram
pub mod A_plane_diagram;

/// Association : A_bpmnElement_edge
pub mod A_bpmnElement_edge;

/// Association : A_bpmnElement_shape
pub mod A_bpmnElement_shape;

/// Association : A_bpmnElement_plane
pub mod A_bpmnElement_plane;

/// Association : A_label_edge
pub mod A_label_edge;

/// Association : A_label_shape
pub mod A_label_shape;

/// Association : A_labelStyle_label
pub mod A_labelStyle_label;

/// Association : A_sourceElement_sourceEdge
pub mod A_sourceElement_sourceEdge;

/// Association : A_targetElement_targetEdge
pub mod A_targetElement_targetEdge;

/// Association : A_labelStyle_diagram
pub mod A_labelStyle_diagram;

/// Association : A_choreographyActivityShape_participantBandShape
pub mod A_choreographyActivityShape_participantBandShape;

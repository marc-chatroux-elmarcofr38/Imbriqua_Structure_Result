//! bpmn_20

/// Conversion of _packageImport.1 (PackageImport)
use crate::dc;

/// Conversion of ProcessType (Enumeration : ProcessType)
pub enum ProcessType {
    /// 'None' from (id : 'ProcessType-None', name : 'None')
    None, 
    /// 'Public' from (id : 'ProcessType-Public', name : 'Public')
    Public, 
    /// 'Private' from (id : 'ProcessType-Private', name : 'Private')
    Private, 
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

/// Conversion of EventBasedGatewayType (Enumeration : EventBasedGatewayType)
pub enum EventBasedGatewayType {
    /// 'Parallel' from (id : 'EventBasedGatewayType-Parallel', name : 'Parallel')
    Parallel, 
    /// 'Exclusive' from (id : 'EventBasedGatewayType-Exclusive', name : 'Exclusive')
    Exclusive, 
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

/// Conversion of ItemKind (Enumeration : ItemKind)
pub enum ItemKind {
    /// 'Physical' from (id : 'ItemKind-Physical', name : 'Physical')
    Physical, 
    /// 'Information' from (id : 'ItemKind-Information', name : 'Information')
    Information, 
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

/// Conversion of AssociationDirection (Enumeration : AssociationDirection)
pub enum AssociationDirection {
    /// 'None' from (id : 'AssociationDirection-None', name : 'None')
    None, 
    /// 'One' from (id : 'AssociationDirection-One', name : 'One')
    One, 
    /// 'Both' from (id : 'AssociationDirection-Both', name : 'Both')
    Both, 
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

/// Conversion of AdHocOrdering (Enumeration : AdHocOrdering)
pub enum AdHocOrdering {
    /// 'Parallel' from (id : 'AdHocOrdering-Parallel', name : 'Parallel')
    Parallel, 
    /// 'Sequential' from (id : 'AdHocOrdering-Sequential', name : 'Sequential')
    Sequential, 
}

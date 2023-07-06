/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.40
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpandedPermissionTree {
    /// The children of the node, possibly none.
    #[serde(rename = "children", skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<crate::models::ExpandedPermissionTree>>,
    #[serde(rename = "tuple", skip_serializing_if = "Option::is_none")]
    pub tuple: Option<Box<crate::models::Relationship>>,
    /// The type of the node. union TreeNodeUnion exclusion TreeNodeExclusion intersection TreeNodeIntersection leaf TreeNodeLeaf tuple_to_subject_set TreeNodeTupleToSubjectSet computed_subject_set TreeNodeComputedSubjectSet not TreeNodeNot unspecified TreeNodeUnspecified
    #[serde(rename = "type")]
    pub _type: TypeEnum,
}


impl ExpandedPermissionTree {
    pub fn new(_type: TypeEnum) -> ExpandedPermissionTree {
        ExpandedPermissionTree {
                children: None,
                tuple: None,
                _type,
        }
    }
}

/// The type of the node. union TreeNodeUnion exclusion TreeNodeExclusion intersection TreeNodeIntersection leaf TreeNodeLeaf tuple_to_subject_set TreeNodeTupleToSubjectSet computed_subject_set TreeNodeComputedSubjectSet not TreeNodeNot unspecified TreeNodeUnspecified
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "union")]
    Union,
    #[serde(rename = "exclusion")]
    Exclusion,
    #[serde(rename = "intersection")]
    Intersection,
    #[serde(rename = "leaf")]
    Leaf,
    #[serde(rename = "tuple_to_subject_set")]
    TupleToSubjectSet,
    #[serde(rename = "computed_subject_set")]
    ComputedSubjectSet,
    #[serde(rename = "not")]
    Not,
    #[serde(rename = "unspecified")]
    Unspecified,
}


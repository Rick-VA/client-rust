/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.46
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// CreateProjectBody : Create Project Request Body



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateProjectBody {
    /// The name of the project to be created
    #[serde(rename = "name")]
    pub name: String,
}


impl CreateProjectBody {
    /// Create Project Request Body
    pub fn new(name: String) -> CreateProjectBody {
        CreateProjectBody {
                name,
        }
    }
}



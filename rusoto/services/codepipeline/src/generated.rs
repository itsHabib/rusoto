// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;
use std::io;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>Represents an AWS session credentials object. These credentials are temporary credentials that are issued by AWS Secure Token Service (STS). They can be used to access input and output artifacts in the Amazon S3 bucket used to store artifact for the pipeline in AWS CodePipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AWSSessionCredentials {
    /// <p>The access key for the session.</p>
    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,
    /// <p>The secret access key for the session.</p>
    #[serde(rename = "secretAccessKey")]
    pub secret_access_key: String,
    /// <p>The token for the session.</p>
    #[serde(rename = "sessionToken")]
    pub session_token: String,
}

/// <p>Represents the input of an AcknowledgeJob action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AcknowledgeJobInput {
    /// <p>The unique system-generated ID of the job for which you want to confirm receipt.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>A system-generated random number that AWS CodePipeline uses to ensure that the job is being worked on by only one job worker. Get this number from the response of the <a>PollForJobs</a> request that returned this job.</p>
    #[serde(rename = "nonce")]
    pub nonce: String,
}

/// <p>Represents the output of an AcknowledgeJob action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AcknowledgeJobOutput {
    /// <p>Whether the job worker has received the specified job.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Represents the input of an AcknowledgeThirdPartyJob action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AcknowledgeThirdPartyJobInput {
    /// <p>The clientToken portion of the clientId and clientToken pair used to verify that the calling entity is allowed access to the job and its details.</p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The unique system-generated ID of the job.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>A system-generated random number that AWS CodePipeline uses to ensure that the job is being worked on by only one job worker. Get this number from the response to a <a>GetThirdPartyJobDetails</a> request.</p>
    #[serde(rename = "nonce")]
    pub nonce: String,
}

/// <p>Represents the output of an AcknowledgeThirdPartyJob action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AcknowledgeThirdPartyJobOutput {
    /// <p>The status information for the third party job, if any.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Represents information about an action configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ActionConfiguration {
    /// <p>The configuration data for the action.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Represents information about an action configuration property.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionConfigurationProperty {
    /// <p>The description of the action configuration property that will be displayed to users.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Whether the configuration property is a key.</p>
    #[serde(rename = "key")]
    pub key: bool,
    /// <p>The name of the action configuration property.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Indicates that the property will be used in conjunction with PollForJobs. When creating a custom action, an action can have up to one queryable property. If it has one, that property must be both required and not secret.</p> <p>If you create a pipeline with a custom action type, and that custom action contains a queryable property, the value for that configuration property is subject to additional restrictions. The value must be less than or equal to twenty (20) characters. The value can contain only alphanumeric characters, underscores, and hyphens.</p>
    #[serde(rename = "queryable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queryable: Option<bool>,
    /// <p>Whether the configuration property is a required value.</p>
    #[serde(rename = "required")]
    pub required: bool,
    /// <p>Whether the configuration property is secret. Secrets are hidden from all calls except for GetJobDetails, GetThirdPartyJobDetails, PollForJobs, and PollForThirdPartyJobs.</p> <p>When updating a pipeline, passing * * * * * without changing any other values of the action will preserve the prior value of the secret.</p>
    #[serde(rename = "secret")]
    pub secret: bool,
    /// <p>The type of the configuration property.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Represents the context of an action within the stage of a pipeline to a job worker.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ActionContext {
    /// <p>The name of the action within the context of a job.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Represents information about an action declaration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionDeclaration {
    /// <p>The configuration information for the action type.</p>
    #[serde(rename = "actionTypeId")]
    pub action_type_id: ActionTypeId,
    /// <p>The action declaration's configuration.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name or ID of the artifact consumed by the action, such as a test or build artifact.</p>
    #[serde(rename = "inputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<Vec<InputArtifact>>,
    /// <p>The action declaration's name.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The name or ID of the result of the action declaration, such as a test or build artifact.</p>
    #[serde(rename = "outputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_artifacts: Option<Vec<OutputArtifact>>,
    /// <p>The action declaration's AWS Region, such as us-east-1.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The ARN of the IAM service role that will perform the declared action. This is assumed through the roleArn for the pipeline.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The order in which actions are run.</p>
    #[serde(rename = "runOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_order: Option<i64>,
}

/// <p>Represents information about the run of an action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ActionExecution {
    /// <p>The details of an error returned by a URL external to AWS.</p>
    #[serde(rename = "errorDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ErrorDetails>,
    /// <p>The external ID of the run of the action.</p>
    #[serde(rename = "externalExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_id: Option<String>,
    /// <p>The URL of a resource external to AWS that will be used when running the action, for example an external repository URL.</p>
    #[serde(rename = "externalExecutionUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_url: Option<String>,
    /// <p>The last status change of the action.</p>
    #[serde(rename = "lastStatusChange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_change: Option<f64>,
    /// <p>The ARN of the user who last changed the pipeline.</p>
    #[serde(rename = "lastUpdatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<String>,
    /// <p>A percentage of completeness of the action as it runs.</p>
    #[serde(rename = "percentComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i64>,
    /// <p>The status of the action, or for a completed action, the last status of the action.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A summary of the run of the action.</p>
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// <p>The system-generated token used to identify a unique approval request. The token for each open approval request can be obtained using the GetPipelineState command and is used to validate that the approval request corresponding to this token is still valid.</p>
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// <p>Represents information about the version (or revision) of an action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionRevision {
    /// <p>The date and time when the most recent version of the action was created, in timestamp format.</p>
    #[serde(rename = "created")]
    pub created: f64,
    /// <p>The unique identifier of the change that set the state to this revision, for example a deployment ID or timestamp.</p>
    #[serde(rename = "revisionChangeId")]
    pub revision_change_id: String,
    /// <p>The system-generated unique ID that identifies the revision number of the action.</p>
    #[serde(rename = "revisionId")]
    pub revision_id: String,
}

/// <p>Represents information about the state of an action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ActionState {
    /// <p>The name of the action.</p>
    #[serde(rename = "actionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    /// <p>Represents information about the version (or revision) of an action.</p>
    #[serde(rename = "currentRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision: Option<ActionRevision>,
    /// <p>A URL link for more information about the state of the action, such as a deployment group details page.</p>
    #[serde(rename = "entityUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_url: Option<String>,
    /// <p>Represents information about the run of an action.</p>
    #[serde(rename = "latestExecution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_execution: Option<ActionExecution>,
    /// <p>A URL link for more information about the revision, such as a commit details page.</p>
    #[serde(rename = "revisionUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_url: Option<String>,
}

/// <p>Returns information about the details of an action type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ActionType {
    /// <p>The configuration properties for the action type.</p>
    #[serde(rename = "actionConfigurationProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_configuration_properties: Option<Vec<ActionConfigurationProperty>>,
    /// <p>Represents information about an action type.</p>
    #[serde(rename = "id")]
    pub id: ActionTypeId,
    /// <p>The details of the input artifact for the action, such as its commit ID.</p>
    #[serde(rename = "inputArtifactDetails")]
    pub input_artifact_details: ArtifactDetails,
    /// <p>The details of the output artifact of the action, such as its commit ID.</p>
    #[serde(rename = "outputArtifactDetails")]
    pub output_artifact_details: ArtifactDetails,
    /// <p>The settings for the action type.</p>
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<ActionTypeSettings>,
}

/// <p>Represents information about an action type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionTypeId {
    /// <p>A category defines what kind of action can be taken in the stage, and constrains the provider type for the action. Valid categories are limited to one of the values below.</p>
    #[serde(rename = "category")]
    pub category: String,
    /// <p>The creator of the action being called.</p>
    #[serde(rename = "owner")]
    pub owner: String,
    /// <p>The provider of the service being called by the action. Valid providers are determined by the action category. For example, an action in the Deploy category type might have a provider of AWS CodeDeploy, which would be specified as CodeDeploy.</p>
    #[serde(rename = "provider")]
    pub provider: String,
    /// <p>A string that describes the action version.</p>
    #[serde(rename = "version")]
    pub version: String,
}

/// <p>Returns information about the settings for an action type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionTypeSettings {
    /// <p>The URL returned to the AWS CodePipeline console that provides a deep link to the resources of the external system, such as the configuration page for an AWS CodeDeploy deployment group. This link is provided as part of the action display within the pipeline.</p>
    #[serde(rename = "entityUrlTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_url_template: Option<String>,
    /// <p>The URL returned to the AWS CodePipeline console that contains a link to the top-level landing page for the external system, such as console page for AWS CodeDeploy. This link is shown on the pipeline view page in the AWS CodePipeline console and provides a link to the execution entity of the external action.</p>
    #[serde(rename = "executionUrlTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_url_template: Option<String>,
    /// <p>The URL returned to the AWS CodePipeline console that contains a link to the page where customers can update or change the configuration of the external action.</p>
    #[serde(rename = "revisionUrlTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_url_template: Option<String>,
    /// <p>The URL of a sign-up page where users can sign up for an external service and perform initial configuration of the action provided by that service.</p>
    #[serde(rename = "thirdPartyConfigurationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_configuration_url: Option<String>,
}

/// <p>Represents information about the result of an approval request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ApprovalResult {
    /// <p>The response submitted by a reviewer assigned to an approval action request.</p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p>The summary of the current status of the approval request.</p>
    #[serde(rename = "summary")]
    pub summary: String,
}

/// <p>Represents information about an artifact that will be worked upon by actions in the pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Artifact {
    /// <p>The location of an artifact.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ArtifactLocation>,
    /// <p>The artifact's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The artifact's revision ID. Depending on the type of object, this could be a commit ID (GitHub) or a revision ID (Amazon S3).</p>
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}

/// <p>Returns information about the details of an artifact.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtifactDetails {
    /// <p>The maximum number of artifacts allowed for the action type.</p>
    #[serde(rename = "maximumCount")]
    pub maximum_count: i64,
    /// <p>The minimum number of artifacts allowed for the action type.</p>
    #[serde(rename = "minimumCount")]
    pub minimum_count: i64,
}

/// <p>Represents information about the location of an artifact.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ArtifactLocation {
    /// <p>The Amazon S3 bucket that contains the artifact.</p>
    #[serde(rename = "s3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_location: Option<S3ArtifactLocation>,
    /// <p>The type of artifact in the location.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Represents revision details of an artifact. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ArtifactRevision {
    /// <p>The date and time when the most recent revision of the artifact was created, in timestamp format.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>The name of an artifact. This name might be system-generated, such as "MyApp", or might be defined by the user when an action is created.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An additional identifier for a revision, such as a commit date or, for artifacts stored in Amazon S3 buckets, the ETag value.</p>
    #[serde(rename = "revisionChangeIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_change_identifier: Option<String>,
    /// <p>The revision ID of the artifact.</p>
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>Summary information about the most recent revision of the artifact. For GitHub and AWS CodeCommit repositories, the commit message. For Amazon S3 buckets or actions, the user-provided content of a <code>codepipeline-artifact-revision-summary</code> key specified in the object metadata.</p>
    #[serde(rename = "revisionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_summary: Option<String>,
    /// <p>The commit ID for the artifact revision. For artifacts stored in GitHub or AWS CodeCommit repositories, the commit ID is linked to a commit details page.</p>
    #[serde(rename = "revisionUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_url: Option<String>,
}

/// <p>The Amazon S3 bucket where artifacts are stored for the pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtifactStore {
    /// <p>The encryption key used to encrypt the data in the artifact store, such as an AWS Key Management Service (AWS KMS) key. If this is undefined, the default key for Amazon S3 is used.</p>
    #[serde(rename = "encryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<EncryptionKey>,
    /// <p>The Amazon S3 bucket used for storing the artifacts for a pipeline. You can specify the name of an S3 bucket but not a folder within the bucket. A folder to contain the pipeline artifacts is created for you based on the name of the pipeline. You can use any Amazon S3 bucket in the same AWS Region as the pipeline to store your pipeline artifacts.</p>
    #[serde(rename = "location")]
    pub location: String,
    /// <p>The type of the artifact store, such as S3.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Reserved for future use.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockerDeclaration {
    /// <p>Reserved for future use.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Represents the input of a CreateCustomActionType operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCustomActionTypeInput {
    /// <p><p>The category of the custom action, such as a build action or a test action.</p> <note> <p>Although Source and Approval are listed as valid values, they are not currently functional. These values are reserved for future use.</p> </note></p>
    #[serde(rename = "category")]
    pub category: String,
    /// <p><p>The configuration properties for the custom action.</p> <note> <p>You can refer to a name in the configuration properties of the custom action within the URL templates by following the format of {Config:name}, as long as the configuration property is both required and not secret. For more information, see <a href="http://docs.aws.amazon.com/codepipeline/latest/userguide/how-to-create-custom-action.html">Create a Custom Action for a Pipeline</a>.</p> </note></p>
    #[serde(rename = "configurationProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_properties: Option<Vec<ActionConfigurationProperty>>,
    /// <p>The details of the input artifact for the action, such as its commit ID.</p>
    #[serde(rename = "inputArtifactDetails")]
    pub input_artifact_details: ArtifactDetails,
    /// <p>The details of the output artifact of the action, such as its commit ID.</p>
    #[serde(rename = "outputArtifactDetails")]
    pub output_artifact_details: ArtifactDetails,
    /// <p>The provider of the service used in the custom action, such as AWS CodeDeploy.</p>
    #[serde(rename = "provider")]
    pub provider: String,
    /// <p>Returns information about the settings for an action type.</p>
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<ActionTypeSettings>,
    /// <p>The version identifier of the custom action.</p>
    #[serde(rename = "version")]
    pub version: String,
}

/// <p>Represents the output of a CreateCustomActionType operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateCustomActionTypeOutput {
    /// <p>Returns information about the details of an action type.</p>
    #[serde(rename = "actionType")]
    pub action_type: ActionType,
}

/// <p>Represents the input of a CreatePipeline action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePipelineInput {
    /// <p>Represents the structure of actions and stages to be performed in the pipeline. </p>
    #[serde(rename = "pipeline")]
    pub pipeline: PipelineDeclaration,
}

/// <p>Represents the output of a CreatePipeline action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreatePipelineOutput {
    /// <p>Represents the structure of actions and stages to be performed in the pipeline. </p>
    #[serde(rename = "pipeline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<PipelineDeclaration>,
}

/// <p>Represents information about a current revision.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CurrentRevision {
    /// <p>The change identifier for the current revision.</p>
    #[serde(rename = "changeIdentifier")]
    pub change_identifier: String,
    /// <p>The date and time when the most recent revision of the artifact was created, in timestamp format.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>The revision ID of the current version of an artifact.</p>
    #[serde(rename = "revision")]
    pub revision: String,
    /// <p>The summary of the most recent revision of the artifact.</p>
    #[serde(rename = "revisionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_summary: Option<String>,
}

/// <p>Represents the input of a DeleteCustomActionType operation. The custom action will be marked as deleted.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteCustomActionTypeInput {
    /// <p>The category of the custom action that you want to delete, such as source or deploy.</p>
    #[serde(rename = "category")]
    pub category: String,
    /// <p>The provider of the service used in the custom action, such as AWS CodeDeploy.</p>
    #[serde(rename = "provider")]
    pub provider: String,
    /// <p>The version of the custom action to delete.</p>
    #[serde(rename = "version")]
    pub version: String,
}

/// <p>Represents the input of a DeletePipeline action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePipelineInput {
    /// <p>The name of the pipeline to be deleted.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteWebhookInput {
    /// <p>The name of the webhook you want to delete.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteWebhookOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeregisterWebhookWithThirdPartyInput {
    /// <p>The name of the webhook you want to deregister.</p>
    #[serde(rename = "webhookName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeregisterWebhookWithThirdPartyOutput {}

/// <p>Represents the input of a DisableStageTransition action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisableStageTransitionInput {
    /// <p>The name of the pipeline in which you want to disable the flow of artifacts from one stage to another.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
    /// <p>The reason given to the user why a stage is disabled, such as waiting for manual approval or manual tests. This message is displayed in the pipeline console UI.</p>
    #[serde(rename = "reason")]
    pub reason: String,
    /// <p>The name of the stage where you want to disable the inbound or outbound transition of artifacts.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
    /// <p>Specifies whether artifacts will be prevented from transitioning into the stage and being processed by the actions in that stage (inbound), or prevented from transitioning from the stage after they have been processed by the actions in that stage (outbound).</p>
    #[serde(rename = "transitionType")]
    pub transition_type: String,
}

/// <p>Represents the input of an EnableStageTransition action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EnableStageTransitionInput {
    /// <p>The name of the pipeline in which you want to enable the flow of artifacts from one stage to another.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
    /// <p>The name of the stage where you want to enable the transition of artifacts, either into the stage (inbound) or from that stage to the next stage (outbound).</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
    /// <p>Specifies whether artifacts will be allowed to enter the stage and be processed by the actions in that stage (inbound) or whether already-processed artifacts will be allowed to transition to the next stage (outbound).</p>
    #[serde(rename = "transitionType")]
    pub transition_type: String,
}

/// <p>Represents information about the key used to encrypt data in the artifact store, such as an AWS Key Management Service (AWS KMS) key.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncryptionKey {
    /// <p>The ID used to identify the key. For an AWS KMS key, this is the key ID or key ARN.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The type of encryption key, such as an AWS Key Management Service (AWS KMS) key. When creating or updating a pipeline, the value must be set to 'KMS'.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Represents information about an error in AWS CodePipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ErrorDetails {
    /// <p>The system ID or error number code of the error.</p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The text of the error message.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The details of the actions taken and results produced on an artifact as it passes through stages in the pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ExecutionDetails {
    /// <p>The system-generated unique ID of this action used to identify this job worker in any external systems, such as AWS CodeDeploy.</p>
    #[serde(rename = "externalExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_id: Option<String>,
    /// <p>The percentage of work completed on the action, represented on a scale of zero to one hundred percent.</p>
    #[serde(rename = "percentComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i64>,
    /// <p>The summary of the current status of the actions.</p>
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

/// <p>Represents information about failure details.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct FailureDetails {
    /// <p>The external ID of the run of the action that failed.</p>
    #[serde(rename = "externalExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_id: Option<String>,
    /// <p>The message about the failure.</p>
    #[serde(rename = "message")]
    pub message: String,
    /// <p>The type of the failure.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Represents the input of a GetJobDetails action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobDetailsInput {
    /// <p>The unique system-generated ID for the job.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

/// <p>Represents the output of a GetJobDetails action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetJobDetailsOutput {
    /// <p><p>The details of the job.</p> <note> <p>If AWSSessionCredentials is used, a long-running job can call GetJobDetails again to obtain new credentials.</p> </note></p>
    #[serde(rename = "jobDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_details: Option<JobDetails>,
}

/// <p>Represents the input of a GetPipelineExecution action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPipelineExecutionInput {
    /// <p>The ID of the pipeline execution about which you want to get execution details.</p>
    #[serde(rename = "pipelineExecutionId")]
    pub pipeline_execution_id: String,
    /// <p>The name of the pipeline about which you want to get execution details.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
}

/// <p>Represents the output of a GetPipelineExecution action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetPipelineExecutionOutput {
    /// <p>Represents information about the execution of a pipeline.</p>
    #[serde(rename = "pipelineExecution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution: Option<PipelineExecution>,
}

/// <p>Represents the input of a GetPipeline action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPipelineInput {
    /// <p>The name of the pipeline for which you want to get information. Pipeline names must be unique under an Amazon Web Services (AWS) user account.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The version number of the pipeline. If you do not specify a version, defaults to the most current version.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Represents the output of a GetPipeline action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetPipelineOutput {
    /// <p>Represents the pipeline metadata information returned as part of the output of a GetPipeline action.</p>
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PipelineMetadata>,
    /// <p>Represents the structure of actions and stages to be performed in the pipeline. </p>
    #[serde(rename = "pipeline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<PipelineDeclaration>,
}

/// <p>Represents the input of a GetPipelineState action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPipelineStateInput {
    /// <p>The name of the pipeline about which you want to get information.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>Represents the output of a GetPipelineState action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetPipelineStateOutput {
    /// <p>The date and time the pipeline was created, in timestamp format.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>The name of the pipeline for which you want to get the state.</p>
    #[serde(rename = "pipelineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    /// <p><p>The version number of the pipeline.</p> <note> <p>A newly-created pipeline is always assigned a version number of <code>1</code>.</p> </note></p>
    #[serde(rename = "pipelineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_version: Option<i64>,
    /// <p>A list of the pipeline stage output information, including stage name, state, most recent run details, whether the stage is disabled, and other data.</p>
    #[serde(rename = "stageStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_states: Option<Vec<StageState>>,
    /// <p>The date and time the pipeline was last updated, in timestamp format.</p>
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<f64>,
}

/// <p>Represents the input of a GetThirdPartyJobDetails action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetThirdPartyJobDetailsInput {
    /// <p>The clientToken portion of the clientId and clientToken pair used to verify that the calling entity is allowed access to the job and its details.</p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The unique system-generated ID used for identifying the job.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

/// <p>Represents the output of a GetThirdPartyJobDetails action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetThirdPartyJobDetailsOutput {
    /// <p>The details of the job, including any protected values defined for the job.</p>
    #[serde(rename = "jobDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_details: Option<ThirdPartyJobDetails>,
}

/// <p>Represents information about an artifact to be worked on, such as a test or build artifact.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputArtifact {
    /// <p>The name of the artifact to be worked on, for example, "My App".</p> <p>The input artifact of an action must exactly match the output artifact declared in a preceding action, but the input artifact does not have to be the next action in strict sequence from the action that provided the output artifact. Actions in parallel can declare different output artifacts, which are in turn consumed by different following actions.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>Represents information about a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Job {
    /// <p>The ID of the AWS account to use when performing the job.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>Additional data about a job.</p>
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JobData>,
    /// <p>The unique system-generated ID of the job.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A system-generated random number that AWS CodePipeline uses to ensure that the job is being worked on by only one job worker. Use this number in an <a>AcknowledgeJob</a> request.</p>
    #[serde(rename = "nonce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

/// <p>Represents additional information about a job required for a job worker to complete the job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct JobData {
    /// <p>Represents information about an action configuration.</p>
    #[serde(rename = "actionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_configuration: Option<ActionConfiguration>,
    /// <p>Represents information about an action type.</p>
    #[serde(rename = "actionTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type_id: Option<ActionTypeId>,
    /// <p>Represents an AWS session credentials object. These credentials are temporary credentials that are issued by AWS Secure Token Service (STS). They can be used to access input and output artifacts in the Amazon S3 bucket used to store artifact for the pipeline in AWS CodePipeline.</p>
    #[serde(rename = "artifactCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_credentials: Option<AWSSessionCredentials>,
    /// <p>A system-generated token, such as a AWS CodeDeploy deployment ID, that a job requires in order to continue the job asynchronously.</p>
    #[serde(rename = "continuationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    /// <p>Represents information about the key used to encrypt data in the artifact store, such as an AWS Key Management Service (AWS KMS) key. </p>
    #[serde(rename = "encryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<EncryptionKey>,
    /// <p>The artifact supplied to the job.</p>
    #[serde(rename = "inputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<Vec<Artifact>>,
    /// <p>The output of the job.</p>
    #[serde(rename = "outputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_artifacts: Option<Vec<Artifact>>,
    /// <p>Represents information about a pipeline to a job worker.</p>
    #[serde(rename = "pipelineContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_context: Option<PipelineContext>,
}

/// <p>Represents information about the details of a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct JobDetails {
    /// <p>The AWS account ID associated with the job.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>Represents additional information about a job required for a job worker to complete the job. </p>
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JobData>,
    /// <p>The unique system-generated ID of the job.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>Represents the input of a ListActionTypes action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListActionTypesInput {
    /// <p>Filters the list of action types to those created by a specified entity.</p>
    #[serde(rename = "actionOwnerFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_owner_filter: Option<String>,
    /// <p>An identifier that was returned from the previous list action types call, which can be used to return the next set of action types in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the output of a ListActionTypes action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListActionTypesOutput {
    /// <p>Provides details of the action types.</p>
    #[serde(rename = "actionTypes")]
    pub action_types: Vec<ActionType>,
    /// <p>If the amount of returned information is significantly large, an identifier is also returned which can be used in a subsequent list action types call to return the next set of action types in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input of a ListPipelineExecutions action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPipelineExecutionsInput {
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned nextToken value. The available pipeline execution history is limited to the most recent 12 months, based on pipeline execution start times. Default value is 100.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that was returned from the previous ListPipelineExecutions call, which can be used to return the next set of pipeline executions in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the pipeline for which you want to get execution summary information.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
}

/// <p>Represents the output of a ListPipelineExecutions action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListPipelineExecutionsOutput {
    /// <p>A token that can be used in the next ListPipelineExecutions call. To view all items in the list, continue to call this operation with each subsequent token until no more nextToken values are returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of executions in the history of a pipeline.</p>
    #[serde(rename = "pipelineExecutionSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_summaries: Option<Vec<PipelineExecutionSummary>>,
}

/// <p>Represents the input of a ListPipelines action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPipelinesInput {
    /// <p>An identifier that was returned from the previous list pipelines call, which can be used to return the next set of pipelines in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the output of a ListPipelines action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListPipelinesOutput {
    /// <p>If the amount of returned information is significantly large, an identifier is also returned which can be used in a subsequent list pipelines call to return the next set of pipelines in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of pipelines.</p>
    #[serde(rename = "pipelines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines: Option<Vec<PipelineSummary>>,
}

/// <p>The detail returned for each webhook after listing webhooks, such as the webhook URL, the webhook name, and the webhook ARN.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListWebhookItem {
    /// <p>The Amazon Resource Name (ARN) of the webhook.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The detail returned for each webhook, such as the webhook authentication type and filter rules.</p>
    #[serde(rename = "definition")]
    pub definition: WebhookDefinition,
    /// <p>The number code of the error.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The text of the error message about the webhook.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The date and time a webhook was last successfully triggered, in timestamp format.</p>
    #[serde(rename = "lastTriggered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_triggered: Option<f64>,
    /// <p>A unique URL generated by CodePipeline. When a POST request is made to this URL, the defined pipeline is started as long as the body of the post request satisfies the defined authentication and filtering conditions. Deleting and re-creating a webhook will make the old URL invalid and generate a new URL.</p>
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListWebhooksInput {
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned nextToken value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that was returned from the previous ListWebhooks call, which can be used to return the next set of webhooks in the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListWebhooksOutput {
    /// <p>If the amount of returned information is significantly large, an identifier is also returned and can be used in a subsequent ListWebhooks call to return the next set of webhooks in the list. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The JSON detail returned for each webhook in the list output for the ListWebhooks call.</p>
    #[serde(rename = "webhooks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<Vec<ListWebhookItem>>,
}

/// <p>Represents information about the output of an action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputArtifact {
    /// <p>The name of the output of an artifact, such as "My App".</p> <p>The input artifact of an action must exactly match the output artifact declared in a preceding action, but the input artifact does not have to be the next action in strict sequence from the action that provided the output artifact. Actions in parallel can declare different output artifacts, which are in turn consumed by different following actions.</p> <p>Output artifact names must be unique within a pipeline.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>Represents information about a pipeline to a job worker.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PipelineContext {
    /// <p>The context of an action to a job worker within the stage of a pipeline.</p>
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionContext>,
    /// <p>The name of the pipeline. This is a user-specified value. Pipeline names must be unique across all pipeline names under an Amazon Web Services account.</p>
    #[serde(rename = "pipelineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    /// <p>The stage of the pipeline.</p>
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<StageContext>,
}

/// <p>Represents the structure of actions and stages to be performed in the pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PipelineDeclaration {
    /// <p>Represents information about the Amazon S3 bucket where artifacts are stored for the pipeline. </p>
    #[serde(rename = "artifactStore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_store: Option<ArtifactStore>,
    /// <p>A mapping of artifactStore objects and their corresponding regions. There must be an artifact store for the pipeline region and for each cross-region action within the pipeline. You can only use either artifactStore or artifactStores, not both.</p> <p>If you create a cross-region action in your pipeline, you must use artifactStores.</p>
    #[serde(rename = "artifactStores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_stores: Option<::std::collections::HashMap<String, ArtifactStore>>,
    /// <p>The name of the action to be performed.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) for AWS CodePipeline to use to either perform actions with no actionRoleArn, or to use to assume roles for actions with an actionRoleArn.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>The stage in which to perform the action.</p>
    #[serde(rename = "stages")]
    pub stages: Vec<StageDeclaration>,
    /// <p>The version number of the pipeline. A new pipeline always has a version number of 1. This number is automatically incremented when a pipeline is updated.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Represents information about an execution of a pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PipelineExecution {
    /// <p>A list of ArtifactRevision objects included in a pipeline execution.</p>
    #[serde(rename = "artifactRevisions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_revisions: Option<Vec<ArtifactRevision>>,
    /// <p>The ID of the pipeline execution.</p>
    #[serde(rename = "pipelineExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
    /// <p>The name of the pipeline that was executed.</p>
    #[serde(rename = "pipelineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    /// <p>The version number of the pipeline that was executed.</p>
    #[serde(rename = "pipelineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_version: Option<i64>,
    /// <p><p>The status of the pipeline execution.</p> <ul> <li> <p>InProgress: The pipeline execution is currently running.</p> </li> <li> <p>Succeeded: The pipeline execution was completed successfully. </p> </li> <li> <p>Superseded: While this pipeline execution was waiting for the next stage to be completed, a newer pipeline execution advanced and continued through the pipeline instead. </p> </li> <li> <p>Failed: The pipeline execution was not completed successfully.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Summary information about a pipeline execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PipelineExecutionSummary {
    /// <p>The date and time of the last change to the pipeline execution, in timestamp format.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The ID of the pipeline execution.</p>
    #[serde(rename = "pipelineExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
    /// <p>A list of the source artifact revisions that initiated a pipeline execution.</p>
    #[serde(rename = "sourceRevisions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_revisions: Option<Vec<SourceRevision>>,
    /// <p>The date and time when the pipeline execution began, in timestamp format.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p><p>The status of the pipeline execution.</p> <ul> <li> <p>InProgress: The pipeline execution is currently running.</p> </li> <li> <p>Succeeded: The pipeline execution was completed successfully. </p> </li> <li> <p>Superseded: While this pipeline execution was waiting for the next stage to be completed, a newer pipeline execution advanced and continued through the pipeline instead. </p> </li> <li> <p>Failed: The pipeline execution was not completed successfully.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information about a pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PipelineMetadata {
    /// <p>The date and time the pipeline was created, in timestamp format.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the pipeline.</p>
    #[serde(rename = "pipelineArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_arn: Option<String>,
    /// <p>The date and time the pipeline was last updated, in timestamp format.</p>
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<f64>,
}

/// <p>Returns a summary of a pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PipelineSummary {
    /// <p>The date and time the pipeline was created, in timestamp format.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>The name of the pipeline.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The date and time of the last update to the pipeline, in timestamp format.</p>
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<f64>,
    /// <p>The version number of the pipeline.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Represents the input of a PollForJobs action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PollForJobsInput {
    /// <p>Represents information about an action type.</p>
    #[serde(rename = "actionTypeId")]
    pub action_type_id: ActionTypeId,
    /// <p>The maximum number of jobs to return in a poll for jobs call.</p>
    #[serde(rename = "maxBatchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_size: Option<i64>,
    /// <p>A map of property names and values. For an action type with no queryable properties, this value must be null or an empty map. For an action type with a queryable property, you must supply that property as a key in the map. Only jobs whose action configuration matches the mapped value will be returned.</p>
    #[serde(rename = "queryParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_param: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Represents the output of a PollForJobs action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PollForJobsOutput {
    /// <p>Information about the jobs to take action on.</p>
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<Job>>,
}

/// <p>Represents the input of a PollForThirdPartyJobs action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PollForThirdPartyJobsInput {
    /// <p>Represents information about an action type.</p>
    #[serde(rename = "actionTypeId")]
    pub action_type_id: ActionTypeId,
    /// <p>The maximum number of jobs to return in a poll for jobs call.</p>
    #[serde(rename = "maxBatchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_size: Option<i64>,
}

/// <p>Represents the output of a PollForThirdPartyJobs action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PollForThirdPartyJobsOutput {
    /// <p>Information about the jobs to take action on.</p>
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<ThirdPartyJob>>,
}

/// <p>Represents the input of a PutActionRevision action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutActionRevisionInput {
    /// <p>The name of the action that will process the revision.</p>
    #[serde(rename = "actionName")]
    pub action_name: String,
    /// <p>Represents information about the version (or revision) of an action.</p>
    #[serde(rename = "actionRevision")]
    pub action_revision: ActionRevision,
    /// <p>The name of the pipeline that will start processing the revision to the source.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
    /// <p>The name of the stage that contains the action that will act upon the revision.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
}

/// <p>Represents the output of a PutActionRevision action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutActionRevisionOutput {
    /// <p>Indicates whether the artifact revision was previously used in an execution of the specified pipeline.</p>
    #[serde(rename = "newRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_revision: Option<bool>,
    /// <p>The ID of the current workflow state of the pipeline.</p>
    #[serde(rename = "pipelineExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
}

/// <p>Represents the input of a PutApprovalResult action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutApprovalResultInput {
    /// <p>The name of the action for which approval is requested.</p>
    #[serde(rename = "actionName")]
    pub action_name: String,
    /// <p>The name of the pipeline that contains the action. </p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
    /// <p>Represents information about the result of the approval request.</p>
    #[serde(rename = "result")]
    pub result: ApprovalResult,
    /// <p>The name of the stage that contains the action.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
    /// <p>The system-generated token used to identify a unique approval request. The token for each open approval request can be obtained using the <a>GetPipelineState</a> action and is used to validate that the approval request corresponding to this token is still valid.</p>
    #[serde(rename = "token")]
    pub token: String,
}

/// <p>Represents the output of a PutApprovalResult action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutApprovalResultOutput {
    /// <p>The timestamp showing when the approval or rejection was submitted.</p>
    #[serde(rename = "approvedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_at: Option<f64>,
}

/// <p>Represents the input of a PutJobFailureResult action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutJobFailureResultInput {
    /// <p>The details about the failure of a job.</p>
    #[serde(rename = "failureDetails")]
    pub failure_details: FailureDetails,
    /// <p>The unique system-generated ID of the job that failed. This is the same ID returned from PollForJobs.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

/// <p>Represents the input of a PutJobSuccessResult action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutJobSuccessResultInput {
    /// <p>A token generated by a job worker, such as an AWS CodeDeploy deployment ID, that a successful job provides to identify a custom action in progress. Future jobs will use this token in order to identify the running instance of the action. It can be reused to return additional information about the progress of the custom action. When the action is complete, no continuation token should be supplied.</p>
    #[serde(rename = "continuationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    /// <p>The ID of the current revision of the artifact successfully worked upon by the job.</p>
    #[serde(rename = "currentRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision: Option<CurrentRevision>,
    /// <p>The execution details of the successful job, such as the actions taken by the job worker.</p>
    #[serde(rename = "executionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_details: Option<ExecutionDetails>,
    /// <p>The unique system-generated ID of the job that succeeded. This is the same ID returned from PollForJobs.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

/// <p>Represents the input of a PutThirdPartyJobFailureResult action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutThirdPartyJobFailureResultInput {
    /// <p>The clientToken portion of the clientId and clientToken pair used to verify that the calling entity is allowed access to the job and its details.</p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>Represents information about failure details.</p>
    #[serde(rename = "failureDetails")]
    pub failure_details: FailureDetails,
    /// <p>The ID of the job that failed. This is the same ID returned from PollForThirdPartyJobs.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

/// <p>Represents the input of a PutThirdPartyJobSuccessResult action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutThirdPartyJobSuccessResultInput {
    /// <p>The clientToken portion of the clientId and clientToken pair used to verify that the calling entity is allowed access to the job and its details.</p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>A token generated by a job worker, such as an AWS CodeDeploy deployment ID, that a successful job provides to identify a partner action in progress. Future jobs will use this token in order to identify the running instance of the action. It can be reused to return additional information about the progress of the partner action. When the action is complete, no continuation token should be supplied.</p>
    #[serde(rename = "continuationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    /// <p>Represents information about a current revision.</p>
    #[serde(rename = "currentRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision: Option<CurrentRevision>,
    /// <p>The details of the actions taken and results produced on an artifact as it passes through stages in the pipeline. </p>
    #[serde(rename = "executionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_details: Option<ExecutionDetails>,
    /// <p>The ID of the job that successfully completed. This is the same ID returned from PollForThirdPartyJobs.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutWebhookInput {
    /// <p>The detail provided in an input file to create the webhook, such as the webhook name, the pipeline name, and the action name. Give the webhook a unique name which identifies the webhook being defined. You may choose to name the webhook after the pipeline and action it targets so that you can easily recognize what it's used for later.</p>
    #[serde(rename = "webhook")]
    pub webhook: WebhookDefinition,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutWebhookOutput {
    /// <p>The detail returned from creating the webhook, such as the webhook name, webhook URL, and webhook ARN.</p>
    #[serde(rename = "webhook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<ListWebhookItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterWebhookWithThirdPartyInput {
    /// <p>The name of an existing webhook created with PutWebhook to register with a supported third party. </p>
    #[serde(rename = "webhookName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RegisterWebhookWithThirdPartyOutput {}

/// <p>Represents the input of a RetryStageExecution action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RetryStageExecutionInput {
    /// <p>The ID of the pipeline execution in the failed stage to be retried. Use the <a>GetPipelineState</a> action to retrieve the current pipelineExecutionId of the failed stage</p>
    #[serde(rename = "pipelineExecutionId")]
    pub pipeline_execution_id: String,
    /// <p>The name of the pipeline that contains the failed stage.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
    /// <p>The scope of the retry attempt. Currently, the only supported value is FAILED_ACTIONS.</p>
    #[serde(rename = "retryMode")]
    pub retry_mode: String,
    /// <p>The name of the failed stage to be retried.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
}

/// <p>Represents the output of a RetryStageExecution action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RetryStageExecutionOutput {
    /// <p>The ID of the current workflow execution in the failed stage.</p>
    #[serde(rename = "pipelineExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
}

/// <p>The location of the Amazon S3 bucket that contains a revision.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct S3ArtifactLocation {
    /// <p>The name of the Amazon S3 bucket.</p>
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    /// <p>The key of the object in the Amazon S3 bucket, which uniquely identifies the object in the bucket.</p>
    #[serde(rename = "objectKey")]
    pub object_key: String,
}

/// <p>Information about the version (or revision) of a source artifact that initiated a pipeline execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SourceRevision {
    /// <p>The name of the action that processed the revision to the source artifact.</p>
    #[serde(rename = "actionName")]
    pub action_name: String,
    /// <p>The system-generated unique ID that identifies the revision number of the artifact.</p>
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>Summary information about the most recent revision of the artifact. For GitHub and AWS CodeCommit repositories, the commit message. For Amazon S3 buckets or actions, the user-provided content of a <code>codepipeline-artifact-revision-summary</code> key specified in the object metadata.</p>
    #[serde(rename = "revisionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_summary: Option<String>,
    /// <p>The commit ID for the artifact revision. For artifacts stored in GitHub or AWS CodeCommit repositories, the commit ID is linked to a commit details page.</p>
    #[serde(rename = "revisionUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_url: Option<String>,
}

/// <p>Represents information about a stage to a job worker.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StageContext {
    /// <p>The name of the stage.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Represents information about a stage and its definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StageDeclaration {
    /// <p>The actions included in a stage.</p>
    #[serde(rename = "actions")]
    pub actions: Vec<ActionDeclaration>,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "blockers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockers: Option<Vec<BlockerDeclaration>>,
    /// <p>The name of the stage.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>Represents information about the run of a stage.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StageExecution {
    /// <p>The ID of the pipeline execution associated with the stage.</p>
    #[serde(rename = "pipelineExecutionId")]
    pub pipeline_execution_id: String,
    /// <p>The status of the stage, or for a completed stage, the last status of the stage.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>Represents information about the state of the stage.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StageState {
    /// <p>The state of the stage.</p>
    #[serde(rename = "actionStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_states: Option<Vec<ActionState>>,
    /// <p>The state of the inbound transition, which is either enabled or disabled.</p>
    #[serde(rename = "inboundTransitionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transition_state: Option<TransitionState>,
    /// <p>Information about the latest execution in the stage, including its ID and status.</p>
    #[serde(rename = "latestExecution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_execution: Option<StageExecution>,
    /// <p>The name of the stage.</p>
    #[serde(rename = "stageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
}

/// <p>Represents the input of a StartPipelineExecution action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartPipelineExecutionInput {
    /// <p>The system-generated unique ID used to identify a unique execution request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The name of the pipeline to start.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>Represents the output of a StartPipelineExecution action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartPipelineExecutionOutput {
    /// <p>The unique system-generated ID of the pipeline execution that was started.</p>
    #[serde(rename = "pipelineExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
}

/// <p>A response to a PollForThirdPartyJobs request returned by AWS CodePipeline when there is a job to be worked upon by a partner action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ThirdPartyJob {
    /// <p>The clientToken portion of the clientId and clientToken pair used to verify that the calling entity is allowed access to the job and its details.</p>
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The identifier used to identify the job in AWS CodePipeline.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

/// <p>Represents information about the job data for a partner action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ThirdPartyJobData {
    /// <p>Represents information about an action configuration.</p>
    #[serde(rename = "actionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_configuration: Option<ActionConfiguration>,
    /// <p>Represents information about an action type.</p>
    #[serde(rename = "actionTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type_id: Option<ActionTypeId>,
    /// <p>Represents an AWS session credentials object. These credentials are temporary credentials that are issued by AWS Secure Token Service (STS). They can be used to access input and output artifacts in the Amazon S3 bucket used to store artifact for the pipeline in AWS CodePipeline. </p>
    #[serde(rename = "artifactCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_credentials: Option<AWSSessionCredentials>,
    /// <p>A system-generated token, such as a AWS CodeDeploy deployment ID, that a job requires in order to continue the job asynchronously.</p>
    #[serde(rename = "continuationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    /// <p>The encryption key used to encrypt and decrypt data in the artifact store for the pipeline, such as an AWS Key Management Service (AWS KMS) key. This is optional and might not be present.</p>
    #[serde(rename = "encryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<EncryptionKey>,
    /// <p>The name of the artifact that will be worked upon by the action, if any. This name might be system-generated, such as "MyApp", or might be defined by the user when the action is created. The input artifact name must match the name of an output artifact generated by an action in an earlier action or stage of the pipeline.</p>
    #[serde(rename = "inputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<Vec<Artifact>>,
    /// <p>The name of the artifact that will be the result of the action, if any. This name might be system-generated, such as "MyBuiltApp", or might be defined by the user when the action is created.</p>
    #[serde(rename = "outputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_artifacts: Option<Vec<Artifact>>,
    /// <p>Represents information about a pipeline to a job worker.</p>
    #[serde(rename = "pipelineContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_context: Option<PipelineContext>,
}

/// <p>The details of a job sent in response to a GetThirdPartyJobDetails request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ThirdPartyJobDetails {
    /// <p>The data to be returned by the third party job worker.</p>
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ThirdPartyJobData>,
    /// <p>The identifier used to identify the job details in AWS CodePipeline.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A system-generated random number that AWS CodePipeline uses to ensure that the job is being worked on by only one job worker. Use this number in an <a>AcknowledgeThirdPartyJob</a> request.</p>
    #[serde(rename = "nonce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

/// <p>Represents information about the state of transitions between one stage and another stage.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TransitionState {
    /// <p>The user-specified reason why the transition between two stages of a pipeline was disabled.</p>
    #[serde(rename = "disabledReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    /// <p>Whether the transition between stages is enabled (true) or disabled (false).</p>
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The timestamp when the transition state was last changed.</p>
    #[serde(rename = "lastChangedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_changed_at: Option<f64>,
    /// <p>The ID of the user who last changed the transition state.</p>
    #[serde(rename = "lastChangedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_changed_by: Option<String>,
}

/// <p>Represents the input of an UpdatePipeline action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePipelineInput {
    /// <p>The name of the pipeline to be updated.</p>
    #[serde(rename = "pipeline")]
    pub pipeline: PipelineDeclaration,
}

/// <p>Represents the output of an UpdatePipeline action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdatePipelineOutput {
    /// <p>The structure of the updated pipeline.</p>
    #[serde(rename = "pipeline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<PipelineDeclaration>,
}

/// <p>The authentication applied to incoming webhook trigger requests.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookAuthConfiguration {
    /// <p>The property used to configure acceptance of webhooks within a specific IP range. For IP, only the AllowedIPRange property must be set, and this property must be set to a valid CIDR range.</p>
    #[serde(rename = "AllowedIPRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_ip_range: Option<String>,
    /// <p>The property used to configure GitHub authentication. For GITHUB_HMAC, only the SecretToken property must be set.</p>
    #[serde(rename = "SecretToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<String>,
}

/// <p>Represents information about a webhook and its definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookDefinition {
    /// <p><p>Supported options are GITHUB<em>HMAC, IP and UNAUTHENTICATED.</p> <ul> <li> <p> GITHUB</em>HMAC implements the authentication scheme described here: https://developer.github.com/webhooks/securing/</p> </li> <li> <p> IP will reject webhooks trigger requests unless they originate from an IP within the IP range whitelisted in the authentication configuration.</p> </li> <li> <p> UNAUTHENTICATED will accept all webhook trigger requests regardless of origin.</p> </li> </ul></p>
    #[serde(rename = "authentication")]
    pub authentication: String,
    /// <p>Properties that configure the authentication applied to incoming webhook trigger requests. The required properties depend on the authentication type. For GITHUB_HMAC, only the SecretToken property must be set. For IP, only the AllowedIPRange property must be set to a valid CIDR range. For UNAUTHENTICATED, no properties can be set.</p>
    #[serde(rename = "authenticationConfiguration")]
    pub authentication_configuration: WebhookAuthConfiguration,
    /// <p>A list of rules applied to the body/payload sent in the POST request to a webhook URL. All defined rules must pass for the request to be accepted and the pipeline started.</p>
    #[serde(rename = "filters")]
    pub filters: Vec<WebhookFilterRule>,
    /// <p>The name of the webhook.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The name of the action in a pipeline you want to connect to the webhook. The action must be from the source (first) stage of the pipeline.</p>
    #[serde(rename = "targetAction")]
    pub target_action: String,
    /// <p>The name of the pipeline you want to connect to the webhook.</p>
    #[serde(rename = "targetPipeline")]
    pub target_pipeline: String,
}

/// <p>The event criteria that specify when a webhook notification is sent to your URL.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookFilterRule {
    /// <p>A JsonPath expression that will be applied to the body/payload of the webhook. The value selected by JsonPath expression must match the value specified in the matchEquals field, otherwise the request will be ignored. More information on JsonPath expressions can be found here: https://github.com/json-path/JsonPath.</p>
    #[serde(rename = "jsonPath")]
    pub json_path: String,
    /// <p>The value selected by the JsonPath expression must match what is supplied in the MatchEquals field, otherwise the request will be ignored. Properties from the target action configuration can be included as placeholders in this value by surrounding the action configuration key with curly braces. For example, if the value supplied here is "refs/heads/{Branch}" and the target action has an action configuration property called "Branch" with a value of "master", the MatchEquals value will be evaluated as "refs/heads/master". A list of action configuration properties for built-in action types can be found here: <a href="http://docs.aws.amazon.com/codepipeline/latest/userguide/reference-pipeline-structure.html#action-requirements">Pipeline Structure Reference Action Requirements</a>.</p>
    #[serde(rename = "matchEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_equals: Option<String>,
}

/// Errors returned by AcknowledgeJob
#[derive(Debug, PartialEq)]
pub enum AcknowledgeJobError {
    /// <p>The specified nonce was specified in an invalid format.</p>
    InvalidNonce(String),
    /// <p>The specified job was specified in an invalid format or cannot be found.</p>
    JobNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AcknowledgeJobError {
    pub fn from_response(res: BufferedHttpResponse) -> AcknowledgeJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidNonceException" => {
                    return AcknowledgeJobError::InvalidNonce(String::from(error_message));
                }
                "JobNotFoundException" => {
                    return AcknowledgeJobError::JobNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return AcknowledgeJobError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return AcknowledgeJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AcknowledgeJobError {
    fn from(err: serde_json::error::Error) -> AcknowledgeJobError {
        AcknowledgeJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AcknowledgeJobError {
    fn from(err: CredentialsError) -> AcknowledgeJobError {
        AcknowledgeJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AcknowledgeJobError {
    fn from(err: HttpDispatchError) -> AcknowledgeJobError {
        AcknowledgeJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for AcknowledgeJobError {
    fn from(err: io::Error) -> AcknowledgeJobError {
        AcknowledgeJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AcknowledgeJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AcknowledgeJobError {
    fn description(&self) -> &str {
        match *self {
            AcknowledgeJobError::InvalidNonce(ref cause) => cause,
            AcknowledgeJobError::JobNotFound(ref cause) => cause,
            AcknowledgeJobError::Validation(ref cause) => cause,
            AcknowledgeJobError::Credentials(ref err) => err.description(),
            AcknowledgeJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AcknowledgeJobError::ParseError(ref cause) => cause,
            AcknowledgeJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AcknowledgeThirdPartyJob
#[derive(Debug, PartialEq)]
pub enum AcknowledgeThirdPartyJobError {
    /// <p>The client token was specified in an invalid format</p>
    InvalidClientToken(String),
    /// <p>The specified nonce was specified in an invalid format.</p>
    InvalidNonce(String),
    /// <p>The specified job was specified in an invalid format or cannot be found.</p>
    JobNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AcknowledgeThirdPartyJobError {
    pub fn from_response(res: BufferedHttpResponse) -> AcknowledgeThirdPartyJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidClientTokenException" => {
                    return AcknowledgeThirdPartyJobError::InvalidClientToken(String::from(
                        error_message,
                    ));
                }
                "InvalidNonceException" => {
                    return AcknowledgeThirdPartyJobError::InvalidNonce(String::from(error_message));
                }
                "JobNotFoundException" => {
                    return AcknowledgeThirdPartyJobError::JobNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return AcknowledgeThirdPartyJobError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return AcknowledgeThirdPartyJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AcknowledgeThirdPartyJobError {
    fn from(err: serde_json::error::Error) -> AcknowledgeThirdPartyJobError {
        AcknowledgeThirdPartyJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AcknowledgeThirdPartyJobError {
    fn from(err: CredentialsError) -> AcknowledgeThirdPartyJobError {
        AcknowledgeThirdPartyJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AcknowledgeThirdPartyJobError {
    fn from(err: HttpDispatchError) -> AcknowledgeThirdPartyJobError {
        AcknowledgeThirdPartyJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for AcknowledgeThirdPartyJobError {
    fn from(err: io::Error) -> AcknowledgeThirdPartyJobError {
        AcknowledgeThirdPartyJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AcknowledgeThirdPartyJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AcknowledgeThirdPartyJobError {
    fn description(&self) -> &str {
        match *self {
            AcknowledgeThirdPartyJobError::InvalidClientToken(ref cause) => cause,
            AcknowledgeThirdPartyJobError::InvalidNonce(ref cause) => cause,
            AcknowledgeThirdPartyJobError::JobNotFound(ref cause) => cause,
            AcknowledgeThirdPartyJobError::Validation(ref cause) => cause,
            AcknowledgeThirdPartyJobError::Credentials(ref err) => err.description(),
            AcknowledgeThirdPartyJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AcknowledgeThirdPartyJobError::ParseError(ref cause) => cause,
            AcknowledgeThirdPartyJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateCustomActionType
#[derive(Debug, PartialEq)]
pub enum CreateCustomActionTypeError {
    /// <p>The number of pipelines associated with the AWS account has exceeded the limit allowed for the account.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateCustomActionTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateCustomActionTypeError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "LimitExceededException" => {
                    return CreateCustomActionTypeError::LimitExceeded(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateCustomActionTypeError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateCustomActionTypeError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateCustomActionTypeError {
    fn from(err: serde_json::error::Error) -> CreateCustomActionTypeError {
        CreateCustomActionTypeError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateCustomActionTypeError {
    fn from(err: CredentialsError) -> CreateCustomActionTypeError {
        CreateCustomActionTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCustomActionTypeError {
    fn from(err: HttpDispatchError) -> CreateCustomActionTypeError {
        CreateCustomActionTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateCustomActionTypeError {
    fn from(err: io::Error) -> CreateCustomActionTypeError {
        CreateCustomActionTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateCustomActionTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCustomActionTypeError {
    fn description(&self) -> &str {
        match *self {
            CreateCustomActionTypeError::LimitExceeded(ref cause) => cause,
            CreateCustomActionTypeError::Validation(ref cause) => cause,
            CreateCustomActionTypeError::Credentials(ref err) => err.description(),
            CreateCustomActionTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateCustomActionTypeError::ParseError(ref cause) => cause,
            CreateCustomActionTypeError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreatePipeline
#[derive(Debug, PartialEq)]
pub enum CreatePipelineError {
    /// <p>The specified action declaration was specified in an invalid format.</p>
    InvalidActionDeclaration(String),
    /// <p>Reserved for future use.</p>
    InvalidBlockerDeclaration(String),
    /// <p>The specified stage declaration was specified in an invalid format.</p>
    InvalidStageDeclaration(String),
    /// <p>The specified structure was specified in an invalid format.</p>
    InvalidStructure(String),
    /// <p>The number of pipelines associated with the AWS account has exceeded the limit allowed for the account.</p>
    LimitExceeded(String),
    /// <p>The specified pipeline name is already in use.</p>
    PipelineNameInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreatePipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> CreatePipelineError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidActionDeclarationException" => {
                    return CreatePipelineError::InvalidActionDeclaration(String::from(
                        error_message,
                    ));
                }
                "InvalidBlockerDeclarationException" => {
                    return CreatePipelineError::InvalidBlockerDeclaration(String::from(
                        error_message,
                    ));
                }
                "InvalidStageDeclarationException" => {
                    return CreatePipelineError::InvalidStageDeclaration(String::from(error_message));
                }
                "InvalidStructureException" => {
                    return CreatePipelineError::InvalidStructure(String::from(error_message));
                }
                "LimitExceededException" => {
                    return CreatePipelineError::LimitExceeded(String::from(error_message));
                }
                "PipelineNameInUseException" => {
                    return CreatePipelineError::PipelineNameInUse(String::from(error_message));
                }
                "ValidationException" => {
                    return CreatePipelineError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreatePipelineError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreatePipelineError {
    fn from(err: serde_json::error::Error) -> CreatePipelineError {
        CreatePipelineError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePipelineError {
    fn from(err: CredentialsError) -> CreatePipelineError {
        CreatePipelineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePipelineError {
    fn from(err: HttpDispatchError) -> CreatePipelineError {
        CreatePipelineError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePipelineError {
    fn from(err: io::Error) -> CreatePipelineError {
        CreatePipelineError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePipelineError {
    fn description(&self) -> &str {
        match *self {
            CreatePipelineError::InvalidActionDeclaration(ref cause) => cause,
            CreatePipelineError::InvalidBlockerDeclaration(ref cause) => cause,
            CreatePipelineError::InvalidStageDeclaration(ref cause) => cause,
            CreatePipelineError::InvalidStructure(ref cause) => cause,
            CreatePipelineError::LimitExceeded(ref cause) => cause,
            CreatePipelineError::PipelineNameInUse(ref cause) => cause,
            CreatePipelineError::Validation(ref cause) => cause,
            CreatePipelineError::Credentials(ref err) => err.description(),
            CreatePipelineError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreatePipelineError::ParseError(ref cause) => cause,
            CreatePipelineError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteCustomActionType
#[derive(Debug, PartialEq)]
pub enum DeleteCustomActionTypeError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteCustomActionTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteCustomActionTypeError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return DeleteCustomActionTypeError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteCustomActionTypeError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteCustomActionTypeError {
    fn from(err: serde_json::error::Error) -> DeleteCustomActionTypeError {
        DeleteCustomActionTypeError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteCustomActionTypeError {
    fn from(err: CredentialsError) -> DeleteCustomActionTypeError {
        DeleteCustomActionTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCustomActionTypeError {
    fn from(err: HttpDispatchError) -> DeleteCustomActionTypeError {
        DeleteCustomActionTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteCustomActionTypeError {
    fn from(err: io::Error) -> DeleteCustomActionTypeError {
        DeleteCustomActionTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteCustomActionTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCustomActionTypeError {
    fn description(&self) -> &str {
        match *self {
            DeleteCustomActionTypeError::Validation(ref cause) => cause,
            DeleteCustomActionTypeError::Credentials(ref err) => err.description(),
            DeleteCustomActionTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteCustomActionTypeError::ParseError(ref cause) => cause,
            DeleteCustomActionTypeError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeletePipeline
#[derive(Debug, PartialEq)]
pub enum DeletePipelineError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeletePipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> DeletePipelineError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return DeletePipelineError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeletePipelineError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeletePipelineError {
    fn from(err: serde_json::error::Error) -> DeletePipelineError {
        DeletePipelineError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeletePipelineError {
    fn from(err: CredentialsError) -> DeletePipelineError {
        DeletePipelineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeletePipelineError {
    fn from(err: HttpDispatchError) -> DeletePipelineError {
        DeletePipelineError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeletePipelineError {
    fn from(err: io::Error) -> DeletePipelineError {
        DeletePipelineError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeletePipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePipelineError {
    fn description(&self) -> &str {
        match *self {
            DeletePipelineError::Validation(ref cause) => cause,
            DeletePipelineError::Credentials(ref err) => err.description(),
            DeletePipelineError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeletePipelineError::ParseError(ref cause) => cause,
            DeletePipelineError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteWebhook
#[derive(Debug, PartialEq)]
pub enum DeleteWebhookError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteWebhookError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteWebhookError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return DeleteWebhookError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteWebhookError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteWebhookError {
    fn from(err: serde_json::error::Error) -> DeleteWebhookError {
        DeleteWebhookError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteWebhookError {
    fn from(err: CredentialsError) -> DeleteWebhookError {
        DeleteWebhookError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteWebhookError {
    fn from(err: HttpDispatchError) -> DeleteWebhookError {
        DeleteWebhookError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteWebhookError {
    fn from(err: io::Error) -> DeleteWebhookError {
        DeleteWebhookError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteWebhookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteWebhookError {
    fn description(&self) -> &str {
        match *self {
            DeleteWebhookError::Validation(ref cause) => cause,
            DeleteWebhookError::Credentials(ref err) => err.description(),
            DeleteWebhookError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteWebhookError::ParseError(ref cause) => cause,
            DeleteWebhookError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeregisterWebhookWithThirdParty
#[derive(Debug, PartialEq)]
pub enum DeregisterWebhookWithThirdPartyError {
    /// <p>The specified webhook was entered in an invalid format or cannot be found.</p>
    WebhookNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeregisterWebhookWithThirdPartyError {
    pub fn from_response(res: BufferedHttpResponse) -> DeregisterWebhookWithThirdPartyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WebhookNotFoundException" => {
                    return DeregisterWebhookWithThirdPartyError::WebhookNotFound(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DeregisterWebhookWithThirdPartyError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return DeregisterWebhookWithThirdPartyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeregisterWebhookWithThirdPartyError {
    fn from(err: serde_json::error::Error) -> DeregisterWebhookWithThirdPartyError {
        DeregisterWebhookWithThirdPartyError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeregisterWebhookWithThirdPartyError {
    fn from(err: CredentialsError) -> DeregisterWebhookWithThirdPartyError {
        DeregisterWebhookWithThirdPartyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterWebhookWithThirdPartyError {
    fn from(err: HttpDispatchError) -> DeregisterWebhookWithThirdPartyError {
        DeregisterWebhookWithThirdPartyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeregisterWebhookWithThirdPartyError {
    fn from(err: io::Error) -> DeregisterWebhookWithThirdPartyError {
        DeregisterWebhookWithThirdPartyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeregisterWebhookWithThirdPartyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterWebhookWithThirdPartyError {
    fn description(&self) -> &str {
        match *self {
            DeregisterWebhookWithThirdPartyError::WebhookNotFound(ref cause) => cause,
            DeregisterWebhookWithThirdPartyError::Validation(ref cause) => cause,
            DeregisterWebhookWithThirdPartyError::Credentials(ref err) => err.description(),
            DeregisterWebhookWithThirdPartyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterWebhookWithThirdPartyError::ParseError(ref cause) => cause,
            DeregisterWebhookWithThirdPartyError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DisableStageTransition
#[derive(Debug, PartialEq)]
pub enum DisableStageTransitionError {
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
    /// <p>The specified stage was specified in an invalid format or cannot be found.</p>
    StageNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DisableStageTransitionError {
    pub fn from_response(res: BufferedHttpResponse) -> DisableStageTransitionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "PipelineNotFoundException" => {
                    return DisableStageTransitionError::PipelineNotFound(String::from(
                        error_message,
                    ));
                }
                "StageNotFoundException" => {
                    return DisableStageTransitionError::StageNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return DisableStageTransitionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DisableStageTransitionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DisableStageTransitionError {
    fn from(err: serde_json::error::Error) -> DisableStageTransitionError {
        DisableStageTransitionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DisableStageTransitionError {
    fn from(err: CredentialsError) -> DisableStageTransitionError {
        DisableStageTransitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableStageTransitionError {
    fn from(err: HttpDispatchError) -> DisableStageTransitionError {
        DisableStageTransitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisableStageTransitionError {
    fn from(err: io::Error) -> DisableStageTransitionError {
        DisableStageTransitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisableStageTransitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableStageTransitionError {
    fn description(&self) -> &str {
        match *self {
            DisableStageTransitionError::PipelineNotFound(ref cause) => cause,
            DisableStageTransitionError::StageNotFound(ref cause) => cause,
            DisableStageTransitionError::Validation(ref cause) => cause,
            DisableStageTransitionError::Credentials(ref err) => err.description(),
            DisableStageTransitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisableStageTransitionError::ParseError(ref cause) => cause,
            DisableStageTransitionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by EnableStageTransition
#[derive(Debug, PartialEq)]
pub enum EnableStageTransitionError {
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
    /// <p>The specified stage was specified in an invalid format or cannot be found.</p>
    StageNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl EnableStageTransitionError {
    pub fn from_response(res: BufferedHttpResponse) -> EnableStageTransitionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "PipelineNotFoundException" => {
                    return EnableStageTransitionError::PipelineNotFound(String::from(error_message));
                }
                "StageNotFoundException" => {
                    return EnableStageTransitionError::StageNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return EnableStageTransitionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return EnableStageTransitionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for EnableStageTransitionError {
    fn from(err: serde_json::error::Error) -> EnableStageTransitionError {
        EnableStageTransitionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for EnableStageTransitionError {
    fn from(err: CredentialsError) -> EnableStageTransitionError {
        EnableStageTransitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableStageTransitionError {
    fn from(err: HttpDispatchError) -> EnableStageTransitionError {
        EnableStageTransitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for EnableStageTransitionError {
    fn from(err: io::Error) -> EnableStageTransitionError {
        EnableStageTransitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for EnableStageTransitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableStageTransitionError {
    fn description(&self) -> &str {
        match *self {
            EnableStageTransitionError::PipelineNotFound(ref cause) => cause,
            EnableStageTransitionError::StageNotFound(ref cause) => cause,
            EnableStageTransitionError::Validation(ref cause) => cause,
            EnableStageTransitionError::Credentials(ref err) => err.description(),
            EnableStageTransitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            EnableStageTransitionError::ParseError(ref cause) => cause,
            EnableStageTransitionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetJobDetails
#[derive(Debug, PartialEq)]
pub enum GetJobDetailsError {
    /// <p>The specified job was specified in an invalid format or cannot be found.</p>
    JobNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetJobDetailsError {
    pub fn from_response(res: BufferedHttpResponse) -> GetJobDetailsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "JobNotFoundException" => {
                    return GetJobDetailsError::JobNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return GetJobDetailsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetJobDetailsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetJobDetailsError {
    fn from(err: serde_json::error::Error) -> GetJobDetailsError {
        GetJobDetailsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetJobDetailsError {
    fn from(err: CredentialsError) -> GetJobDetailsError {
        GetJobDetailsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetJobDetailsError {
    fn from(err: HttpDispatchError) -> GetJobDetailsError {
        GetJobDetailsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetJobDetailsError {
    fn from(err: io::Error) -> GetJobDetailsError {
        GetJobDetailsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetJobDetailsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobDetailsError {
    fn description(&self) -> &str {
        match *self {
            GetJobDetailsError::JobNotFound(ref cause) => cause,
            GetJobDetailsError::Validation(ref cause) => cause,
            GetJobDetailsError::Credentials(ref err) => err.description(),
            GetJobDetailsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetJobDetailsError::ParseError(ref cause) => cause,
            GetJobDetailsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetPipeline
#[derive(Debug, PartialEq)]
pub enum GetPipelineError {
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
    /// <p>The specified pipeline version was specified in an invalid format or cannot be found.</p>
    PipelineVersionNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetPipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> GetPipelineError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "PipelineNotFoundException" => {
                    return GetPipelineError::PipelineNotFound(String::from(error_message));
                }
                "PipelineVersionNotFoundException" => {
                    return GetPipelineError::PipelineVersionNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return GetPipelineError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetPipelineError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetPipelineError {
    fn from(err: serde_json::error::Error) -> GetPipelineError {
        GetPipelineError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetPipelineError {
    fn from(err: CredentialsError) -> GetPipelineError {
        GetPipelineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPipelineError {
    fn from(err: HttpDispatchError) -> GetPipelineError {
        GetPipelineError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetPipelineError {
    fn from(err: io::Error) -> GetPipelineError {
        GetPipelineError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetPipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPipelineError {
    fn description(&self) -> &str {
        match *self {
            GetPipelineError::PipelineNotFound(ref cause) => cause,
            GetPipelineError::PipelineVersionNotFound(ref cause) => cause,
            GetPipelineError::Validation(ref cause) => cause,
            GetPipelineError::Credentials(ref err) => err.description(),
            GetPipelineError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetPipelineError::ParseError(ref cause) => cause,
            GetPipelineError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetPipelineExecution
#[derive(Debug, PartialEq)]
pub enum GetPipelineExecutionError {
    /// <p>The pipeline execution was specified in an invalid format or cannot be found, or an execution ID does not belong to the specified pipeline. </p>
    PipelineExecutionNotFound(String),
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetPipelineExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> GetPipelineExecutionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "PipelineExecutionNotFoundException" => {
                    return GetPipelineExecutionError::PipelineExecutionNotFound(String::from(
                        error_message,
                    ));
                }
                "PipelineNotFoundException" => {
                    return GetPipelineExecutionError::PipelineNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return GetPipelineExecutionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetPipelineExecutionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetPipelineExecutionError {
    fn from(err: serde_json::error::Error) -> GetPipelineExecutionError {
        GetPipelineExecutionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetPipelineExecutionError {
    fn from(err: CredentialsError) -> GetPipelineExecutionError {
        GetPipelineExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPipelineExecutionError {
    fn from(err: HttpDispatchError) -> GetPipelineExecutionError {
        GetPipelineExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetPipelineExecutionError {
    fn from(err: io::Error) -> GetPipelineExecutionError {
        GetPipelineExecutionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetPipelineExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPipelineExecutionError {
    fn description(&self) -> &str {
        match *self {
            GetPipelineExecutionError::PipelineExecutionNotFound(ref cause) => cause,
            GetPipelineExecutionError::PipelineNotFound(ref cause) => cause,
            GetPipelineExecutionError::Validation(ref cause) => cause,
            GetPipelineExecutionError::Credentials(ref err) => err.description(),
            GetPipelineExecutionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetPipelineExecutionError::ParseError(ref cause) => cause,
            GetPipelineExecutionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetPipelineState
#[derive(Debug, PartialEq)]
pub enum GetPipelineStateError {
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetPipelineStateError {
    pub fn from_response(res: BufferedHttpResponse) -> GetPipelineStateError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "PipelineNotFoundException" => {
                    return GetPipelineStateError::PipelineNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return GetPipelineStateError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetPipelineStateError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetPipelineStateError {
    fn from(err: serde_json::error::Error) -> GetPipelineStateError {
        GetPipelineStateError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetPipelineStateError {
    fn from(err: CredentialsError) -> GetPipelineStateError {
        GetPipelineStateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPipelineStateError {
    fn from(err: HttpDispatchError) -> GetPipelineStateError {
        GetPipelineStateError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetPipelineStateError {
    fn from(err: io::Error) -> GetPipelineStateError {
        GetPipelineStateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetPipelineStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPipelineStateError {
    fn description(&self) -> &str {
        match *self {
            GetPipelineStateError::PipelineNotFound(ref cause) => cause,
            GetPipelineStateError::Validation(ref cause) => cause,
            GetPipelineStateError::Credentials(ref err) => err.description(),
            GetPipelineStateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetPipelineStateError::ParseError(ref cause) => cause,
            GetPipelineStateError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetThirdPartyJobDetails
#[derive(Debug, PartialEq)]
pub enum GetThirdPartyJobDetailsError {
    /// <p>The client token was specified in an invalid format</p>
    InvalidClientToken(String),
    /// <p>The specified job was specified in an invalid format or cannot be found.</p>
    InvalidJob(String),
    /// <p>The specified job was specified in an invalid format or cannot be found.</p>
    JobNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetThirdPartyJobDetailsError {
    pub fn from_response(res: BufferedHttpResponse) -> GetThirdPartyJobDetailsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidClientTokenException" => {
                    return GetThirdPartyJobDetailsError::InvalidClientToken(String::from(
                        error_message,
                    ));
                }
                "InvalidJobException" => {
                    return GetThirdPartyJobDetailsError::InvalidJob(String::from(error_message));
                }
                "JobNotFoundException" => {
                    return GetThirdPartyJobDetailsError::JobNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return GetThirdPartyJobDetailsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetThirdPartyJobDetailsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetThirdPartyJobDetailsError {
    fn from(err: serde_json::error::Error) -> GetThirdPartyJobDetailsError {
        GetThirdPartyJobDetailsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetThirdPartyJobDetailsError {
    fn from(err: CredentialsError) -> GetThirdPartyJobDetailsError {
        GetThirdPartyJobDetailsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetThirdPartyJobDetailsError {
    fn from(err: HttpDispatchError) -> GetThirdPartyJobDetailsError {
        GetThirdPartyJobDetailsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetThirdPartyJobDetailsError {
    fn from(err: io::Error) -> GetThirdPartyJobDetailsError {
        GetThirdPartyJobDetailsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetThirdPartyJobDetailsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetThirdPartyJobDetailsError {
    fn description(&self) -> &str {
        match *self {
            GetThirdPartyJobDetailsError::InvalidClientToken(ref cause) => cause,
            GetThirdPartyJobDetailsError::InvalidJob(ref cause) => cause,
            GetThirdPartyJobDetailsError::JobNotFound(ref cause) => cause,
            GetThirdPartyJobDetailsError::Validation(ref cause) => cause,
            GetThirdPartyJobDetailsError::Credentials(ref err) => err.description(),
            GetThirdPartyJobDetailsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetThirdPartyJobDetailsError::ParseError(ref cause) => cause,
            GetThirdPartyJobDetailsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListActionTypes
#[derive(Debug, PartialEq)]
pub enum ListActionTypesError {
    /// <p>The next token was specified in an invalid format. Make sure that the next token you provided is the token returned by a previous call.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListActionTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListActionTypesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidNextTokenException" => {
                    return ListActionTypesError::InvalidNextToken(String::from(error_message));
                }
                "ValidationException" => {
                    return ListActionTypesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListActionTypesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListActionTypesError {
    fn from(err: serde_json::error::Error) -> ListActionTypesError {
        ListActionTypesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListActionTypesError {
    fn from(err: CredentialsError) -> ListActionTypesError {
        ListActionTypesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListActionTypesError {
    fn from(err: HttpDispatchError) -> ListActionTypesError {
        ListActionTypesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListActionTypesError {
    fn from(err: io::Error) -> ListActionTypesError {
        ListActionTypesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListActionTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListActionTypesError {
    fn description(&self) -> &str {
        match *self {
            ListActionTypesError::InvalidNextToken(ref cause) => cause,
            ListActionTypesError::Validation(ref cause) => cause,
            ListActionTypesError::Credentials(ref err) => err.description(),
            ListActionTypesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListActionTypesError::ParseError(ref cause) => cause,
            ListActionTypesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListPipelineExecutions
#[derive(Debug, PartialEq)]
pub enum ListPipelineExecutionsError {
    /// <p>The next token was specified in an invalid format. Make sure that the next token you provided is the token returned by a previous call.</p>
    InvalidNextToken(String),
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListPipelineExecutionsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListPipelineExecutionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidNextTokenException" => {
                    return ListPipelineExecutionsError::InvalidNextToken(String::from(
                        error_message,
                    ));
                }
                "PipelineNotFoundException" => {
                    return ListPipelineExecutionsError::PipelineNotFound(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return ListPipelineExecutionsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListPipelineExecutionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListPipelineExecutionsError {
    fn from(err: serde_json::error::Error) -> ListPipelineExecutionsError {
        ListPipelineExecutionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPipelineExecutionsError {
    fn from(err: CredentialsError) -> ListPipelineExecutionsError {
        ListPipelineExecutionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPipelineExecutionsError {
    fn from(err: HttpDispatchError) -> ListPipelineExecutionsError {
        ListPipelineExecutionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPipelineExecutionsError {
    fn from(err: io::Error) -> ListPipelineExecutionsError {
        ListPipelineExecutionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPipelineExecutionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPipelineExecutionsError {
    fn description(&self) -> &str {
        match *self {
            ListPipelineExecutionsError::InvalidNextToken(ref cause) => cause,
            ListPipelineExecutionsError::PipelineNotFound(ref cause) => cause,
            ListPipelineExecutionsError::Validation(ref cause) => cause,
            ListPipelineExecutionsError::Credentials(ref err) => err.description(),
            ListPipelineExecutionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPipelineExecutionsError::ParseError(ref cause) => cause,
            ListPipelineExecutionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListPipelines
#[derive(Debug, PartialEq)]
pub enum ListPipelinesError {
    /// <p>The next token was specified in an invalid format. Make sure that the next token you provided is the token returned by a previous call.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListPipelinesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListPipelinesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidNextTokenException" => {
                    return ListPipelinesError::InvalidNextToken(String::from(error_message));
                }
                "ValidationException" => {
                    return ListPipelinesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListPipelinesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListPipelinesError {
    fn from(err: serde_json::error::Error) -> ListPipelinesError {
        ListPipelinesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPipelinesError {
    fn from(err: CredentialsError) -> ListPipelinesError {
        ListPipelinesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPipelinesError {
    fn from(err: HttpDispatchError) -> ListPipelinesError {
        ListPipelinesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPipelinesError {
    fn from(err: io::Error) -> ListPipelinesError {
        ListPipelinesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPipelinesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPipelinesError {
    fn description(&self) -> &str {
        match *self {
            ListPipelinesError::InvalidNextToken(ref cause) => cause,
            ListPipelinesError::Validation(ref cause) => cause,
            ListPipelinesError::Credentials(ref err) => err.description(),
            ListPipelinesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListPipelinesError::ParseError(ref cause) => cause,
            ListPipelinesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListWebhooks
#[derive(Debug, PartialEq)]
pub enum ListWebhooksError {
    /// <p>The next token was specified in an invalid format. Make sure that the next token you provided is the token returned by a previous call.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListWebhooksError {
    pub fn from_response(res: BufferedHttpResponse) -> ListWebhooksError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidNextTokenException" => {
                    return ListWebhooksError::InvalidNextToken(String::from(error_message));
                }
                "ValidationException" => {
                    return ListWebhooksError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListWebhooksError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListWebhooksError {
    fn from(err: serde_json::error::Error) -> ListWebhooksError {
        ListWebhooksError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListWebhooksError {
    fn from(err: CredentialsError) -> ListWebhooksError {
        ListWebhooksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListWebhooksError {
    fn from(err: HttpDispatchError) -> ListWebhooksError {
        ListWebhooksError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListWebhooksError {
    fn from(err: io::Error) -> ListWebhooksError {
        ListWebhooksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListWebhooksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListWebhooksError {
    fn description(&self) -> &str {
        match *self {
            ListWebhooksError::InvalidNextToken(ref cause) => cause,
            ListWebhooksError::Validation(ref cause) => cause,
            ListWebhooksError::Credentials(ref err) => err.description(),
            ListWebhooksError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListWebhooksError::ParseError(ref cause) => cause,
            ListWebhooksError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PollForJobs
#[derive(Debug, PartialEq)]
pub enum PollForJobsError {
    /// <p>The specified action type cannot be found.</p>
    ActionTypeNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PollForJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> PollForJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ActionTypeNotFoundException" => {
                    return PollForJobsError::ActionTypeNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return PollForJobsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PollForJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PollForJobsError {
    fn from(err: serde_json::error::Error) -> PollForJobsError {
        PollForJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PollForJobsError {
    fn from(err: CredentialsError) -> PollForJobsError {
        PollForJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PollForJobsError {
    fn from(err: HttpDispatchError) -> PollForJobsError {
        PollForJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for PollForJobsError {
    fn from(err: io::Error) -> PollForJobsError {
        PollForJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PollForJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PollForJobsError {
    fn description(&self) -> &str {
        match *self {
            PollForJobsError::ActionTypeNotFound(ref cause) => cause,
            PollForJobsError::Validation(ref cause) => cause,
            PollForJobsError::Credentials(ref err) => err.description(),
            PollForJobsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PollForJobsError::ParseError(ref cause) => cause,
            PollForJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PollForThirdPartyJobs
#[derive(Debug, PartialEq)]
pub enum PollForThirdPartyJobsError {
    /// <p>The specified action type cannot be found.</p>
    ActionTypeNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PollForThirdPartyJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> PollForThirdPartyJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ActionTypeNotFoundException" => {
                    return PollForThirdPartyJobsError::ActionTypeNotFound(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return PollForThirdPartyJobsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PollForThirdPartyJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PollForThirdPartyJobsError {
    fn from(err: serde_json::error::Error) -> PollForThirdPartyJobsError {
        PollForThirdPartyJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PollForThirdPartyJobsError {
    fn from(err: CredentialsError) -> PollForThirdPartyJobsError {
        PollForThirdPartyJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PollForThirdPartyJobsError {
    fn from(err: HttpDispatchError) -> PollForThirdPartyJobsError {
        PollForThirdPartyJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for PollForThirdPartyJobsError {
    fn from(err: io::Error) -> PollForThirdPartyJobsError {
        PollForThirdPartyJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PollForThirdPartyJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PollForThirdPartyJobsError {
    fn description(&self) -> &str {
        match *self {
            PollForThirdPartyJobsError::ActionTypeNotFound(ref cause) => cause,
            PollForThirdPartyJobsError::Validation(ref cause) => cause,
            PollForThirdPartyJobsError::Credentials(ref err) => err.description(),
            PollForThirdPartyJobsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PollForThirdPartyJobsError::ParseError(ref cause) => cause,
            PollForThirdPartyJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutActionRevision
#[derive(Debug, PartialEq)]
pub enum PutActionRevisionError {
    /// <p>The specified action cannot be found.</p>
    ActionNotFound(String),
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
    /// <p>The specified stage was specified in an invalid format or cannot be found.</p>
    StageNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutActionRevisionError {
    pub fn from_response(res: BufferedHttpResponse) -> PutActionRevisionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ActionNotFoundException" => {
                    return PutActionRevisionError::ActionNotFound(String::from(error_message));
                }
                "PipelineNotFoundException" => {
                    return PutActionRevisionError::PipelineNotFound(String::from(error_message));
                }
                "StageNotFoundException" => {
                    return PutActionRevisionError::StageNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return PutActionRevisionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PutActionRevisionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutActionRevisionError {
    fn from(err: serde_json::error::Error) -> PutActionRevisionError {
        PutActionRevisionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutActionRevisionError {
    fn from(err: CredentialsError) -> PutActionRevisionError {
        PutActionRevisionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutActionRevisionError {
    fn from(err: HttpDispatchError) -> PutActionRevisionError {
        PutActionRevisionError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutActionRevisionError {
    fn from(err: io::Error) -> PutActionRevisionError {
        PutActionRevisionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutActionRevisionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutActionRevisionError {
    fn description(&self) -> &str {
        match *self {
            PutActionRevisionError::ActionNotFound(ref cause) => cause,
            PutActionRevisionError::PipelineNotFound(ref cause) => cause,
            PutActionRevisionError::StageNotFound(ref cause) => cause,
            PutActionRevisionError::Validation(ref cause) => cause,
            PutActionRevisionError::Credentials(ref err) => err.description(),
            PutActionRevisionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutActionRevisionError::ParseError(ref cause) => cause,
            PutActionRevisionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutApprovalResult
#[derive(Debug, PartialEq)]
pub enum PutApprovalResultError {
    /// <p>The specified action cannot be found.</p>
    ActionNotFound(String),
    /// <p>The approval action has already been approved or rejected.</p>
    ApprovalAlreadyCompleted(String),
    /// <p>The approval request already received a response or has expired.</p>
    InvalidApprovalToken(String),
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
    /// <p>The specified stage was specified in an invalid format or cannot be found.</p>
    StageNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutApprovalResultError {
    pub fn from_response(res: BufferedHttpResponse) -> PutApprovalResultError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ActionNotFoundException" => {
                    return PutApprovalResultError::ActionNotFound(String::from(error_message));
                }
                "ApprovalAlreadyCompletedException" => {
                    return PutApprovalResultError::ApprovalAlreadyCompleted(String::from(
                        error_message,
                    ));
                }
                "InvalidApprovalTokenException" => {
                    return PutApprovalResultError::InvalidApprovalToken(String::from(error_message));
                }
                "PipelineNotFoundException" => {
                    return PutApprovalResultError::PipelineNotFound(String::from(error_message));
                }
                "StageNotFoundException" => {
                    return PutApprovalResultError::StageNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return PutApprovalResultError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PutApprovalResultError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutApprovalResultError {
    fn from(err: serde_json::error::Error) -> PutApprovalResultError {
        PutApprovalResultError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutApprovalResultError {
    fn from(err: CredentialsError) -> PutApprovalResultError {
        PutApprovalResultError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutApprovalResultError {
    fn from(err: HttpDispatchError) -> PutApprovalResultError {
        PutApprovalResultError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutApprovalResultError {
    fn from(err: io::Error) -> PutApprovalResultError {
        PutApprovalResultError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutApprovalResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutApprovalResultError {
    fn description(&self) -> &str {
        match *self {
            PutApprovalResultError::ActionNotFound(ref cause) => cause,
            PutApprovalResultError::ApprovalAlreadyCompleted(ref cause) => cause,
            PutApprovalResultError::InvalidApprovalToken(ref cause) => cause,
            PutApprovalResultError::PipelineNotFound(ref cause) => cause,
            PutApprovalResultError::StageNotFound(ref cause) => cause,
            PutApprovalResultError::Validation(ref cause) => cause,
            PutApprovalResultError::Credentials(ref err) => err.description(),
            PutApprovalResultError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutApprovalResultError::ParseError(ref cause) => cause,
            PutApprovalResultError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutJobFailureResult
#[derive(Debug, PartialEq)]
pub enum PutJobFailureResultError {
    /// <p>The specified job state was specified in an invalid format.</p>
    InvalidJobState(String),
    /// <p>The specified job was specified in an invalid format or cannot be found.</p>
    JobNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutJobFailureResultError {
    pub fn from_response(res: BufferedHttpResponse) -> PutJobFailureResultError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidJobStateException" => {
                    return PutJobFailureResultError::InvalidJobState(String::from(error_message));
                }
                "JobNotFoundException" => {
                    return PutJobFailureResultError::JobNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return PutJobFailureResultError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PutJobFailureResultError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutJobFailureResultError {
    fn from(err: serde_json::error::Error) -> PutJobFailureResultError {
        PutJobFailureResultError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutJobFailureResultError {
    fn from(err: CredentialsError) -> PutJobFailureResultError {
        PutJobFailureResultError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutJobFailureResultError {
    fn from(err: HttpDispatchError) -> PutJobFailureResultError {
        PutJobFailureResultError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutJobFailureResultError {
    fn from(err: io::Error) -> PutJobFailureResultError {
        PutJobFailureResultError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutJobFailureResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutJobFailureResultError {
    fn description(&self) -> &str {
        match *self {
            PutJobFailureResultError::InvalidJobState(ref cause) => cause,
            PutJobFailureResultError::JobNotFound(ref cause) => cause,
            PutJobFailureResultError::Validation(ref cause) => cause,
            PutJobFailureResultError::Credentials(ref err) => err.description(),
            PutJobFailureResultError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutJobFailureResultError::ParseError(ref cause) => cause,
            PutJobFailureResultError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutJobSuccessResult
#[derive(Debug, PartialEq)]
pub enum PutJobSuccessResultError {
    /// <p>The specified job state was specified in an invalid format.</p>
    InvalidJobState(String),
    /// <p>The specified job was specified in an invalid format or cannot be found.</p>
    JobNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutJobSuccessResultError {
    pub fn from_response(res: BufferedHttpResponse) -> PutJobSuccessResultError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidJobStateException" => {
                    return PutJobSuccessResultError::InvalidJobState(String::from(error_message));
                }
                "JobNotFoundException" => {
                    return PutJobSuccessResultError::JobNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return PutJobSuccessResultError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PutJobSuccessResultError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutJobSuccessResultError {
    fn from(err: serde_json::error::Error) -> PutJobSuccessResultError {
        PutJobSuccessResultError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutJobSuccessResultError {
    fn from(err: CredentialsError) -> PutJobSuccessResultError {
        PutJobSuccessResultError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutJobSuccessResultError {
    fn from(err: HttpDispatchError) -> PutJobSuccessResultError {
        PutJobSuccessResultError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutJobSuccessResultError {
    fn from(err: io::Error) -> PutJobSuccessResultError {
        PutJobSuccessResultError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutJobSuccessResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutJobSuccessResultError {
    fn description(&self) -> &str {
        match *self {
            PutJobSuccessResultError::InvalidJobState(ref cause) => cause,
            PutJobSuccessResultError::JobNotFound(ref cause) => cause,
            PutJobSuccessResultError::Validation(ref cause) => cause,
            PutJobSuccessResultError::Credentials(ref err) => err.description(),
            PutJobSuccessResultError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutJobSuccessResultError::ParseError(ref cause) => cause,
            PutJobSuccessResultError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutThirdPartyJobFailureResult
#[derive(Debug, PartialEq)]
pub enum PutThirdPartyJobFailureResultError {
    /// <p>The client token was specified in an invalid format</p>
    InvalidClientToken(String),
    /// <p>The specified job state was specified in an invalid format.</p>
    InvalidJobState(String),
    /// <p>The specified job was specified in an invalid format or cannot be found.</p>
    JobNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutThirdPartyJobFailureResultError {
    pub fn from_response(res: BufferedHttpResponse) -> PutThirdPartyJobFailureResultError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidClientTokenException" => {
                    return PutThirdPartyJobFailureResultError::InvalidClientToken(String::from(
                        error_message,
                    ));
                }
                "InvalidJobStateException" => {
                    return PutThirdPartyJobFailureResultError::InvalidJobState(String::from(
                        error_message,
                    ));
                }
                "JobNotFoundException" => {
                    return PutThirdPartyJobFailureResultError::JobNotFound(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return PutThirdPartyJobFailureResultError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PutThirdPartyJobFailureResultError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutThirdPartyJobFailureResultError {
    fn from(err: serde_json::error::Error) -> PutThirdPartyJobFailureResultError {
        PutThirdPartyJobFailureResultError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutThirdPartyJobFailureResultError {
    fn from(err: CredentialsError) -> PutThirdPartyJobFailureResultError {
        PutThirdPartyJobFailureResultError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutThirdPartyJobFailureResultError {
    fn from(err: HttpDispatchError) -> PutThirdPartyJobFailureResultError {
        PutThirdPartyJobFailureResultError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutThirdPartyJobFailureResultError {
    fn from(err: io::Error) -> PutThirdPartyJobFailureResultError {
        PutThirdPartyJobFailureResultError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutThirdPartyJobFailureResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutThirdPartyJobFailureResultError {
    fn description(&self) -> &str {
        match *self {
            PutThirdPartyJobFailureResultError::InvalidClientToken(ref cause) => cause,
            PutThirdPartyJobFailureResultError::InvalidJobState(ref cause) => cause,
            PutThirdPartyJobFailureResultError::JobNotFound(ref cause) => cause,
            PutThirdPartyJobFailureResultError::Validation(ref cause) => cause,
            PutThirdPartyJobFailureResultError::Credentials(ref err) => err.description(),
            PutThirdPartyJobFailureResultError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutThirdPartyJobFailureResultError::ParseError(ref cause) => cause,
            PutThirdPartyJobFailureResultError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutThirdPartyJobSuccessResult
#[derive(Debug, PartialEq)]
pub enum PutThirdPartyJobSuccessResultError {
    /// <p>The client token was specified in an invalid format</p>
    InvalidClientToken(String),
    /// <p>The specified job state was specified in an invalid format.</p>
    InvalidJobState(String),
    /// <p>The specified job was specified in an invalid format or cannot be found.</p>
    JobNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutThirdPartyJobSuccessResultError {
    pub fn from_response(res: BufferedHttpResponse) -> PutThirdPartyJobSuccessResultError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidClientTokenException" => {
                    return PutThirdPartyJobSuccessResultError::InvalidClientToken(String::from(
                        error_message,
                    ));
                }
                "InvalidJobStateException" => {
                    return PutThirdPartyJobSuccessResultError::InvalidJobState(String::from(
                        error_message,
                    ));
                }
                "JobNotFoundException" => {
                    return PutThirdPartyJobSuccessResultError::JobNotFound(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return PutThirdPartyJobSuccessResultError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PutThirdPartyJobSuccessResultError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutThirdPartyJobSuccessResultError {
    fn from(err: serde_json::error::Error) -> PutThirdPartyJobSuccessResultError {
        PutThirdPartyJobSuccessResultError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutThirdPartyJobSuccessResultError {
    fn from(err: CredentialsError) -> PutThirdPartyJobSuccessResultError {
        PutThirdPartyJobSuccessResultError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutThirdPartyJobSuccessResultError {
    fn from(err: HttpDispatchError) -> PutThirdPartyJobSuccessResultError {
        PutThirdPartyJobSuccessResultError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutThirdPartyJobSuccessResultError {
    fn from(err: io::Error) -> PutThirdPartyJobSuccessResultError {
        PutThirdPartyJobSuccessResultError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutThirdPartyJobSuccessResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutThirdPartyJobSuccessResultError {
    fn description(&self) -> &str {
        match *self {
            PutThirdPartyJobSuccessResultError::InvalidClientToken(ref cause) => cause,
            PutThirdPartyJobSuccessResultError::InvalidJobState(ref cause) => cause,
            PutThirdPartyJobSuccessResultError::JobNotFound(ref cause) => cause,
            PutThirdPartyJobSuccessResultError::Validation(ref cause) => cause,
            PutThirdPartyJobSuccessResultError::Credentials(ref err) => err.description(),
            PutThirdPartyJobSuccessResultError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutThirdPartyJobSuccessResultError::ParseError(ref cause) => cause,
            PutThirdPartyJobSuccessResultError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutWebhook
#[derive(Debug, PartialEq)]
pub enum PutWebhookError {
    /// <p>The specified authentication type is in an invalid format.</p>
    InvalidWebhookAuthenticationParameters(String),
    /// <p>The specified event filter rule is in an invalid format.</p>
    InvalidWebhookFilterPattern(String),
    /// <p>The number of pipelines associated with the AWS account has exceeded the limit allowed for the account.</p>
    LimitExceeded(String),
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutWebhookError {
    pub fn from_response(res: BufferedHttpResponse) -> PutWebhookError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidWebhookAuthenticationParametersException" => {
                    return PutWebhookError::InvalidWebhookAuthenticationParameters(String::from(
                        error_message,
                    ));
                }
                "InvalidWebhookFilterPatternException" => {
                    return PutWebhookError::InvalidWebhookFilterPattern(String::from(error_message));
                }
                "LimitExceededException" => {
                    return PutWebhookError::LimitExceeded(String::from(error_message));
                }
                "PipelineNotFoundException" => {
                    return PutWebhookError::PipelineNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return PutWebhookError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PutWebhookError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutWebhookError {
    fn from(err: serde_json::error::Error) -> PutWebhookError {
        PutWebhookError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutWebhookError {
    fn from(err: CredentialsError) -> PutWebhookError {
        PutWebhookError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutWebhookError {
    fn from(err: HttpDispatchError) -> PutWebhookError {
        PutWebhookError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutWebhookError {
    fn from(err: io::Error) -> PutWebhookError {
        PutWebhookError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutWebhookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutWebhookError {
    fn description(&self) -> &str {
        match *self {
            PutWebhookError::InvalidWebhookAuthenticationParameters(ref cause) => cause,
            PutWebhookError::InvalidWebhookFilterPattern(ref cause) => cause,
            PutWebhookError::LimitExceeded(ref cause) => cause,
            PutWebhookError::PipelineNotFound(ref cause) => cause,
            PutWebhookError::Validation(ref cause) => cause,
            PutWebhookError::Credentials(ref err) => err.description(),
            PutWebhookError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutWebhookError::ParseError(ref cause) => cause,
            PutWebhookError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RegisterWebhookWithThirdParty
#[derive(Debug, PartialEq)]
pub enum RegisterWebhookWithThirdPartyError {
    /// <p>The specified webhook was entered in an invalid format or cannot be found.</p>
    WebhookNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl RegisterWebhookWithThirdPartyError {
    pub fn from_response(res: BufferedHttpResponse) -> RegisterWebhookWithThirdPartyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WebhookNotFoundException" => {
                    return RegisterWebhookWithThirdPartyError::WebhookNotFound(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return RegisterWebhookWithThirdPartyError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return RegisterWebhookWithThirdPartyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RegisterWebhookWithThirdPartyError {
    fn from(err: serde_json::error::Error) -> RegisterWebhookWithThirdPartyError {
        RegisterWebhookWithThirdPartyError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterWebhookWithThirdPartyError {
    fn from(err: CredentialsError) -> RegisterWebhookWithThirdPartyError {
        RegisterWebhookWithThirdPartyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterWebhookWithThirdPartyError {
    fn from(err: HttpDispatchError) -> RegisterWebhookWithThirdPartyError {
        RegisterWebhookWithThirdPartyError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterWebhookWithThirdPartyError {
    fn from(err: io::Error) -> RegisterWebhookWithThirdPartyError {
        RegisterWebhookWithThirdPartyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterWebhookWithThirdPartyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterWebhookWithThirdPartyError {
    fn description(&self) -> &str {
        match *self {
            RegisterWebhookWithThirdPartyError::WebhookNotFound(ref cause) => cause,
            RegisterWebhookWithThirdPartyError::Validation(ref cause) => cause,
            RegisterWebhookWithThirdPartyError::Credentials(ref err) => err.description(),
            RegisterWebhookWithThirdPartyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterWebhookWithThirdPartyError::ParseError(ref cause) => cause,
            RegisterWebhookWithThirdPartyError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RetryStageExecution
#[derive(Debug, PartialEq)]
pub enum RetryStageExecutionError {
    /// <p>The stage has failed in a later run of the pipeline and the pipelineExecutionId associated with the request is out of date.</p>
    NotLatestPipelineExecution(String),
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
    /// <p>The specified stage was specified in an invalid format or cannot be found.</p>
    StageNotFound(String),
    /// <p>The specified stage can't be retried because the pipeline structure or stage state changed after the stage was not completed; the stage contains no failed actions; one or more actions are still in progress; or another retry attempt is already in progress. </p>
    StageNotRetryable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl RetryStageExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RetryStageExecutionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotLatestPipelineExecutionException" => {
                    return RetryStageExecutionError::NotLatestPipelineExecution(String::from(
                        error_message,
                    ));
                }
                "PipelineNotFoundException" => {
                    return RetryStageExecutionError::PipelineNotFound(String::from(error_message));
                }
                "StageNotFoundException" => {
                    return RetryStageExecutionError::StageNotFound(String::from(error_message));
                }
                "StageNotRetryableException" => {
                    return RetryStageExecutionError::StageNotRetryable(String::from(error_message));
                }
                "ValidationException" => {
                    return RetryStageExecutionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return RetryStageExecutionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RetryStageExecutionError {
    fn from(err: serde_json::error::Error) -> RetryStageExecutionError {
        RetryStageExecutionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RetryStageExecutionError {
    fn from(err: CredentialsError) -> RetryStageExecutionError {
        RetryStageExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RetryStageExecutionError {
    fn from(err: HttpDispatchError) -> RetryStageExecutionError {
        RetryStageExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for RetryStageExecutionError {
    fn from(err: io::Error) -> RetryStageExecutionError {
        RetryStageExecutionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RetryStageExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RetryStageExecutionError {
    fn description(&self) -> &str {
        match *self {
            RetryStageExecutionError::NotLatestPipelineExecution(ref cause) => cause,
            RetryStageExecutionError::PipelineNotFound(ref cause) => cause,
            RetryStageExecutionError::StageNotFound(ref cause) => cause,
            RetryStageExecutionError::StageNotRetryable(ref cause) => cause,
            RetryStageExecutionError::Validation(ref cause) => cause,
            RetryStageExecutionError::Credentials(ref err) => err.description(),
            RetryStageExecutionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RetryStageExecutionError::ParseError(ref cause) => cause,
            RetryStageExecutionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartPipelineExecution
#[derive(Debug, PartialEq)]
pub enum StartPipelineExecutionError {
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl StartPipelineExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> StartPipelineExecutionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "PipelineNotFoundException" => {
                    return StartPipelineExecutionError::PipelineNotFound(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return StartPipelineExecutionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return StartPipelineExecutionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartPipelineExecutionError {
    fn from(err: serde_json::error::Error) -> StartPipelineExecutionError {
        StartPipelineExecutionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartPipelineExecutionError {
    fn from(err: CredentialsError) -> StartPipelineExecutionError {
        StartPipelineExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartPipelineExecutionError {
    fn from(err: HttpDispatchError) -> StartPipelineExecutionError {
        StartPipelineExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartPipelineExecutionError {
    fn from(err: io::Error) -> StartPipelineExecutionError {
        StartPipelineExecutionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartPipelineExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartPipelineExecutionError {
    fn description(&self) -> &str {
        match *self {
            StartPipelineExecutionError::PipelineNotFound(ref cause) => cause,
            StartPipelineExecutionError::Validation(ref cause) => cause,
            StartPipelineExecutionError::Credentials(ref err) => err.description(),
            StartPipelineExecutionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartPipelineExecutionError::ParseError(ref cause) => cause,
            StartPipelineExecutionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdatePipeline
#[derive(Debug, PartialEq)]
pub enum UpdatePipelineError {
    /// <p>The specified action declaration was specified in an invalid format.</p>
    InvalidActionDeclaration(String),
    /// <p>Reserved for future use.</p>
    InvalidBlockerDeclaration(String),
    /// <p>The specified stage declaration was specified in an invalid format.</p>
    InvalidStageDeclaration(String),
    /// <p>The specified structure was specified in an invalid format.</p>
    InvalidStructure(String),
    /// <p>The number of pipelines associated with the AWS account has exceeded the limit allowed for the account.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdatePipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdatePipelineError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidActionDeclarationException" => {
                    return UpdatePipelineError::InvalidActionDeclaration(String::from(
                        error_message,
                    ));
                }
                "InvalidBlockerDeclarationException" => {
                    return UpdatePipelineError::InvalidBlockerDeclaration(String::from(
                        error_message,
                    ));
                }
                "InvalidStageDeclarationException" => {
                    return UpdatePipelineError::InvalidStageDeclaration(String::from(error_message));
                }
                "InvalidStructureException" => {
                    return UpdatePipelineError::InvalidStructure(String::from(error_message));
                }
                "LimitExceededException" => {
                    return UpdatePipelineError::LimitExceeded(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdatePipelineError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdatePipelineError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdatePipelineError {
    fn from(err: serde_json::error::Error) -> UpdatePipelineError {
        UpdatePipelineError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdatePipelineError {
    fn from(err: CredentialsError) -> UpdatePipelineError {
        UpdatePipelineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdatePipelineError {
    fn from(err: HttpDispatchError) -> UpdatePipelineError {
        UpdatePipelineError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdatePipelineError {
    fn from(err: io::Error) -> UpdatePipelineError {
        UpdatePipelineError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdatePipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePipelineError {
    fn description(&self) -> &str {
        match *self {
            UpdatePipelineError::InvalidActionDeclaration(ref cause) => cause,
            UpdatePipelineError::InvalidBlockerDeclaration(ref cause) => cause,
            UpdatePipelineError::InvalidStageDeclaration(ref cause) => cause,
            UpdatePipelineError::InvalidStructure(ref cause) => cause,
            UpdatePipelineError::LimitExceeded(ref cause) => cause,
            UpdatePipelineError::Validation(ref cause) => cause,
            UpdatePipelineError::Credentials(ref err) => err.description(),
            UpdatePipelineError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdatePipelineError::ParseError(ref cause) => cause,
            UpdatePipelineError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the CodePipeline API. CodePipeline clients implement this trait.
pub trait CodePipeline {
    /// <p>Returns information about a specified job and whether that job has been received by the job worker. Only used for custom actions.</p>
    fn acknowledge_job(
        &self,
        input: AcknowledgeJobInput,
    ) -> RusotoFuture<AcknowledgeJobOutput, AcknowledgeJobError>;

    /// <p>Confirms a job worker has received the specified job. Only used for partner actions.</p>
    fn acknowledge_third_party_job(
        &self,
        input: AcknowledgeThirdPartyJobInput,
    ) -> RusotoFuture<AcknowledgeThirdPartyJobOutput, AcknowledgeThirdPartyJobError>;

    /// <p>Creates a new custom action that can be used in all pipelines associated with the AWS account. Only used for custom actions.</p>
    fn create_custom_action_type(
        &self,
        input: CreateCustomActionTypeInput,
    ) -> RusotoFuture<CreateCustomActionTypeOutput, CreateCustomActionTypeError>;

    /// <p>Creates a pipeline.</p>
    fn create_pipeline(
        &self,
        input: CreatePipelineInput,
    ) -> RusotoFuture<CreatePipelineOutput, CreatePipelineError>;

    /// <p><p>Marks a custom action as deleted. PollForJobs for the custom action will fail after the action is marked for deletion. Only used for custom actions.</p> <important> <p>To re-create a custom action after it has been deleted you must use a string in the version field that has never been used before. This string can be an incremented version number, for example. To restore a deleted custom action, use a JSON file that is identical to the deleted action, including the original string in the version field.</p> </important></p>
    fn delete_custom_action_type(
        &self,
        input: DeleteCustomActionTypeInput,
    ) -> RusotoFuture<(), DeleteCustomActionTypeError>;

    /// <p>Deletes the specified pipeline.</p>
    fn delete_pipeline(&self, input: DeletePipelineInput) -> RusotoFuture<(), DeletePipelineError>;

    /// <p>Deletes a previously created webhook by name. Deleting the webhook stops AWS CodePipeline from starting a pipeline every time an external event occurs. The API will return successfully when trying to delete a webhook that is already deleted. If a deleted webhook is re-created by calling PutWebhook with the same name, it will have a different URL.</p>
    fn delete_webhook(
        &self,
        input: DeleteWebhookInput,
    ) -> RusotoFuture<DeleteWebhookOutput, DeleteWebhookError>;

    /// <p>Removes the connection between the webhook that was created by CodePipeline and the external tool with events to be detected. Currently only supported for webhooks that target an action type of GitHub.</p>
    fn deregister_webhook_with_third_party(
        &self,
        input: DeregisterWebhookWithThirdPartyInput,
    ) -> RusotoFuture<DeregisterWebhookWithThirdPartyOutput, DeregisterWebhookWithThirdPartyError>;

    /// <p>Prevents artifacts in a pipeline from transitioning to the next stage in the pipeline.</p>
    fn disable_stage_transition(
        &self,
        input: DisableStageTransitionInput,
    ) -> RusotoFuture<(), DisableStageTransitionError>;

    /// <p>Enables artifacts in a pipeline to transition to a stage in a pipeline.</p>
    fn enable_stage_transition(
        &self,
        input: EnableStageTransitionInput,
    ) -> RusotoFuture<(), EnableStageTransitionError>;

    /// <p><p>Returns information about a job. Only used for custom actions.</p> <important> <p>When this API is called, AWS CodePipeline returns temporary credentials for the Amazon S3 bucket used to store artifacts for the pipeline, if the action requires access to that Amazon S3 bucket for input or output artifacts. Additionally, this API returns any secret values defined for the action.</p> </important></p>
    fn get_job_details(
        &self,
        input: GetJobDetailsInput,
    ) -> RusotoFuture<GetJobDetailsOutput, GetJobDetailsError>;

    /// <p>Returns the metadata, structure, stages, and actions of a pipeline. Can be used to return the entire structure of a pipeline in JSON format, which can then be modified and used to update the pipeline structure with <a>UpdatePipeline</a>.</p>
    fn get_pipeline(
        &self,
        input: GetPipelineInput,
    ) -> RusotoFuture<GetPipelineOutput, GetPipelineError>;

    /// <p>Returns information about an execution of a pipeline, including details about artifacts, the pipeline execution ID, and the name, version, and status of the pipeline.</p>
    fn get_pipeline_execution(
        &self,
        input: GetPipelineExecutionInput,
    ) -> RusotoFuture<GetPipelineExecutionOutput, GetPipelineExecutionError>;

    /// <p>Returns information about the state of a pipeline, including the stages and actions.</p>
    fn get_pipeline_state(
        &self,
        input: GetPipelineStateInput,
    ) -> RusotoFuture<GetPipelineStateOutput, GetPipelineStateError>;

    /// <p><p>Requests the details of a job for a third party action. Only used for partner actions.</p> <important> <p>When this API is called, AWS CodePipeline returns temporary credentials for the Amazon S3 bucket used to store artifacts for the pipeline, if the action requires access to that Amazon S3 bucket for input or output artifacts. Additionally, this API returns any secret values defined for the action.</p> </important></p>
    fn get_third_party_job_details(
        &self,
        input: GetThirdPartyJobDetailsInput,
    ) -> RusotoFuture<GetThirdPartyJobDetailsOutput, GetThirdPartyJobDetailsError>;

    /// <p>Gets a summary of all AWS CodePipeline action types associated with your account.</p>
    fn list_action_types(
        &self,
        input: ListActionTypesInput,
    ) -> RusotoFuture<ListActionTypesOutput, ListActionTypesError>;

    /// <p>Gets a summary of the most recent executions for a pipeline.</p>
    fn list_pipeline_executions(
        &self,
        input: ListPipelineExecutionsInput,
    ) -> RusotoFuture<ListPipelineExecutionsOutput, ListPipelineExecutionsError>;

    /// <p>Gets a summary of all of the pipelines associated with your account.</p>
    fn list_pipelines(
        &self,
        input: ListPipelinesInput,
    ) -> RusotoFuture<ListPipelinesOutput, ListPipelinesError>;

    /// <p>Gets a listing of all the webhooks in this region for this account. The output lists all webhooks and includes the webhook URL and ARN, as well the configuration for each webhook.</p>
    fn list_webhooks(
        &self,
        input: ListWebhooksInput,
    ) -> RusotoFuture<ListWebhooksOutput, ListWebhooksError>;

    /// <p><p>Returns information about any jobs for AWS CodePipeline to act upon. PollForJobs is only valid for action types with &quot;Custom&quot; in the owner field. If the action type contains &quot;AWS&quot; or &quot;ThirdParty&quot; in the owner field, the PollForJobs action returns an error.</p> <important> <p>When this API is called, AWS CodePipeline returns temporary credentials for the Amazon S3 bucket used to store artifacts for the pipeline, if the action requires access to that Amazon S3 bucket for input or output artifacts. Additionally, this API returns any secret values defined for the action.</p> </important></p>
    fn poll_for_jobs(
        &self,
        input: PollForJobsInput,
    ) -> RusotoFuture<PollForJobsOutput, PollForJobsError>;

    /// <p><p>Determines whether there are any third party jobs for a job worker to act on. Only used for partner actions.</p> <important> <p>When this API is called, AWS CodePipeline returns temporary credentials for the Amazon S3 bucket used to store artifacts for the pipeline, if the action requires access to that Amazon S3 bucket for input or output artifacts.</p> </important></p>
    fn poll_for_third_party_jobs(
        &self,
        input: PollForThirdPartyJobsInput,
    ) -> RusotoFuture<PollForThirdPartyJobsOutput, PollForThirdPartyJobsError>;

    /// <p>Provides information to AWS CodePipeline about new revisions to a source.</p>
    fn put_action_revision(
        &self,
        input: PutActionRevisionInput,
    ) -> RusotoFuture<PutActionRevisionOutput, PutActionRevisionError>;

    /// <p>Provides the response to a manual approval request to AWS CodePipeline. Valid responses include Approved and Rejected.</p>
    fn put_approval_result(
        &self,
        input: PutApprovalResultInput,
    ) -> RusotoFuture<PutApprovalResultOutput, PutApprovalResultError>;

    /// <p>Represents the failure of a job as returned to the pipeline by a job worker. Only used for custom actions.</p>
    fn put_job_failure_result(
        &self,
        input: PutJobFailureResultInput,
    ) -> RusotoFuture<(), PutJobFailureResultError>;

    /// <p>Represents the success of a job as returned to the pipeline by a job worker. Only used for custom actions.</p>
    fn put_job_success_result(
        &self,
        input: PutJobSuccessResultInput,
    ) -> RusotoFuture<(), PutJobSuccessResultError>;

    /// <p>Represents the failure of a third party job as returned to the pipeline by a job worker. Only used for partner actions.</p>
    fn put_third_party_job_failure_result(
        &self,
        input: PutThirdPartyJobFailureResultInput,
    ) -> RusotoFuture<(), PutThirdPartyJobFailureResultError>;

    /// <p>Represents the success of a third party job as returned to the pipeline by a job worker. Only used for partner actions.</p>
    fn put_third_party_job_success_result(
        &self,
        input: PutThirdPartyJobSuccessResultInput,
    ) -> RusotoFuture<(), PutThirdPartyJobSuccessResultError>;

    /// <p>Defines a webhook and returns a unique webhook URL generated by CodePipeline. This URL can be supplied to third party source hosting providers to call every time there's a code change. When CodePipeline receives a POST request on this URL, the pipeline defined in the webhook is started as long as the POST request satisfied the authentication and filtering requirements supplied when defining the webhook. RegisterWebhookWithThirdParty and DeregisterWebhookWithThirdParty APIs can be used to automatically configure supported third parties to call the generated webhook URL.</p>
    fn put_webhook(
        &self,
        input: PutWebhookInput,
    ) -> RusotoFuture<PutWebhookOutput, PutWebhookError>;

    /// <p>Configures a connection between the webhook that was created and the external tool with events to be detected.</p>
    fn register_webhook_with_third_party(
        &self,
        input: RegisterWebhookWithThirdPartyInput,
    ) -> RusotoFuture<RegisterWebhookWithThirdPartyOutput, RegisterWebhookWithThirdPartyError>;

    /// <p>Resumes the pipeline execution by retrying the last failed actions in a stage.</p>
    fn retry_stage_execution(
        &self,
        input: RetryStageExecutionInput,
    ) -> RusotoFuture<RetryStageExecutionOutput, RetryStageExecutionError>;

    /// <p>Starts the specified pipeline. Specifically, it begins processing the latest commit to the source location specified as part of the pipeline.</p>
    fn start_pipeline_execution(
        &self,
        input: StartPipelineExecutionInput,
    ) -> RusotoFuture<StartPipelineExecutionOutput, StartPipelineExecutionError>;

    /// <p>Updates a specified pipeline with edits or changes to its structure. Use a JSON file with the pipeline structure in conjunction with UpdatePipeline to provide the full structure of the pipeline. Updating the pipeline increases the version number of the pipeline by 1.</p>
    fn update_pipeline(
        &self,
        input: UpdatePipelineInput,
    ) -> RusotoFuture<UpdatePipelineOutput, UpdatePipelineError>;
}
/// A client for the CodePipeline API.
#[derive(Clone)]
pub struct CodePipelineClient {
    client: Client,
    region: region::Region,
}

impl CodePipelineClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CodePipelineClient {
        CodePipelineClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CodePipelineClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        CodePipelineClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl CodePipeline for CodePipelineClient {
    /// <p>Returns information about a specified job and whether that job has been received by the job worker. Only used for custom actions.</p>
    fn acknowledge_job(
        &self,
        input: AcknowledgeJobInput,
    ) -> RusotoFuture<AcknowledgeJobOutput, AcknowledgeJobError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.AcknowledgeJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AcknowledgeJobOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AcknowledgeJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Confirms a job worker has received the specified job. Only used for partner actions.</p>
    fn acknowledge_third_party_job(
        &self,
        input: AcknowledgeThirdPartyJobInput,
    ) -> RusotoFuture<AcknowledgeThirdPartyJobOutput, AcknowledgeThirdPartyJobError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.AcknowledgeThirdPartyJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AcknowledgeThirdPartyJobOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AcknowledgeThirdPartyJobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a new custom action that can be used in all pipelines associated with the AWS account. Only used for custom actions.</p>
    fn create_custom_action_type(
        &self,
        input: CreateCustomActionTypeInput,
    ) -> RusotoFuture<CreateCustomActionTypeOutput, CreateCustomActionTypeError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.CreateCustomActionType",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateCustomActionTypeOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateCustomActionTypeError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a pipeline.</p>
    fn create_pipeline(
        &self,
        input: CreatePipelineInput,
    ) -> RusotoFuture<CreatePipelineOutput, CreatePipelineError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.CreatePipeline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreatePipelineOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreatePipelineError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Marks a custom action as deleted. PollForJobs for the custom action will fail after the action is marked for deletion. Only used for custom actions.</p> <important> <p>To re-create a custom action after it has been deleted you must use a string in the version field that has never been used before. This string can be an incremented version number, for example. To restore a deleted custom action, use a JSON file that is identical to the deleted action, including the original string in the version field.</p> </important></p>
    fn delete_custom_action_type(
        &self,
        input: DeleteCustomActionTypeInput,
    ) -> RusotoFuture<(), DeleteCustomActionTypeError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.DeleteCustomActionType",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteCustomActionTypeError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes the specified pipeline.</p>
    fn delete_pipeline(&self, input: DeletePipelineInput) -> RusotoFuture<(), DeletePipelineError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.DeletePipeline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeletePipelineError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a previously created webhook by name. Deleting the webhook stops AWS CodePipeline from starting a pipeline every time an external event occurs. The API will return successfully when trying to delete a webhook that is already deleted. If a deleted webhook is re-created by calling PutWebhook with the same name, it will have a different URL.</p>
    fn delete_webhook(
        &self,
        input: DeleteWebhookInput,
    ) -> RusotoFuture<DeleteWebhookOutput, DeleteWebhookError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.DeleteWebhook");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteWebhookOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteWebhookError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes the connection between the webhook that was created by CodePipeline and the external tool with events to be detected. Currently only supported for webhooks that target an action type of GitHub.</p>
    fn deregister_webhook_with_third_party(
        &self,
        input: DeregisterWebhookWithThirdPartyInput,
    ) -> RusotoFuture<DeregisterWebhookWithThirdPartyOutput, DeregisterWebhookWithThirdPartyError>
    {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.DeregisterWebhookWithThirdParty",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeregisterWebhookWithThirdPartyOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeregisterWebhookWithThirdPartyError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Prevents artifacts in a pipeline from transitioning to the next stage in the pipeline.</p>
    fn disable_stage_transition(
        &self,
        input: DisableStageTransitionInput,
    ) -> RusotoFuture<(), DisableStageTransitionError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.DisableStageTransition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DisableStageTransitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Enables artifacts in a pipeline to transition to a stage in a pipeline.</p>
    fn enable_stage_transition(
        &self,
        input: EnableStageTransitionInput,
    ) -> RusotoFuture<(), EnableStageTransitionError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.EnableStageTransition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(EnableStageTransitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Returns information about a job. Only used for custom actions.</p> <important> <p>When this API is called, AWS CodePipeline returns temporary credentials for the Amazon S3 bucket used to store artifacts for the pipeline, if the action requires access to that Amazon S3 bucket for input or output artifacts. Additionally, this API returns any secret values defined for the action.</p> </important></p>
    fn get_job_details(
        &self,
        input: GetJobDetailsInput,
    ) -> RusotoFuture<GetJobDetailsOutput, GetJobDetailsError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.GetJobDetails");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetJobDetailsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetJobDetailsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the metadata, structure, stages, and actions of a pipeline. Can be used to return the entire structure of a pipeline in JSON format, which can then be modified and used to update the pipeline structure with <a>UpdatePipeline</a>.</p>
    fn get_pipeline(
        &self,
        input: GetPipelineInput,
    ) -> RusotoFuture<GetPipelineOutput, GetPipelineError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.GetPipeline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetPipelineOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetPipelineError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about an execution of a pipeline, including details about artifacts, the pipeline execution ID, and the name, version, and status of the pipeline.</p>
    fn get_pipeline_execution(
        &self,
        input: GetPipelineExecutionInput,
    ) -> RusotoFuture<GetPipelineExecutionOutput, GetPipelineExecutionError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.GetPipelineExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetPipelineExecutionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetPipelineExecutionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns information about the state of a pipeline, including the stages and actions.</p>
    fn get_pipeline_state(
        &self,
        input: GetPipelineStateInput,
    ) -> RusotoFuture<GetPipelineStateOutput, GetPipelineStateError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.GetPipelineState");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetPipelineStateOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetPipelineStateError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Requests the details of a job for a third party action. Only used for partner actions.</p> <important> <p>When this API is called, AWS CodePipeline returns temporary credentials for the Amazon S3 bucket used to store artifacts for the pipeline, if the action requires access to that Amazon S3 bucket for input or output artifacts. Additionally, this API returns any secret values defined for the action.</p> </important></p>
    fn get_third_party_job_details(
        &self,
        input: GetThirdPartyJobDetailsInput,
    ) -> RusotoFuture<GetThirdPartyJobDetailsOutput, GetThirdPartyJobDetailsError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.GetThirdPartyJobDetails",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetThirdPartyJobDetailsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetThirdPartyJobDetailsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets a summary of all AWS CodePipeline action types associated with your account.</p>
    fn list_action_types(
        &self,
        input: ListActionTypesInput,
    ) -> RusotoFuture<ListActionTypesOutput, ListActionTypesError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.ListActionTypes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListActionTypesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListActionTypesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a summary of the most recent executions for a pipeline.</p>
    fn list_pipeline_executions(
        &self,
        input: ListPipelineExecutionsInput,
    ) -> RusotoFuture<ListPipelineExecutionsOutput, ListPipelineExecutionsError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.ListPipelineExecutions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListPipelineExecutionsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListPipelineExecutionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets a summary of all of the pipelines associated with your account.</p>
    fn list_pipelines(
        &self,
        input: ListPipelinesInput,
    ) -> RusotoFuture<ListPipelinesOutput, ListPipelinesError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.ListPipelines");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListPipelinesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListPipelinesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a listing of all the webhooks in this region for this account. The output lists all webhooks and includes the webhook URL and ARN, as well the configuration for each webhook.</p>
    fn list_webhooks(
        &self,
        input: ListWebhooksInput,
    ) -> RusotoFuture<ListWebhooksOutput, ListWebhooksError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.ListWebhooks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListWebhooksOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListWebhooksError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Returns information about any jobs for AWS CodePipeline to act upon. PollForJobs is only valid for action types with &quot;Custom&quot; in the owner field. If the action type contains &quot;AWS&quot; or &quot;ThirdParty&quot; in the owner field, the PollForJobs action returns an error.</p> <important> <p>When this API is called, AWS CodePipeline returns temporary credentials for the Amazon S3 bucket used to store artifacts for the pipeline, if the action requires access to that Amazon S3 bucket for input or output artifacts. Additionally, this API returns any secret values defined for the action.</p> </important></p>
    fn poll_for_jobs(
        &self,
        input: PollForJobsInput,
    ) -> RusotoFuture<PollForJobsOutput, PollForJobsError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.PollForJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PollForJobsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PollForJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Determines whether there are any third party jobs for a job worker to act on. Only used for partner actions.</p> <important> <p>When this API is called, AWS CodePipeline returns temporary credentials for the Amazon S3 bucket used to store artifacts for the pipeline, if the action requires access to that Amazon S3 bucket for input or output artifacts.</p> </important></p>
    fn poll_for_third_party_jobs(
        &self,
        input: PollForThirdPartyJobsInput,
    ) -> RusotoFuture<PollForThirdPartyJobsOutput, PollForThirdPartyJobsError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.PollForThirdPartyJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PollForThirdPartyJobsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PollForThirdPartyJobsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Provides information to AWS CodePipeline about new revisions to a source.</p>
    fn put_action_revision(
        &self,
        input: PutActionRevisionInput,
    ) -> RusotoFuture<PutActionRevisionOutput, PutActionRevisionError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.PutActionRevision");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutActionRevisionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutActionRevisionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Provides the response to a manual approval request to AWS CodePipeline. Valid responses include Approved and Rejected.</p>
    fn put_approval_result(
        &self,
        input: PutApprovalResultInput,
    ) -> RusotoFuture<PutApprovalResultOutput, PutApprovalResultError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.PutApprovalResult");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutApprovalResultOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutApprovalResultError::from_response(response))),
                )
            }
        })
    }

    /// <p>Represents the failure of a job as returned to the pipeline by a job worker. Only used for custom actions.</p>
    fn put_job_failure_result(
        &self,
        input: PutJobFailureResultInput,
    ) -> RusotoFuture<(), PutJobFailureResultError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.PutJobFailureResult");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutJobFailureResultError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Represents the success of a job as returned to the pipeline by a job worker. Only used for custom actions.</p>
    fn put_job_success_result(
        &self,
        input: PutJobSuccessResultInput,
    ) -> RusotoFuture<(), PutJobSuccessResultError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.PutJobSuccessResult");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutJobSuccessResultError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Represents the failure of a third party job as returned to the pipeline by a job worker. Only used for partner actions.</p>
    fn put_third_party_job_failure_result(
        &self,
        input: PutThirdPartyJobFailureResultInput,
    ) -> RusotoFuture<(), PutThirdPartyJobFailureResultError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.PutThirdPartyJobFailureResult",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutThirdPartyJobFailureResultError::from_response(response))
                }))
            }
        })
    }

    /// <p>Represents the success of a third party job as returned to the pipeline by a job worker. Only used for partner actions.</p>
    fn put_third_party_job_success_result(
        &self,
        input: PutThirdPartyJobSuccessResultInput,
    ) -> RusotoFuture<(), PutThirdPartyJobSuccessResultError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.PutThirdPartyJobSuccessResult",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutThirdPartyJobSuccessResultError::from_response(response))
                }))
            }
        })
    }

    /// <p>Defines a webhook and returns a unique webhook URL generated by CodePipeline. This URL can be supplied to third party source hosting providers to call every time there's a code change. When CodePipeline receives a POST request on this URL, the pipeline defined in the webhook is started as long as the POST request satisfied the authentication and filtering requirements supplied when defining the webhook. RegisterWebhookWithThirdParty and DeregisterWebhookWithThirdParty APIs can be used to automatically configure supported third parties to call the generated webhook URL.</p>
    fn put_webhook(
        &self,
        input: PutWebhookInput,
    ) -> RusotoFuture<PutWebhookOutput, PutWebhookError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.PutWebhook");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutWebhookOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutWebhookError::from_response(response))),
                )
            }
        })
    }

    /// <p>Configures a connection between the webhook that was created and the external tool with events to be detected.</p>
    fn register_webhook_with_third_party(
        &self,
        input: RegisterWebhookWithThirdPartyInput,
    ) -> RusotoFuture<RegisterWebhookWithThirdPartyOutput, RegisterWebhookWithThirdPartyError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.RegisterWebhookWithThirdParty",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RegisterWebhookWithThirdPartyOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RegisterWebhookWithThirdPartyError::from_response(response))
                }))
            }
        })
    }

    /// <p>Resumes the pipeline execution by retrying the last failed actions in a stage.</p>
    fn retry_stage_execution(
        &self,
        input: RetryStageExecutionInput,
    ) -> RusotoFuture<RetryStageExecutionOutput, RetryStageExecutionError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.RetryStageExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RetryStageExecutionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(RetryStageExecutionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Starts the specified pipeline. Specifically, it begins processing the latest commit to the source location specified as part of the pipeline.</p>
    fn start_pipeline_execution(
        &self,
        input: StartPipelineExecutionInput,
    ) -> RusotoFuture<StartPipelineExecutionOutput, StartPipelineExecutionError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.StartPipelineExecution",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartPipelineExecutionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartPipelineExecutionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates a specified pipeline with edits or changes to its structure. Use a JSON file with the pipeline structure in conjunction with UpdatePipeline to provide the full structure of the pipeline. Updating the pipeline increases the version number of the pipeline by 1.</p>
    fn update_pipeline(
        &self,
        input: UpdatePipelineInput,
    ) -> RusotoFuture<UpdatePipelineOutput, UpdatePipelineError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.UpdatePipeline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdatePipelineOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdatePipelineError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}

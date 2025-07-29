//! OpenAI API types.
//!
//! This file is auto-generated.

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(tag = "type")]
pub enum Annotation {
    #[serde(rename = "file_citation")]
    FileCitationBody {
        /// The ID of the file.
        file_id: String,
        /// The filename of the file cited.
        filename: String,
        /// The index of the file in the list of files.
        index: i64,
    },
    #[serde(rename = "url_citation")]
    UrlCitationBody {
        /// The index of the last character of the URL citation in the message.
        end_index: i64,
        /// The index of the first character of the URL citation in the message.
        start_index: i64,
        /// The title of the web resource.
        title: String,
        /// The URL of the web resource.
        url: String,
    },
    #[serde(rename = "container_file_citation")]
    ContainerFileCitationBody {
        /// The ID of the container file.
        container_id: String,
        /// The index of the last character of the container file citation in the message.
        end_index: i64,
        /// The ID of the file.
        file_id: String,
        /// The filename of the container file cited.
        filename: String,
        /// The index of the first character of the container file citation in the message.
        start_index: i64,
    },
    #[serde(rename = "file_path")]
    FilePath {
        /// The ID of the file.
        file_id: String,
        /// The index of the file in the list of files.
        index: i64,
    },
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ApproximateLocation {
    /// Free text input for the city of the user, e.g. `San Francisco`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The two-letter [ISO country code](https://en.wikipedia.org/wiki/ISO_3166-1) of the user, e.g. `US`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Free text input for the region of the user, e.g. `California`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The [IANA timezone](https://timeapi.io/documentation/iana-timezones) of the user, e.g. `America/Los_Angeles`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// The type of location approximation. Always `approximate`.
    #[serde(rename = "type")]
    pub r#type: ApproximateLocationType,
}

/// The type of location approximation. Always `approximate`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ApproximateLocationType {
    #[default]
    #[serde(rename = "approximate")]
    Approximate,
}

impl ApproximateLocationType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Approximate => "approximate",
        }
    }
}

/// A click action.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Click {
    /// Indicates which mouse button was pressed during the click. One of `left`, `right`, `wheel`, `back`, or `forward`.
    pub button: ClickButton,
    /// Specifies the event type. For a click action, this property is
    /// always set to `click`.
    #[serde(rename = "type")]
    pub r#type: ClickType,
    /// The x-coordinate where the click occurred.
    pub x: i64,
    /// The y-coordinate where the click occurred.
    pub y: i64,
}

/// Indicates which mouse button was pressed during the click. One of `left`, `right`, `wheel`, `back`, or `forward`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum ClickButton {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "wheel")]
    Wheel,
    #[serde(rename = "back")]
    Back,
    #[serde(rename = "forward")]
    Forward,
}

impl ClickButton {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Wheel => "wheel",
            Self::Back => "back",
            Self::Forward => "forward",
        }
    }
}

/// Specifies the event type. For a click action, this property is
/// always set to `click`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ClickType {
    #[default]
    #[serde(rename = "click")]
    Click,
}

impl ClickType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Click => "click",
        }
    }
}

/// The image output from the code interpreter.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct CodeInterpreterOutputImage {
    /// The type of the output. Always 'image'.
    #[serde(rename = "type")]
    pub r#type: CodeInterpreterOutputImageType,
    /// The URL of the image output from the code interpreter.
    pub url: String,
}

/// The type of the output. Always 'image'.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum CodeInterpreterOutputImageType {
    #[default]
    #[serde(rename = "image")]
    Image,
}

impl CodeInterpreterOutputImageType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Image => "image",
        }
    }
}

/// The logs output from the code interpreter.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct CodeInterpreterOutputLogs {
    /// The logs output from the code interpreter.
    pub logs: String,
    /// The type of the output. Always 'logs'.
    #[serde(rename = "type")]
    pub r#type: CodeInterpreterOutputLogsType,
}

/// The type of the output. Always 'logs'.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum CodeInterpreterOutputLogsType {
    #[default]
    #[serde(rename = "logs")]
    Logs,
}

impl CodeInterpreterOutputLogsType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Logs => "logs",
        }
    }
}

/// A tool that runs Python code to help generate a response to a prompt.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct CodeInterpreterTool {
    /// The code interpreter container. Can be a container ID or an object that
    /// specifies uploaded file IDs to make available to your code.
    pub container: CodeInterpreterToolContainer,
    /// The type of the code interpreter tool. Always `code_interpreter`.
    #[serde(rename = "type")]
    pub r#type: CodeInterpreterToolType,
}

/// Configuration for a code interpreter container. Optionally specify the IDs
/// of the files to run the code on.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct CodeInterpreterToolAuto {
    /// An optional list of uploaded files to make available to your code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    /// Always `auto`.
    #[serde(rename = "type")]
    pub r#type: CodeInterpreterToolAutoType,
}

/// Always `auto`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum CodeInterpreterToolAutoType {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}

impl CodeInterpreterToolAutoType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Auto => "auto",
        }
    }
}

/// A tool call to run code.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct CodeInterpreterToolCall {
    /// The code to run, or null if not available.
    pub code: Option<String>,
    /// The ID of the container used to run the code.
    pub container_id: String,
    /// The unique ID of the code interpreter tool call.
    pub id: String,
    /// The outputs generated by the code interpreter, such as logs or images.
    /// Can be null if no outputs are available.
    pub outputs: Option<Vec<CodeInterpreterToolCallOutputsItem>>,
    /// The status of the code interpreter tool call. Valid values are `in_progress`, `completed`, `incomplete`, `interpreting`, and `failed`.
    pub status: CodeInterpreterToolCallStatus,
    /// The type of the code interpreter tool call. Always `code_interpreter_call`.
    #[serde(rename = "type")]
    pub r#type: CodeInterpreterToolCallType,
}

impl crate::HasId for CodeInterpreterToolCall {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum CodeInterpreterToolCallOutputsItem {
    #[serde(untagged)]
    CodeInterpreterOutputLogs(CodeInterpreterOutputLogs),
    #[serde(untagged)]
    CodeInterpreterOutputImage(CodeInterpreterOutputImage),
}

impl From<CodeInterpreterOutputImage> for CodeInterpreterToolCallOutputsItem {
    fn from(value: CodeInterpreterOutputImage) -> Self {
        CodeInterpreterToolCallOutputsItem::CodeInterpreterOutputImage(value)
    }
}
impl From<CodeInterpreterOutputLogs> for CodeInterpreterToolCallOutputsItem {
    fn from(value: CodeInterpreterOutputLogs) -> Self {
        CodeInterpreterToolCallOutputsItem::CodeInterpreterOutputLogs(value)
    }
}
/// The status of the code interpreter tool call. Valid values are `in_progress`, `completed`, `incomplete`, `interpreting`, and `failed`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum CodeInterpreterToolCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "interpreting")]
    Interpreting,
    #[serde(rename = "failed")]
    Failed,
}

impl CodeInterpreterToolCallStatus {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Incomplete => "incomplete",
            Self::Interpreting => "interpreting",
            Self::Failed => "failed",
        }
    }
}

/// The type of the code interpreter tool call. Always `code_interpreter_call`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum CodeInterpreterToolCallType {
    #[default]
    #[serde(rename = "code_interpreter_call")]
    CodeInterpreterCall,
}

impl CodeInterpreterToolCallType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::CodeInterpreterCall => "code_interpreter_call",
        }
    }
}

/// The code interpreter container. Can be a container ID or an object that
/// specifies uploaded file IDs to make available to your code.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum CodeInterpreterToolContainer {
    #[serde(untagged)]
    Variant0(String),
    #[serde(untagged)]
    CodeInterpreterToolAuto(CodeInterpreterToolAuto),
}

impl From<CodeInterpreterToolAuto> for CodeInterpreterToolContainer {
    fn from(value: CodeInterpreterToolAuto) -> Self {
        CodeInterpreterToolContainer::CodeInterpreterToolAuto(value)
    }
}
impl From<String> for CodeInterpreterToolContainer {
    fn from(value: String) -> Self {
        CodeInterpreterToolContainer::Variant0(value)
    }
}
/// The type of the code interpreter tool. Always `code_interpreter`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum CodeInterpreterToolType {
    #[default]
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
}

impl CodeInterpreterToolType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::CodeInterpreter => "code_interpreter",
        }
    }
}

/// A filter used to compare a specified attribute key to a given value using a defined comparison operation.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ComparisonFilter {
    /// The key to compare against the value.
    pub key: String,
    /// Specifies the comparison operator: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`.
    /// - `eq`: equals
    /// - `ne`: not equal
    /// - `gt`: greater than
    /// - `gte`: greater than or equal
    /// - `lt`: less than
    /// - `lte`: less than or equal
    #[serde(rename = "type")]
    pub r#type: ComparisonFilterType,
    /// The value to compare against the attribute key; supports string, number, or boolean types.
    pub value: ComparisonFilterValue,
}

/// Specifies the comparison operator: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`.
/// - `eq`: equals
/// - `ne`: not equal
/// - `gt`: greater than
/// - `gte`: greater than or equal
/// - `lt`: less than
/// - `lte`: less than or equal
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ComparisonFilterType {
    #[default]
    #[serde(rename = "eq")]
    Eq,
    #[serde(rename = "ne")]
    Ne,
    #[serde(rename = "gt")]
    Gt,
    #[serde(rename = "gte")]
    Gte,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "lte")]
    Lte,
}

impl ComparisonFilterType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Eq => "eq",
            Self::Ne => "ne",
            Self::Gt => "gt",
            Self::Gte => "gte",
            Self::Lt => "lt",
            Self::Lte => "lte",
        }
    }
}

/// The value to compare against the attribute key; supports string, number, or boolean types.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum ComparisonFilterValue {
    #[serde(untagged)]
    Variant0(String),
    #[serde(untagged)]
    Variant1(f64),
    #[serde(untagged)]
    Variant2(bool),
}

impl From<String> for ComparisonFilterValue {
    fn from(value: String) -> Self {
        ComparisonFilterValue::Variant0(value)
    }
}
impl From<f64> for ComparisonFilterValue {
    fn from(value: f64) -> Self {
        ComparisonFilterValue::Variant1(value)
    }
}
impl From<bool> for ComparisonFilterValue {
    fn from(value: bool) -> Self {
        ComparisonFilterValue::Variant2(value)
    }
}
/// Combine multiple filters using `and` or `or`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct CompoundFilter {
    /// Array of filters to combine. Items can be `ComparisonFilter` or `CompoundFilter`.
    pub filters: Vec<CompoundFilterFiltersItem>,
    /// Type of operation: `and` or `or`.
    #[serde(rename = "type")]
    pub r#type: CompoundFilterType,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum CompoundFilterFiltersItem {
    #[serde(untagged)]
    ComparisonFilter(ComparisonFilter),
    #[serde(untagged)]
    Variant1(serde_json::Map<String, serde_json::Value>),
}

impl From<ComparisonFilter> for CompoundFilterFiltersItem {
    fn from(value: ComparisonFilter) -> Self {
        CompoundFilterFiltersItem::ComparisonFilter(value)
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for CompoundFilterFiltersItem {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        CompoundFilterFiltersItem::Variant1(value)
    }
}
/// Type of operation: `and` or `or`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum CompoundFilterType {
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
}

impl CompoundFilterType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::And => "and",
            Self::Or => "or",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum ComputerAction {
    #[serde(untagged)]
    Click(Click),
    #[serde(untagged)]
    DoubleClick(DoubleClick),
    #[serde(untagged)]
    Drag(Drag),
    #[serde(untagged)]
    KeyPress(KeyPress),
    #[serde(untagged)]
    Move(Move),
    #[serde(untagged)]
    Screenshot(Screenshot),
    #[serde(untagged)]
    Scroll(Scroll),
    #[serde(untagged)]
    Type(Type),
    #[serde(untagged)]
    Wait(Wait),
}

impl From<Click> for ComputerAction {
    fn from(value: Click) -> Self {
        ComputerAction::Click(value)
    }
}
impl From<DoubleClick> for ComputerAction {
    fn from(value: DoubleClick) -> Self {
        ComputerAction::DoubleClick(value)
    }
}
impl From<Drag> for ComputerAction {
    fn from(value: Drag) -> Self {
        ComputerAction::Drag(value)
    }
}
impl From<KeyPress> for ComputerAction {
    fn from(value: KeyPress) -> Self {
        ComputerAction::KeyPress(value)
    }
}
impl From<Move> for ComputerAction {
    fn from(value: Move) -> Self {
        ComputerAction::Move(value)
    }
}
impl From<Screenshot> for ComputerAction {
    fn from(value: Screenshot) -> Self {
        ComputerAction::Screenshot(value)
    }
}
impl From<Scroll> for ComputerAction {
    fn from(value: Scroll) -> Self {
        ComputerAction::Scroll(value)
    }
}
impl From<Type> for ComputerAction {
    fn from(value: Type) -> Self {
        ComputerAction::Type(value)
    }
}
impl From<Wait> for ComputerAction {
    fn from(value: Wait) -> Self {
        ComputerAction::Wait(value)
    }
}
/// The output of a computer tool call.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ComputerCallOutputItemParam {
    /// The safety checks reported by the API that have been acknowledged by the developer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acknowledged_safety_checks: Option<Vec<ComputerCallSafetyCheckParam>>,
    /// The ID of the computer tool call that produced the output.
    pub call_id: String,
    /// The ID of the computer tool call output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub output: ComputerScreenshotImage,
    /// The status of the message input. One of `in_progress`, `completed`, or `incomplete`. Populated when input items are returned via API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The type of the computer tool call output. Always `computer_call_output`.
    #[serde(rename = "type")]
    pub r#type: ComputerCallOutputItemParamType,
}

impl crate::HasId for ComputerCallOutputItemParam {
    fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}

/// The type of the computer tool call output. Always `computer_call_output`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ComputerCallOutputItemParamType {
    #[default]
    #[serde(rename = "computer_call_output")]
    ComputerCallOutput,
}

impl ComputerCallOutputItemParamType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ComputerCallOutput => "computer_call_output",
        }
    }
}

/// A pending safety check for the computer call.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ComputerCallSafetyCheckParam {
    /// The type of the pending safety check.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The ID of the pending safety check.
    pub id: String,
    /// Details about the pending safety check.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl crate::HasId for ComputerCallSafetyCheckParam {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}

/// A computer screenshot image used with the computer use tool.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ComputerScreenshotImage {
    /// The identifier of an uploaded file that contains the screenshot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// The URL of the screenshot image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// Specifies the event type. For a computer screenshot, this property is
    /// always set to `computer_screenshot`.
    #[serde(rename = "type")]
    pub r#type: ComputerScreenshotImageType,
}

/// Specifies the event type. For a computer screenshot, this property is
/// always set to `computer_screenshot`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ComputerScreenshotImageType {
    #[default]
    #[serde(rename = "computer_screenshot")]
    ComputerScreenshot,
}

impl ComputerScreenshotImageType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ComputerScreenshot => "computer_screenshot",
        }
    }
}

/// A tool call to a computer use tool. See the
/// [computer use guide](/docs/guides/tools-computer-use) for more information.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ComputerToolCall {
    pub action: ComputerAction,
    /// An identifier used when responding to the tool call with output.
    pub call_id: String,
    /// The unique ID of the computer call.
    pub id: String,
    /// The pending safety checks for the computer call.
    pub pending_safety_checks: Vec<ComputerToolCallSafetyCheck>,
    /// The status of the item. One of `in_progress`, `completed`, or
    /// `incomplete`. Populated when items are returned via API.
    pub status: Status,
    /// The type of the computer call. Always `computer_call`.
    #[serde(rename = "type")]
    pub r#type: ComputerToolCallType,
}

impl crate::HasId for ComputerToolCall {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}

/// A pending safety check for the computer call.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ComputerToolCallSafetyCheck {
    /// The type of the pending safety check.
    pub code: String,
    /// The ID of the pending safety check.
    pub id: String,
    /// Details about the pending safety check.
    pub message: String,
}

impl crate::HasId for ComputerToolCallSafetyCheck {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}

/// The type of the computer call. Always `computer_call`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ComputerToolCallType {
    #[default]
    #[serde(rename = "computer_call")]
    ComputerCall,
}

impl ComputerToolCallType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ComputerCall => "computer_call",
        }
    }
}

/// A tool that controls a virtual computer. Learn more about the [computer tool](https://platform.openai.com/docs/guides/tools-computer-use).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ComputerUsePreviewTool {
    /// The height of the computer display.
    pub display_height: i64,
    /// The width of the computer display.
    pub display_width: i64,
    /// The type of computer environment to control.
    pub environment: ComputerUsePreviewToolEnvironment,
    /// The type of the computer use tool. Always `computer_use_preview`.
    #[serde(rename = "type")]
    pub r#type: ComputerUsePreviewToolType,
}

/// The type of computer environment to control.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum ComputerUsePreviewToolEnvironment {
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "mac")]
    Mac,
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "ubuntu")]
    Ubuntu,
    #[serde(rename = "browser")]
    Browser,
}

impl ComputerUsePreviewToolEnvironment {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Windows => "windows",
            Self::Mac => "mac",
            Self::Linux => "linux",
            Self::Ubuntu => "ubuntu",
            Self::Browser => "browser",
        }
    }
}

/// The type of the computer use tool. Always `computer_use_preview`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ComputerUsePreviewToolType {
    #[default]
    #[serde(rename = "computer_use_preview")]
    ComputerUsePreview,
}

impl ComputerUsePreviewToolType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ComputerUsePreview => "computer_use_preview",
        }
    }
}

/// A citation for a container file used to generate a model response.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ContainerFileCitationBody {
    /// The ID of the container file.
    pub container_id: String,
    /// The index of the last character of the container file citation in the message.
    pub end_index: i64,
    /// The ID of the file.
    pub file_id: String,
    /// The filename of the container file cited.
    pub filename: String,
    /// The index of the first character of the container file citation in the message.
    pub start_index: i64,
    /// The type of the container file citation. Always `container_file_citation`.
    #[serde(rename = "type")]
    pub r#type: ContainerFileCitationBodyType,
}

/// The type of the container file citation. Always `container_file_citation`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ContainerFileCitationBodyType {
    #[default]
    #[serde(rename = "container_file_citation")]
    ContainerFileCitation,
}

impl ContainerFileCitationBodyType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ContainerFileCitation => "container_file_citation",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum ConversationItemRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
}

impl ConversationItemRole {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::User => "user",
            Self::Assistant => "assistant",
            Self::System => "system",
        }
    }
}

/// An x/y coordinate pair, e.g. `{ x: 100, y: 200 }`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Coordinate {
    /// The x-coordinate.
    pub x: i64,
    /// The y-coordinate.
    pub y: i64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct CreateModelResponseProperties {
    #[serde(flatten)]
    pub model_response_properties: ModelResponseProperties,
    /// An integer between 0 and 20 specifying the number of most likely tokens to
    /// return at each token position, each with an associated log probability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i64>,
}

impl AsRef<ModelResponseProperties> for CreateModelResponseProperties {
    fn as_ref(&self) -> &ModelResponseProperties {
        &self.model_response_properties
    }
}
impl AsMut<ModelResponseProperties> for CreateModelResponseProperties {
    fn as_mut(&mut self) -> &mut ModelResponseProperties {
        &mut self.model_response_properties
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct CreateResponse {
    #[serde(flatten)]
    pub create_model_response_properties: CreateModelResponseProperties,
    #[serde(flatten)]
    pub response_properties: ResponseProperties,
    /// Specify additional output data to include in the model response. Currently
    /// supported values are:
    /// - `code_interpreter_call.outputs`: Includes the outputs of python code execution
    ///   in code interpreter tool call items.
    /// - `computer_call_output.output.image_url`: Include image urls from the computer call output.
    /// - `file_search_call.results`: Include the search results of
    ///   the file search tool call.
    /// - `message.input_image.image_url`: Include image urls from the input message.
    /// - `message.output_text.logprobs`: Include logprobs with assistant messages.
    /// - `reasoning.encrypted_content`: Includes an encrypted version of reasoning
    ///   tokens in reasoning item outputs. This enables reasoning items to be used in
    ///   multi-turn conversations when using the Responses API statelessly (like
    ///   when the `store` parameter is set to `false`, or when an organization is
    ///   enrolled in the zero data retention program).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<Includable>>,
    /// Text, image, or file inputs to the model, used to generate a response.
    ///
    /// Learn more:
    /// - [Text inputs and outputs](/docs/guides/text)
    /// - [Image inputs](/docs/guides/images)
    /// - [File inputs](/docs/guides/pdf-files)
    /// - [Conversation state](/docs/guides/conversation-state)
    /// - [Function calling](/docs/guides/function-calling)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<CreateResponseInput>,
    /// A system (or developer) message inserted into the model's context.
    ///
    /// When using along with `previous_response_id`, the instructions from a previous
    /// response will not be carried over to the next response. This makes it simple
    /// to swap out system (or developer) messages in new responses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Whether to allow the model to run tool calls in parallel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    /// Whether to store the generated model response for later retrieval via
    /// API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// If set to true, the model response data will be streamed to the client
    /// as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).
    /// See the [Streaming section below](/docs/api-reference/responses-streaming)
    /// for more information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

impl AsRef<CreateModelResponseProperties> for CreateResponse {
    fn as_ref(&self) -> &CreateModelResponseProperties {
        &self.create_model_response_properties
    }
}
impl AsMut<CreateModelResponseProperties> for CreateResponse {
    fn as_mut(&mut self) -> &mut CreateModelResponseProperties {
        &mut self.create_model_response_properties
    }
}
impl AsRef<ResponseProperties> for CreateResponse {
    fn as_ref(&self) -> &ResponseProperties {
        &self.response_properties
    }
}
impl AsMut<ResponseProperties> for CreateResponse {
    fn as_mut(&mut self) -> &mut ResponseProperties {
        &mut self.response_properties
    }
}
/// Text, image, or file inputs to the model, used to generate a response.
///
/// Learn more:
/// - [Text inputs and outputs](/docs/guides/text)
/// - [Image inputs](/docs/guides/images)
/// - [File inputs](/docs/guides/pdf-files)
/// - [Conversation state](/docs/guides/conversation-state)
/// - [Function calling](/docs/guides/function-calling)
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum CreateResponseInput {
    #[serde(untagged)]
    Variant0(String),
    #[serde(untagged)]
    Variant1(Vec<InputItem>),
}

impl From<String> for CreateResponseInput {
    fn from(value: String) -> Self {
        CreateResponseInput::Variant0(value)
    }
}
impl From<Vec<InputItem>> for CreateResponseInput {
    fn from(value: Vec<InputItem>) -> Self {
        CreateResponseInput::Variant1(value)
    }
}
/// A double click action.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct DoubleClick {
    /// Specifies the event type. For a double click action, this property is
    /// always set to `double_click`.
    #[serde(rename = "type")]
    pub r#type: DoubleClickType,
    /// The x-coordinate where the double click occurred.
    pub x: i64,
    /// The y-coordinate where the double click occurred.
    pub y: i64,
}

/// Specifies the event type. For a double click action, this property is
/// always set to `double_click`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum DoubleClickType {
    #[default]
    #[serde(rename = "double_click")]
    DoubleClick,
}

impl DoubleClickType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::DoubleClick => "double_click",
        }
    }
}

/// A drag action.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Drag {
    /// An array of coordinates representing the path of the drag action. Coordinates will appear as an array
    /// of objects, eg
    /// ```txt
    /// [
    ///   { x: 100, y: 200 },
    ///   { x: 200, y: 300 }
    /// ]
    /// ```
    pub path: Vec<Coordinate>,
    /// Specifies the event type. For a drag action, this property is
    /// always set to `drag`.
    #[serde(rename = "type")]
    pub r#type: DragType,
}

/// Specifies the event type. For a drag action, this property is
/// always set to `drag`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum DragType {
    #[default]
    #[serde(rename = "drag")]
    Drag,
}

impl DragType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Drag => "drag",
        }
    }
}

/// A message input to the model with a role indicating instruction following
/// hierarchy. Instructions given with the `developer` or `system` role take
/// precedence over instructions given with the `user` role. Messages with the
/// `assistant` role are presumed to have been generated by the model in previous
/// interactions.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct EasyInputMessage {
    /// Text, image, or audio input to the model, used to generate a response.
    /// Can also contain previous assistant responses.
    pub content: EasyInputMessageContent,
    /// The role of the message input. One of `user`, `assistant`, `system`, or
    /// `developer`.
    pub role: EasyInputMessageRole,
    /// The type of the message input. Always `message`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<EasyInputMessageType>,
}

/// Text, image, or audio input to the model, used to generate a response.
/// Can also contain previous assistant responses.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum EasyInputMessageContent {
    #[serde(untagged)]
    Variant0(String),
    #[serde(untagged)]
    InputMessageContentList(InputMessageContentList),
}

impl From<InputMessageContentList> for EasyInputMessageContent {
    fn from(value: InputMessageContentList) -> Self {
        EasyInputMessageContent::InputMessageContentList(value)
    }
}
impl From<String> for EasyInputMessageContent {
    fn from(value: String) -> Self {
        EasyInputMessageContent::Variant0(value)
    }
}
/// The role of the message input. One of `user`, `assistant`, `system`, or
/// `developer`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum EasyInputMessageRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "developer")]
    Developer,
}

impl EasyInputMessageRole {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::User => "user",
            Self::Assistant => "assistant",
            Self::System => "system",
            Self::Developer => "developer",
        }
    }
}

/// The type of the message input. Always `message`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum EasyInputMessageType {
    #[default]
    #[serde(rename = "message")]
    Message,
}

impl EasyInputMessageType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Message => "message",
        }
    }
}

/// A citation to a file.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct FileCitationBody {
    /// The ID of the file.
    pub file_id: String,
    /// The filename of the file cited.
    pub filename: String,
    /// The index of the file in the list of files.
    pub index: i64,
    /// The type of the file citation. Always `file_citation`.
    #[serde(rename = "type")]
    pub r#type: FileCitationBodyType,
}

/// The type of the file citation. Always `file_citation`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum FileCitationBodyType {
    #[default]
    #[serde(rename = "file_citation")]
    FileCitation,
}

impl FileCitationBodyType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::FileCitation => "file_citation",
        }
    }
}

/// A path to a file.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct FilePath {
    /// The ID of the file.
    pub file_id: String,
    /// The index of the file in the list of files.
    pub index: i64,
    /// The type of the file path. Always `file_path`.
    #[serde(rename = "type")]
    pub r#type: FilePathType,
}

/// The type of the file path. Always `file_path`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum FilePathType {
    #[default]
    #[serde(rename = "file_path")]
    FilePath,
}

impl FilePathType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::FilePath => "file_path",
        }
    }
}

/// A tool that searches for relevant content from uploaded files. Learn more about the [file search tool](https://platform.openai.com/docs/guides/tools-file-search).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct FileSearchTool {
    /// A filter to apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Filters>,
    /// The maximum number of results to return. This number should be between 1 and 50 inclusive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_num_results: Option<i64>,
    /// Ranking options for search.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ranking_options: Option<RankingOptions>,
    /// The type of the file search tool. Always `file_search`.
    #[serde(rename = "type")]
    pub r#type: FileSearchToolType,
    /// The IDs of the vector stores to search.
    pub vector_store_ids: Vec<String>,
}

/// The results of a file search tool call. See the
/// [file search guide](/docs/guides/tools-file-search) for more information.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct FileSearchToolCall {
    /// The unique ID of the file search tool call.
    pub id: String,
    /// The queries used to search for files.
    pub queries: Vec<String>,
    /// The results of the file search tool call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<FileSearchToolCallResultsItem>>,
    /// The status of the file search tool call. One of `in_progress`,
    /// `searching`, `incomplete` or `failed`,
    pub status: FileSearchToolCallStatus,
    /// The type of the file search tool call. Always `file_search_call`.
    #[serde(rename = "type")]
    pub r#type: FileSearchToolCallType,
}

impl crate::HasId for FileSearchToolCall {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct FileSearchToolCallResultsItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<VectorStoreFileAttributes>,
    /// The unique ID of the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// The name of the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// The relevance score of the file - a value between 0 and 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    /// The text that was retrieved from the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// The status of the file search tool call. One of `in_progress`,
/// `searching`, `incomplete` or `failed`,
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum FileSearchToolCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "searching")]
    Searching,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "failed")]
    Failed,
}

impl FileSearchToolCallStatus {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::InProgress => "in_progress",
            Self::Searching => "searching",
            Self::Completed => "completed",
            Self::Incomplete => "incomplete",
            Self::Failed => "failed",
        }
    }
}

/// The type of the file search tool call. Always `file_search_call`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum FileSearchToolCallType {
    #[default]
    #[serde(rename = "file_search_call")]
    FileSearchCall,
}

impl FileSearchToolCallType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::FileSearchCall => "file_search_call",
        }
    }
}

/// The type of the file search tool. Always `file_search`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum FileSearchToolType {
    #[default]
    #[serde(rename = "file_search")]
    FileSearch,
}

impl FileSearchToolType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::FileSearch => "file_search",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum Filters {
    #[serde(untagged)]
    ComparisonFilter(ComparisonFilter),
    #[serde(untagged)]
    CompoundFilter(CompoundFilter),
}

impl From<ComparisonFilter> for Filters {
    fn from(value: ComparisonFilter) -> Self {
        Filters::ComparisonFilter(value)
    }
}
impl From<CompoundFilter> for Filters {
    fn from(value: CompoundFilter) -> Self {
        Filters::CompoundFilter(value)
    }
}
/// The output of a function tool call.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct FunctionCallOutputItemParam {
    /// The unique ID of the function tool call generated by the model.
    pub call_id: String,
    /// The unique ID of the function tool call output. Populated when this item is returned via API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A JSON string of the output of the function tool call.
    pub output: String,
    /// The status of the item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The type of the function tool call output. Always `function_call_output`.
    #[serde(rename = "type")]
    pub r#type: FunctionCallOutputItemParamType,
}

impl crate::HasId for FunctionCallOutputItemParam {
    fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}

/// The type of the function tool call output. Always `function_call_output`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum FunctionCallOutputItemParamType {
    #[default]
    #[serde(rename = "function_call_output")]
    FunctionCallOutput,
}

impl FunctionCallOutputItemParamType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::FunctionCallOutput => "function_call_output",
        }
    }
}

/// Defines a function in your own code the model can choose to call. Learn more about [function calling](https://platform.openai.com/docs/guides/function-calling).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct FunctionTool {
    /// A description of the function. Used by the model to determine whether or not to call the function.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the function to call.
    pub name: String,
    /// A JSON schema object describing the parameters of the function.
    pub parameters: Option<serde_json::Map<String, serde_json::Value>>,
    /// Whether to enforce strict parameter validation. Default `true`.
    pub strict: Option<bool>,
    /// The type of the function tool. Always `function`.
    #[serde(rename = "type")]
    pub r#type: FunctionToolType,
}

/// A tool call to run a function. See the
/// [function calling guide](/docs/guides/function-calling) for more information.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct FunctionToolCall {
    /// A JSON string of the arguments to pass to the function.
    pub arguments: String,
    /// The unique ID of the function tool call generated by the model.
    pub call_id: String,
    /// The unique ID of the function tool call.
    pub id: String,
    /// The name of the function to run.
    pub name: String,
    /// The status of the item. One of `in_progress`, `completed`, or
    /// `incomplete`. Populated when items are returned via API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The type of the function tool call. Always `function_call`.
    #[serde(rename = "type")]
    pub r#type: FunctionToolCallType,
}

impl crate::HasId for FunctionToolCall {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}

/// The type of the function tool call. Always `function_call`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum FunctionToolCallType {
    #[default]
    #[serde(rename = "function_call")]
    FunctionCall,
}

impl FunctionToolCallType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::FunctionCall => "function_call",
        }
    }
}

/// The type of the function tool. Always `function`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum FunctionToolType {
    #[default]
    #[serde(rename = "function")]
    Function,
}

impl FunctionToolType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Function => "function",
        }
    }
}

/// A tool that generates images using a model like `gpt-image-1`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ImageGenTool {
    /// Background type for the generated image. One of `transparent`,
    /// `opaque`, or `auto`. Default: `auto`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<ImageGenToolBackground>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_fidelity: Option<ImageInputFidelity>,
    /// Optional mask for inpainting. Contains `image_url`
    /// (string, optional) and `file_id` (string, optional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_image_mask: Option<ImageGenToolInputImageMask>,
    /// The image generation model to use. Default: `gpt-image-1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<ImageGenToolModel>,
    /// Moderation level for the generated image. Default: `auto`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub moderation: Option<ImageGenToolModeration>,
    /// Compression level for the output image. Default: 100.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_compression: Option<i64>,
    /// The output format of the generated image. One of `png`, `webp`, or
    /// `jpeg`. Default: `png`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_format: Option<ImageGenToolOutputFormat>,
    /// Number of partial images to generate in streaming mode, from 0 (default value) to 3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partial_images: Option<i64>,
    /// The quality of the generated image. One of `low`, `medium`, `high`,
    /// or `auto`. Default: `auto`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageGenToolQuality>,
    /// The size of the generated image. One of `1024x1024`, `1024x1536`,
    /// `1536x1024`, or `auto`. Default: `auto`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageGenToolSize>,
    /// The type of the image generation tool. Always `image_generation`.
    #[serde(rename = "type")]
    pub r#type: ImageGenToolType,
}

/// Background type for the generated image. One of `transparent`,
/// `opaque`, or `auto`. Default: `auto`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ImageGenToolBackground {
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "opaque")]
    Opaque,
    #[default]
    #[serde(rename = "auto")]
    Auto,
}

impl ImageGenToolBackground {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Transparent => "transparent",
            Self::Opaque => "opaque",
            Self::Auto => "auto",
        }
    }
}

/// An image generation request made by the model.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ImageGenToolCall {
    /// The unique ID of the image generation call.
    pub id: String,
    /// The generated image encoded in base64.
    pub result: Option<String>,
    /// The status of the image generation call.
    pub status: ImageGenToolCallStatus,
    /// The type of the image generation call. Always `image_generation_call`.
    #[serde(rename = "type")]
    pub r#type: ImageGenToolCallType,
}

impl crate::HasId for ImageGenToolCall {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}

/// The status of the image generation call.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum ImageGenToolCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "generating")]
    Generating,
    #[serde(rename = "failed")]
    Failed,
}

impl ImageGenToolCallStatus {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Generating => "generating",
            Self::Failed => "failed",
        }
    }
}

/// The type of the image generation call. Always `image_generation_call`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ImageGenToolCallType {
    #[default]
    #[serde(rename = "image_generation_call")]
    ImageGenerationCall,
}

impl ImageGenToolCallType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ImageGenerationCall => "image_generation_call",
        }
    }
}

/// Optional mask for inpainting. Contains `image_url`
/// (string, optional) and `file_id` (string, optional).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ImageGenToolInputImageMask {
    /// File ID for the mask image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// Base64-encoded mask image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

/// The image generation model to use. Default: `gpt-image-1`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ImageGenToolModel {
    #[default]
    #[serde(rename = "gpt-image-1")]
    GptImage1,
}

impl ImageGenToolModel {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::GptImage1 => "gpt-image-1",
        }
    }
}

/// Moderation level for the generated image. Default: `auto`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ImageGenToolModeration {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "low")]
    Low,
}

impl ImageGenToolModeration {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Auto => "auto",
            Self::Low => "low",
        }
    }
}

/// The output format of the generated image. One of `png`, `webp`, or
/// `jpeg`. Default: `png`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ImageGenToolOutputFormat {
    #[default]
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    Webp,
    #[serde(rename = "jpeg")]
    Jpeg,
}

impl ImageGenToolOutputFormat {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Png => "png",
            Self::Webp => "webp",
            Self::Jpeg => "jpeg",
        }
    }
}

/// The quality of the generated image. One of `low`, `medium`, `high`,
/// or `auto`. Default: `auto`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ImageGenToolQuality {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[default]
    #[serde(rename = "auto")]
    Auto,
}

impl ImageGenToolQuality {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Auto => "auto",
        }
    }
}

/// The size of the generated image. One of `1024x1024`, `1024x1536`,
/// `1536x1024`, or `auto`. Default: `auto`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ImageGenToolSize {
    #[serde(rename = "1024x1024")]
    Value1024x1024,
    #[serde(rename = "1024x1536")]
    Value1024x1536,
    #[serde(rename = "1536x1024")]
    Value1536x1024,
    #[default]
    #[serde(rename = "auto")]
    Auto,
}

impl ImageGenToolSize {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Value1024x1024 => "1024x1024",
            Self::Value1024x1536 => "1024x1536",
            Self::Value1536x1024 => "1536x1024",
            Self::Auto => "auto",
        }
    }
}

/// The type of the image generation tool. Always `image_generation`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ImageGenToolType {
    #[default]
    #[serde(rename = "image_generation")]
    ImageGeneration,
}

impl ImageGenToolType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ImageGeneration => "image_generation",
        }
    }
}

/// Control how much effort the model will exert to match the style and features,
/// especially facial features, of input images. This parameter is only supported
/// for `gpt-image-1`. Supports `high` and `low`. Defaults to `low`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ImageInputFidelity {
    #[serde(rename = "high")]
    High,
    #[default]
    #[serde(rename = "low")]
    Low,
}

impl ImageInputFidelity {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::High => "high",
            Self::Low => "low",
        }
    }
}

/// Specify additional output data to include in the model response. Currently
/// supported values are:
/// - `code_interpreter_call.outputs`: Includes the outputs of python code execution
///   in code interpreter tool call items.
/// - `computer_call_output.output.image_url`: Include image urls from the computer call output.
/// - `file_search_call.results`: Include the search results of
///   the file search tool call.
/// - `message.input_image.image_url`: Include image urls from the input message.
/// - `message.output_text.logprobs`: Include logprobs with assistant messages.
/// - `reasoning.encrypted_content`: Includes an encrypted version of reasoning
///   tokens in reasoning item outputs. This enables reasoning items to be used in
///   multi-turn conversations when using the Responses API statelessly (like
///   when the `store` parameter is set to `false`, or when an organization is
///   enrolled in the zero data retention program).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum Includable {
    #[serde(rename = "code_interpreter_call.outputs")]
    CodeInterpreterCallOutputs,
    #[serde(rename = "computer_call_output.output.image_url")]
    ComputerCallOutputOutputImageUrl,
    #[serde(rename = "file_search_call.results")]
    FileSearchCallResults,
    #[serde(rename = "message.input_image.image_url")]
    MessageInputImageImageUrl,
    #[serde(rename = "message.output_text.logprobs")]
    MessageOutputTextLogprobs,
    #[serde(rename = "reasoning.encrypted_content")]
    ReasoningEncryptedContent,
}

impl Includable {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::CodeInterpreterCallOutputs => "code_interpreter_call.outputs",
            Self::ComputerCallOutputOutputImageUrl => "computer_call_output.output.image_url",
            Self::FileSearchCallResults => "file_search_call.results",
            Self::MessageInputImageImageUrl => "message.input_image.image_url",
            Self::MessageOutputTextLogprobs => "message.output_text.logprobs",
            Self::ReasoningEncryptedContent => "reasoning.encrypted_content",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum InputAudioStreamFormat {
    #[default]
    #[serde(rename = "pcm16")]
    Pcm16,
    #[serde(rename = "g711_ulaw")]
    G711Ulaw,
    #[serde(rename = "g711_alaw")]
    G711Alaw,
}

impl InputAudioStreamFormat {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Pcm16 => "pcm16",
            Self::G711Ulaw => "g711_ulaw",
            Self::G711Alaw => "g711_alaw",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum InputAudioStreamFormatNoiseReductionType {
    #[serde(rename = "near_field")]
    NearField,
    #[serde(rename = "far_field")]
    FarField,
}

impl InputAudioStreamFormatNoiseReductionType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::NearField => "near_field",
            Self::FarField => "far_field",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum InputContent {
    #[serde(untagged)]
    InputTextContent(InputTextContent),
    #[serde(untagged)]
    InputImageContent(InputImageContent),
    #[serde(untagged)]
    InputFileContent(InputFileContent),
}

impl From<InputFileContent> for InputContent {
    fn from(value: InputFileContent) -> Self {
        InputContent::InputFileContent(value)
    }
}
impl From<InputImageContent> for InputContent {
    fn from(value: InputImageContent) -> Self {
        InputContent::InputImageContent(value)
    }
}
impl From<InputTextContent> for InputContent {
    fn from(value: InputTextContent) -> Self {
        InputContent::InputTextContent(value)
    }
}
/// A file input to the model.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct InputFileContent {
    /// The content of the file to be sent to the model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_data: Option<String>,
    /// The ID of the file to be sent to the model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// The URL of the file to be sent to the model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    /// The name of the file to be sent to the model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// The type of the input item. Always `input_file`.
    #[serde(rename = "type")]
    pub r#type: InputFileContentType,
}

/// The type of the input item. Always `input_file`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum InputFileContentType {
    #[default]
    #[serde(rename = "input_file")]
    InputFile,
}

impl InputFileContentType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::InputFile => "input_file",
        }
    }
}

/// An image input to the model. Learn about [image inputs](/docs/guides/vision).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct InputImageContent {
    /// The detail level of the image to be sent to the model. One of `high`, `low`, or `auto`. Defaults to `auto`.
    pub detail: InputImageContentDetail,
    /// The ID of the file to be sent to the model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// The URL of the image to be sent to the model. A fully qualified URL or base64 encoded image in a data URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// The type of the input item. Always `input_image`.
    #[serde(rename = "type")]
    pub r#type: InputImageContentType,
}

/// The detail level of the image to be sent to the model. One of `high`, `low`, or `auto`. Defaults to `auto`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum InputImageContentDetail {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "auto")]
    Auto,
}

impl InputImageContentDetail {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Low => "low",
            Self::High => "high",
            Self::Auto => "auto",
        }
    }
}

/// The type of the input item. Always `input_image`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum InputImageContentType {
    #[default]
    #[serde(rename = "input_image")]
    InputImage,
}

impl InputImageContentType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::InputImage => "input_image",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(tag = "type")]
pub enum InputItem {
    /// An item representing part of the context for the response to be
    /// generated by the model. Can contain text, images, and audio inputs,
    /// as well as previous assistant responses and tool call outputs.
    #[serde(untagged)]
    EasyInputMessage(EasyInputMessage),
    #[serde(untagged)]
    Item(Item),
    #[serde(untagged)]
    ItemReferenceParam(ItemReferenceParam),
}

impl From<EasyInputMessage> for InputItem {
    fn from(value: EasyInputMessage) -> Self {
        InputItem::EasyInputMessage(value)
    }
}
impl From<Item> for InputItem {
    fn from(value: Item) -> Self {
        InputItem::Item(value)
    }
}
impl From<ItemReferenceParam> for InputItem {
    fn from(value: ItemReferenceParam) -> Self {
        InputItem::ItemReferenceParam(value)
    }
}
/// A message input to the model with a role indicating instruction following
/// hierarchy. Instructions given with the `developer` or `system` role take
/// precedence over instructions given with the `user` role.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct InputMessage {
    pub content: InputMessageContentList,
    /// The role of the message input. One of `user`, `system`, or `developer`.
    pub role: InputMessageRole,
    /// The status of item. One of `in_progress`, `completed`, or
    /// `incomplete`. Populated when items are returned via API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The type of the message input. Always set to `message`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<InputMessageType>,
}

/// A list of one or many input items to the model, containing different content
/// types.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(transparent)]
pub struct InputMessageContentList(pub Vec<InputContent>);

impl From<InputMessageContentList> for Vec<InputContent> {
    fn from(value: InputMessageContentList) -> Vec<InputContent> {
        value.0
    }
}

impl From<Vec<InputContent>> for InputMessageContentList {
    fn from(value: Vec<InputContent>) -> Self {
        InputMessageContentList(value)
    }
}

impl AsRef<Vec<InputContent>> for InputMessageContentList {
    fn as_ref(&self) -> &Vec<InputContent> {
        &self.0
    }
}

impl AsMut<Vec<InputContent>> for InputMessageContentList {
    fn as_mut(&mut self) -> &mut Vec<InputContent> {
        &mut self.0
    }
}
/// The role of the message input. One of `user`, `system`, or `developer`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum InputMessageRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "developer")]
    Developer,
}

impl InputMessageRole {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::User => "user",
            Self::System => "system",
            Self::Developer => "developer",
        }
    }
}

/// The type of the message input. Always set to `message`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum InputMessageType {
    #[default]
    #[serde(rename = "message")]
    Message,
}

impl InputMessageType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Message => "message",
        }
    }
}

/// A text input to the model.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct InputTextContent {
    /// The text input to the model.
    pub text: String,
    /// The type of the input item. Always `input_text`.
    #[serde(rename = "type")]
    pub r#type: InputTextContentType,
}

/// The type of the input item. Always `input_text`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum InputTextContentType {
    #[default]
    #[serde(rename = "input_text")]
    InputText,
    #[serde(rename = "output_text")]
    OutputText,
}

impl InputTextContentType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::InputText => "input_text",
            Self::OutputText => "output_text",
        }
    }
}

/// Content item used to generate a response.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(tag = "type")]
pub enum Item {
    #[serde(rename = "message")]
    OutputMessage {
        /// The content of the output message.
        content: Vec<OutputContent>,
        /// The unique ID of the output message.
        id: String,
        /// The role of the output message. Always `assistant`.
        role: OutputMessageRole,
        /// The status of the message input. One of `in_progress`, `completed`, or
        /// `incomplete`. Populated when input items are returned via API.
        status: Status,
    },
    #[serde(rename = "file_search_call")]
    FileSearchToolCall {
        /// The unique ID of the file search tool call.
        id: String,
        /// The queries used to search for files.
        queries: Vec<String>,
        /// The results of the file search tool call.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        results: Option<Vec<FileSearchToolCallResultsItem>>,
        /// The status of the file search tool call. One of `in_progress`,
        /// `searching`, `incomplete` or `failed`,
        status: FileSearchToolCallStatus,
    },
    #[serde(rename = "computer_call")]
    ComputerToolCall {
        action: ComputerAction,
        /// An identifier used when responding to the tool call with output.
        call_id: String,
        /// The unique ID of the computer call.
        id: String,
        /// The pending safety checks for the computer call.
        pending_safety_checks: Vec<ComputerToolCallSafetyCheck>,
        /// The status of the item. One of `in_progress`, `completed`, or
        /// `incomplete`. Populated when items are returned via API.
        status: Status,
    },
    #[serde(rename = "computer_call_output")]
    ComputerCallOutputItemParam {
        /// The safety checks reported by the API that have been acknowledged by the developer.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        acknowledged_safety_checks: Option<Vec<ComputerCallSafetyCheckParam>>,
        /// The ID of the computer tool call that produced the output.
        call_id: String,
        /// The ID of the computer tool call output.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        output: ComputerScreenshotImage,
        /// The status of the message input. One of `in_progress`, `completed`, or `incomplete`. Populated when input items are returned via API.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        status: Option<Status>,
    },
    #[serde(rename = "web_search_call")]
    WebSearchToolCall {
        /// An object describing the specific action taken in this web search call.
        /// Includes details on how the model used the web (search, open_page, find).
        action: WebSearchToolCallAction,
        /// The unique ID of the web search tool call.
        id: String,
        /// The status of the web search tool call.
        status: WebSearchToolCallStatus,
    },
    #[serde(rename = "function_call")]
    FunctionToolCall {
        /// A JSON string of the arguments to pass to the function.
        arguments: String,
        /// The unique ID of the function tool call generated by the model.
        call_id: String,
        /// The unique ID of the function tool call.
        id: String,
        /// The name of the function to run.
        name: String,
        /// The status of the item. One of `in_progress`, `completed`, or
        /// `incomplete`. Populated when items are returned via API.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        status: Option<Status>,
    },
    #[serde(rename = "function_call_output")]
    FunctionCallOutputItemParam {
        /// The unique ID of the function tool call generated by the model.
        call_id: String,
        /// The unique ID of the function tool call output. Populated when this item is returned via API.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        /// A JSON string of the output of the function tool call.
        output: String,
        /// The status of the item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        status: Option<Status>,
    },
    #[serde(rename = "reasoning")]
    ReasoningItem {
        /// The encrypted content of the reasoning item - populated when a response is
        /// generated with `reasoning.encrypted_content` in the `include` parameter.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        encrypted_content: Option<String>,
        /// The unique identifier of the reasoning content.
        id: String,
        /// The status of the item. One of `in_progress`, `completed`, or
        /// `incomplete`. Populated when items are returned via API.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        status: Option<Status>,
        /// Reasoning text contents.
        summary: Vec<ReasoningItemSummaryItem>,
    },
    #[serde(rename = "image_generation_call")]
    ImageGenToolCall {
        /// The unique ID of the image generation call.
        id: String,
        /// The generated image encoded in base64.
        result: Option<String>,
        /// The status of the image generation call.
        status: ImageGenToolCallStatus,
    },
    #[serde(rename = "code_interpreter_call")]
    CodeInterpreterToolCall {
        /// The code to run, or null if not available.
        code: Option<String>,
        /// The ID of the container used to run the code.
        container_id: String,
        /// The unique ID of the code interpreter tool call.
        id: String,
        /// The outputs generated by the code interpreter, such as logs or images.
        /// Can be null if no outputs are available.
        outputs: Option<Vec<CodeInterpreterToolCallOutputsItem>>,
        /// The status of the code interpreter tool call. Valid values are `in_progress`, `completed`, `incomplete`, `interpreting`, and `failed`.
        status: CodeInterpreterToolCallStatus,
    },
    #[serde(rename = "local_shell_call")]
    LocalShellToolCall {
        action: LocalShellExecAction,
        /// The unique ID of the local shell tool call generated by the model.
        call_id: String,
        /// The unique ID of the local shell call.
        id: String,
        /// The status of the local shell call.
        status: Status,
    },
    #[serde(rename = "local_shell_call_output")]
    LocalShellToolCallOutput {
        /// The unique ID of the local shell tool call generated by the model.
        id: String,
        /// A JSON string of the output of the local shell tool call.
        output: String,
        /// The status of the item. One of `in_progress`, `completed`, or `incomplete`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        status: Option<Status>,
    },
    #[serde(rename = "mcp_list_tools")]
    MCPListTools {
        /// Error message if the server could not list tools.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        error: Option<String>,
        /// The unique ID of the list.
        id: String,
        /// The label of the MCP server.
        server_label: String,
        /// The tools available on the server.
        tools: Vec<MCPListToolsTool>,
    },
    #[serde(rename = "mcp_approval_request")]
    MCPApprovalRequest {
        /// A JSON string of arguments for the tool.
        arguments: String,
        /// The unique ID of the approval request.
        id: String,
        /// The name of the tool to run.
        name: String,
        /// The label of the MCP server making the request.
        server_label: String,
    },
    #[serde(rename = "mcp_approval_response")]
    MCPApprovalResponse {
        /// The ID of the approval request being answered.
        approval_request_id: String,
        /// Whether the request was approved.
        approve: bool,
        /// The unique ID of the approval response
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        /// Optional reason for the decision.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reason: Option<String>,
    },
    #[serde(rename = "mcp_call")]
    MCPToolCall {
        /// A JSON string of the arguments passed to the tool.
        arguments: String,
        /// The error from the tool call, if any.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        error: Option<String>,
        /// The unique ID of the tool call.
        id: String,
        /// The name of the tool that was run.
        name: String,
        /// The output from the tool call.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        output: Option<String>,
        /// The label of the MCP server running the tool.
        server_label: String,
    },
    #[serde(untagged)]
    InputMessage(InputMessage),
}

impl From<InputMessage> for Item {
    fn from(value: InputMessage) -> Self {
        Item::InputMessage(value)
    }
}
/// An internal identifier for an item to reference.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ItemReferenceParam {
    /// The ID of the item to reference.
    pub id: String,
    /// The type of item to reference. Always `item_reference`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<ItemReferenceParamType>,
}

impl crate::HasId for ItemReferenceParam {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}

/// The type of item to reference. Always `item_reference`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ItemReferenceParamType {
    #[default]
    #[serde(rename = "item_reference")]
    ItemReference,
}

impl ItemReferenceParamType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ItemReference => "item_reference",
        }
    }
}

/// A collection of keypresses the model would like to perform.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct KeyPress {
    /// The combination of keys the model is requesting to be pressed. This is an
    /// array of strings, each representing a key.
    pub keys: Vec<String>,
    /// Specifies the event type. For a keypress action, this property is
    /// always set to `keypress`.
    #[serde(rename = "type")]
    pub r#type: KeyPressType,
}

/// Specifies the event type. For a keypress action, this property is
/// always set to `keypress`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum KeyPressType {
    #[default]
    #[serde(rename = "keypress")]
    Keypress,
}

impl KeyPressType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Keypress => "keypress",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ListModelsResponse {
    pub data: Vec<Model>,
    pub object: ListModelsResponseObject,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ListModelsResponseObject {
    #[default]
    #[serde(rename = "list")]
    List,
}

impl ListModelsResponseObject {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::List => "list",
        }
    }
}

/// Execute a shell command on the server.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct LocalShellExecAction {
    /// The command to run.
    pub command: Vec<String>,
    /// Environment variables to set for the command.
    pub env: serde_json::Map<String, serde_json::Value>,
    /// Optional timeout in milliseconds for the command.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<i64>,
    /// The type of the local shell action. Always `exec`.
    #[serde(rename = "type")]
    pub r#type: LocalShellExecActionType,
    /// Optional user to run the command as.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Optional working directory to run the command in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

/// The type of the local shell action. Always `exec`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum LocalShellExecActionType {
    #[default]
    #[serde(rename = "exec")]
    Exec,
}

impl LocalShellExecActionType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Exec => "exec",
        }
    }
}

/// A tool that allows the model to execute shell commands in a local environment.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct LocalShellTool {
    /// The type of the local shell tool. Always `local_shell`.
    #[serde(rename = "type")]
    pub r#type: LocalShellToolType,
}

/// A tool call to run a command on the local shell.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct LocalShellToolCall {
    pub action: LocalShellExecAction,
    /// The unique ID of the local shell tool call generated by the model.
    pub call_id: String,
    /// The unique ID of the local shell call.
    pub id: String,
    /// The status of the local shell call.
    pub status: Status,
    /// The type of the local shell call. Always `local_shell_call`.
    #[serde(rename = "type")]
    pub r#type: LocalShellToolCallType,
}

impl crate::HasId for LocalShellToolCall {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}

/// The output of a local shell tool call.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct LocalShellToolCallOutput {
    /// The unique ID of the local shell tool call generated by the model.
    pub id: String,
    /// A JSON string of the output of the local shell tool call.
    pub output: String,
    /// The status of the item. One of `in_progress`, `completed`, or `incomplete`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The type of the local shell tool call output. Always `local_shell_call_output`.
    #[serde(rename = "type")]
    pub r#type: LocalShellToolCallOutputType,
}

impl crate::HasId for LocalShellToolCallOutput {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}

/// The type of the local shell tool call output. Always `local_shell_call_output`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum LocalShellToolCallOutputType {
    #[default]
    #[serde(rename = "local_shell_call_output")]
    LocalShellCallOutput,
}

impl LocalShellToolCallOutputType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::LocalShellCallOutput => "local_shell_call_output",
        }
    }
}

/// The type of the local shell call. Always `local_shell_call`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum LocalShellToolCallType {
    #[default]
    #[serde(rename = "local_shell_call")]
    LocalShellCall,
}

impl LocalShellToolCallType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::LocalShellCall => "local_shell_call",
        }
    }
}

/// The type of the local shell tool. Always `local_shell`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum LocalShellToolType {
    #[default]
    #[serde(rename = "local_shell")]
    LocalShell,
}

impl LocalShellToolType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::LocalShell => "local_shell",
        }
    }
}

/// The log probability of a token.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct LogProb {
    pub bytes: Vec<i64>,
    pub logprob: f64,
    pub token: String,
    pub top_logprobs: Vec<TopLogProb>,
}

/// A log probability object.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct LogProbProperties {
    /// The bytes that were used to generate the log probability.
    pub bytes: Vec<i64>,
    /// The log probability of the token.
    pub logprob: f64,
    /// The token that was used to generate the log probability.
    pub token: String,
}

/// A request for human approval of a tool invocation.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct MCPApprovalRequest {
    /// A JSON string of arguments for the tool.
    pub arguments: String,
    /// The unique ID of the approval request.
    pub id: String,
    /// The name of the tool to run.
    pub name: String,
    /// The label of the MCP server making the request.
    pub server_label: String,
    /// The type of the item. Always `mcp_approval_request`.
    #[serde(rename = "type")]
    pub r#type: MCPApprovalRequestType,
}

impl crate::HasId for MCPApprovalRequest {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}

/// The type of the item. Always `mcp_approval_request`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum MCPApprovalRequestType {
    #[default]
    #[serde(rename = "mcp_approval_request")]
    McpApprovalRequest,
}

impl MCPApprovalRequestType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::McpApprovalRequest => "mcp_approval_request",
        }
    }
}

/// A response to an MCP approval request.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct MCPApprovalResponse {
    /// The ID of the approval request being answered.
    pub approval_request_id: String,
    /// Whether the request was approved.
    pub approve: bool,
    /// The unique ID of the approval response
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Optional reason for the decision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// The type of the item. Always `mcp_approval_response`.
    #[serde(rename = "type")]
    pub r#type: MCPApprovalResponseType,
}

impl crate::HasId for MCPApprovalResponse {
    fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}

/// The type of the item. Always `mcp_approval_response`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum MCPApprovalResponseType {
    #[default]
    #[serde(rename = "mcp_approval_response")]
    McpApprovalResponse,
}

impl MCPApprovalResponseType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::McpApprovalResponse => "mcp_approval_response",
        }
    }
}

/// A list of tools available on an MCP server.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct MCPListTools {
    /// Error message if the server could not list tools.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The unique ID of the list.
    pub id: String,
    /// The label of the MCP server.
    pub server_label: String,
    /// The tools available on the server.
    pub tools: Vec<MCPListToolsTool>,
    /// The type of the item. Always `mcp_list_tools`.
    #[serde(rename = "type")]
    pub r#type: MCPListToolsType,
}

impl crate::HasId for MCPListTools {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}

/// A tool available on an MCP server.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct MCPListToolsTool {
    /// Additional annotations about the tool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<serde_json::Map<String, serde_json::Value>>,
    /// The description of the tool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The JSON schema describing the tool's input.
    pub input_schema: serde_json::Map<String, serde_json::Value>,
    /// The name of the tool.
    pub name: String,
}

/// The type of the item. Always `mcp_list_tools`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum MCPListToolsType {
    #[default]
    #[serde(rename = "mcp_list_tools")]
    McpListTools,
}

impl MCPListToolsType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::McpListTools => "mcp_list_tools",
        }
    }
}

/// Give the model access to additional tools via remote Model Context Protocol
/// (MCP) servers. [Learn more about MCP](/docs/guides/tools-remote-mcp).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct MCPTool {
    /// List of allowed tool names or a filter object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<MCPToolAllowedTools>,
    /// Optional HTTP headers to send to the MCP server. Use for authentication
    /// or other purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Map<String, serde_json::Value>>,
    /// Specify which of the MCP server's tools require approval.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_approval: Option<MCPToolRequireApproval>,
    /// Optional description of the MCP server, used to provide more context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_description: Option<String>,
    /// A label for this MCP server, used to identify it in tool calls.
    pub server_label: String,
    /// The URL for the MCP server.
    pub server_url: String,
    /// The type of the MCP tool. Always `mcp`.
    #[serde(rename = "type")]
    pub r#type: MCPToolType,
}

/// List of allowed tool names or a filter object.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum MCPToolAllowedTools {
    #[serde(untagged)]
    Variant0(Vec<String>),
    #[serde(untagged)]
    Variant1(serde_json::Map<String, serde_json::Value>),
}

impl From<Vec<String>> for MCPToolAllowedTools {
    fn from(value: Vec<String>) -> Self {
        MCPToolAllowedTools::Variant0(value)
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for MCPToolAllowedTools {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        MCPToolAllowedTools::Variant1(value)
    }
}
/// An invocation of a tool on an MCP server.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct MCPToolCall {
    /// A JSON string of the arguments passed to the tool.
    pub arguments: String,
    /// The error from the tool call, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The unique ID of the tool call.
    pub id: String,
    /// The name of the tool that was run.
    pub name: String,
    /// The output from the tool call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// The label of the MCP server running the tool.
    pub server_label: String,
    /// The type of the item. Always `mcp_call`.
    #[serde(rename = "type")]
    pub r#type: MCPToolCallType,
}

impl crate::HasId for MCPToolCall {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}

/// The type of the item. Always `mcp_call`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum MCPToolCallType {
    #[default]
    #[serde(rename = "mcp_call")]
    McpCall,
}

impl MCPToolCallType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::McpCall => "mcp_call",
        }
    }
}

/// Specify which of the MCP server's tools require approval.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum MCPToolRequireApproval {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "never")]
    Never,
    #[serde(untagged)]
    Variant0(serde_json::Map<String, serde_json::Value>),
}

impl From<serde_json::Map<String, serde_json::Value>> for MCPToolRequireApproval {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        MCPToolRequireApproval::Variant0(value)
    }
}
/// The type of the MCP tool. Always `mcp`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum MCPToolType {
    #[default]
    #[serde(rename = "mcp")]
    Mcp,
}

impl MCPToolType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Mcp => "mcp",
        }
    }
}

/// Set of 16 key-value pairs that can be attached to an object. This can be
/// useful for storing additional information about the object in a structured
/// format, and querying for objects via API or the dashboard.
///
/// Keys are strings with a maximum length of 64 characters. Values are strings
/// with a maximum length of 512 characters.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Metadata {
    #[serde(flatten)]
    pub extra_fields: std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum Modalities {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}

impl Modalities {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Text => "text",
            Self::Audio => "audio",
        }
    }
}

/// Describes an OpenAI model offering that can be used with the API.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Model {
    /// The Unix timestamp (in seconds) when the model was created.
    pub created: i64,
    /// The model identifier, which can be referenced in the API endpoints.
    pub id: String,
    /// The object type, which is always "model".
    pub object: ModelObject,
    /// The organization that owns the model.
    pub owned_by: String,
}

impl crate::HasId for Model {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(transparent)]
pub struct ModelId(pub String);

impl From<ModelId> for String {
    fn from(value: ModelId) -> String {
        value.0
    }
}

impl From<String> for ModelId {
    fn from(value: String) -> Self {
        ModelId(value)
    }
}

impl AsRef<String> for ModelId {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl AsMut<String> for ModelId {
    fn as_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
/// The object type, which is always "model".
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ModelObject {
    #[default]
    #[serde(rename = "model")]
    Model,
}

impl ModelObject {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Model => "model",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ModelResponseProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,
    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    /// We generally recommend altering this or `top_p` but not both.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// An integer between 0 and 20 specifying the number of most likely tokens to
    /// return at each token position, each with an associated log probability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i64>,
    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability
    /// mass. So 0.1 means only the tokens comprising the top 10% probability mass
    /// are considered.
    ///
    /// We generally recommend altering this or `temperature` but not both.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// A stable identifier for your end-users.
    /// Used to boost cache hit rates by better bucketing similar requests and  to help OpenAI detect and prevent abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// A mouse move action.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Move {
    /// Specifies the event type. For a move action, this property is
    /// always set to `move`.
    #[serde(rename = "type")]
    pub r#type: MoveType,
    /// The x-coordinate to move to.
    pub x: i64,
    /// The y-coordinate to move to.
    pub y: i64,
}

/// Specifies the event type. For a move action, this property is
/// always set to `move`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum MoveType {
    #[default]
    #[serde(rename = "move")]
    Move,
}

impl MoveType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Move => "move",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum OutputContent {
    #[serde(untagged)]
    OutputTextContent(OutputTextContent),
    #[serde(untagged)]
    RefusalContent(RefusalContent),
}

impl From<OutputTextContent> for OutputContent {
    fn from(value: OutputTextContent) -> Self {
        OutputContent::OutputTextContent(value)
    }
}
impl From<RefusalContent> for OutputContent {
    fn from(value: RefusalContent) -> Self {
        OutputContent::RefusalContent(value)
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(tag = "type")]
pub enum OutputItem {
    #[serde(rename = "message")]
    OutputMessage {
        /// The content of the output message.
        content: Vec<OutputContent>,
        /// The unique ID of the output message.
        id: String,
        /// The role of the output message. Always `assistant`.
        role: OutputMessageRole,
        /// The status of the message input. One of `in_progress`, `completed`, or
        /// `incomplete`. Populated when input items are returned via API.
        status: Status,
    },
    #[serde(rename = "file_search_call")]
    FileSearchToolCall {
        /// The unique ID of the file search tool call.
        id: String,
        /// The queries used to search for files.
        queries: Vec<String>,
        /// The results of the file search tool call.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        results: Option<Vec<FileSearchToolCallResultsItem>>,
        /// The status of the file search tool call. One of `in_progress`,
        /// `searching`, `incomplete` or `failed`,
        status: FileSearchToolCallStatus,
    },
    #[serde(rename = "function_call")]
    FunctionToolCall {
        /// A JSON string of the arguments to pass to the function.
        arguments: String,
        /// The unique ID of the function tool call generated by the model.
        call_id: String,
        /// The unique ID of the function tool call.
        id: String,
        /// The name of the function to run.
        name: String,
        /// The status of the item. One of `in_progress`, `completed`, or
        /// `incomplete`. Populated when items are returned via API.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        status: Option<Status>,
    },
    #[serde(rename = "web_search_call")]
    WebSearchToolCall {
        /// An object describing the specific action taken in this web search call.
        /// Includes details on how the model used the web (search, open_page, find).
        action: WebSearchToolCallAction,
        /// The unique ID of the web search tool call.
        id: String,
        /// The status of the web search tool call.
        status: WebSearchToolCallStatus,
    },
    #[serde(rename = "computer_call")]
    ComputerToolCall {
        action: ComputerAction,
        /// An identifier used when responding to the tool call with output.
        call_id: String,
        /// The unique ID of the computer call.
        id: String,
        /// The pending safety checks for the computer call.
        pending_safety_checks: Vec<ComputerToolCallSafetyCheck>,
        /// The status of the item. One of `in_progress`, `completed`, or
        /// `incomplete`. Populated when items are returned via API.
        status: Status,
    },
    #[serde(rename = "reasoning")]
    ReasoningItem {
        /// The encrypted content of the reasoning item - populated when a response is
        /// generated with `reasoning.encrypted_content` in the `include` parameter.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        encrypted_content: Option<String>,
        /// The unique identifier of the reasoning content.
        id: String,
        /// The status of the item. One of `in_progress`, `completed`, or
        /// `incomplete`. Populated when items are returned via API.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        status: Option<Status>,
        /// Reasoning text contents.
        summary: Vec<ReasoningItemSummaryItem>,
    },
    #[serde(rename = "image_generation_call")]
    ImageGenToolCall {
        /// The unique ID of the image generation call.
        id: String,
        /// The generated image encoded in base64.
        result: Option<String>,
        /// The status of the image generation call.
        status: ImageGenToolCallStatus,
    },
    #[serde(rename = "code_interpreter_call")]
    CodeInterpreterToolCall {
        /// The code to run, or null if not available.
        code: Option<String>,
        /// The ID of the container used to run the code.
        container_id: String,
        /// The unique ID of the code interpreter tool call.
        id: String,
        /// The outputs generated by the code interpreter, such as logs or images.
        /// Can be null if no outputs are available.
        outputs: Option<Vec<CodeInterpreterToolCallOutputsItem>>,
        /// The status of the code interpreter tool call. Valid values are `in_progress`, `completed`, `incomplete`, `interpreting`, and `failed`.
        status: CodeInterpreterToolCallStatus,
    },
    #[serde(rename = "local_shell_call")]
    LocalShellToolCall {
        action: LocalShellExecAction,
        /// The unique ID of the local shell tool call generated by the model.
        call_id: String,
        /// The unique ID of the local shell call.
        id: String,
        /// The status of the local shell call.
        status: Status,
    },
    #[serde(rename = "mcp_call")]
    MCPToolCall {
        /// A JSON string of the arguments passed to the tool.
        arguments: String,
        /// The error from the tool call, if any.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        error: Option<String>,
        /// The unique ID of the tool call.
        id: String,
        /// The name of the tool that was run.
        name: String,
        /// The output from the tool call.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        output: Option<String>,
        /// The label of the MCP server running the tool.
        server_label: String,
    },
    #[serde(rename = "mcp_list_tools")]
    MCPListTools {
        /// Error message if the server could not list tools.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        error: Option<String>,
        /// The unique ID of the list.
        id: String,
        /// The label of the MCP server.
        server_label: String,
        /// The tools available on the server.
        tools: Vec<MCPListToolsTool>,
    },
    #[serde(rename = "mcp_approval_request")]
    MCPApprovalRequest {
        /// A JSON string of arguments for the tool.
        arguments: String,
        /// The unique ID of the approval request.
        id: String,
        /// The name of the tool to run.
        name: String,
        /// The label of the MCP server making the request.
        server_label: String,
    },
}

impl crate::HasId for OutputItem {
    fn get_id(&self) -> Option<&str> {
        match self {
            OutputItem::OutputMessage { id, .. } => Some(id.as_str()),
            OutputItem::FileSearchToolCall { id, .. } => Some(id.as_str()),
            OutputItem::FunctionToolCall { id, .. } => Some(id.as_str()),
            OutputItem::WebSearchToolCall { id, .. } => Some(id.as_str()),
            OutputItem::ComputerToolCall { id, .. } => Some(id.as_str()),
            OutputItem::ReasoningItem { id, .. } => Some(id.as_str()),
            OutputItem::ImageGenToolCall { id, .. } => Some(id.as_str()),
            OutputItem::CodeInterpreterToolCall { id, .. } => Some(id.as_str()),
            OutputItem::LocalShellToolCall { id, .. } => Some(id.as_str()),
            OutputItem::MCPToolCall { id, .. } => Some(id.as_str()),
            OutputItem::MCPListTools { id, .. } => Some(id.as_str()),
            OutputItem::MCPApprovalRequest { id, .. } => Some(id.as_str()),
        }
    }
}
/// An output message from the model.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct OutputMessage {
    /// The content of the output message.
    pub content: Vec<OutputContent>,
    /// The unique ID of the output message.
    pub id: String,
    /// The role of the output message. Always `assistant`.
    pub role: OutputMessageRole,
    /// The status of the message input. One of `in_progress`, `completed`, or
    /// `incomplete`. Populated when input items are returned via API.
    pub status: Status,
    /// The type of the output message. Always `message`.
    #[serde(rename = "type")]
    pub r#type: OutputMessageType,
}

impl crate::HasId for OutputMessage {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}

/// The role of the output message. Always `assistant`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum OutputMessageRole {
    #[default]
    #[serde(rename = "assistant")]
    Assistant,
}

impl OutputMessageRole {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Assistant => "assistant",
        }
    }
}

/// The type of the output message. Always `message`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum OutputMessageType {
    #[default]
    #[serde(rename = "message")]
    Message,
}

impl OutputMessageType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Message => "message",
        }
    }
}

/// A text output from the model.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct OutputTextContent {
    /// The annotations of the text output.
    pub annotations: Vec<Annotation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Vec<LogProb>>,
    /// The text output from the model.
    pub text: String,
    /// The type of the output text. Always `output_text`.
    #[serde(rename = "type")]
    pub r#type: OutputTextContentType,
}

/// The type of the output text. Always `output_text`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum OutputTextContentType {
    #[default]
    #[serde(rename = "output_text")]
    OutputText,
}

impl OutputTextContentType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::OutputText => "output_text",
        }
    }
}

/// Reference to a prompt template and its variables.
/// [Learn more](/docs/guides/text?api-mode=responses#reusable-prompts).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Prompt {
    /// The unique identifier of the prompt template to use.
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<ResponsePromptVariables>,
    /// Optional version of the prompt template.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl crate::HasId for Prompt {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RankingOptions {
    /// The ranker to use for the file search.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ranker: Option<RankingOptionsRanker>,
    /// The score threshold for the file search, a number between 0 and 1. Numbers closer to 1 will attempt to return only the most relevant results, but may return fewer results.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f64>,
}

/// The ranker to use for the file search.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum RankingOptionsRanker {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "default-2024-11-15")]
    Default20241115,
}

impl RankingOptionsRanker {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Auto => "auto",
            Self::Default20241115 => "default-2024-11-15",
        }
    }
}

/// A realtime client event.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(tag = "type")]
pub enum RealtimeClientEvent {
    #[serde(rename = "conversation.item.create")]
    ConversationItemCreate {
        /// Optional client-generated ID used to identify this event.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        event_id: Option<String>,
        item: RealtimeConversationItem,
        /// The ID of the preceding item after which the new item will be inserted.
        /// If not set, the new item will be appended to the end of the conversation.
        /// If set to `root`, the new item will be added to the beginning of the conversation.
        /// If set to an existing ID, it allows an item to be inserted mid-conversation. If the
        /// ID cannot be found, an error will be returned and the item will not be added.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        previous_item_id: Option<String>,
    },
    #[serde(rename = "conversation.item.delete")]
    ConversationItemDelete {
        /// Optional client-generated ID used to identify this event.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        event_id: Option<String>,
        /// The ID of the item to delete.
        item_id: String,
    },
    #[serde(rename = "conversation.item.retrieve")]
    ConversationItemRetrieve {
        /// Optional client-generated ID used to identify this event.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        event_id: Option<String>,
        /// The ID of the item to retrieve.
        item_id: String,
    },
    #[serde(rename = "conversation.item.truncate")]
    ConversationItemTruncate {
        /// Inclusive duration up to which audio is truncated, in milliseconds. If
        /// the audio_end_ms is greater than the actual audio duration, the server
        /// will respond with an error.
        audio_end_ms: i64,
        /// The index of the content part to truncate. Set this to 0.
        content_index: i64,
        /// Optional client-generated ID used to identify this event.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        event_id: Option<String>,
        /// The ID of the assistant message item to truncate. Only assistant message
        /// items can be truncated.
        item_id: String,
    },
    #[serde(rename = "input_audio_buffer.append")]
    InputAudioBufferAppend {
        /// Base64-encoded audio bytes. This must be in the format specified by the
        /// `input_audio_format` field in the session configuration.
        audio: String,
        /// Optional client-generated ID used to identify this event.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        event_id: Option<String>,
    },
    #[serde(rename = "input_audio_buffer.clear")]
    InputAudioBufferClear {
        /// Optional client-generated ID used to identify this event.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        event_id: Option<String>,
    },
    #[serde(rename = "output_audio_buffer.clear")]
    OutputAudioBufferClear {
        /// The unique ID of the client event used for error handling.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        event_id: Option<String>,
    },
    #[serde(rename = "input_audio_buffer.commit")]
    InputAudioBufferCommit {
        /// Optional client-generated ID used to identify this event.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        event_id: Option<String>,
    },
    #[serde(rename = "response.cancel")]
    ResponseCancel {
        /// Optional client-generated ID used to identify this event.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        event_id: Option<String>,
        /// A specific response ID to cancel - if not provided, will cancel an
        /// in-progress response in the default conversation.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        response_id: Option<String>,
    },
    #[serde(rename = "response.create")]
    ResponseCreate {
        /// Optional client-generated ID used to identify this event.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        event_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        response: Option<RealtimeResponseCreateParams>,
    },
    #[serde(rename = "session.update")]
    SessionUpdate {
        /// Optional client-generated ID used to identify this event.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        event_id: Option<String>,
        session: RealtimeSessionCreateRequest,
    },
    #[serde(rename = "transcription_session.update")]
    TranscriptionSessionUpdate {
        /// Optional client-generated ID used to identify this event.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        event_id: Option<String>,
        session: RealtimeTranscriptionSessionCreateRequest,
    },
}

/// Add a new Item to the Conversation's context, including messages, function
/// calls, and function call responses. This event can be used both to populate a
/// "history" of the conversation and to add new items mid-stream, but has the
/// current limitation that it cannot populate assistant audio messages.
///
/// If successful, the server will respond with a `conversation.item.created`
/// event, otherwise an `error` event will be sent.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeClientEventConversationItemCreate {
    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    pub item: RealtimeConversationItem,
    /// The ID of the preceding item after which the new item will be inserted.
    /// If not set, the new item will be appended to the end of the conversation.
    /// If set to `root`, the new item will be added to the beginning of the conversation.
    /// If set to an existing ID, it allows an item to be inserted mid-conversation. If the
    /// ID cannot be found, an error will be returned and the item will not be added.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_item_id: Option<String>,
    /// The event type, must be `conversation.item.create`.
    #[serde(rename = "type")]
    pub r#type: RealtimeClientEventConversationItemCreateType,
}

/// The event type, must be `conversation.item.create`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeClientEventConversationItemCreateType {
    #[default]
    #[serde(rename = "conversation.item.create")]
    ConversationItemCreate,
}

impl RealtimeClientEventConversationItemCreateType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ConversationItemCreate => "conversation.item.create",
        }
    }
}

/// Send this event when you want to remove any item from the conversation
/// history. The server will respond with a `conversation.item.deleted` event,
/// unless the item does not exist in the conversation history, in which case the
/// server will respond with an error.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeClientEventConversationItemDelete {
    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// The ID of the item to delete.
    pub item_id: String,
    /// The event type, must be `conversation.item.delete`.
    #[serde(rename = "type")]
    pub r#type: RealtimeClientEventConversationItemDeleteType,
}

/// The event type, must be `conversation.item.delete`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeClientEventConversationItemDeleteType {
    #[default]
    #[serde(rename = "conversation.item.delete")]
    ConversationItemDelete,
}

impl RealtimeClientEventConversationItemDeleteType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ConversationItemDelete => "conversation.item.delete",
        }
    }
}

/// Send this event when you want to retrieve the server's representation of a specific item in the conversation history. This is useful, for example, to inspect user audio after noise cancellation and VAD.
/// The server will respond with a `conversation.item.retrieved` event,
/// unless the item does not exist in the conversation history, in which case the
/// server will respond with an error.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeClientEventConversationItemRetrieve {
    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// The ID of the item to retrieve.
    pub item_id: String,
    /// The event type, must be `conversation.item.retrieve`.
    #[serde(rename = "type")]
    pub r#type: RealtimeClientEventConversationItemRetrieveType,
}

/// The event type, must be `conversation.item.retrieve`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeClientEventConversationItemRetrieveType {
    #[default]
    #[serde(rename = "conversation.item.retrieve")]
    ConversationItemRetrieve,
}

impl RealtimeClientEventConversationItemRetrieveType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ConversationItemRetrieve => "conversation.item.retrieve",
        }
    }
}

/// Send this event to truncate a previous assistant message’s audio. The server
/// will produce audio faster than realtime, so this event is useful when the user
/// interrupts to truncate audio that has already been sent to the client but not
/// yet played. This will synchronize the server's understanding of the audio with
/// the client's playback.
///
/// Truncating audio will delete the server-side text transcript to ensure there
/// is not text in the context that hasn't been heard by the user.
///
/// If successful, the server will respond with a `conversation.item.truncated`
/// event.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeClientEventConversationItemTruncate {
    /// Inclusive duration up to which audio is truncated, in milliseconds. If
    /// the audio_end_ms is greater than the actual audio duration, the server
    /// will respond with an error.
    pub audio_end_ms: i64,
    /// The index of the content part to truncate. Set this to 0.
    pub content_index: i64,
    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// The ID of the assistant message item to truncate. Only assistant message
    /// items can be truncated.
    pub item_id: String,
    /// The event type, must be `conversation.item.truncate`.
    #[serde(rename = "type")]
    pub r#type: RealtimeClientEventConversationItemTruncateType,
}

/// The event type, must be `conversation.item.truncate`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeClientEventConversationItemTruncateType {
    #[default]
    #[serde(rename = "conversation.item.truncate")]
    ConversationItemTruncate,
}

impl RealtimeClientEventConversationItemTruncateType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ConversationItemTruncate => "conversation.item.truncate",
        }
    }
}

/// Send this event to append audio bytes to the input audio buffer. The audio
/// buffer is temporary storage you can write to and later commit. In Server VAD
/// mode, the audio buffer is used to detect speech and the server will decide
/// when to commit. When Server VAD is disabled, you must commit the audio buffer
/// manually.
///
/// The client may choose how much audio to place in each event up to a maximum
/// of 15 MiB, for example streaming smaller chunks from the client may allow the
/// VAD to be more responsive. Unlike made other client events, the server will
/// not send a confirmation response to this event.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeClientEventInputAudioBufferAppend {
    /// Base64-encoded audio bytes. This must be in the format specified by the
    /// `input_audio_format` field in the session configuration.
    pub audio: String,
    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// The event type, must be `input_audio_buffer.append`.
    #[serde(rename = "type")]
    pub r#type: RealtimeClientEventInputAudioBufferAppendType,
}

/// The event type, must be `input_audio_buffer.append`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeClientEventInputAudioBufferAppendType {
    #[default]
    #[serde(rename = "input_audio_buffer.append")]
    InputAudioBufferAppend,
}

impl RealtimeClientEventInputAudioBufferAppendType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::InputAudioBufferAppend => "input_audio_buffer.append",
        }
    }
}

/// Send this event to clear the audio bytes in the buffer. The server will
/// respond with an `input_audio_buffer.cleared` event.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeClientEventInputAudioBufferClear {
    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// The event type, must be `input_audio_buffer.clear`.
    #[serde(rename = "type")]
    pub r#type: RealtimeClientEventInputAudioBufferClearType,
}

/// The event type, must be `input_audio_buffer.clear`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeClientEventInputAudioBufferClearType {
    #[default]
    #[serde(rename = "input_audio_buffer.clear")]
    InputAudioBufferClear,
}

impl RealtimeClientEventInputAudioBufferClearType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::InputAudioBufferClear => "input_audio_buffer.clear",
        }
    }
}

/// Send this event to commit the user input audio buffer, which will create a
/// new user message item in the conversation. This event will produce an error
/// if the input audio buffer is empty. When in Server VAD mode, the client does
/// not need to send this event, the server will commit the audio buffer
/// automatically.
///
/// Committing the input audio buffer will trigger input audio transcription
/// (if enabled in session configuration), but it will not create a response
/// from the model. The server will respond with an `input_audio_buffer.committed`
/// event.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeClientEventInputAudioBufferCommit {
    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// The event type, must be `input_audio_buffer.commit`.
    #[serde(rename = "type")]
    pub r#type: RealtimeClientEventInputAudioBufferCommitType,
}

/// The event type, must be `input_audio_buffer.commit`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeClientEventInputAudioBufferCommitType {
    #[default]
    #[serde(rename = "input_audio_buffer.commit")]
    InputAudioBufferCommit,
}

impl RealtimeClientEventInputAudioBufferCommitType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::InputAudioBufferCommit => "input_audio_buffer.commit",
        }
    }
}

/// **WebRTC Only:** Emit to cut off the current audio response. This will trigger the server to
/// stop generating audio and emit a `output_audio_buffer.cleared` event. This
/// event should be preceded by a `response.cancel` client event to stop the
/// generation of the current response.
/// [Learn more](/docs/guides/realtime-conversations#client-and-server-events-for-audio-in-webrtc).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeClientEventOutputAudioBufferClear {
    /// The unique ID of the client event used for error handling.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// The event type, must be `output_audio_buffer.clear`.
    #[serde(rename = "type")]
    pub r#type: RealtimeClientEventOutputAudioBufferClearType,
}

/// The event type, must be `output_audio_buffer.clear`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeClientEventOutputAudioBufferClearType {
    #[default]
    #[serde(rename = "output_audio_buffer.clear")]
    OutputAudioBufferClear,
}

impl RealtimeClientEventOutputAudioBufferClearType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::OutputAudioBufferClear => "output_audio_buffer.clear",
        }
    }
}

/// Send this event to cancel an in-progress response. The server will respond
/// with a `response.cancelled` event or an error if there is no response to
/// cancel.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeClientEventResponseCancel {
    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// A specific response ID to cancel - if not provided, will cancel an
    /// in-progress response in the default conversation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_id: Option<String>,
    /// The event type, must be `response.cancel`.
    #[serde(rename = "type")]
    pub r#type: RealtimeClientEventResponseCancelType,
}

/// The event type, must be `response.cancel`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeClientEventResponseCancelType {
    #[default]
    #[serde(rename = "response.cancel")]
    ResponseCancel,
}

impl RealtimeClientEventResponseCancelType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseCancel => "response.cancel",
        }
    }
}

/// This event instructs the server to create a Response, which means triggering
/// model inference. When in Server VAD mode, the server will create Responses
/// automatically.
///
/// A Response will include at least one Item, and may have two, in which case
/// the second will be a function call. These Items will be appended to the
/// conversation history.
///
/// The server will respond with a `response.created` event, events for Items
/// and content created, and finally a `response.done` event to indicate the
/// Response is complete.
///
/// The `response.create` event includes inference configuration like
/// `instructions`, and `temperature`. These fields will override the Session's
/// configuration for this Response only.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeClientEventResponseCreate {
    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<RealtimeResponseCreateParams>,
    /// The event type, must be `response.create`.
    #[serde(rename = "type")]
    pub r#type: RealtimeClientEventResponseCreateType,
}

/// The event type, must be `response.create`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeClientEventResponseCreateType {
    #[default]
    #[serde(rename = "response.create")]
    ResponseCreate,
}

impl RealtimeClientEventResponseCreateType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseCreate => "response.create",
        }
    }
}

/// Send this event to update the session’s default configuration.
/// The client may send this event at any time to update any field,
/// except for `voice`. However, note that once a session has been
/// initialized with a particular `model`, it can’t be changed to
/// another model using `session.update`.
///
/// When the server receives a `session.update`, it will respond
/// with a `session.updated` event showing the full, effective configuration.
/// Only the fields that are present are updated. To clear a field like
/// `instructions`, pass an empty string.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeClientEventSessionUpdate {
    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    pub session: RealtimeSessionCreateRequest,
    /// The event type, must be `session.update`.
    #[serde(rename = "type")]
    pub r#type: RealtimeClientEventSessionUpdateType,
}

/// The event type, must be `session.update`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeClientEventSessionUpdateType {
    #[default]
    #[serde(rename = "session.update")]
    SessionUpdate,
}

impl RealtimeClientEventSessionUpdateType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::SessionUpdate => "session.update",
        }
    }
}

/// Send this event to update a transcription session.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeClientEventTranscriptionSessionUpdate {
    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    pub session: RealtimeTranscriptionSessionCreateRequest,
    /// The event type, must be `transcription_session.update`.
    #[serde(rename = "type")]
    pub r#type: RealtimeClientEventTranscriptionSessionUpdateType,
}

/// The event type, must be `transcription_session.update`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeClientEventTranscriptionSessionUpdateType {
    #[default]
    #[serde(rename = "transcription_session.update")]
    TranscriptionSessionUpdate,
}

impl RealtimeClientEventTranscriptionSessionUpdateType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::TranscriptionSessionUpdate => "transcription_session.update",
        }
    }
}

/// The item to add to the conversation.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeConversationItem {
    /// The arguments of the function call (for `function_call` items).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
    /// The ID of the function call (for `function_call` and
    /// `function_call_output` items). If passed on a `function_call_output`
    /// item, the server will check that a `function_call` item with the same
    /// ID exists in the conversation history.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    /// The content of the message, applicable for `message` items.
    /// - Message items of role `system` support only `input_text` content
    /// - Message items of role `user` support `input_text` and `input_audio`
    ///   content
    /// - Message items of role `assistant` support `text` content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<RealtimeConversationItemContentItem>>,
    /// The unique ID of the item, this can be generated by the client to help
    /// manage server-side context, but is not required because the server will
    /// generate one if not provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the function being called (for `function_call` items).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identifier for the API object being returned - always `realtime.item`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<RealtimeConversationItemObject>,
    /// The output of the function call (for `function_call_output` items).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// The role of the message sender (`user`, `assistant`, `system`), only
    /// applicable for `message` items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<ConversationItemRole>,
    /// The status of the item (`completed`, `incomplete`, `in_progress`). These have no effect
    /// on the conversation, but are accepted for consistency with the
    /// `conversation.item.created` event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The type of the item (`message`, `function_call`, `function_call_output`).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<RealtimeConversationItemType>,
}

impl crate::HasId for RealtimeConversationItem {
    fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeConversationItemContentItem {
    /// Base64-encoded audio bytes, used for `input_audio` content type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    /// ID of a previous conversation item to reference (for `item_reference`
    /// content types in `response.create` events). These can reference both
    /// client and server created items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The text content, used for `input_text` and `text` content types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The transcript of the audio, used for `input_audio` and `audio`
    /// content types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
    /// The content type (`input_text`, `input_audio`, `item_reference`, `text`, `audio`).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<RealtimeConversationItemContentItemType>,
}

impl crate::HasId for RealtimeConversationItemContentItem {
    fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}

/// The content type (`input_text`, `input_audio`, `item_reference`, `text`, `audio`).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum RealtimeConversationItemContentItemType {
    #[serde(rename = "input_audio")]
    InputAudio,
    #[serde(rename = "input_text")]
    InputText,
    #[serde(rename = "item_reference")]
    ItemReference,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}

impl RealtimeConversationItemContentItemType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::InputAudio => "input_audio",
            Self::InputText => "input_text",
            Self::ItemReference => "item_reference",
            Self::Text => "text",
            Self::Audio => "audio",
        }
    }
}

/// Identifier for the API object being returned - always `realtime.item`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeConversationItemObject {
    #[default]
    #[serde(rename = "realtime.item")]
    RealtimeItem,
}

impl RealtimeConversationItemObject {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::RealtimeItem => "realtime.item",
        }
    }
}

/// The type of the item (`message`, `function_call`, `function_call_output`).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum RealtimeConversationItemType {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "function_call")]
    FunctionCall,
    #[serde(rename = "function_call_output")]
    FunctionCallOutput,
}

impl RealtimeConversationItemType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Message => "message",
            Self::FunctionCall => "function_call",
            Self::FunctionCallOutput => "function_call_output",
        }
    }
}

/// The item to add to the conversation.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeConversationItemWithReference {
    /// The arguments of the function call (for `function_call` items).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
    /// The ID of the function call (for `function_call` and
    /// `function_call_output` items). If passed on a `function_call_output`
    /// item, the server will check that a `function_call` item with the same
    /// ID exists in the conversation history.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    /// The content of the message, applicable for `message` items.
    /// - Message items of role `system` support only `input_text` content
    /// - Message items of role `user` support `input_text` and `input_audio`
    ///   content
    /// - Message items of role `assistant` support `text` content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<RealtimeConversationItemWithReferenceContentItem>>,
    /// For an item of type (`message` | `function_call` | `function_call_output`)
    /// this field allows the client to assign the unique ID of the item. It is
    /// not required because the server will generate one if not provided.
    ///
    /// For an item of type `item_reference`, this field is required and is a
    /// reference to any item that has previously existed in the conversation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the function being called (for `function_call` items).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identifier for the API object being returned - always `realtime.item`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<RealtimeConversationItemWithReferenceObject>,
    /// The output of the function call (for `function_call_output` items).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// The role of the message sender (`user`, `assistant`, `system`), only
    /// applicable for `message` items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<ConversationItemRole>,
    /// The status of the item (`completed`, `incomplete`, `in_progress`). These have no effect
    /// on the conversation, but are accepted for consistency with the
    /// `conversation.item.created` event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The type of the item (`message`, `function_call`, `function_call_output`, `item_reference`).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<RealtimeConversationItemWithReferenceType>,
}

impl crate::HasId for RealtimeConversationItemWithReference {
    fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeConversationItemWithReferenceContentItem {
    /// Base64-encoded audio bytes, used for `input_audio` content type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    /// ID of a previous conversation item to reference (for `item_reference`
    /// content types in `response.create` events). These can reference both
    /// client and server created items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The text content, used for `input_text` and `text` content types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The transcript of the audio, used for `input_audio` content type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
    /// The content type (`input_text`, `input_audio`, `item_reference`, `text`).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<RealtimeConversationItemWithReferenceContentItemType>,
}

impl crate::HasId for RealtimeConversationItemWithReferenceContentItem {
    fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}

/// The content type (`input_text`, `input_audio`, `item_reference`, `text`).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum RealtimeConversationItemWithReferenceContentItemType {
    #[serde(rename = "input_audio")]
    InputAudio,
    #[serde(rename = "input_text")]
    InputText,
    #[serde(rename = "item_reference")]
    ItemReference,
    #[serde(rename = "text")]
    Text,
}

impl RealtimeConversationItemWithReferenceContentItemType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::InputAudio => "input_audio",
            Self::InputText => "input_text",
            Self::ItemReference => "item_reference",
            Self::Text => "text",
        }
    }
}

/// Identifier for the API object being returned - always `realtime.item`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeConversationItemWithReferenceObject {
    #[default]
    #[serde(rename = "realtime.item")]
    RealtimeItem,
}

impl RealtimeConversationItemWithReferenceObject {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::RealtimeItem => "realtime.item",
        }
    }
}

/// The type of the item (`message`, `function_call`, `function_call_output`, `item_reference`).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum RealtimeConversationItemWithReferenceType {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "function_call")]
    FunctionCall,
    #[serde(rename = "function_call_output")]
    FunctionCallOutput,
}

impl RealtimeConversationItemWithReferenceType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Message => "message",
            Self::FunctionCall => "function_call",
            Self::FunctionCallOutput => "function_call_output",
        }
    }
}

/// The response resource.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeResponse {
    /// Which conversation the response is added to, determined by the `conversation`
    /// field in the `response.create` event. If `auto`, the response will be added to
    /// the default conversation and the value of `conversation_id` will be an id like
    /// `conv_1234`. If `none`, the response will not be added to any conversation and
    /// the value of `conversation_id` will be `null`. If responses are being triggered
    /// by server VAD, the response will be added to the default conversation, thus
    /// the `conversation_id` will be an id like `conv_1234`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// The unique ID of the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Maximum number of output tokens for a single assistant response,
    /// inclusive of tool calls, that was used in this response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<RealtimeResponseMaxOutputTokens>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// The set of modalities the model used to respond. If there are multiple modalities,
    /// the model will pick one, for example if `modalities` is `["text", "audio"]`, the model
    /// could be responding in either text or audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<Modalities>>,
    /// The object type, must be `realtime.response`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<RealtimeResponseObject>,
    /// The list of output items generated by the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<RealtimeConversationItem>>,
    /// The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_audio_format: Option<InputAudioStreamFormat>,
    /// The final status of the response (`completed`, `cancelled`, `failed`, or
    /// `incomplete`, `in_progress`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RealtimeResponseStatus>,
    /// Additional details about the status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<RealtimeResponseStatusDetails>,
    /// Sampling temperature for the model, limited to [0.6, 1.2]. Defaults to 0.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Usage statistics for the Response, this will correspond to billing. A
    /// Realtime API session will maintain a conversation context and append new
    /// Items to the Conversation, thus output from previous turns (text and
    /// audio tokens) will become the input for later turns.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<RealtimeResponseUsage>,
    /// The voice the model used to respond.
    /// Current voice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `sage`,
    /// `shimmer`, and `verse`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<VoiceId>,
}

impl crate::HasId for RealtimeResponse {
    fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}

/// Create a new Realtime response with these parameters
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeResponseCreateParams {
    /// Controls which conversation the response is added to. Currently supports
    /// `auto` and `none`, with `auto` as the default value. The `auto` value
    /// means that the contents of the response will be added to the default
    /// conversation. Set this to `none` to create an out-of-band response which
    /// will not add items to default conversation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation: Option<RealtimeResponseCreateParamsConversation>,
    /// Input items to include in the prompt for the model. Using this field
    /// creates a new context for this Response instead of using the default
    /// conversation. An empty array `[]` will clear the context for this Response.
    /// Note that this can include references to items from the default conversation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<Vec<RealtimeConversationItemWithReference>>,
    /// The default system instructions (i.e. system message) prepended to model
    /// calls. This field allows the client to guide the model on desired
    /// responses. The model can be instructed on response content and format,
    /// (e.g. "be extremely succinct", "act friendly", "here are examples of good
    /// responses") and on audio behavior (e.g. "talk quickly", "inject emotion
    /// into your voice", "laugh frequently"). The instructions are not guaranteed
    /// to be followed by the model, but they provide guidance to the model on the
    /// desired behavior.
    ///
    /// Note that the server sets default instructions which will be used if this
    /// field is not set and are visible in the `session.created` event at the
    /// start of the session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Maximum number of output tokens for a single assistant response,
    /// inclusive of tool calls. Provide an integer between 1 and 4096 to
    /// limit output tokens, or `inf` for the maximum available tokens for a
    /// given model. Defaults to `inf`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_response_output_tokens: Option<RealtimeResponseCreateParamsMaxResponseOutputTokens>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// The set of modalities the model can respond with. To disable audio,
    /// set this to ["text"].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<Modalities>>,
    /// The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_audio_format: Option<InputAudioStreamFormat>,
    /// Sampling temperature for the model, limited to [0.6, 1.2]. Defaults to 0.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// How the model chooses tools. Options are `auto`, `none`, `required`, or
    /// specify a function, like `{"type": "function", "function": {"name": "my_function"}}`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,
    /// Tools (functions) available to the model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<RealtimeResponseCreateParamsToolsItem>>,
    /// The voice the model uses to respond. Voice cannot be changed during the
    /// session once the model has responded with audio at least once. Current
    /// voice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `sage`,
    /// `shimmer`, and `verse`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<VoiceId>,
}

/// Controls which conversation the response is added to. Currently supports
/// `auto` and `none`, with `auto` as the default value. The `auto` value
/// means that the contents of the response will be added to the default
/// conversation. Set this to `none` to create an out-of-band response which
/// will not add items to default conversation.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum RealtimeResponseCreateParamsConversation {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "none")]
    None,
    #[serde(untagged)]
    Variant0(String),
}

impl From<String> for RealtimeResponseCreateParamsConversation {
    fn from(value: String) -> Self {
        RealtimeResponseCreateParamsConversation::Variant0(value)
    }
}
/// Maximum number of output tokens for a single assistant response,
/// inclusive of tool calls. Provide an integer between 1 and 4096 to
/// limit output tokens, or `inf` for the maximum available tokens for a
/// given model. Defaults to `inf`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum RealtimeResponseCreateParamsMaxResponseOutputTokens {
    #[serde(rename = "inf")]
    Inf,
    #[serde(untagged)]
    Variant0(i64),
}

impl From<i64> for RealtimeResponseCreateParamsMaxResponseOutputTokens {
    fn from(value: i64) -> Self {
        RealtimeResponseCreateParamsMaxResponseOutputTokens::Variant0(value)
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeResponseCreateParamsToolsItem {
    /// The description of the function, including guidance on when and how
    /// to call it, and guidance about what to tell the user when calling
    /// (if anything).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the function.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Parameters of the function in JSON Schema.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Map<String, serde_json::Value>>,
    /// The type of the tool, i.e. `function`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<RealtimeResponseCreateParamsToolsItemType>,
}

/// The type of the tool, i.e. `function`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeResponseCreateParamsToolsItemType {
    #[default]
    #[serde(rename = "function")]
    Function,
}

impl RealtimeResponseCreateParamsToolsItemType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Function => "function",
        }
    }
}

/// Maximum number of output tokens for a single assistant response,
/// inclusive of tool calls, that was used in this response.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum RealtimeResponseMaxOutputTokens {
    #[serde(rename = "inf")]
    Inf,
    #[serde(untagged)]
    Variant0(i64),
}

impl From<i64> for RealtimeResponseMaxOutputTokens {
    fn from(value: i64) -> Self {
        RealtimeResponseMaxOutputTokens::Variant0(value)
    }
}
/// The object type, must be `realtime.response`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeResponseObject {
    #[default]
    #[serde(rename = "realtime.response")]
    RealtimeResponse,
}

impl RealtimeResponseObject {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::RealtimeResponse => "realtime.response",
        }
    }
}

/// The final status of the response (`completed`, `cancelled`, `failed`, or
/// `incomplete`, `in_progress`).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum RealtimeResponseStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "in_progress")]
    InProgress,
}

impl RealtimeResponseStatus {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
            Self::Incomplete => "incomplete",
            Self::InProgress => "in_progress",
        }
    }
}

/// Additional details about the status.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeResponseStatusDetails {
    /// A description of the error that caused the response to fail,
    /// populated when the `status` is `failed`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<RealtimeResponseStatusDetailsError>,
    /// The reason the Response did not complete. For a `cancelled` Response,
    /// one of `turn_detected` (the server VAD detected a new start of speech)
    /// or `client_cancelled` (the client sent a cancel event). For an
    /// `incomplete` Response, one of `max_output_tokens` or `content_filter`
    /// (the server-side safety filter activated and cut off the response).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<RealtimeResponseStatusDetailsReason>,
    /// The type of error that caused the response to fail, corresponding
    /// with the `status` field (`completed`, `cancelled`, `incomplete`,
    /// `failed`).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<RealtimeResponseStatusDetailsType>,
}

/// A description of the error that caused the response to fail,
/// populated when the `status` is `failed`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeResponseStatusDetailsError {
    /// Error code, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The type of error.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// The reason the Response did not complete. For a `cancelled` Response,
/// one of `turn_detected` (the server VAD detected a new start of speech)
/// or `client_cancelled` (the client sent a cancel event). For an
/// `incomplete` Response, one of `max_output_tokens` or `content_filter`
/// (the server-side safety filter activated and cut off the response).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum RealtimeResponseStatusDetailsReason {
    #[serde(rename = "turn_detected")]
    TurnDetected,
    #[serde(rename = "client_cancelled")]
    ClientCancelled,
    #[serde(rename = "max_output_tokens")]
    MaxOutputTokens,
    #[serde(rename = "content_filter")]
    ContentFilter,
}

impl RealtimeResponseStatusDetailsReason {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::TurnDetected => "turn_detected",
            Self::ClientCancelled => "client_cancelled",
            Self::MaxOutputTokens => "max_output_tokens",
            Self::ContentFilter => "content_filter",
        }
    }
}

/// The type of error that caused the response to fail, corresponding
/// with the `status` field (`completed`, `cancelled`, `incomplete`,
/// `failed`).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum RealtimeResponseStatusDetailsType {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "incomplete")]
    Incomplete,
}

impl RealtimeResponseStatusDetailsType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
            Self::Incomplete => "incomplete",
        }
    }
}

/// Usage statistics for the Response, this will correspond to billing. A
/// Realtime API session will maintain a conversation context and append new
/// Items to the Conversation, thus output from previous turns (text and
/// audio tokens) will become the input for later turns.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeResponseUsage {
    /// Details about the input tokens used in the Response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_token_details: Option<RealtimeResponseUsageInputTokenDetails>,
    /// The number of input tokens used in the Response, including text and
    /// audio tokens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<i64>,
    /// Details about the output tokens used in the Response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_token_details: Option<RealtimeResponseUsageOutputTokenDetails>,
    /// The number of output tokens sent in the Response, including text and
    /// audio tokens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<i64>,
    /// The total number of tokens in the Response including input and output
    /// text and audio tokens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<i64>,
}

/// Details about the input tokens used in the Response.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeResponseUsageInputTokenDetails {
    /// The number of audio tokens used in the Response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<i64>,
    /// The number of cached tokens used in the Response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cached_tokens: Option<i64>,
    /// The number of text tokens used in the Response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_tokens: Option<i64>,
}

/// Details about the output tokens used in the Response.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeResponseUsageOutputTokenDetails {
    /// The number of audio tokens used in the Response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<i64>,
    /// The number of text tokens used in the Response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_tokens: Option<i64>,
}

/// A realtime server event.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(tag = "type")]
pub enum RealtimeServerEvent {
    #[serde(rename = "conversation.created")]
    ConversationCreated {
        /// The conversation resource.
        conversation: RealtimeServerEventConversationCreatedConversation,
        /// The unique ID of the server event.
        event_id: String,
    },
    #[serde(rename = "conversation.item.created")]
    ConversationItemCreated {
        /// The unique ID of the server event.
        event_id: String,
        item: RealtimeConversationItem,
        /// The ID of the preceding item in the Conversation context, allows the
        /// client to understand the order of the conversation. Can be `null` if the
        /// item has no predecessor.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        previous_item_id: Option<String>,
    },
    #[serde(rename = "conversation.item.deleted")]
    ConversationItemDeleted {
        /// The unique ID of the server event.
        event_id: String,
        /// The ID of the item that was deleted.
        item_id: String,
    },
    #[serde(rename = "conversation.item.input_audio_transcription.completed")]
    ConversationItemInputAudioTranscriptionCompleted {
        /// The index of the content part containing the audio.
        content_index: i64,
        /// The unique ID of the server event.
        event_id: String,
        /// The ID of the user message item containing the audio.
        item_id: String,
        /// The log probabilities of the transcription.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        logprobs: Option<Vec<LogProbProperties>>,
        /// The transcribed text.
        transcript: String,
        /// Usage statistics for the transcription.
        usage: RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage,
    },
    #[serde(rename = "conversation.item.input_audio_transcription.delta")]
    ConversationItemInputAudioTranscriptionDelta {
        /// The index of the content part in the item's content array.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        content_index: Option<i64>,
        /// The text delta.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        delta: Option<String>,
        /// The unique ID of the server event.
        event_id: String,
        /// The ID of the item.
        item_id: String,
        /// The log probabilities of the transcription.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        logprobs: Option<Vec<LogProbProperties>>,
    },
    #[serde(rename = "conversation.item.input_audio_transcription.failed")]
    ConversationItemInputAudioTranscriptionFailed {
        /// The index of the content part containing the audio.
        content_index: i64,
        /// Details of the transcription error.
        error: RealtimeServerEventConversationItemInputAudioTranscriptionFailedError,
        /// The unique ID of the server event.
        event_id: String,
        /// The ID of the user message item.
        item_id: String,
    },
    #[serde(rename = "conversation.item.retrieved")]
    ConversationItemRetrieved {
        /// The unique ID of the server event.
        event_id: String,
        item: RealtimeConversationItem,
    },
    #[serde(rename = "conversation.item.truncated")]
    ConversationItemTruncated {
        /// The duration up to which the audio was truncated, in milliseconds.
        audio_end_ms: i64,
        /// The index of the content part that was truncated.
        content_index: i64,
        /// The unique ID of the server event.
        event_id: String,
        /// The ID of the assistant message item that was truncated.
        item_id: String,
    },
    #[serde(rename = "error")]
    Error {
        /// Details of the error.
        error: RealtimeServerEventErrorError,
        /// The unique ID of the server event.
        event_id: String,
    },
    #[serde(rename = "input_audio_buffer.cleared")]
    InputAudioBufferCleared {
        /// The unique ID of the server event.
        event_id: String,
    },
    #[serde(rename = "input_audio_buffer.committed")]
    InputAudioBufferCommitted {
        /// The unique ID of the server event.
        event_id: String,
        /// The ID of the user message item that will be created.
        item_id: String,
        /// The ID of the preceding item after which the new item will be inserted.
        /// Can be `null` if the item has no predecessor.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        previous_item_id: Option<String>,
    },
    #[serde(rename = "input_audio_buffer.speech_started")]
    InputAudioBufferSpeechStarted {
        /// Milliseconds from the start of all audio written to the buffer during the
        /// session when speech was first detected. This will correspond to the
        /// beginning of audio sent to the model, and thus includes the
        /// `prefix_padding_ms` configured in the Session.
        audio_start_ms: i64,
        /// The unique ID of the server event.
        event_id: String,
        /// The ID of the user message item that will be created when speech stops.
        item_id: String,
    },
    #[serde(rename = "input_audio_buffer.speech_stopped")]
    InputAudioBufferSpeechStopped {
        /// Milliseconds since the session started when speech stopped. This will
        /// correspond to the end of audio sent to the model, and thus includes the
        /// `min_silence_duration_ms` configured in the Session.
        audio_end_ms: i64,
        /// The unique ID of the server event.
        event_id: String,
        /// The ID of the user message item that will be created.
        item_id: String,
    },
    #[serde(rename = "rate_limits.updated")]
    RateLimitsUpdated {
        /// The unique ID of the server event.
        event_id: String,
        /// List of rate limit information.
        rate_limits: Vec<RealtimeServerEventRateLimitsUpdatedRateLimitsItem>,
    },
    #[serde(rename = "response.audio.delta")]
    ResponseAudioDelta {
        /// The index of the content part in the item's content array.
        content_index: i64,
        /// Base64-encoded audio data delta.
        delta: String,
        /// The unique ID of the server event.
        event_id: String,
        /// The ID of the item.
        item_id: String,
        /// The index of the output item in the response.
        output_index: i64,
        /// The ID of the response.
        response_id: String,
    },
    #[serde(rename = "response.audio.done")]
    ResponseAudioDone {
        /// The index of the content part in the item's content array.
        content_index: i64,
        /// The unique ID of the server event.
        event_id: String,
        /// The ID of the item.
        item_id: String,
        /// The index of the output item in the response.
        output_index: i64,
        /// The ID of the response.
        response_id: String,
    },
    #[serde(rename = "response.audio_transcript.delta")]
    ResponseAudioTranscriptDelta {
        /// The index of the content part in the item's content array.
        content_index: i64,
        /// The transcript delta.
        delta: String,
        /// The unique ID of the server event.
        event_id: String,
        /// The ID of the item.
        item_id: String,
        /// The index of the output item in the response.
        output_index: i64,
        /// The ID of the response.
        response_id: String,
    },
    #[serde(rename = "response.audio_transcript.done")]
    ResponseAudioTranscriptDone {
        /// The index of the content part in the item's content array.
        content_index: i64,
        /// The unique ID of the server event.
        event_id: String,
        /// The ID of the item.
        item_id: String,
        /// The index of the output item in the response.
        output_index: i64,
        /// The ID of the response.
        response_id: String,
        /// The final transcript of the audio.
        transcript: String,
    },
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAdded {
        /// The index of the content part in the item's content array.
        content_index: i64,
        /// The unique ID of the server event.
        event_id: String,
        /// The ID of the item to which the content part was added.
        item_id: String,
        /// The index of the output item in the response.
        output_index: i64,
        /// The content part that was added.
        part: RealtimeServerEventResponseContentPartAddedPart,
        /// The ID of the response.
        response_id: String,
    },
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDone {
        /// The index of the content part in the item's content array.
        content_index: i64,
        /// The unique ID of the server event.
        event_id: String,
        /// The ID of the item.
        item_id: String,
        /// The index of the output item in the response.
        output_index: i64,
        /// The content part that is done.
        part: RealtimeServerEventResponseContentPartDonePart,
        /// The ID of the response.
        response_id: String,
    },
    #[serde(rename = "response.created")]
    ResponseCreated {
        /// The unique ID of the server event.
        event_id: String,
        response: RealtimeResponse,
    },
    #[serde(rename = "response.done")]
    ResponseDone {
        /// The unique ID of the server event.
        event_id: String,
        response: RealtimeResponse,
    },
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDelta {
        /// The ID of the function call.
        call_id: String,
        /// The arguments delta as a JSON string.
        delta: String,
        /// The unique ID of the server event.
        event_id: String,
        /// The ID of the function call item.
        item_id: String,
        /// The index of the output item in the response.
        output_index: i64,
        /// The ID of the response.
        response_id: String,
    },
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDone {
        /// The final arguments as a JSON string.
        arguments: String,
        /// The ID of the function call.
        call_id: String,
        /// The unique ID of the server event.
        event_id: String,
        /// The ID of the function call item.
        item_id: String,
        /// The index of the output item in the response.
        output_index: i64,
        /// The ID of the response.
        response_id: String,
    },
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAdded {
        /// The unique ID of the server event.
        event_id: String,
        item: RealtimeConversationItem,
        /// The index of the output item in the Response.
        output_index: i64,
        /// The ID of the Response to which the item belongs.
        response_id: String,
    },
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDone {
        /// The unique ID of the server event.
        event_id: String,
        item: RealtimeConversationItem,
        /// The index of the output item in the Response.
        output_index: i64,
        /// The ID of the Response to which the item belongs.
        response_id: String,
    },
    #[serde(rename = "response.text.delta")]
    ResponseTextDelta {
        /// The index of the content part in the item's content array.
        content_index: i64,
        /// The text delta.
        delta: String,
        /// The unique ID of the server event.
        event_id: String,
        /// The ID of the item.
        item_id: String,
        /// The index of the output item in the response.
        output_index: i64,
        /// The ID of the response.
        response_id: String,
    },
    #[serde(rename = "response.text.done")]
    ResponseTextDone {
        /// The index of the content part in the item's content array.
        content_index: i64,
        /// The unique ID of the server event.
        event_id: String,
        /// The ID of the item.
        item_id: String,
        /// The index of the output item in the response.
        output_index: i64,
        /// The ID of the response.
        response_id: String,
        /// The final text content.
        text: String,
    },
    #[serde(rename = "session.created")]
    SessionCreated {
        /// The unique ID of the server event.
        event_id: String,
        session: RealtimeSession,
    },
    #[serde(rename = "session.updated")]
    SessionUpdated {
        /// The unique ID of the server event.
        event_id: String,
        session: RealtimeSession,
    },
    #[serde(rename = "transcription_session.updated")]
    TranscriptionSessionUpdated {
        /// The unique ID of the server event.
        event_id: String,
        session: RealtimeTranscriptionSessionCreateResponse,
    },
    #[serde(rename = "output_audio_buffer.started")]
    OutputAudioBufferStarted {
        /// The unique ID of the server event.
        event_id: String,
        /// The unique ID of the response that produced the audio.
        response_id: String,
    },
    #[serde(rename = "output_audio_buffer.stopped")]
    OutputAudioBufferStopped {
        /// The unique ID of the server event.
        event_id: String,
        /// The unique ID of the response that produced the audio.
        response_id: String,
    },
    #[serde(rename = "output_audio_buffer.cleared")]
    OutputAudioBufferCleared {
        /// The unique ID of the server event.
        event_id: String,
        /// The unique ID of the response that produced the audio.
        response_id: String,
    },
    #[serde(rename = "transcription_session.created")]
    TranscriptionSessionCreated {
        /// The unique ID of the server event.
        event_id: String,
        session: RealtimeTranscriptionSessionCreateResponse,
    },
}

/// Returned when a conversation is created. Emitted right after session creation.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventConversationCreated {
    /// The conversation resource.
    pub conversation: RealtimeServerEventConversationCreatedConversation,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be `conversation.created`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventConversationCreatedType,
}

/// The conversation resource.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventConversationCreatedConversation {
    /// The unique ID of the conversation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The object type, must be `realtime.conversation`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

impl crate::HasId for RealtimeServerEventConversationCreatedConversation {
    fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}

/// The event type, must be `conversation.created`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventConversationCreatedType {
    #[default]
    #[serde(rename = "conversation.created")]
    ConversationCreated,
}

impl RealtimeServerEventConversationCreatedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ConversationCreated => "conversation.created",
        }
    }
}

/// Returned when a conversation item is created. There are several scenarios that produce this event:
///   - The server is generating a Response, which if successful will produce
///     either one or two Items, which will be of type `message`
///     (role `assistant`) or type `function_call`.
///   - The input audio buffer has been committed, either by the client or the
///     server (in `server_vad` mode). The server will take the content of the
///     input audio buffer and add it to a new user message Item.
///   - The client has sent a `conversation.item.create` event to add a new Item
///     to the Conversation.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventConversationItemCreated {
    /// The unique ID of the server event.
    pub event_id: String,
    pub item: RealtimeConversationItem,
    /// The ID of the preceding item in the Conversation context, allows the
    /// client to understand the order of the conversation. Can be `null` if the
    /// item has no predecessor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_item_id: Option<String>,
    /// The event type, must be `conversation.item.created`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventConversationItemCreatedType,
}

/// The event type, must be `conversation.item.created`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventConversationItemCreatedType {
    #[default]
    #[serde(rename = "conversation.item.created")]
    ConversationItemCreated,
}

impl RealtimeServerEventConversationItemCreatedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ConversationItemCreated => "conversation.item.created",
        }
    }
}

/// Returned when an item in the conversation is deleted by the client with a
/// `conversation.item.delete` event. This event is used to synchronize the
/// server's understanding of the conversation history with the client's view.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventConversationItemDeleted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item that was deleted.
    pub item_id: String,
    /// The event type, must be `conversation.item.deleted`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventConversationItemDeletedType,
}

/// The event type, must be `conversation.item.deleted`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventConversationItemDeletedType {
    #[default]
    #[serde(rename = "conversation.item.deleted")]
    ConversationItemDeleted,
}

impl RealtimeServerEventConversationItemDeletedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ConversationItemDeleted => "conversation.item.deleted",
        }
    }
}

/// This event is the output of audio transcription for user audio written to the
/// user audio buffer. Transcription begins when the input audio buffer is
/// committed by the client or server (in `server_vad` mode). Transcription runs
/// asynchronously with Response creation, so this event may come before or after
/// the Response events.
///
/// Realtime API models accept audio natively, and thus input transcription is a
/// separate process run on a separate ASR (Automatic Speech Recognition) model.
/// The transcript may diverge somewhat from the model's interpretation, and
/// should be treated as a rough guide.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionCompleted {
    /// The index of the content part containing the audio.
    pub content_index: i64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the user message item containing the audio.
    pub item_id: String,
    /// The log probabilities of the transcription.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Vec<LogProbProperties>>,
    /// The transcribed text.
    pub transcript: String,
    /// The event type, must be
    /// `conversation.item.input_audio_transcription.completed`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventConversationItemInputAudioTranscriptionCompletedType,
    /// Usage statistics for the transcription.
    pub usage: RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage,
}

/// The event type, must be
/// `conversation.item.input_audio_transcription.completed`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionCompletedType {
    #[default]
    #[serde(rename = "conversation.item.input_audio_transcription.completed")]
    ConversationItemInputAudioTranscriptionCompleted,
}

impl RealtimeServerEventConversationItemInputAudioTranscriptionCompletedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ConversationItemInputAudioTranscriptionCompleted => {
                "conversation.item.input_audio_transcription.completed"
            }
        }
    }
}

/// Usage statistics for the transcription.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage {
    #[serde(untagged)]
    TranscriptTextUsageTokens(TranscriptTextUsageTokens),
    #[serde(untagged)]
    TranscriptTextUsageDuration(TranscriptTextUsageDuration),
}

impl From<TranscriptTextUsageDuration>
    for RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage
{
    fn from(value: TranscriptTextUsageDuration) -> Self {
        RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage::TranscriptTextUsageDuration(value)
    }
}
impl From<TranscriptTextUsageTokens>
    for RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage
{
    fn from(value: TranscriptTextUsageTokens) -> Self {
        RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage::TranscriptTextUsageTokens(value)
    }
}
/// Returned when the text value of an input audio transcription content part is updated.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionDelta {
    /// The index of the content part in the item's content array.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_index: Option<i64>,
    /// The text delta.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delta: Option<String>,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The log probabilities of the transcription.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Vec<LogProbProperties>>,
    /// The event type, must be `conversation.item.input_audio_transcription.delta`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventConversationItemInputAudioTranscriptionDeltaType,
}

/// The event type, must be `conversation.item.input_audio_transcription.delta`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionDeltaType {
    #[default]
    #[serde(rename = "conversation.item.input_audio_transcription.delta")]
    ConversationItemInputAudioTranscriptionDelta,
}

impl RealtimeServerEventConversationItemInputAudioTranscriptionDeltaType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ConversationItemInputAudioTranscriptionDelta => {
                "conversation.item.input_audio_transcription.delta"
            }
        }
    }
}

/// Returned when input audio transcription is configured, and a transcription
/// request for a user message failed. These events are separate from other
/// `error` events so that the client can identify the related Item.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionFailed {
    /// The index of the content part containing the audio.
    pub content_index: i64,
    /// Details of the transcription error.
    pub error: RealtimeServerEventConversationItemInputAudioTranscriptionFailedError,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the user message item.
    pub item_id: String,
    /// The event type, must be
    /// `conversation.item.input_audio_transcription.failed`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventConversationItemInputAudioTranscriptionFailedType,
}

/// Details of the transcription error.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionFailedError {
    /// Error code, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// A human-readable error message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Parameter related to the error, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    /// The type of error.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// The event type, must be
/// `conversation.item.input_audio_transcription.failed`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionFailedType {
    #[default]
    #[serde(rename = "conversation.item.input_audio_transcription.failed")]
    ConversationItemInputAudioTranscriptionFailed,
}

impl RealtimeServerEventConversationItemInputAudioTranscriptionFailedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ConversationItemInputAudioTranscriptionFailed => {
                "conversation.item.input_audio_transcription.failed"
            }
        }
    }
}

/// Returned when a conversation item is retrieved with `conversation.item.retrieve`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventConversationItemRetrieved {
    /// The unique ID of the server event.
    pub event_id: String,
    pub item: RealtimeConversationItem,
    /// The event type, must be `conversation.item.retrieved`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventConversationItemRetrievedType,
}

/// The event type, must be `conversation.item.retrieved`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventConversationItemRetrievedType {
    #[default]
    #[serde(rename = "conversation.item.retrieved")]
    ConversationItemRetrieved,
}

impl RealtimeServerEventConversationItemRetrievedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ConversationItemRetrieved => "conversation.item.retrieved",
        }
    }
}

/// Returned when an earlier assistant audio message item is truncated by the
/// client with a `conversation.item.truncate` event. This event is used to
/// synchronize the server's understanding of the audio with the client's playback.
///
/// This action will truncate the audio and remove the server-side text transcript
/// to ensure there is no text in the context that hasn't been heard by the user.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventConversationItemTruncated {
    /// The duration up to which the audio was truncated, in milliseconds.
    pub audio_end_ms: i64,
    /// The index of the content part that was truncated.
    pub content_index: i64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the assistant message item that was truncated.
    pub item_id: String,
    /// The event type, must be `conversation.item.truncated`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventConversationItemTruncatedType,
}

/// The event type, must be `conversation.item.truncated`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventConversationItemTruncatedType {
    #[default]
    #[serde(rename = "conversation.item.truncated")]
    ConversationItemTruncated,
}

impl RealtimeServerEventConversationItemTruncatedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ConversationItemTruncated => "conversation.item.truncated",
        }
    }
}

/// Returned when an error occurs, which could be a client problem or a server
/// problem. Most errors are recoverable and the session will stay open, we
/// recommend to implementors to monitor and log error messages by default.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventError {
    /// Details of the error.
    pub error: RealtimeServerEventErrorError,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be `error`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventErrorType,
}

/// Details of the error.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventErrorError {
    /// Error code, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The event_id of the client event that caused the error, if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// A human-readable error message.
    pub message: String,
    /// Parameter related to the error, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    /// The type of error (e.g., "invalid_request_error", "server_error").
    #[serde(rename = "type")]
    pub r#type: String,
}

/// The event type, must be `error`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventErrorType {
    #[default]
    #[serde(rename = "error")]
    Error,
}

impl RealtimeServerEventErrorType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Error => "error",
        }
    }
}

/// Returned when the input audio buffer is cleared by the client with a
/// `input_audio_buffer.clear` event.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventInputAudioBufferCleared {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The event type, must be `input_audio_buffer.cleared`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventInputAudioBufferClearedType,
}

/// The event type, must be `input_audio_buffer.cleared`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventInputAudioBufferClearedType {
    #[default]
    #[serde(rename = "input_audio_buffer.cleared")]
    InputAudioBufferCleared,
}

impl RealtimeServerEventInputAudioBufferClearedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::InputAudioBufferCleared => "input_audio_buffer.cleared",
        }
    }
}

/// Returned when an input audio buffer is committed, either by the client or
/// automatically in server VAD mode. The `item_id` property is the ID of the user
/// message item that will be created, thus a `conversation.item.created` event
/// will also be sent to the client.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventInputAudioBufferCommitted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the user message item that will be created.
    pub item_id: String,
    /// The ID of the preceding item after which the new item will be inserted.
    /// Can be `null` if the item has no predecessor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_item_id: Option<String>,
    /// The event type, must be `input_audio_buffer.committed`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventInputAudioBufferCommittedType,
}

/// The event type, must be `input_audio_buffer.committed`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventInputAudioBufferCommittedType {
    #[default]
    #[serde(rename = "input_audio_buffer.committed")]
    InputAudioBufferCommitted,
}

impl RealtimeServerEventInputAudioBufferCommittedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::InputAudioBufferCommitted => "input_audio_buffer.committed",
        }
    }
}

/// Sent by the server when in `server_vad` mode to indicate that speech has been
/// detected in the audio buffer. This can happen any time audio is added to the
/// buffer (unless speech is already detected). The client may want to use this
/// event to interrupt audio playback or provide visual feedback to the user.
///
/// The client should expect to receive a `input_audio_buffer.speech_stopped` event
/// when speech stops. The `item_id` property is the ID of the user message item
/// that will be created when speech stops and will also be included in the
/// `input_audio_buffer.speech_stopped` event (unless the client manually commits
/// the audio buffer during VAD activation).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventInputAudioBufferSpeechStarted {
    /// Milliseconds from the start of all audio written to the buffer during the
    /// session when speech was first detected. This will correspond to the
    /// beginning of audio sent to the model, and thus includes the
    /// `prefix_padding_ms` configured in the Session.
    pub audio_start_ms: i64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the user message item that will be created when speech stops.
    pub item_id: String,
    /// The event type, must be `input_audio_buffer.speech_started`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventInputAudioBufferSpeechStartedType,
}

/// The event type, must be `input_audio_buffer.speech_started`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventInputAudioBufferSpeechStartedType {
    #[default]
    #[serde(rename = "input_audio_buffer.speech_started")]
    InputAudioBufferSpeechStarted,
}

impl RealtimeServerEventInputAudioBufferSpeechStartedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::InputAudioBufferSpeechStarted => "input_audio_buffer.speech_started",
        }
    }
}

/// Returned in `server_vad` mode when the server detects the end of speech in
/// the audio buffer. The server will also send an `conversation.item.created`
/// event with the user message item that is created from the audio buffer.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventInputAudioBufferSpeechStopped {
    /// Milliseconds since the session started when speech stopped. This will
    /// correspond to the end of audio sent to the model, and thus includes the
    /// `min_silence_duration_ms` configured in the Session.
    pub audio_end_ms: i64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the user message item that will be created.
    pub item_id: String,
    /// The event type, must be `input_audio_buffer.speech_stopped`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventInputAudioBufferSpeechStoppedType,
}

/// The event type, must be `input_audio_buffer.speech_stopped`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventInputAudioBufferSpeechStoppedType {
    #[default]
    #[serde(rename = "input_audio_buffer.speech_stopped")]
    InputAudioBufferSpeechStopped,
}

impl RealtimeServerEventInputAudioBufferSpeechStoppedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::InputAudioBufferSpeechStopped => "input_audio_buffer.speech_stopped",
        }
    }
}

/// **WebRTC Only:** Emitted when the output audio buffer is cleared. This happens either in VAD
/// mode when the user has interrupted (`input_audio_buffer.speech_started`),
/// or when the client has emitted the `output_audio_buffer.clear` event to manually
/// cut off the current audio response.
/// [Learn more](/docs/guides/realtime-conversations#client-and-server-events-for-audio-in-webrtc).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventOutputAudioBufferCleared {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The unique ID of the response that produced the audio.
    pub response_id: String,
    /// The event type, must be `output_audio_buffer.cleared`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventOutputAudioBufferClearedType,
}

/// The event type, must be `output_audio_buffer.cleared`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventOutputAudioBufferClearedType {
    #[default]
    #[serde(rename = "output_audio_buffer.cleared")]
    OutputAudioBufferCleared,
}

impl RealtimeServerEventOutputAudioBufferClearedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::OutputAudioBufferCleared => "output_audio_buffer.cleared",
        }
    }
}

/// **WebRTC Only:** Emitted when the server begins streaming audio to the client. This event is
/// emitted after an audio content part has been added (`response.content_part.added`)
/// to the response.
/// [Learn more](/docs/guides/realtime-conversations#client-and-server-events-for-audio-in-webrtc).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventOutputAudioBufferStarted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The unique ID of the response that produced the audio.
    pub response_id: String,
    /// The event type, must be `output_audio_buffer.started`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventOutputAudioBufferStartedType,
}

/// The event type, must be `output_audio_buffer.started`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventOutputAudioBufferStartedType {
    #[default]
    #[serde(rename = "output_audio_buffer.started")]
    OutputAudioBufferStarted,
}

impl RealtimeServerEventOutputAudioBufferStartedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::OutputAudioBufferStarted => "output_audio_buffer.started",
        }
    }
}

/// **WebRTC Only:** Emitted when the output audio buffer has been completely drained on the server,
/// and no more audio is forthcoming. This event is emitted after the full response
/// data has been sent to the client (`response.done`).
/// [Learn more](/docs/guides/realtime-conversations#client-and-server-events-for-audio-in-webrtc).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventOutputAudioBufferStopped {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The unique ID of the response that produced the audio.
    pub response_id: String,
    /// The event type, must be `output_audio_buffer.stopped`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventOutputAudioBufferStoppedType,
}

/// The event type, must be `output_audio_buffer.stopped`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventOutputAudioBufferStoppedType {
    #[default]
    #[serde(rename = "output_audio_buffer.stopped")]
    OutputAudioBufferStopped,
}

impl RealtimeServerEventOutputAudioBufferStoppedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::OutputAudioBufferStopped => "output_audio_buffer.stopped",
        }
    }
}

/// Emitted at the beginning of a Response to indicate the updated rate limits.
/// When a Response is created some tokens will be "reserved" for the output
/// tokens, the rate limits shown here reflect that reservation, which is then
/// adjusted accordingly once the Response is completed.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventRateLimitsUpdated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// List of rate limit information.
    pub rate_limits: Vec<RealtimeServerEventRateLimitsUpdatedRateLimitsItem>,
    /// The event type, must be `rate_limits.updated`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventRateLimitsUpdatedType,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventRateLimitsUpdatedRateLimitsItem {
    /// The maximum allowed value for the rate limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The name of the rate limit (`requests`, `tokens`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<RealtimeServerEventRateLimitsUpdatedRateLimitsItemName>,
    /// The remaining value before the limit is reached.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i64>,
    /// Seconds until the rate limit resets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reset_seconds: Option<f64>,
}

/// The name of the rate limit (`requests`, `tokens`).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum RealtimeServerEventRateLimitsUpdatedRateLimitsItemName {
    #[serde(rename = "requests")]
    Requests,
    #[serde(rename = "tokens")]
    Tokens,
}

impl RealtimeServerEventRateLimitsUpdatedRateLimitsItemName {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Requests => "requests",
            Self::Tokens => "tokens",
        }
    }
}

/// The event type, must be `rate_limits.updated`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventRateLimitsUpdatedType {
    #[default]
    #[serde(rename = "rate_limits.updated")]
    RateLimitsUpdated,
}

impl RealtimeServerEventRateLimitsUpdatedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::RateLimitsUpdated => "rate_limits.updated",
        }
    }
}

/// Returned when the model-generated audio is updated.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventResponseAudioDelta {
    /// The index of the content part in the item's content array.
    pub content_index: i64,
    /// Base64-encoded audio data delta.
    pub delta: String,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: i64,
    /// The ID of the response.
    pub response_id: String,
    /// The event type, must be `response.audio.delta`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventResponseAudioDeltaType,
}

/// The event type, must be `response.audio.delta`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventResponseAudioDeltaType {
    #[default]
    #[serde(rename = "response.audio.delta")]
    ResponseAudioDelta,
}

impl RealtimeServerEventResponseAudioDeltaType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseAudioDelta => "response.audio.delta",
        }
    }
}

/// Returned when the model-generated audio is done. Also emitted when a Response
/// is interrupted, incomplete, or cancelled.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventResponseAudioDone {
    /// The index of the content part in the item's content array.
    pub content_index: i64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: i64,
    /// The ID of the response.
    pub response_id: String,
    /// The event type, must be `response.audio.done`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventResponseAudioDoneType,
}

/// The event type, must be `response.audio.done`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventResponseAudioDoneType {
    #[default]
    #[serde(rename = "response.audio.done")]
    ResponseAudioDone,
}

impl RealtimeServerEventResponseAudioDoneType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseAudioDone => "response.audio.done",
        }
    }
}

/// Returned when the model-generated transcription of audio output is updated.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventResponseAudioTranscriptDelta {
    /// The index of the content part in the item's content array.
    pub content_index: i64,
    /// The transcript delta.
    pub delta: String,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: i64,
    /// The ID of the response.
    pub response_id: String,
    /// The event type, must be `response.audio_transcript.delta`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventResponseAudioTranscriptDeltaType,
}

/// The event type, must be `response.audio_transcript.delta`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventResponseAudioTranscriptDeltaType {
    #[default]
    #[serde(rename = "response.audio_transcript.delta")]
    ResponseAudioTranscriptDelta,
}

impl RealtimeServerEventResponseAudioTranscriptDeltaType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseAudioTranscriptDelta => "response.audio_transcript.delta",
        }
    }
}

/// Returned when the model-generated transcription of audio output is done
/// streaming. Also emitted when a Response is interrupted, incomplete, or
/// cancelled.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventResponseAudioTranscriptDone {
    /// The index of the content part in the item's content array.
    pub content_index: i64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: i64,
    /// The ID of the response.
    pub response_id: String,
    /// The final transcript of the audio.
    pub transcript: String,
    /// The event type, must be `response.audio_transcript.done`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventResponseAudioTranscriptDoneType,
}

/// The event type, must be `response.audio_transcript.done`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventResponseAudioTranscriptDoneType {
    #[default]
    #[serde(rename = "response.audio_transcript.done")]
    ResponseAudioTranscriptDone,
}

impl RealtimeServerEventResponseAudioTranscriptDoneType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseAudioTranscriptDone => "response.audio_transcript.done",
        }
    }
}

/// Returned when a new content part is added to an assistant message item during
/// response generation.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventResponseContentPartAdded {
    /// The index of the content part in the item's content array.
    pub content_index: i64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item to which the content part was added.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: i64,
    /// The content part that was added.
    pub part: RealtimeServerEventResponseContentPartAddedPart,
    /// The ID of the response.
    pub response_id: String,
    /// The event type, must be `response.content_part.added`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventResponseContentPartAddedType,
}

/// The content part that was added.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventResponseContentPartAddedPart {
    /// Base64-encoded audio data (if type is "audio").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    /// The text content (if type is "text").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The transcript of the audio (if type is "audio").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
    /// The content type ("text", "audio").
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<Modalities>,
}

/// The event type, must be `response.content_part.added`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventResponseContentPartAddedType {
    #[default]
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAdded,
}

impl RealtimeServerEventResponseContentPartAddedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseContentPartAdded => "response.content_part.added",
        }
    }
}

/// Returned when a content part is done streaming in an assistant message item.
/// Also emitted when a Response is interrupted, incomplete, or cancelled.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventResponseContentPartDone {
    /// The index of the content part in the item's content array.
    pub content_index: i64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: i64,
    /// The content part that is done.
    pub part: RealtimeServerEventResponseContentPartDonePart,
    /// The ID of the response.
    pub response_id: String,
    /// The event type, must be `response.content_part.done`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventResponseContentPartDoneType,
}

/// The content part that is done.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventResponseContentPartDonePart {
    /// Base64-encoded audio data (if type is "audio").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    /// The text content (if type is "text").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The transcript of the audio (if type is "audio").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
    /// The content type ("text", "audio").
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<Modalities>,
}

/// The event type, must be `response.content_part.done`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventResponseContentPartDoneType {
    #[default]
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDone,
}

impl RealtimeServerEventResponseContentPartDoneType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseContentPartDone => "response.content_part.done",
        }
    }
}

/// Returned when a new Response is created. The first event of response creation,
/// where the response is in an initial state of `in_progress`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventResponseCreated {
    /// The unique ID of the server event.
    pub event_id: String,
    pub response: RealtimeResponse,
    /// The event type, must be `response.created`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventResponseCreatedType,
}

/// The event type, must be `response.created`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventResponseCreatedType {
    #[default]
    #[serde(rename = "response.created")]
    ResponseCreated,
}

impl RealtimeServerEventResponseCreatedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseCreated => "response.created",
        }
    }
}

/// Returned when a Response is done streaming. Always emitted, no matter the
/// final state. The Response object included in the `response.done` event will
/// include all output Items in the Response but will omit the raw audio data.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventResponseDone {
    /// The unique ID of the server event.
    pub event_id: String,
    pub response: RealtimeResponse,
    /// The event type, must be `response.done`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventResponseDoneType,
}

/// The event type, must be `response.done`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventResponseDoneType {
    #[default]
    #[serde(rename = "response.done")]
    ResponseDone,
}

impl RealtimeServerEventResponseDoneType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseDone => "response.done",
        }
    }
}

/// Returned when the model-generated function call arguments are updated.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventResponseFunctionCallArgumentsDelta {
    /// The ID of the function call.
    pub call_id: String,
    /// The arguments delta as a JSON string.
    pub delta: String,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the function call item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: i64,
    /// The ID of the response.
    pub response_id: String,
    /// The event type, must be `response.function_call_arguments.delta`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventResponseFunctionCallArgumentsDeltaType,
}

/// The event type, must be `response.function_call_arguments.delta`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventResponseFunctionCallArgumentsDeltaType {
    #[default]
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDelta,
}

impl RealtimeServerEventResponseFunctionCallArgumentsDeltaType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseFunctionCallArgumentsDelta => "response.function_call_arguments.delta",
        }
    }
}

/// Returned when the model-generated function call arguments are done streaming.
/// Also emitted when a Response is interrupted, incomplete, or cancelled.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventResponseFunctionCallArgumentsDone {
    /// The final arguments as a JSON string.
    pub arguments: String,
    /// The ID of the function call.
    pub call_id: String,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the function call item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: i64,
    /// The ID of the response.
    pub response_id: String,
    /// The event type, must be `response.function_call_arguments.done`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventResponseFunctionCallArgumentsDoneType,
}

/// The event type, must be `response.function_call_arguments.done`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventResponseFunctionCallArgumentsDoneType {
    #[default]
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDone,
}

impl RealtimeServerEventResponseFunctionCallArgumentsDoneType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseFunctionCallArgumentsDone => "response.function_call_arguments.done",
        }
    }
}

/// Returned when a new Item is created during Response generation.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventResponseOutputItemAdded {
    /// The unique ID of the server event.
    pub event_id: String,
    pub item: RealtimeConversationItem,
    /// The index of the output item in the Response.
    pub output_index: i64,
    /// The ID of the Response to which the item belongs.
    pub response_id: String,
    /// The event type, must be `response.output_item.added`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventResponseOutputItemAddedType,
}

/// The event type, must be `response.output_item.added`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventResponseOutputItemAddedType {
    #[default]
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAdded,
}

impl RealtimeServerEventResponseOutputItemAddedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseOutputItemAdded => "response.output_item.added",
        }
    }
}

/// Returned when an Item is done streaming. Also emitted when a Response is
/// interrupted, incomplete, or cancelled.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventResponseOutputItemDone {
    /// The unique ID of the server event.
    pub event_id: String,
    pub item: RealtimeConversationItem,
    /// The index of the output item in the Response.
    pub output_index: i64,
    /// The ID of the Response to which the item belongs.
    pub response_id: String,
    /// The event type, must be `response.output_item.done`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventResponseOutputItemDoneType,
}

/// The event type, must be `response.output_item.done`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventResponseOutputItemDoneType {
    #[default]
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDone,
}

impl RealtimeServerEventResponseOutputItemDoneType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseOutputItemDone => "response.output_item.done",
        }
    }
}

/// Returned when the text value of a "text" content part is updated.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventResponseTextDelta {
    /// The index of the content part in the item's content array.
    pub content_index: i64,
    /// The text delta.
    pub delta: String,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: i64,
    /// The ID of the response.
    pub response_id: String,
    /// The event type, must be `response.text.delta`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventResponseTextDeltaType,
}

/// The event type, must be `response.text.delta`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventResponseTextDeltaType {
    #[default]
    #[serde(rename = "response.text.delta")]
    ResponseTextDelta,
}

impl RealtimeServerEventResponseTextDeltaType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseTextDelta => "response.text.delta",
        }
    }
}

/// Returned when the text value of a "text" content part is done streaming. Also
/// emitted when a Response is interrupted, incomplete, or cancelled.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventResponseTextDone {
    /// The index of the content part in the item's content array.
    pub content_index: i64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: i64,
    /// The ID of the response.
    pub response_id: String,
    /// The final text content.
    pub text: String,
    /// The event type, must be `response.text.done`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventResponseTextDoneType,
}

/// The event type, must be `response.text.done`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventResponseTextDoneType {
    #[default]
    #[serde(rename = "response.text.done")]
    ResponseTextDone,
}

impl RealtimeServerEventResponseTextDoneType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseTextDone => "response.text.done",
        }
    }
}

/// Returned when a Session is created. Emitted automatically when a new
/// connection is established as the first server event. This event will contain
/// the default Session configuration.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventSessionCreated {
    /// The unique ID of the server event.
    pub event_id: String,
    pub session: RealtimeSession,
    /// The event type, must be `session.created`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventSessionCreatedType,
}

/// The event type, must be `session.created`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventSessionCreatedType {
    #[default]
    #[serde(rename = "session.created")]
    SessionCreated,
}

impl RealtimeServerEventSessionCreatedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::SessionCreated => "session.created",
        }
    }
}

/// Returned when a session is updated with a `session.update` event, unless
/// there is an error.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventSessionUpdated {
    /// The unique ID of the server event.
    pub event_id: String,
    pub session: RealtimeSession,
    /// The event type, must be `session.updated`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventSessionUpdatedType,
}

/// The event type, must be `session.updated`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventSessionUpdatedType {
    #[default]
    #[serde(rename = "session.updated")]
    SessionUpdated,
}

impl RealtimeServerEventSessionUpdatedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::SessionUpdated => "session.updated",
        }
    }
}

/// Returned when a transcription session is updated with a `transcription_session.update` event, unless
/// there is an error.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventTranscriptionSessionCreated {
    /// The unique ID of the server event.
    pub event_id: String,
    pub session: RealtimeTranscriptionSessionCreateResponse,
    /// The event type, must be `transcription_session.updated`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventTranscriptionSessionCreatedType,
}

/// The event type, must be `transcription_session.updated`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventTranscriptionSessionCreatedType {
    #[default]
    #[serde(rename = "transcription_session.created")]
    TranscriptionSessionCreated,
}

impl RealtimeServerEventTranscriptionSessionCreatedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::TranscriptionSessionCreated => "transcription_session.created",
        }
    }
}

/// Returned when a transcription session is updated with a `transcription_session.update` event, unless
/// there is an error.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeServerEventTranscriptionSessionUpdated {
    /// The unique ID of the server event.
    pub event_id: String,
    pub session: RealtimeTranscriptionSessionCreateResponse,
    /// The event type, must be `transcription_session.updated`.
    #[serde(rename = "type")]
    pub r#type: RealtimeServerEventTranscriptionSessionUpdatedType,
}

/// The event type, must be `transcription_session.updated`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeServerEventTranscriptionSessionUpdatedType {
    #[default]
    #[serde(rename = "transcription_session.updated")]
    TranscriptionSessionUpdated,
}

impl RealtimeServerEventTranscriptionSessionUpdatedType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::TranscriptionSessionUpdated => "transcription_session.updated",
        }
    }
}

/// Realtime session object configuration.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeSession {
    /// Unique identifier for the session that looks like `sess_1234567890abcdef`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
    /// For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate,
    /// single channel (mono), and little-endian byte order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_format: Option<InputAudioStreamFormat>,
    /// Configuration for input audio noise reduction. This can be set to `null` to turn off.
    /// Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
    /// Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_noise_reduction: Option<RealtimeSessionInputAudioNoiseReduction>,
    /// Configuration for input audio transcription, defaults to off and can be  set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs  asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_transcription: Option<RealtimeSessionInputAudioTranscription>,
    /// The default system instructions (i.e. system message) prepended to model
    /// calls. This field allows the client to guide the model on desired
    /// responses. The model can be instructed on response content and format,
    /// (e.g. "be extremely succinct", "act friendly", "here are examples of good
    /// responses") and on audio behavior (e.g. "talk quickly", "inject emotion
    /// into your voice", "laugh frequently"). The instructions are not
    /// guaranteed to be followed by the model, but they provide guidance to the
    /// model on the desired behavior.
    ///
    ///
    /// Note that the server sets default instructions which will be used if this
    /// field is not set and are visible in the `session.created` event at the
    /// start of the session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Maximum number of output tokens for a single assistant response,
    /// inclusive of tool calls. Provide an integer between 1 and 4096 to
    /// limit output tokens, or `inf` for the maximum available tokens for a
    /// given model. Defaults to `inf`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_response_output_tokens: Option<RealtimeSessionMaxResponseOutputTokens>,
    /// The set of modalities the model can respond with. To disable audio,
    /// set this to ["text"].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<Modalities>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<ModelId>,
    /// The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
    /// For `pcm16`, output audio is sampled at a rate of 24kHz.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_audio_format: Option<InputAudioStreamFormat>,
    /// The speed of the model's spoken response. 1.0 is the default speed. 0.25 is
    /// the minimum speed. 1.5 is the maximum speed. This value can only be changed
    /// in between model turns, not while a response is in progress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    /// Sampling temperature for the model, limited to [0.6, 1.2]. For audio models a temperature of 0.8 is highly recommended for best performance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// How the model chooses tools. Options are `auto`, `none`, `required`, or
    /// specify a function.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// Configuration options for tracing. Set to null to disable tracing. Once
    /// tracing is enabled for a session, the configuration cannot be modified.
    ///
    /// `auto` will create a trace for the session with default values for the
    /// workflow name, group id, and metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracing: Option<TracingConfigOrAuto>,
    /// Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.
    /// Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.
    /// Semantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with "uhhm", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<RealtimeSessionTurnDetection>,
    /// The voice the model uses to respond. Voice cannot be changed during the
    /// session once the model has responded with audio at least once. Current
    /// voice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `sage`,
    /// `shimmer`, and `verse`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<VoiceId>,
}

impl crate::HasId for RealtimeSession {
    fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}

/// Realtime session object configuration.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeSessionCreateRequest {
    /// Configuration options for the generated client secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<RealtimeSessionCreateRequestClientSecret>,
    /// The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
    /// For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate,
    /// single channel (mono), and little-endian byte order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_format: Option<InputAudioStreamFormat>,
    /// Configuration for input audio noise reduction. This can be set to `null` to turn off.
    /// Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
    /// Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_noise_reduction: Option<RealtimeSessionCreateRequestInputAudioNoiseReduction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_transcription: Option<RealtimeSessionInputAudioTranscription>,
    /// The default system instructions (i.e. system message) prepended to model calls. This field allows the client to guide the model on desired responses. The model can be instructed on response content and format, (e.g. "be extremely succinct", "act friendly", "here are examples of good responses") and on audio behavior (e.g. "talk quickly", "inject emotion into your voice", "laugh frequently"). The instructions are not guaranteed to be followed by the model, but they provide guidance to the model on the desired behavior.
    ///
    /// Note that the server sets default instructions which will be used if this field is not set and are visible in the `session.created` event at the start of the session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Maximum number of output tokens for a single assistant response,
    /// inclusive of tool calls. Provide an integer between 1 and 4096 to
    /// limit output tokens, or `inf` for the maximum available tokens for a
    /// given model. Defaults to `inf`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_response_output_tokens: Option<RealtimeSessionCreateRequestMaxResponseOutputTokens>,
    /// The set of modalities the model can respond with. To disable audio,
    /// set this to ["text"].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<Modalities>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<ModelId>,
    /// The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
    /// For `pcm16`, output audio is sampled at a rate of 24kHz.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_audio_format: Option<InputAudioStreamFormat>,
    /// The speed of the model's spoken response. 1.0 is the default speed. 0.25 is
    /// the minimum speed. 1.5 is the maximum speed. This value can only be changed
    /// in between model turns, not while a response is in progress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    /// Sampling temperature for the model, limited to [0.6, 1.2]. For audio models a temperature of 0.8 is highly recommended for best performance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// How the model chooses tools. Options are `auto`, `none`, `required`, or
    /// specify a function.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// Configuration options for tracing. Set to null to disable tracing. Once
    /// tracing is enabled for a session, the configuration cannot be modified.
    ///
    /// `auto` will create a trace for the session with default values for the
    /// workflow name, group id, and metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracing: Option<TracingConfigOrAuto>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<RealtimeSessionTurnDetection>,
    /// The voice the model uses to respond. Voice cannot be changed during the
    /// session once the model has responded with audio at least once. Current
    /// voice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `sage`,
    /// `shimmer`, and `verse`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<VoiceId>,
}

/// Configuration options for the generated client secret.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeSessionCreateRequestClientSecret {
    /// Configuration for the ephemeral token expiration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<RealtimeSessionCreateRequestClientSecretExpiresAfter>,
}

/// Configuration for the ephemeral token expiration.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeSessionCreateRequestClientSecretExpiresAfter {
    /// The anchor point for the ephemeral token expiration. Only `created_at` is currently supported.
    pub anchor: RealtimeSessionCreateRequestClientSecretExpiresAfterAnchor,
    /// The number of seconds from the anchor point to the expiration. Select a value between `10` and `7200`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seconds: Option<i64>,
}

/// The anchor point for the ephemeral token expiration. Only `created_at` is currently supported.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeSessionCreateRequestClientSecretExpiresAfterAnchor {
    #[default]
    #[serde(rename = "created_at")]
    CreatedAt,
}

impl RealtimeSessionCreateRequestClientSecretExpiresAfterAnchor {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::CreatedAt => "created_at",
        }
    }
}

/// Configuration for input audio noise reduction. This can be set to `null` to turn off.
/// Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
/// Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeSessionCreateRequestInputAudioNoiseReduction {
    /// Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<InputAudioStreamFormatNoiseReductionType>,
}

/// Maximum number of output tokens for a single assistant response,
/// inclusive of tool calls. Provide an integer between 1 and 4096 to
/// limit output tokens, or `inf` for the maximum available tokens for a
/// given model. Defaults to `inf`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum RealtimeSessionCreateRequestMaxResponseOutputTokens {
    #[serde(rename = "inf")]
    Inf,
    #[serde(untagged)]
    Variant0(i64),
}

impl From<i64> for RealtimeSessionCreateRequestMaxResponseOutputTokens {
    fn from(value: i64) -> Self {
        RealtimeSessionCreateRequestMaxResponseOutputTokens::Variant0(value)
    }
}
/// A new Realtime session configuration, with an ephermeral key. Default TTL
/// for keys is one minute.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeSessionCreateResponse {
    /// Ephemeral key returned by the API.
    pub client_secret: RealtimeSessionCreateResponseClientSecret,
    /// The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
    /// For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate,
    /// single channel (mono), and little-endian byte order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_format: Option<InputAudioStreamFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_transcription: Option<RealtimeSessionInputAudioTranscription>,
    /// The default system instructions (i.e. system message) prepended to model
    /// calls. This field allows the client to guide the model on desired
    /// responses. The model can be instructed on response content and format,
    /// (e.g. "be extremely succinct", "act friendly", "here are examples of good
    /// responses") and on audio behavior (e.g. "talk quickly", "inject emotion
    /// into your voice", "laugh frequently"). The instructions are not guaranteed
    /// to be followed by the model, but they provide guidance to the model on the
    /// desired behavior.
    ///
    /// Note that the server sets default instructions which will be used if this
    /// field is not set and are visible in the `session.created` event at the
    /// start of the session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Maximum number of output tokens for a single assistant response,
    /// inclusive of tool calls. Provide an integer between 1 and 4096 to
    /// limit output tokens, or `inf` for the maximum available tokens for a
    /// given model. Defaults to `inf`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_response_output_tokens: Option<RealtimeSessionCreateResponseMaxResponseOutputTokens>,
    /// The set of modalities the model can respond with. To disable audio,
    /// set this to ["text"].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<Modalities>>,
    /// The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_audio_format: Option<String>,
    /// The speed of the model's spoken response. 1.0 is the default speed. 0.25 is
    /// the minimum speed. 1.5 is the maximum speed. This value can only be changed
    /// in between model turns, not while a response is in progress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    /// Sampling temperature for the model, limited to [0.6, 1.2]. Defaults to 0.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// How the model chooses tools. Options are `auto`, `none`, `required`, or
    /// specify a function.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,
    /// Tools (functions) available to the model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<RealtimeSessionCreateResponseToolsItem>>,
    /// Configuration options for tracing. Set to null to disable tracing. Once
    /// tracing is enabled for a session, the configuration cannot be modified.
    ///
    /// `auto` will create a trace for the session with default values for the
    /// workflow name, group id, and metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracing: Option<TracingConfigOrAuto>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<RealtimeSessionTurnDetection>,
    /// The voice the model uses to respond. Voice cannot be changed during the
    /// session once the model has responded with audio at least once. Current
    /// voice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `sage`,
    /// `shimmer`, and `verse`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<VoiceId>,
}

/// Ephemeral key returned by the API.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeSessionCreateResponseClientSecret {
    /// Timestamp for when the token expires. Currently, all tokens expire
    /// after one minute.
    pub expires_at: i64,
    /// Ephemeral key usable in client environments to authenticate connections
    /// to the Realtime API. Use this in client-side environments rather than
    /// a standard API token, which should only be used server-side.
    pub value: String,
}

/// Maximum number of output tokens for a single assistant response,
/// inclusive of tool calls. Provide an integer between 1 and 4096 to
/// limit output tokens, or `inf` for the maximum available tokens for a
/// given model. Defaults to `inf`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum RealtimeSessionCreateResponseMaxResponseOutputTokens {
    #[serde(rename = "inf")]
    Inf,
    #[serde(untagged)]
    Variant0(i64),
}

impl From<i64> for RealtimeSessionCreateResponseMaxResponseOutputTokens {
    fn from(value: i64) -> Self {
        RealtimeSessionCreateResponseMaxResponseOutputTokens::Variant0(value)
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeSessionCreateResponseToolsItem {
    /// The description of the function, including guidance on when and how
    /// to call it, and guidance about what to tell the user when calling
    /// (if anything).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the function.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Parameters of the function in JSON Schema.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Map<String, serde_json::Value>>,
    /// The type of the tool, i.e. `function`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<RealtimeSessionCreateResponseToolsItemType>,
}

/// The type of the tool, i.e. `function`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeSessionCreateResponseToolsItemType {
    #[default]
    #[serde(rename = "function")]
    Function,
}

impl RealtimeSessionCreateResponseToolsItemType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Function => "function",
        }
    }
}

/// Configuration for input audio noise reduction. This can be set to `null` to turn off.
/// Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
/// Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeSessionInputAudioNoiseReduction {
    /// Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<InputAudioStreamFormatNoiseReductionType>,
}

/// Configuration for input audio transcription, defaults to off and can be  set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs  asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeSessionInputAudioTranscription {
    /// The language of the input audio. Supplying the input language in
    /// [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format
    /// will improve accuracy and latency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<TranscriptionModel>,
    /// An optional text to guide the model's style or continue a previous audio
    /// segment.
    /// For `whisper-1`, the [prompt is a list of keywords](/docs/guides/speech-to-text#prompting).
    /// For `gpt-4o-transcribe` models, the prompt is a free text string, for example "expect words related to technology".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}

/// Maximum number of output tokens for a single assistant response,
/// inclusive of tool calls. Provide an integer between 1 and 4096 to
/// limit output tokens, or `inf` for the maximum available tokens for a
/// given model. Defaults to `inf`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum RealtimeSessionMaxResponseOutputTokens {
    #[serde(rename = "inf")]
    Inf,
    #[serde(untagged)]
    Variant0(i64),
}

impl From<i64> for RealtimeSessionMaxResponseOutputTokens {
    fn from(value: i64) -> Self {
        RealtimeSessionMaxResponseOutputTokens::Variant0(value)
    }
}
/// Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.
/// Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.
/// Semantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with "uhhm", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeSessionTurnDetection {
    /// Whether or not to automatically generate a response when a VAD stop event occurs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_response: Option<bool>,
    /// Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eagerness: Option<RealtimeSessionTurnDetectionEagerness>,
    /// Whether or not to automatically interrupt any ongoing response with output to the default
    /// conversation (i.e. `conversation` of `auto`) when a VAD start event occurs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interrupt_response: Option<bool>,
    /// Used only for `server_vad` mode. Amount of audio to include before the VAD detected speech (in
    /// milliseconds). Defaults to 300ms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix_padding_ms: Option<i64>,
    /// Used only for `server_vad` mode. Duration of silence to detect speech stop (in milliseconds). Defaults
    /// to 500ms. With shorter values the model will respond more quickly,
    /// but may jump in on short pauses from the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub silence_duration_ms: Option<i64>,
    /// Used only for `server_vad` mode. Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A
    /// higher threshold will require louder audio to activate the model, and
    /// thus might perform better in noisy environments.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    /// Type of turn detection.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<RealtimeSessionTurnDetectionType>,
}

/// Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeSessionTurnDetectionEagerness {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[default]
    #[serde(rename = "auto")]
    Auto,
}

impl RealtimeSessionTurnDetectionEagerness {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Auto => "auto",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeSessionTurnDetectionType {
    #[default]
    #[serde(rename = "server_vad")]
    ServerVad,
    #[serde(rename = "semantic_vad")]
    SemanticVad,
}

impl RealtimeSessionTurnDetectionType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ServerVad => "server_vad",
            Self::SemanticVad => "semantic_vad",
        }
    }
}

/// Realtime transcription session object configuration.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeTranscriptionSessionCreateRequest {
    /// Configuration options for the generated client secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<RealtimeTranscriptionSessionCreateRequestClientSecret>,
    /// The set of items to include in the transcription. Current available items are:
    /// - `item.input_audio_transcription.logprobs`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    /// The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
    /// For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate,
    /// single channel (mono), and little-endian byte order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_format: Option<InputAudioStreamFormat>,
    /// Configuration for input audio noise reduction. This can be set to `null` to turn off.
    /// Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
    /// Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_noise_reduction:
        Option<RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReduction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_transcription: Option<RealtimeSessionInputAudioTranscription>,
    /// The set of modalities the model can respond with. To disable audio,
    /// set this to ["text"].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<Modalities>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<RealtimeSessionTurnDetection>,
}

/// Configuration options for the generated client secret.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeTranscriptionSessionCreateRequestClientSecret {
    /// Configuration for the ephemeral token expiration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<RealtimeTranscriptionSessionCreateRequestClientSecretExpiresAt>,
}

/// Configuration for the ephemeral token expiration.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeTranscriptionSessionCreateRequestClientSecretExpiresAt {
    /// The anchor point for the ephemeral token expiration. Only `created_at` is currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<RealtimeTranscriptionSessionCreateRequestClientSecretExpiresAtAnchor>,
    /// The number of seconds from the anchor point to the expiration. Select a value between `10` and `7200`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seconds: Option<i64>,
}

/// The anchor point for the ephemeral token expiration. Only `created_at` is currently supported.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RealtimeTranscriptionSessionCreateRequestClientSecretExpiresAtAnchor {
    #[default]
    #[serde(rename = "created_at")]
    CreatedAt,
}

impl RealtimeTranscriptionSessionCreateRequestClientSecretExpiresAtAnchor {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::CreatedAt => "created_at",
        }
    }
}

/// Configuration for input audio noise reduction. This can be set to `null` to turn off.
/// Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
/// Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReduction {
    /// Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<InputAudioStreamFormatNoiseReductionType>,
}

/// A new Realtime transcription session configuration.
///
/// When a session is created on the server via REST API, the session object
/// also contains an ephemeral key. Default TTL for keys is 10 minutes. This
/// property is not present when a session is updated via the WebSocket API.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeTranscriptionSessionCreateResponse {
    /// Ephemeral key returned by the API. Only present when the session is
    /// created on the server via REST API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<RealtimeTranscriptionSessionCreateResponseClientSecret>,
    pub id: String,
    /// The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
    /// For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate,
    /// single channel (mono), and little-endian byte order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_format: Option<InputAudioStreamFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_transcription: Option<RealtimeSessionInputAudioTranscription>,
    /// The set of modalities the model can respond with. To disable audio,
    /// set this to ["text"].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<Modalities>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<RealtimeSessionTurnDetection>,
}

impl crate::HasId for RealtimeTranscriptionSessionCreateResponse {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}

/// Ephemeral key returned by the API. Only present when the session is
/// created on the server via REST API.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RealtimeTranscriptionSessionCreateResponseClientSecret {
    /// Timestamp for when the token expires. Currently, all tokens expire
    /// after one minute.
    pub expires_at: i64,
    /// Ephemeral key usable in client environments to authenticate connections
    /// to the Realtime API. Use this in client-side environments rather than
    /// a standard API token, which should only be used server-side.
    pub value: String,
}

/// **o-series models only**
///
/// Configuration options for
/// [reasoning models](https://platform.openai.com/docs/guides/reasoning).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Reasoning {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effort: Option<ReasoningEffort>,
    /// **Deprecated:** use `summary` instead.
    ///
    /// A summary of the reasoning performed by the model. This can be
    /// useful for debugging and understanding the model's reasoning process.
    /// One of `auto`, `concise`, or `detailed`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generate_summary: Option<ReasoningGenerateSummary>,
    /// A summary of the reasoning performed by the model. This can be
    /// useful for debugging and understanding the model's reasoning process.
    /// One of `auto`, `concise`, or `detailed`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<ReasoningSummary>,
}

/// **o-series models only**
///
/// Constrains effort on reasoning for
/// [reasoning models](https://platform.openai.com/docs/guides/reasoning).
/// Currently supported values are `low`, `medium`, and `high`. Reducing
/// reasoning effort can result in faster responses and fewer tokens used
/// on reasoning in a response.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ReasoningEffort {
    #[serde(rename = "low")]
    Low,
    #[default]
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}

impl ReasoningEffort {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}

/// **Deprecated:** use `summary` instead.
///
/// A summary of the reasoning performed by the model. This can be
/// useful for debugging and understanding the model's reasoning process.
/// One of `auto`, `concise`, or `detailed`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum ReasoningGenerateSummary {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "concise")]
    Concise,
    #[serde(rename = "detailed")]
    Detailed,
}

impl ReasoningGenerateSummary {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Auto => "auto",
            Self::Concise => "concise",
            Self::Detailed => "detailed",
        }
    }
}

/// A description of the chain of thought used by a reasoning model while generating
/// a response. Be sure to include these items in your `input` to the Responses API
/// for subsequent turns of a conversation if you are manually
/// [managing context](/docs/guides/conversation-state).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ReasoningItem {
    /// The encrypted content of the reasoning item - populated when a response is
    /// generated with `reasoning.encrypted_content` in the `include` parameter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,
    /// The unique identifier of the reasoning content.
    pub id: String,
    /// The status of the item. One of `in_progress`, `completed`, or
    /// `incomplete`. Populated when items are returned via API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Reasoning text contents.
    pub summary: Vec<ReasoningItemSummaryItem>,
    /// The type of the object. Always `reasoning`.
    #[serde(rename = "type")]
    pub r#type: ReasoningItemType,
}

impl crate::HasId for ReasoningItem {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ReasoningItemSummaryItem {
    /// A short summary of the reasoning used by the model when generating
    /// the response.
    pub text: String,
    /// The type of the object. Always `summary_text`.
    #[serde(rename = "type")]
    pub r#type: ReasoningItemSummaryItemType,
}

/// The type of the object. Always `summary_text`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ReasoningItemSummaryItemType {
    #[default]
    #[serde(rename = "summary_text")]
    SummaryText,
}

impl ReasoningItemSummaryItemType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::SummaryText => "summary_text",
        }
    }
}

/// The type of the object. Always `reasoning`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ReasoningItemType {
    #[default]
    #[serde(rename = "reasoning")]
    Reasoning,
}

impl ReasoningItemType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Reasoning => "reasoning",
        }
    }
}

/// A summary of the reasoning performed by the model. This can be
/// useful for debugging and understanding the model's reasoning process.
/// One of `auto`, `concise`, or `detailed`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum ReasoningSummary {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "concise")]
    Concise,
    #[serde(rename = "detailed")]
    Detailed,
}

impl ReasoningSummary {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Auto => "auto",
            Self::Concise => "concise",
            Self::Detailed => "detailed",
        }
    }
}

/// A refusal from the model.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct RefusalContent {
    /// The refusal explanation from the model.
    pub refusal: String,
    /// The type of the refusal. Always `refusal`.
    #[serde(rename = "type")]
    pub r#type: RefusalContentType,
}

/// The type of the refusal. Always `refusal`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum RefusalContentType {
    #[default]
    #[serde(rename = "refusal")]
    Refusal,
}

impl RefusalContentType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Refusal => "refusal",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Response {
    #[serde(flatten)]
    pub model_response_properties: ModelResponseProperties,
    #[serde(flatten)]
    pub response_properties: ResponseProperties,
    /// Unix timestamp (in seconds) of when this Response was created.
    pub created_at: i64,
    pub error: Option<ResponseError>,
    /// Unique identifier for this Response.
    pub id: String,
    /// Details about why the response is incomplete.
    pub incomplete_details: Option<ResponseIncompleteDetails>,
    /// A system (or developer) message inserted into the model's context.
    ///
    /// When using along with `previous_response_id`, the instructions from a previous
    /// response will not be carried over to the next response. This makes it simple
    /// to swap out system (or developer) messages in new responses.
    pub instructions: Option<ResponseInstructions>,
    /// The object type of this resource - always set to `response`.
    pub object: ResponseObject,
    /// An array of content items generated by the model.
    ///
    /// - The length and order of items in the `output` array is dependent
    ///   on the model's response.
    /// - Rather than accessing the first item in the `output` array and
    ///   assuming it's an `assistant` message with the content generated by
    ///   the model, you might consider using the `output_text` property where
    ///   supported in SDKs.
    pub output: Vec<OutputItem>,
    /// Whether to allow the model to run tool calls in parallel.
    pub parallel_tool_calls: bool,
    /// The status of the response generation. One of `completed`, `failed`,
    /// `in_progress`, `cancelled`, `queued`, or `incomplete`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResponseStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ResponseUsage>,
}

impl AsRef<ModelResponseProperties> for Response {
    fn as_ref(&self) -> &ModelResponseProperties {
        &self.model_response_properties
    }
}
impl AsMut<ModelResponseProperties> for Response {
    fn as_mut(&mut self) -> &mut ModelResponseProperties {
        &mut self.model_response_properties
    }
}
impl AsRef<ResponseProperties> for Response {
    fn as_ref(&self) -> &ResponseProperties {
        &self.response_properties
    }
}
impl AsMut<ResponseProperties> for Response {
    fn as_mut(&mut self) -> &mut ResponseProperties {
        &mut self.response_properties
    }
}
/// Emitted when there is a partial audio response.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseAudioDeltaEvent {
    /// A chunk of Base64 encoded response audio bytes.
    pub delta: String,
    pub response_id: String,
    /// A sequence number for this chunk of the stream response.
    pub sequence_number: i64,
    /// The type of the event. Always `response.audio.delta`.
    #[serde(rename = "type")]
    pub r#type: ResponseAudioDeltaEventType,
}

/// The type of the event. Always `response.audio.delta`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseAudioDeltaEventType {
    #[default]
    #[serde(rename = "response.audio.delta")]
    ResponseAudioDelta,
}

impl ResponseAudioDeltaEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseAudioDelta => "response.audio.delta",
        }
    }
}

/// Emitted when the audio response is complete.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseAudioDoneEvent {
    pub response_id: String,
    /// The sequence number of the delta.
    pub sequence_number: i64,
    /// The type of the event. Always `response.audio.done`.
    #[serde(rename = "type")]
    pub r#type: ResponseAudioDoneEventType,
}

/// The type of the event. Always `response.audio.done`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseAudioDoneEventType {
    #[default]
    #[serde(rename = "response.audio.done")]
    ResponseAudioDone,
}

impl ResponseAudioDoneEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseAudioDone => "response.audio.done",
        }
    }
}

/// Emitted when there is a partial transcript of audio.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseAudioTranscriptDeltaEvent {
    /// The partial transcript of the audio response.
    pub delta: String,
    pub response_id: String,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always `response.audio.transcript.delta`.
    #[serde(rename = "type")]
    pub r#type: ResponseAudioTranscriptDeltaEventType,
}

/// The type of the event. Always `response.audio.transcript.delta`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseAudioTranscriptDeltaEventType {
    #[default]
    #[serde(rename = "response.audio.transcript.delta")]
    ResponseAudioTranscriptDelta,
}

impl ResponseAudioTranscriptDeltaEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseAudioTranscriptDelta => "response.audio.transcript.delta",
        }
    }
}

/// Emitted when the full audio transcript is completed.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseAudioTranscriptDoneEvent {
    pub response_id: String,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always `response.audio.transcript.done`.
    #[serde(rename = "type")]
    pub r#type: ResponseAudioTranscriptDoneEventType,
}

/// The type of the event. Always `response.audio.transcript.done`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseAudioTranscriptDoneEventType {
    #[default]
    #[serde(rename = "response.audio.transcript.done")]
    ResponseAudioTranscriptDone,
}

impl ResponseAudioTranscriptDoneEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseAudioTranscriptDone => "response.audio.transcript.done",
        }
    }
}

/// Emitted when a partial code snippet is streamed by the code interpreter.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseCodeInterpreterCallCodeDeltaEvent {
    /// The partial code snippet being streamed by the code interpreter.
    pub delta: String,
    /// The unique identifier of the code interpreter tool call item.
    pub item_id: String,
    /// The index of the output item in the response for which the code is being streamed.
    pub output_index: i64,
    /// The sequence number of this event, used to order streaming events.
    pub sequence_number: i64,
    /// The type of the event. Always `response.code_interpreter_call_code.delta`.
    #[serde(rename = "type")]
    pub r#type: ResponseCodeInterpreterCallCodeDeltaEventType,
}

/// The type of the event. Always `response.code_interpreter_call_code.delta`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseCodeInterpreterCallCodeDeltaEventType {
    #[default]
    #[serde(rename = "response.code_interpreter_call_code.delta")]
    ResponseCodeInterpreterCallCodeDelta,
}

impl ResponseCodeInterpreterCallCodeDeltaEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseCodeInterpreterCallCodeDelta => {
                "response.code_interpreter_call_code.delta"
            }
        }
    }
}

/// Emitted when the code snippet is finalized by the code interpreter.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseCodeInterpreterCallCodeDoneEvent {
    /// The final code snippet output by the code interpreter.
    pub code: String,
    /// The unique identifier of the code interpreter tool call item.
    pub item_id: String,
    /// The index of the output item in the response for which the code is finalized.
    pub output_index: i64,
    /// The sequence number of this event, used to order streaming events.
    pub sequence_number: i64,
    /// The type of the event. Always `response.code_interpreter_call_code.done`.
    #[serde(rename = "type")]
    pub r#type: ResponseCodeInterpreterCallCodeDoneEventType,
}

/// The type of the event. Always `response.code_interpreter_call_code.done`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseCodeInterpreterCallCodeDoneEventType {
    #[default]
    #[serde(rename = "response.code_interpreter_call_code.done")]
    ResponseCodeInterpreterCallCodeDone,
}

impl ResponseCodeInterpreterCallCodeDoneEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseCodeInterpreterCallCodeDone => "response.code_interpreter_call_code.done",
        }
    }
}

/// Emitted when the code interpreter call is completed.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseCodeInterpreterCallCompletedEvent {
    /// The unique identifier of the code interpreter tool call item.
    pub item_id: String,
    /// The index of the output item in the response for which the code interpreter call is completed.
    pub output_index: i64,
    /// The sequence number of this event, used to order streaming events.
    pub sequence_number: i64,
    /// The type of the event. Always `response.code_interpreter_call.completed`.
    #[serde(rename = "type")]
    pub r#type: ResponseCodeInterpreterCallCompletedEventType,
}

/// The type of the event. Always `response.code_interpreter_call.completed`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseCodeInterpreterCallCompletedEventType {
    #[default]
    #[serde(rename = "response.code_interpreter_call.completed")]
    ResponseCodeInterpreterCallCompleted,
}

impl ResponseCodeInterpreterCallCompletedEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseCodeInterpreterCallCompleted => {
                "response.code_interpreter_call.completed"
            }
        }
    }
}

/// Emitted when a code interpreter call is in progress.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseCodeInterpreterCallInProgressEvent {
    /// The unique identifier of the code interpreter tool call item.
    pub item_id: String,
    /// The index of the output item in the response for which the code interpreter call is in progress.
    pub output_index: i64,
    /// The sequence number of this event, used to order streaming events.
    pub sequence_number: i64,
    /// The type of the event. Always `response.code_interpreter_call.in_progress`.
    #[serde(rename = "type")]
    pub r#type: ResponseCodeInterpreterCallInProgressEventType,
}

/// The type of the event. Always `response.code_interpreter_call.in_progress`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseCodeInterpreterCallInProgressEventType {
    #[default]
    #[serde(rename = "response.code_interpreter_call.in_progress")]
    ResponseCodeInterpreterCallInProgress,
}

impl ResponseCodeInterpreterCallInProgressEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseCodeInterpreterCallInProgress => {
                "response.code_interpreter_call.in_progress"
            }
        }
    }
}

/// Emitted when the code interpreter is actively interpreting the code snippet.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseCodeInterpreterCallInterpretingEvent {
    /// The unique identifier of the code interpreter tool call item.
    pub item_id: String,
    /// The index of the output item in the response for which the code interpreter is interpreting code.
    pub output_index: i64,
    /// The sequence number of this event, used to order streaming events.
    pub sequence_number: i64,
    /// The type of the event. Always `response.code_interpreter_call.interpreting`.
    #[serde(rename = "type")]
    pub r#type: ResponseCodeInterpreterCallInterpretingEventType,
}

/// The type of the event. Always `response.code_interpreter_call.interpreting`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseCodeInterpreterCallInterpretingEventType {
    #[default]
    #[serde(rename = "response.code_interpreter_call.interpreting")]
    ResponseCodeInterpreterCallInterpreting,
}

impl ResponseCodeInterpreterCallInterpretingEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseCodeInterpreterCallInterpreting => {
                "response.code_interpreter_call.interpreting"
            }
        }
    }
}

/// Emitted when the model response is complete.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseCompletedEvent {
    /// Properties of the completed response.
    pub response: Response,
    /// The sequence number for this event.
    pub sequence_number: i64,
    /// The type of the event. Always `response.completed`.
    #[serde(rename = "type")]
    pub r#type: ResponseCompletedEventType,
}

/// The type of the event. Always `response.completed`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseCompletedEventType {
    #[default]
    #[serde(rename = "response.completed")]
    ResponseCompleted,
}

impl ResponseCompletedEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseCompleted => "response.completed",
        }
    }
}

/// Emitted when a new content part is added.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseContentPartAddedEvent {
    /// The index of the content part that was added.
    pub content_index: i64,
    /// The ID of the output item that the content part was added to.
    pub item_id: String,
    /// The index of the output item that the content part was added to.
    pub output_index: i64,
    /// The content part that was added.
    pub part: OutputContent,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always `response.content_part.added`.
    #[serde(rename = "type")]
    pub r#type: ResponseContentPartAddedEventType,
}

/// The type of the event. Always `response.content_part.added`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseContentPartAddedEventType {
    #[default]
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAdded,
}

impl ResponseContentPartAddedEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseContentPartAdded => "response.content_part.added",
        }
    }
}

/// Emitted when a content part is done.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseContentPartDoneEvent {
    /// The index of the content part that is done.
    pub content_index: i64,
    /// The ID of the output item that the content part was added to.
    pub item_id: String,
    /// The index of the output item that the content part was added to.
    pub output_index: i64,
    /// The content part that is done.
    pub part: OutputContent,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always `response.content_part.done`.
    #[serde(rename = "type")]
    pub r#type: ResponseContentPartDoneEventType,
}

/// The type of the event. Always `response.content_part.done`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseContentPartDoneEventType {
    #[default]
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDone,
}

impl ResponseContentPartDoneEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseContentPartDone => "response.content_part.done",
        }
    }
}

/// An event that is emitted when a response is created.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseCreatedEvent {
    /// The response that was created.
    pub response: Response,
    /// The sequence number for this event.
    pub sequence_number: i64,
    /// The type of the event. Always `response.created`.
    #[serde(rename = "type")]
    pub r#type: ResponseCreatedEventType,
}

/// The type of the event. Always `response.created`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseCreatedEventType {
    #[default]
    #[serde(rename = "response.created")]
    ResponseCreated,
}

impl ResponseCreatedEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseCreated => "response.created",
        }
    }
}

/// An error object returned when the model fails to generate a Response.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseError {
    pub code: ResponseErrorCode,
    /// A human-readable description of the error.
    pub message: String,
}

/// The error code for the response.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum ResponseErrorCode {
    #[serde(rename = "server_error")]
    ServerError,
    #[serde(rename = "rate_limit_exceeded")]
    RateLimitExceeded,
    #[serde(rename = "invalid_prompt")]
    InvalidPrompt,
    #[serde(rename = "vector_store_timeout")]
    VectorStoreTimeout,
    #[serde(rename = "invalid_image")]
    InvalidImage,
    #[serde(rename = "invalid_image_format")]
    InvalidImageFormat,
    #[serde(rename = "invalid_base64_image")]
    InvalidBase64Image,
    #[serde(rename = "invalid_image_url")]
    InvalidImageUrl,
    #[serde(rename = "image_too_large")]
    ImageTooLarge,
    #[serde(rename = "image_too_small")]
    ImageTooSmall,
    #[serde(rename = "image_parse_error")]
    ImageParseError,
    #[serde(rename = "image_content_policy_violation")]
    ImageContentPolicyViolation,
    #[serde(rename = "invalid_image_mode")]
    InvalidImageMode,
    #[serde(rename = "image_file_too_large")]
    ImageFileTooLarge,
    #[serde(rename = "unsupported_image_media_type")]
    UnsupportedImageMediaType,
    #[serde(rename = "empty_image_file")]
    EmptyImageFile,
    #[serde(rename = "failed_to_download_image")]
    FailedToDownloadImage,
    #[serde(rename = "image_file_not_found")]
    ImageFileNotFound,
}

impl ResponseErrorCode {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ServerError => "server_error",
            Self::RateLimitExceeded => "rate_limit_exceeded",
            Self::InvalidPrompt => "invalid_prompt",
            Self::VectorStoreTimeout => "vector_store_timeout",
            Self::InvalidImage => "invalid_image",
            Self::InvalidImageFormat => "invalid_image_format",
            Self::InvalidBase64Image => "invalid_base64_image",
            Self::InvalidImageUrl => "invalid_image_url",
            Self::ImageTooLarge => "image_too_large",
            Self::ImageTooSmall => "image_too_small",
            Self::ImageParseError => "image_parse_error",
            Self::ImageContentPolicyViolation => "image_content_policy_violation",
            Self::InvalidImageMode => "invalid_image_mode",
            Self::ImageFileTooLarge => "image_file_too_large",
            Self::UnsupportedImageMediaType => "unsupported_image_media_type",
            Self::EmptyImageFile => "empty_image_file",
            Self::FailedToDownloadImage => "failed_to_download_image",
            Self::ImageFileNotFound => "image_file_not_found",
        }
    }
}

/// Emitted when an error occurs.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseErrorEvent {
    /// The error code.
    pub code: Option<String>,
    /// The error message.
    pub message: String,
    /// The error parameter.
    pub param: Option<String>,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always `error`.
    #[serde(rename = "type")]
    pub r#type: ResponseErrorEventType,
}

/// The type of the event. Always `error`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseErrorEventType {
    #[default]
    #[serde(rename = "error")]
    Error,
}

impl ResponseErrorEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Error => "error",
        }
    }
}

/// An event that is emitted when a response fails.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseFailedEvent {
    /// The response that failed.
    pub response: Response,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always `response.failed`.
    #[serde(rename = "type")]
    pub r#type: ResponseFailedEventType,
}

/// The type of the event. Always `response.failed`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseFailedEventType {
    #[default]
    #[serde(rename = "response.failed")]
    ResponseFailed,
}

impl ResponseFailedEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseFailed => "response.failed",
        }
    }
}

/// Emitted when a file search call is completed (results found).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseFileSearchCallCompletedEvent {
    /// The ID of the output item that the file search call is initiated.
    pub item_id: String,
    /// The index of the output item that the file search call is initiated.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always `response.file_search_call.completed`.
    #[serde(rename = "type")]
    pub r#type: ResponseFileSearchCallCompletedEventType,
}

/// The type of the event. Always `response.file_search_call.completed`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseFileSearchCallCompletedEventType {
    #[default]
    #[serde(rename = "response.file_search_call.completed")]
    ResponseFileSearchCallCompleted,
}

impl ResponseFileSearchCallCompletedEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseFileSearchCallCompleted => "response.file_search_call.completed",
        }
    }
}

/// Emitted when a file search call is initiated.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseFileSearchCallInProgressEvent {
    /// The ID of the output item that the file search call is initiated.
    pub item_id: String,
    /// The index of the output item that the file search call is initiated.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always `response.file_search_call.in_progress`.
    #[serde(rename = "type")]
    pub r#type: ResponseFileSearchCallInProgressEventType,
}

/// The type of the event. Always `response.file_search_call.in_progress`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseFileSearchCallInProgressEventType {
    #[default]
    #[serde(rename = "response.file_search_call.in_progress")]
    ResponseFileSearchCallInProgress,
}

impl ResponseFileSearchCallInProgressEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseFileSearchCallInProgress => "response.file_search_call.in_progress",
        }
    }
}

/// Emitted when a file search is currently searching.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseFileSearchCallSearchingEvent {
    /// The ID of the output item that the file search call is initiated.
    pub item_id: String,
    /// The index of the output item that the file search call is searching.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always `response.file_search_call.searching`.
    #[serde(rename = "type")]
    pub r#type: ResponseFileSearchCallSearchingEventType,
}

/// The type of the event. Always `response.file_search_call.searching`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseFileSearchCallSearchingEventType {
    #[default]
    #[serde(rename = "response.file_search_call.searching")]
    ResponseFileSearchCallSearching,
}

impl ResponseFileSearchCallSearchingEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseFileSearchCallSearching => "response.file_search_call.searching",
        }
    }
}

/// JSON object response format. An older method of generating JSON responses.
/// Using `json_schema` is recommended for models that support it. Note that the
/// model will not generate JSON without a system or user message instructing it
/// to do so.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseFormatJsonObject {
    /// The type of response format being defined. Always `json_object`.
    #[serde(rename = "type")]
    pub r#type: ResponseFormatJsonObjectType,
}

/// The type of response format being defined. Always `json_object`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseFormatJsonObjectType {
    #[default]
    #[serde(rename = "json_object")]
    JsonObject,
}

impl ResponseFormatJsonObjectType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::JsonObject => "json_object",
        }
    }
}

/// The schema for the response format, described as a JSON Schema object.
/// Learn how to build JSON schemas [here](https://json-schema.org/).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseFormatJsonSchemaSchema {
    #[serde(flatten)]
    pub extra_fields: serde_json::Map<String, serde_json::Value>,
}

/// Default response format. Used to generate text responses.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseFormatText {
    /// The type of response format being defined. Always `text`.
    #[serde(rename = "type")]
    pub r#type: ResponseFormatTextType,
}

/// The type of response format being defined. Always `text`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseFormatTextType {
    #[default]
    #[serde(rename = "text")]
    Text,
}

impl ResponseFormatTextType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Text => "text",
        }
    }
}

/// Emitted when there is a partial function-call arguments delta.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseFunctionCallArgumentsDeltaEvent {
    /// The function-call arguments delta that is added.
    pub delta: String,
    /// The ID of the output item that the function-call arguments delta is added to.
    pub item_id: String,
    /// The index of the output item that the function-call arguments delta is added to.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always `response.function_call_arguments.delta`.
    #[serde(rename = "type")]
    pub r#type: ResponseFunctionCallArgumentsDeltaEventType,
}

/// The type of the event. Always `response.function_call_arguments.delta`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseFunctionCallArgumentsDeltaEventType {
    #[default]
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDelta,
}

impl ResponseFunctionCallArgumentsDeltaEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseFunctionCallArgumentsDelta => "response.function_call_arguments.delta",
        }
    }
}

/// Emitted when function-call arguments are finalized.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseFunctionCallArgumentsDoneEvent {
    /// The function-call arguments.
    pub arguments: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    #[serde(rename = "type")]
    pub r#type: ResponseFunctionCallArgumentsDoneEventType,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseFunctionCallArgumentsDoneEventType {
    #[default]
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDone,
}

impl ResponseFunctionCallArgumentsDoneEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseFunctionCallArgumentsDone => "response.function_call_arguments.done",
        }
    }
}

/// Emitted when an image generation tool call has completed and the final image is available.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseImageGenCallCompletedEvent {
    /// The unique identifier of the image generation item being processed.
    pub item_id: String,
    /// The index of the output item in the response's output array.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always 'response.image_generation_call.completed'.
    #[serde(rename = "type")]
    pub r#type: ResponseImageGenCallCompletedEventType,
}

/// The type of the event. Always 'response.image_generation_call.completed'.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseImageGenCallCompletedEventType {
    #[default]
    #[serde(rename = "response.image_generation_call.completed")]
    ResponseImageGenerationCallCompleted,
}

impl ResponseImageGenCallCompletedEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseImageGenerationCallCompleted => {
                "response.image_generation_call.completed"
            }
        }
    }
}

/// Emitted when an image generation tool call is actively generating an image (intermediate state).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseImageGenCallGeneratingEvent {
    /// The unique identifier of the image generation item being processed.
    pub item_id: String,
    /// The index of the output item in the response's output array.
    pub output_index: i64,
    /// The sequence number of the image generation item being processed.
    pub sequence_number: i64,
    /// The type of the event. Always 'response.image_generation_call.generating'.
    #[serde(rename = "type")]
    pub r#type: ResponseImageGenCallGeneratingEventType,
}

/// The type of the event. Always 'response.image_generation_call.generating'.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseImageGenCallGeneratingEventType {
    #[default]
    #[serde(rename = "response.image_generation_call.generating")]
    ResponseImageGenerationCallGenerating,
}

impl ResponseImageGenCallGeneratingEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseImageGenerationCallGenerating => {
                "response.image_generation_call.generating"
            }
        }
    }
}

/// Emitted when an image generation tool call is in progress.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseImageGenCallInProgressEvent {
    /// The unique identifier of the image generation item being processed.
    pub item_id: String,
    /// The index of the output item in the response's output array.
    pub output_index: i64,
    /// The sequence number of the image generation item being processed.
    pub sequence_number: i64,
    /// The type of the event. Always 'response.image_generation_call.in_progress'.
    #[serde(rename = "type")]
    pub r#type: ResponseImageGenCallInProgressEventType,
}

/// The type of the event. Always 'response.image_generation_call.in_progress'.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseImageGenCallInProgressEventType {
    #[default]
    #[serde(rename = "response.image_generation_call.in_progress")]
    ResponseImageGenerationCallInProgress,
}

impl ResponseImageGenCallInProgressEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseImageGenerationCallInProgress => {
                "response.image_generation_call.in_progress"
            }
        }
    }
}

/// Emitted when a partial image is available during image generation streaming.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseImageGenCallPartialImageEvent {
    /// The unique identifier of the image generation item being processed.
    pub item_id: String,
    /// The index of the output item in the response's output array.
    pub output_index: i64,
    /// Base64-encoded partial image data, suitable for rendering as an image.
    pub partial_image_b64: String,
    /// 0-based index for the partial image (backend is 1-based, but this is 0-based for the user).
    pub partial_image_index: i64,
    /// The sequence number of the image generation item being processed.
    pub sequence_number: i64,
    /// The type of the event. Always 'response.image_generation_call.partial_image'.
    #[serde(rename = "type")]
    pub r#type: ResponseImageGenCallPartialImageEventType,
}

/// The type of the event. Always 'response.image_generation_call.partial_image'.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseImageGenCallPartialImageEventType {
    #[default]
    #[serde(rename = "response.image_generation_call.partial_image")]
    ResponseImageGenerationCallPartialImage,
}

impl ResponseImageGenCallPartialImageEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseImageGenerationCallPartialImage => {
                "response.image_generation_call.partial_image"
            }
        }
    }
}

/// Emitted when the response is in progress.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseInProgressEvent {
    /// The response that is in progress.
    pub response: Response,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always `response.in_progress`.
    #[serde(rename = "type")]
    pub r#type: ResponseInProgressEventType,
}

/// The type of the event. Always `response.in_progress`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseInProgressEventType {
    #[default]
    #[serde(rename = "response.in_progress")]
    ResponseInProgress,
}

impl ResponseInProgressEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseInProgress => "response.in_progress",
        }
    }
}

/// Details about why the response is incomplete.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseIncompleteDetails {
    /// The reason why the response is incomplete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<ResponseIncompleteDetailsReason>,
}

/// The reason why the response is incomplete.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum ResponseIncompleteDetailsReason {
    #[serde(rename = "max_output_tokens")]
    MaxOutputTokens,
    #[serde(rename = "content_filter")]
    ContentFilter,
}

impl ResponseIncompleteDetailsReason {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::MaxOutputTokens => "max_output_tokens",
            Self::ContentFilter => "content_filter",
        }
    }
}

/// An event that is emitted when a response finishes as incomplete.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseIncompleteEvent {
    /// The response that was incomplete.
    pub response: Response,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always `response.incomplete`.
    #[serde(rename = "type")]
    pub r#type: ResponseIncompleteEventType,
}

/// The type of the event. Always `response.incomplete`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseIncompleteEventType {
    #[default]
    #[serde(rename = "response.incomplete")]
    ResponseIncomplete,
}

impl ResponseIncompleteEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseIncomplete => "response.incomplete",
        }
    }
}

/// A system (or developer) message inserted into the model's context.
///
/// When using along with `previous_response_id`, the instructions from a previous
/// response will not be carried over to the next response. This makes it simple
/// to swap out system (or developer) messages in new responses.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum ResponseInstructions {
    #[serde(untagged)]
    Variant0(String),
    #[serde(untagged)]
    Variant1(Vec<InputItem>),
}

impl From<String> for ResponseInstructions {
    fn from(value: String) -> Self {
        ResponseInstructions::Variant0(value)
    }
}
impl From<Vec<InputItem>> for ResponseInstructions {
    fn from(value: Vec<InputItem>) -> Self {
        ResponseInstructions::Variant1(value)
    }
}
/// A logprob is the logarithmic probability that the model assigns to producing
/// a particular token at a given position in the sequence. Less-negative (higher)
/// logprob values indicate greater model confidence in that token choice.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseLogProb {
    /// The log probability of this token.
    pub logprob: f64,
    /// A possible text token.
    pub token: String,
    /// The log probability of the top 20 most likely tokens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<Vec<ResponseLogProbTopLogprobsItem>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseLogProbTopLogprobsItem {
    /// The log probability of this token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprob: Option<f64>,
    /// A possible text token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// Emitted when there is a delta (partial update) to the arguments of an MCP tool call.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseMCPCallArgumentsDeltaEvent {
    /// A JSON string containing the partial update to the arguments for the MCP tool call.
    pub delta: String,
    /// The unique identifier of the MCP tool call item being processed.
    pub item_id: String,
    /// The index of the output item in the response's output array.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always 'response.mcp_call_arguments.delta'.
    #[serde(rename = "type")]
    pub r#type: ResponseMCPCallArgumentsDeltaEventType,
}

/// The type of the event. Always 'response.mcp_call_arguments.delta'.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseMCPCallArgumentsDeltaEventType {
    #[default]
    #[serde(rename = "response.mcp_call_arguments.delta")]
    ResponseMcpCallArgumentsDelta,
}

impl ResponseMCPCallArgumentsDeltaEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseMcpCallArgumentsDelta => "response.mcp_call_arguments.delta",
        }
    }
}

/// Emitted when the arguments for an MCP tool call are finalized.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseMCPCallArgumentsDoneEvent {
    /// A JSON string containing the finalized arguments for the MCP tool call.
    pub arguments: String,
    /// The unique identifier of the MCP tool call item being processed.
    pub item_id: String,
    /// The index of the output item in the response's output array.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always 'response.mcp_call_arguments.done'.
    #[serde(rename = "type")]
    pub r#type: ResponseMCPCallArgumentsDoneEventType,
}

/// The type of the event. Always 'response.mcp_call_arguments.done'.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseMCPCallArgumentsDoneEventType {
    #[default]
    #[serde(rename = "response.mcp_call_arguments.done")]
    ResponseMcpCallArgumentsDone,
}

impl ResponseMCPCallArgumentsDoneEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseMcpCallArgumentsDone => "response.mcp_call_arguments.done",
        }
    }
}

/// Emitted when an MCP  tool call has completed successfully.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseMCPCallCompletedEvent {
    /// The ID of the MCP tool call item that completed.
    pub item_id: String,
    /// The index of the output item that completed.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always 'response.mcp_call.completed'.
    #[serde(rename = "type")]
    pub r#type: ResponseMCPCallCompletedEventType,
}

/// The type of the event. Always 'response.mcp_call.completed'.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseMCPCallCompletedEventType {
    #[default]
    #[serde(rename = "response.mcp_call.completed")]
    ResponseMcpCallCompleted,
}

impl ResponseMCPCallCompletedEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseMcpCallCompleted => "response.mcp_call.completed",
        }
    }
}

/// Emitted when an MCP  tool call has failed.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseMCPCallFailedEvent {
    /// The ID of the MCP tool call item that failed.
    pub item_id: String,
    /// The index of the output item that failed.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always 'response.mcp_call.failed'.
    #[serde(rename = "type")]
    pub r#type: ResponseMCPCallFailedEventType,
}

/// The type of the event. Always 'response.mcp_call.failed'.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseMCPCallFailedEventType {
    #[default]
    #[serde(rename = "response.mcp_call.failed")]
    ResponseMcpCallFailed,
}

impl ResponseMCPCallFailedEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseMcpCallFailed => "response.mcp_call.failed",
        }
    }
}

/// Emitted when an MCP  tool call is in progress.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseMCPCallInProgressEvent {
    /// The unique identifier of the MCP tool call item being processed.
    pub item_id: String,
    /// The index of the output item in the response's output array.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always 'response.mcp_call.in_progress'.
    #[serde(rename = "type")]
    pub r#type: ResponseMCPCallInProgressEventType,
}

/// The type of the event. Always 'response.mcp_call.in_progress'.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseMCPCallInProgressEventType {
    #[default]
    #[serde(rename = "response.mcp_call.in_progress")]
    ResponseMcpCallInProgress,
}

impl ResponseMCPCallInProgressEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseMcpCallInProgress => "response.mcp_call.in_progress",
        }
    }
}

/// Emitted when the list of available MCP tools has been successfully retrieved.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseMCPListToolsCompletedEvent {
    /// The ID of the MCP tool call item that produced this output.
    pub item_id: String,
    /// The index of the output item that was processed.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always 'response.mcp_list_tools.completed'.
    #[serde(rename = "type")]
    pub r#type: ResponseMCPListToolsCompletedEventType,
}

/// The type of the event. Always 'response.mcp_list_tools.completed'.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseMCPListToolsCompletedEventType {
    #[default]
    #[serde(rename = "response.mcp_list_tools.completed")]
    ResponseMcpListToolsCompleted,
}

impl ResponseMCPListToolsCompletedEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseMcpListToolsCompleted => "response.mcp_list_tools.completed",
        }
    }
}

/// Emitted when the attempt to list available MCP tools has failed.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseMCPListToolsFailedEvent {
    /// The ID of the MCP tool call item that failed.
    pub item_id: String,
    /// The index of the output item that failed.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always 'response.mcp_list_tools.failed'.
    #[serde(rename = "type")]
    pub r#type: ResponseMCPListToolsFailedEventType,
}

/// The type of the event. Always 'response.mcp_list_tools.failed'.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseMCPListToolsFailedEventType {
    #[default]
    #[serde(rename = "response.mcp_list_tools.failed")]
    ResponseMcpListToolsFailed,
}

impl ResponseMCPListToolsFailedEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseMcpListToolsFailed => "response.mcp_list_tools.failed",
        }
    }
}

/// Emitted when the system is in the process of retrieving the list of available MCP tools.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseMCPListToolsInProgressEvent {
    /// The ID of the MCP tool call item that is being processed.
    pub item_id: String,
    /// The index of the output item that is being processed.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always 'response.mcp_list_tools.in_progress'.
    #[serde(rename = "type")]
    pub r#type: ResponseMCPListToolsInProgressEventType,
}

/// The type of the event. Always 'response.mcp_list_tools.in_progress'.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseMCPListToolsInProgressEventType {
    #[default]
    #[serde(rename = "response.mcp_list_tools.in_progress")]
    ResponseMcpListToolsInProgress,
}

impl ResponseMCPListToolsInProgressEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseMcpListToolsInProgress => "response.mcp_list_tools.in_progress",
        }
    }
}

/// The object type of this resource - always set to `response`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseObject {
    #[default]
    #[serde(rename = "response")]
    Response,
}

impl ResponseObject {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Response => "response",
        }
    }
}

/// Emitted when a new output item is added.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseOutputItemAddedEvent {
    /// The output item that was added.
    pub item: OutputItem,
    /// The index of the output item that was added.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always `response.output_item.added`.
    #[serde(rename = "type")]
    pub r#type: ResponseOutputItemAddedEventType,
}

/// The type of the event. Always `response.output_item.added`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseOutputItemAddedEventType {
    #[default]
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAdded,
}

impl ResponseOutputItemAddedEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseOutputItemAdded => "response.output_item.added",
        }
    }
}

/// Emitted when an output item is marked done.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseOutputItemDoneEvent {
    /// The output item that was marked done.
    pub item: OutputItem,
    /// The index of the output item that was marked done.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always `response.output_item.done`.
    #[serde(rename = "type")]
    pub r#type: ResponseOutputItemDoneEventType,
}

/// The type of the event. Always `response.output_item.done`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseOutputItemDoneEventType {
    #[default]
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDone,
}

impl ResponseOutputItemDoneEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseOutputItemDone => "response.output_item.done",
        }
    }
}

/// Emitted when an annotation is added to output text content.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseOutputTextAnnotationAddedEvent {
    /// The annotation object being added. (See annotation schema for details.)
    pub annotation: serde_json::Map<String, serde_json::Value>,
    /// The index of the annotation within the content part.
    pub annotation_index: i64,
    /// The index of the content part within the output item.
    pub content_index: i64,
    /// The unique identifier of the item to which the annotation is being added.
    pub item_id: String,
    /// The index of the output item in the response's output array.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always 'response.output_text.annotation.added'.
    #[serde(rename = "type")]
    pub r#type: ResponseOutputTextAnnotationAddedEventType,
}

/// The type of the event. Always 'response.output_text.annotation.added'.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseOutputTextAnnotationAddedEventType {
    #[default]
    #[serde(rename = "response.output_text.annotation.added")]
    ResponseOutputTextAnnotationAdded,
}

impl ResponseOutputTextAnnotationAddedEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseOutputTextAnnotationAdded => "response.output_text.annotation.added",
        }
    }
}

/// Optional map of values to substitute in for variables in your
/// prompt. The substitution values can either be strings, or other
/// Response input types like images or files.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponsePromptVariables {
    #[serde(flatten)]
    pub extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseProperties {
    /// Whether to run the model response in the background.
    /// [Learn more](/docs/guides/background).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    /// An upper bound for the number of tokens that can be generated for a response, including visible output tokens and [reasoning tokens](/docs/guides/reasoning).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i64>,
    /// The maximum number of total calls to built-in tools that can be processed in a response. This maximum number applies across all built-in tool calls, not per individual tool. Any further attempts to call a tool by the model will be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tool_calls: Option<i64>,
    /// Model ID used to generate the response, like `gpt-4o` or `o3`. OpenAI
    /// offers a wide range of models with different capabilities, performance
    /// characteristics, and price points. Refer to the [model guide](/docs/models)
    /// to browse and compare available models.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<ModelId>,
    /// The unique ID of the previous response to the model. Use this to
    /// create multi-turn conversations. Learn more about
    /// [conversation state](/docs/guides/conversation-state).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Prompt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Reasoning>,
    /// Configuration options for a text response from the model. Can be plain
    /// text or structured JSON data. Learn more:
    /// - [Text inputs and outputs](/docs/guides/text)
    /// - [Structured Outputs](/docs/guides/structured-outputs)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponsePropertiesText>,
    /// How the model should select which tool (or tools) to use when generating
    /// a response. See the `tools` parameter to see how to specify which tools
    /// the model can call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ResponsePropertiesToolChoice>,
    /// An array of tools the model may call while generating a response. You
    /// can specify which tool to use by setting the `tool_choice` parameter.
    ///
    /// The two categories of tools you can provide the model are:
    ///
    /// - **Built-in tools**: Tools that are provided by OpenAI that extend the
    ///   model's capabilities, like [web search](/docs/guides/tools-web-search)
    ///   or [file search](/docs/guides/tools-file-search). Learn more about
    ///   [built-in tools](/docs/guides/tools).
    /// - **Function calls (custom tools)**: Functions that are defined by you,
    ///   enabling the model to call your own code. Learn more about
    ///   [function calling](/docs/guides/function-calling).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// The truncation strategy to use for the model response.
    /// - `auto`: If the context of this response and previous ones exceeds
    ///   the model's context window size, the model will truncate the
    ///   response to fit the context window by dropping input items in the
    ///   middle of the conversation.
    /// - `disabled` (default): If a model response will exceed the context window
    ///   size for a model, the request will fail with a 400 error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub truncation: Option<ResponsePropertiesTruncation>,
}

/// Configuration options for a text response from the model. Can be plain
/// text or structured JSON data. Learn more:
/// - [Text inputs and outputs](/docs/guides/text)
/// - [Structured Outputs](/docs/guides/structured-outputs)
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponsePropertiesText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<TextResponseFormatConfiguration>,
}

/// How the model should select which tool (or tools) to use when generating
/// a response. See the `tools` parameter to see how to specify which tools
/// the model can call.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum ResponsePropertiesToolChoice {
    #[serde(untagged)]
    ToolChoiceOptions(ToolChoiceOptions),
    #[serde(untagged)]
    ToolChoiceTypes(ToolChoiceTypes),
    #[serde(untagged)]
    ToolChoiceFunction(ToolChoiceFunction),
    #[serde(untagged)]
    ToolChoiceMCP(ToolChoiceMCP),
}

impl From<ToolChoiceFunction> for ResponsePropertiesToolChoice {
    fn from(value: ToolChoiceFunction) -> Self {
        ResponsePropertiesToolChoice::ToolChoiceFunction(value)
    }
}
impl From<ToolChoiceMCP> for ResponsePropertiesToolChoice {
    fn from(value: ToolChoiceMCP) -> Self {
        ResponsePropertiesToolChoice::ToolChoiceMCP(value)
    }
}
impl From<ToolChoiceOptions> for ResponsePropertiesToolChoice {
    fn from(value: ToolChoiceOptions) -> Self {
        ResponsePropertiesToolChoice::ToolChoiceOptions(value)
    }
}
impl From<ToolChoiceTypes> for ResponsePropertiesToolChoice {
    fn from(value: ToolChoiceTypes) -> Self {
        ResponsePropertiesToolChoice::ToolChoiceTypes(value)
    }
}
/// The truncation strategy to use for the model response.
/// - `auto`: If the context of this response and previous ones exceeds
///   the model's context window size, the model will truncate the
///   response to fit the context window by dropping input items in the
///   middle of the conversation.
/// - `disabled` (default): If a model response will exceed the context window
///   size for a model, the request will fail with a 400 error.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponsePropertiesTruncation {
    #[serde(rename = "auto")]
    Auto,
    #[default]
    #[serde(rename = "disabled")]
    Disabled,
}

impl ResponsePropertiesTruncation {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Auto => "auto",
            Self::Disabled => "disabled",
        }
    }
}

/// Emitted when a response is queued and waiting to be processed.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseQueuedEvent {
    /// The full response object that is queued.
    pub response: Response,
    /// The sequence number for this event.
    pub sequence_number: i64,
    /// The type of the event. Always 'response.queued'.
    #[serde(rename = "type")]
    pub r#type: ResponseQueuedEventType,
}

/// The type of the event. Always 'response.queued'.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseQueuedEventType {
    #[default]
    #[serde(rename = "response.queued")]
    ResponseQueued,
}

impl ResponseQueuedEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseQueued => "response.queued",
        }
    }
}

/// Emitted when there is a delta (partial update) to the reasoning summary content.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseReasoningSummaryDeltaEvent {
    /// The partial update to the reasoning summary content.
    pub delta: serde_json::Map<String, serde_json::Value>,
    /// The unique identifier of the item for which the reasoning summary is being updated.
    pub item_id: String,
    /// The index of the output item in the response's output array.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The index of the summary part within the output item.
    pub summary_index: i64,
    /// The type of the event. Always 'response.reasoning_summary.delta'.
    #[serde(rename = "type")]
    pub r#type: ResponseReasoningSummaryDeltaEventType,
}

/// The type of the event. Always 'response.reasoning_summary.delta'.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseReasoningSummaryDeltaEventType {
    #[default]
    #[serde(rename = "response.reasoning_summary.delta")]
    ResponseReasoningSummaryDelta,
}

impl ResponseReasoningSummaryDeltaEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseReasoningSummaryDelta => "response.reasoning_summary.delta",
        }
    }
}

/// Emitted when the reasoning summary content is finalized for an item.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseReasoningSummaryDoneEvent {
    /// The unique identifier of the item for which the reasoning summary is finalized.
    pub item_id: String,
    /// The index of the output item in the response's output array.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The index of the summary part within the output item.
    pub summary_index: i64,
    /// The finalized reasoning summary text.
    pub text: String,
    /// The type of the event. Always 'response.reasoning_summary.done'.
    #[serde(rename = "type")]
    pub r#type: ResponseReasoningSummaryDoneEventType,
}

/// The type of the event. Always 'response.reasoning_summary.done'.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseReasoningSummaryDoneEventType {
    #[default]
    #[serde(rename = "response.reasoning_summary.done")]
    ResponseReasoningSummaryDone,
}

impl ResponseReasoningSummaryDoneEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseReasoningSummaryDone => "response.reasoning_summary.done",
        }
    }
}

/// Emitted when a new reasoning summary part is added.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseReasoningSummaryPartAddedEvent {
    /// The ID of the item this summary part is associated with.
    pub item_id: String,
    /// The index of the output item this summary part is associated with.
    pub output_index: i64,
    /// The summary part that was added.
    pub part: ResponseReasoningSummaryPartAddedEventPart,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The index of the summary part within the reasoning summary.
    pub summary_index: i64,
    /// The type of the event. Always `response.reasoning_summary_part.added`.
    #[serde(rename = "type")]
    pub r#type: ResponseReasoningSummaryPartAddedEventType,
}

/// The summary part that was added.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseReasoningSummaryPartAddedEventPart {
    /// The text of the summary part.
    pub text: String,
    /// The type of the summary part. Always `summary_text`.
    #[serde(rename = "type")]
    pub r#type: ResponseReasoningSummaryPartAddedEventPartType,
}

/// The type of the summary part. Always `summary_text`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseReasoningSummaryPartAddedEventPartType {
    #[default]
    #[serde(rename = "summary_text")]
    SummaryText,
}

impl ResponseReasoningSummaryPartAddedEventPartType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::SummaryText => "summary_text",
        }
    }
}

/// The type of the event. Always `response.reasoning_summary_part.added`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseReasoningSummaryPartAddedEventType {
    #[default]
    #[serde(rename = "response.reasoning_summary_part.added")]
    ResponseReasoningSummaryPartAdded,
}

impl ResponseReasoningSummaryPartAddedEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseReasoningSummaryPartAdded => "response.reasoning_summary_part.added",
        }
    }
}

/// Emitted when a reasoning summary part is completed.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseReasoningSummaryPartDoneEvent {
    /// The ID of the item this summary part is associated with.
    pub item_id: String,
    /// The index of the output item this summary part is associated with.
    pub output_index: i64,
    /// The completed summary part.
    pub part: ResponseReasoningSummaryPartDoneEventPart,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The index of the summary part within the reasoning summary.
    pub summary_index: i64,
    /// The type of the event. Always `response.reasoning_summary_part.done`.
    #[serde(rename = "type")]
    pub r#type: ResponseReasoningSummaryPartDoneEventType,
}

/// The completed summary part.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseReasoningSummaryPartDoneEventPart {
    /// The text of the summary part.
    pub text: String,
    /// The type of the summary part. Always `summary_text`.
    #[serde(rename = "type")]
    pub r#type: ResponseReasoningSummaryPartDoneEventPartType,
}

/// The type of the summary part. Always `summary_text`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseReasoningSummaryPartDoneEventPartType {
    #[default]
    #[serde(rename = "summary_text")]
    SummaryText,
}

impl ResponseReasoningSummaryPartDoneEventPartType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::SummaryText => "summary_text",
        }
    }
}

/// The type of the event. Always `response.reasoning_summary_part.done`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseReasoningSummaryPartDoneEventType {
    #[default]
    #[serde(rename = "response.reasoning_summary_part.done")]
    ResponseReasoningSummaryPartDone,
}

impl ResponseReasoningSummaryPartDoneEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseReasoningSummaryPartDone => "response.reasoning_summary_part.done",
        }
    }
}

/// Emitted when a delta is added to a reasoning summary text.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseReasoningSummaryTextDeltaEvent {
    /// The text delta that was added to the summary.
    pub delta: String,
    /// The ID of the item this summary text delta is associated with.
    pub item_id: String,
    /// The index of the output item this summary text delta is associated with.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The index of the summary part within the reasoning summary.
    pub summary_index: i64,
    /// The type of the event. Always `response.reasoning_summary_text.delta`.
    #[serde(rename = "type")]
    pub r#type: ResponseReasoningSummaryTextDeltaEventType,
}

/// The type of the event. Always `response.reasoning_summary_text.delta`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseReasoningSummaryTextDeltaEventType {
    #[default]
    #[serde(rename = "response.reasoning_summary_text.delta")]
    ResponseReasoningSummaryTextDelta,
}

impl ResponseReasoningSummaryTextDeltaEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseReasoningSummaryTextDelta => "response.reasoning_summary_text.delta",
        }
    }
}

/// Emitted when a reasoning summary text is completed.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseReasoningSummaryTextDoneEvent {
    /// The ID of the item this summary text is associated with.
    pub item_id: String,
    /// The index of the output item this summary text is associated with.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The index of the summary part within the reasoning summary.
    pub summary_index: i64,
    /// The full text of the completed reasoning summary.
    pub text: String,
    /// The type of the event. Always `response.reasoning_summary_text.done`.
    #[serde(rename = "type")]
    pub r#type: ResponseReasoningSummaryTextDoneEventType,
}

/// The type of the event. Always `response.reasoning_summary_text.done`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseReasoningSummaryTextDoneEventType {
    #[default]
    #[serde(rename = "response.reasoning_summary_text.done")]
    ResponseReasoningSummaryTextDone,
}

impl ResponseReasoningSummaryTextDoneEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseReasoningSummaryTextDone => "response.reasoning_summary_text.done",
        }
    }
}

/// Emitted when there is a partial refusal text.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseRefusalDeltaEvent {
    /// The index of the content part that the refusal text is added to.
    pub content_index: i64,
    /// The refusal text that is added.
    pub delta: String,
    /// The ID of the output item that the refusal text is added to.
    pub item_id: String,
    /// The index of the output item that the refusal text is added to.
    pub output_index: i64,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always `response.refusal.delta`.
    #[serde(rename = "type")]
    pub r#type: ResponseRefusalDeltaEventType,
}

/// The type of the event. Always `response.refusal.delta`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseRefusalDeltaEventType {
    #[default]
    #[serde(rename = "response.refusal.delta")]
    ResponseRefusalDelta,
}

impl ResponseRefusalDeltaEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseRefusalDelta => "response.refusal.delta",
        }
    }
}

/// Emitted when refusal text is finalized.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseRefusalDoneEvent {
    /// The index of the content part that the refusal text is finalized.
    pub content_index: i64,
    /// The ID of the output item that the refusal text is finalized.
    pub item_id: String,
    /// The index of the output item that the refusal text is finalized.
    pub output_index: i64,
    /// The refusal text that is finalized.
    pub refusal: String,
    /// The sequence number of this event.
    pub sequence_number: i64,
    /// The type of the event. Always `response.refusal.done`.
    #[serde(rename = "type")]
    pub r#type: ResponseRefusalDoneEventType,
}

/// The type of the event. Always `response.refusal.done`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseRefusalDoneEventType {
    #[default]
    #[serde(rename = "response.refusal.done")]
    ResponseRefusalDone,
}

impl ResponseRefusalDoneEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseRefusalDone => "response.refusal.done",
        }
    }
}

/// The status of the response generation. One of `completed`, `failed`,
/// `in_progress`, `cancelled`, `queued`, or `incomplete`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum ResponseStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "incomplete")]
    Incomplete,
}

impl ResponseStatus {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::InProgress => "in_progress",
            Self::Cancelled => "cancelled",
            Self::Queued => "queued",
            Self::Incomplete => "incomplete",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(tag = "type")]
pub enum ResponseStreamEvent {
    #[serde(rename = "response.audio.delta")]
    ResponseAudioDeltaEvent {
        /// A chunk of Base64 encoded response audio bytes.
        delta: String,
        response_id: String,
        /// A sequence number for this chunk of the stream response.
        sequence_number: i64,
    },
    #[serde(rename = "response.audio.done")]
    ResponseAudioDoneEvent {
        response_id: String,
        /// The sequence number of the delta.
        sequence_number: i64,
    },
    #[serde(rename = "response.audio.transcript.delta")]
    ResponseAudioTranscriptDeltaEvent {
        /// The partial transcript of the audio response.
        delta: String,
        response_id: String,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.audio.transcript.done")]
    ResponseAudioTranscriptDoneEvent {
        response_id: String,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.code_interpreter_call_code.delta")]
    ResponseCodeInterpreterCallCodeDeltaEvent {
        /// The partial code snippet being streamed by the code interpreter.
        delta: String,
        /// The unique identifier of the code interpreter tool call item.
        item_id: String,
        /// The index of the output item in the response for which the code is being streamed.
        output_index: i64,
        /// The sequence number of this event, used to order streaming events.
        sequence_number: i64,
    },
    #[serde(rename = "response.code_interpreter_call_code.done")]
    ResponseCodeInterpreterCallCodeDoneEvent {
        /// The final code snippet output by the code interpreter.
        code: String,
        /// The unique identifier of the code interpreter tool call item.
        item_id: String,
        /// The index of the output item in the response for which the code is finalized.
        output_index: i64,
        /// The sequence number of this event, used to order streaming events.
        sequence_number: i64,
    },
    #[serde(rename = "response.code_interpreter_call.completed")]
    ResponseCodeInterpreterCallCompletedEvent {
        /// The unique identifier of the code interpreter tool call item.
        item_id: String,
        /// The index of the output item in the response for which the code interpreter call is completed.
        output_index: i64,
        /// The sequence number of this event, used to order streaming events.
        sequence_number: i64,
    },
    #[serde(rename = "response.code_interpreter_call.in_progress")]
    ResponseCodeInterpreterCallInProgressEvent {
        /// The unique identifier of the code interpreter tool call item.
        item_id: String,
        /// The index of the output item in the response for which the code interpreter call is in progress.
        output_index: i64,
        /// The sequence number of this event, used to order streaming events.
        sequence_number: i64,
    },
    #[serde(rename = "response.code_interpreter_call.interpreting")]
    ResponseCodeInterpreterCallInterpretingEvent {
        /// The unique identifier of the code interpreter tool call item.
        item_id: String,
        /// The index of the output item in the response for which the code interpreter is interpreting code.
        output_index: i64,
        /// The sequence number of this event, used to order streaming events.
        sequence_number: i64,
    },
    #[serde(rename = "response.completed")]
    ResponseCompletedEvent {
        /// Properties of the completed response.
        response: Response,
        /// The sequence number for this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAddedEvent {
        /// The index of the content part that was added.
        content_index: i64,
        /// The ID of the output item that the content part was added to.
        item_id: String,
        /// The index of the output item that the content part was added to.
        output_index: i64,
        /// The content part that was added.
        part: OutputContent,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDoneEvent {
        /// The index of the content part that is done.
        content_index: i64,
        /// The ID of the output item that the content part was added to.
        item_id: String,
        /// The index of the output item that the content part was added to.
        output_index: i64,
        /// The content part that is done.
        part: OutputContent,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.created")]
    ResponseCreatedEvent {
        /// The response that was created.
        response: Response,
        /// The sequence number for this event.
        sequence_number: i64,
    },
    #[serde(rename = "error")]
    ResponseErrorEvent {
        /// The error code.
        code: Option<String>,
        /// The error message.
        message: String,
        /// The error parameter.
        param: Option<String>,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.file_search_call.completed")]
    ResponseFileSearchCallCompletedEvent {
        /// The ID of the output item that the file search call is initiated.
        item_id: String,
        /// The index of the output item that the file search call is initiated.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.file_search_call.in_progress")]
    ResponseFileSearchCallInProgressEvent {
        /// The ID of the output item that the file search call is initiated.
        item_id: String,
        /// The index of the output item that the file search call is initiated.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.file_search_call.searching")]
    ResponseFileSearchCallSearchingEvent {
        /// The ID of the output item that the file search call is initiated.
        item_id: String,
        /// The index of the output item that the file search call is searching.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDeltaEvent {
        /// The function-call arguments delta that is added.
        delta: String,
        /// The ID of the output item that the function-call arguments delta is added to.
        item_id: String,
        /// The index of the output item that the function-call arguments delta is added to.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDoneEvent {
        /// The function-call arguments.
        arguments: String,
        /// The ID of the item.
        item_id: String,
        /// The index of the output item.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.in_progress")]
    ResponseInProgressEvent {
        /// The response that is in progress.
        response: Response,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.failed")]
    ResponseFailedEvent {
        /// The response that failed.
        response: Response,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.incomplete")]
    ResponseIncompleteEvent {
        /// The response that was incomplete.
        response: Response,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAddedEvent {
        /// The output item that was added.
        item: OutputItem,
        /// The index of the output item that was added.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDoneEvent {
        /// The output item that was marked done.
        item: OutputItem,
        /// The index of the output item that was marked done.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.reasoning_summary_part.added")]
    ResponseReasoningSummaryPartAddedEvent {
        /// The ID of the item this summary part is associated with.
        item_id: String,
        /// The index of the output item this summary part is associated with.
        output_index: i64,
        /// The summary part that was added.
        part: ResponseReasoningSummaryPartAddedEventPart,
        /// The sequence number of this event.
        sequence_number: i64,
        /// The index of the summary part within the reasoning summary.
        summary_index: i64,
    },
    #[serde(rename = "response.reasoning_summary_part.done")]
    ResponseReasoningSummaryPartDoneEvent {
        /// The ID of the item this summary part is associated with.
        item_id: String,
        /// The index of the output item this summary part is associated with.
        output_index: i64,
        /// The completed summary part.
        part: ResponseReasoningSummaryPartDoneEventPart,
        /// The sequence number of this event.
        sequence_number: i64,
        /// The index of the summary part within the reasoning summary.
        summary_index: i64,
    },
    #[serde(rename = "response.reasoning_summary_text.delta")]
    ResponseReasoningSummaryTextDeltaEvent {
        /// The text delta that was added to the summary.
        delta: String,
        /// The ID of the item this summary text delta is associated with.
        item_id: String,
        /// The index of the output item this summary text delta is associated with.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
        /// The index of the summary part within the reasoning summary.
        summary_index: i64,
    },
    #[serde(rename = "response.reasoning_summary_text.done")]
    ResponseReasoningSummaryTextDoneEvent {
        /// The ID of the item this summary text is associated with.
        item_id: String,
        /// The index of the output item this summary text is associated with.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
        /// The index of the summary part within the reasoning summary.
        summary_index: i64,
        /// The full text of the completed reasoning summary.
        text: String,
    },
    #[serde(rename = "response.refusal.delta")]
    ResponseRefusalDeltaEvent {
        /// The index of the content part that the refusal text is added to.
        content_index: i64,
        /// The refusal text that is added.
        delta: String,
        /// The ID of the output item that the refusal text is added to.
        item_id: String,
        /// The index of the output item that the refusal text is added to.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.refusal.done")]
    ResponseRefusalDoneEvent {
        /// The index of the content part that the refusal text is finalized.
        content_index: i64,
        /// The ID of the output item that the refusal text is finalized.
        item_id: String,
        /// The index of the output item that the refusal text is finalized.
        output_index: i64,
        /// The refusal text that is finalized.
        refusal: String,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.output_text.delta")]
    ResponseTextDeltaEvent {
        /// The index of the content part that the text delta was added to.
        content_index: i64,
        /// The text delta that was added.
        delta: String,
        /// The ID of the output item that the text delta was added to.
        item_id: String,
        /// The log probabilities of the tokens in the delta.
        logprobs: Vec<ResponseLogProb>,
        /// The index of the output item that the text delta was added to.
        output_index: i64,
        /// The sequence number for this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.output_text.done")]
    ResponseTextDoneEvent {
        /// The index of the content part that the text content is finalized.
        content_index: i64,
        /// The ID of the output item that the text content is finalized.
        item_id: String,
        /// The log probabilities of the tokens in the delta.
        logprobs: Vec<ResponseLogProb>,
        /// The index of the output item that the text content is finalized.
        output_index: i64,
        /// The sequence number for this event.
        sequence_number: i64,
        /// The text content that is finalized.
        text: String,
    },
    #[serde(rename = "response.web_search_call.completed")]
    ResponseWebSearchCallCompletedEvent {
        /// Unique ID for the output item associated with the web search call.
        item_id: String,
        /// The index of the output item that the web search call is associated with.
        output_index: i64,
        /// The sequence number of the web search call being processed.
        sequence_number: i64,
    },
    #[serde(rename = "response.web_search_call.in_progress")]
    ResponseWebSearchCallInProgressEvent {
        /// Unique ID for the output item associated with the web search call.
        item_id: String,
        /// The index of the output item that the web search call is associated with.
        output_index: i64,
        /// The sequence number of the web search call being processed.
        sequence_number: i64,
    },
    #[serde(rename = "response.web_search_call.searching")]
    ResponseWebSearchCallSearchingEvent {
        /// Unique ID for the output item associated with the web search call.
        item_id: String,
        /// The index of the output item that the web search call is associated with.
        output_index: i64,
        /// The sequence number of the web search call being processed.
        sequence_number: i64,
    },
    #[serde(rename = "response.image_generation_call.completed")]
    ResponseImageGenCallCompletedEvent {
        /// The unique identifier of the image generation item being processed.
        item_id: String,
        /// The index of the output item in the response's output array.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.image_generation_call.generating")]
    ResponseImageGenCallGeneratingEvent {
        /// The unique identifier of the image generation item being processed.
        item_id: String,
        /// The index of the output item in the response's output array.
        output_index: i64,
        /// The sequence number of the image generation item being processed.
        sequence_number: i64,
    },
    #[serde(rename = "response.image_generation_call.in_progress")]
    ResponseImageGenCallInProgressEvent {
        /// The unique identifier of the image generation item being processed.
        item_id: String,
        /// The index of the output item in the response's output array.
        output_index: i64,
        /// The sequence number of the image generation item being processed.
        sequence_number: i64,
    },
    #[serde(rename = "response.image_generation_call.partial_image")]
    ResponseImageGenCallPartialImageEvent {
        /// The unique identifier of the image generation item being processed.
        item_id: String,
        /// The index of the output item in the response's output array.
        output_index: i64,
        /// Base64-encoded partial image data, suitable for rendering as an image.
        partial_image_b64: String,
        /// 0-based index for the partial image (backend is 1-based, but this is 0-based for the user).
        partial_image_index: i64,
        /// The sequence number of the image generation item being processed.
        sequence_number: i64,
    },
    #[serde(rename = "response.mcp_call_arguments.delta")]
    ResponseMCPCallArgumentsDeltaEvent {
        /// A JSON string containing the partial update to the arguments for the MCP tool call.
        delta: String,
        /// The unique identifier of the MCP tool call item being processed.
        item_id: String,
        /// The index of the output item in the response's output array.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.mcp_call_arguments.done")]
    ResponseMCPCallArgumentsDoneEvent {
        /// A JSON string containing the finalized arguments for the MCP tool call.
        arguments: String,
        /// The unique identifier of the MCP tool call item being processed.
        item_id: String,
        /// The index of the output item in the response's output array.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.mcp_call.completed")]
    ResponseMCPCallCompletedEvent {
        /// The ID of the MCP tool call item that completed.
        item_id: String,
        /// The index of the output item that completed.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.mcp_call.failed")]
    ResponseMCPCallFailedEvent {
        /// The ID of the MCP tool call item that failed.
        item_id: String,
        /// The index of the output item that failed.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.mcp_call.in_progress")]
    ResponseMCPCallInProgressEvent {
        /// The unique identifier of the MCP tool call item being processed.
        item_id: String,
        /// The index of the output item in the response's output array.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.mcp_list_tools.completed")]
    ResponseMCPListToolsCompletedEvent {
        /// The ID of the MCP tool call item that produced this output.
        item_id: String,
        /// The index of the output item that was processed.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.mcp_list_tools.failed")]
    ResponseMCPListToolsFailedEvent {
        /// The ID of the MCP tool call item that failed.
        item_id: String,
        /// The index of the output item that failed.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.mcp_list_tools.in_progress")]
    ResponseMCPListToolsInProgressEvent {
        /// The ID of the MCP tool call item that is being processed.
        item_id: String,
        /// The index of the output item that is being processed.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.output_text.annotation.added")]
    ResponseOutputTextAnnotationAddedEvent {
        /// The annotation object being added. (See annotation schema for details.)
        annotation: serde_json::Map<String, serde_json::Value>,
        /// The index of the annotation within the content part.
        annotation_index: i64,
        /// The index of the content part within the output item.
        content_index: i64,
        /// The unique identifier of the item to which the annotation is being added.
        item_id: String,
        /// The index of the output item in the response's output array.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.queued")]
    ResponseQueuedEvent {
        /// The full response object that is queued.
        response: Response,
        /// The sequence number for this event.
        sequence_number: i64,
    },
    #[serde(rename = "response.reasoning_summary.delta")]
    ResponseReasoningSummaryDeltaEvent {
        /// The partial update to the reasoning summary content.
        delta: serde_json::Map<String, serde_json::Value>,
        /// The unique identifier of the item for which the reasoning summary is being updated.
        item_id: String,
        /// The index of the output item in the response's output array.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
        /// The index of the summary part within the output item.
        summary_index: i64,
    },
    #[serde(rename = "response.reasoning_summary.done")]
    ResponseReasoningSummaryDoneEvent {
        /// The unique identifier of the item for which the reasoning summary is finalized.
        item_id: String,
        /// The index of the output item in the response's output array.
        output_index: i64,
        /// The sequence number of this event.
        sequence_number: i64,
        /// The index of the summary part within the output item.
        summary_index: i64,
        /// The finalized reasoning summary text.
        text: String,
    },
}

/// Emitted when there is an additional text delta.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseTextDeltaEvent {
    /// The index of the content part that the text delta was added to.
    pub content_index: i64,
    /// The text delta that was added.
    pub delta: String,
    /// The ID of the output item that the text delta was added to.
    pub item_id: String,
    /// The log probabilities of the tokens in the delta.
    pub logprobs: Vec<ResponseLogProb>,
    /// The index of the output item that the text delta was added to.
    pub output_index: i64,
    /// The sequence number for this event.
    pub sequence_number: i64,
    /// The type of the event. Always `response.output_text.delta`.
    #[serde(rename = "type")]
    pub r#type: ResponseTextDeltaEventType,
}

/// The type of the event. Always `response.output_text.delta`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseTextDeltaEventType {
    #[default]
    #[serde(rename = "response.output_text.delta")]
    ResponseOutputTextDelta,
}

impl ResponseTextDeltaEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseOutputTextDelta => "response.output_text.delta",
        }
    }
}

/// Emitted when text content is finalized.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseTextDoneEvent {
    /// The index of the content part that the text content is finalized.
    pub content_index: i64,
    /// The ID of the output item that the text content is finalized.
    pub item_id: String,
    /// The log probabilities of the tokens in the delta.
    pub logprobs: Vec<ResponseLogProb>,
    /// The index of the output item that the text content is finalized.
    pub output_index: i64,
    /// The sequence number for this event.
    pub sequence_number: i64,
    /// The text content that is finalized.
    pub text: String,
    /// The type of the event. Always `response.output_text.done`.
    #[serde(rename = "type")]
    pub r#type: ResponseTextDoneEventType,
}

/// The type of the event. Always `response.output_text.done`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseTextDoneEventType {
    #[default]
    #[serde(rename = "response.output_text.done")]
    ResponseOutputTextDone,
}

impl ResponseTextDoneEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseOutputTextDone => "response.output_text.done",
        }
    }
}

/// Represents token usage details including input tokens, output tokens,
/// a breakdown of output tokens, and the total tokens used.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseUsage {
    /// The number of input tokens.
    pub input_tokens: i64,
    /// A detailed breakdown of the input tokens.
    pub input_tokens_details: ResponseUsageInputTokensDetails,
    /// The number of output tokens.
    pub output_tokens: i64,
    /// A detailed breakdown of the output tokens.
    pub output_tokens_details: ResponseUsageOutputTokensDetails,
    /// The total number of tokens used.
    pub total_tokens: i64,
}

/// A detailed breakdown of the input tokens.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseUsageInputTokensDetails {
    /// The number of tokens that were retrieved from the cache.
    /// [More on prompt caching](/docs/guides/prompt-caching).
    pub cached_tokens: i64,
}

/// A detailed breakdown of the output tokens.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseUsageOutputTokensDetails {
    /// The number of reasoning tokens.
    pub reasoning_tokens: i64,
}

/// Emitted when a web search call is completed.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseWebSearchCallCompletedEvent {
    /// Unique ID for the output item associated with the web search call.
    pub item_id: String,
    /// The index of the output item that the web search call is associated with.
    pub output_index: i64,
    /// The sequence number of the web search call being processed.
    pub sequence_number: i64,
    /// The type of the event. Always `response.web_search_call.completed`.
    #[serde(rename = "type")]
    pub r#type: ResponseWebSearchCallCompletedEventType,
}

/// The type of the event. Always `response.web_search_call.completed`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseWebSearchCallCompletedEventType {
    #[default]
    #[serde(rename = "response.web_search_call.completed")]
    ResponseWebSearchCallCompleted,
}

impl ResponseWebSearchCallCompletedEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseWebSearchCallCompleted => "response.web_search_call.completed",
        }
    }
}

/// Emitted when a web search call is initiated.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseWebSearchCallInProgressEvent {
    /// Unique ID for the output item associated with the web search call.
    pub item_id: String,
    /// The index of the output item that the web search call is associated with.
    pub output_index: i64,
    /// The sequence number of the web search call being processed.
    pub sequence_number: i64,
    /// The type of the event. Always `response.web_search_call.in_progress`.
    #[serde(rename = "type")]
    pub r#type: ResponseWebSearchCallInProgressEventType,
}

/// The type of the event. Always `response.web_search_call.in_progress`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseWebSearchCallInProgressEventType {
    #[default]
    #[serde(rename = "response.web_search_call.in_progress")]
    ResponseWebSearchCallInProgress,
}

impl ResponseWebSearchCallInProgressEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseWebSearchCallInProgress => "response.web_search_call.in_progress",
        }
    }
}

/// Emitted when a web search call is executing.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResponseWebSearchCallSearchingEvent {
    /// Unique ID for the output item associated with the web search call.
    pub item_id: String,
    /// The index of the output item that the web search call is associated with.
    pub output_index: i64,
    /// The sequence number of the web search call being processed.
    pub sequence_number: i64,
    /// The type of the event. Always `response.web_search_call.searching`.
    #[serde(rename = "type")]
    pub r#type: ResponseWebSearchCallSearchingEventType,
}

/// The type of the event. Always `response.web_search_call.searching`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ResponseWebSearchCallSearchingEventType {
    #[default]
    #[serde(rename = "response.web_search_call.searching")]
    ResponseWebSearchCallSearching,
}

impl ResponseWebSearchCallSearchingEventType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::ResponseWebSearchCallSearching => "response.web_search_call.searching",
        }
    }
}

/// A screenshot action.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Screenshot {
    /// Specifies the event type. For a screenshot action, this property is
    /// always set to `screenshot`.
    #[serde(rename = "type")]
    pub r#type: ScreenshotType,
}

/// Specifies the event type. For a screenshot action, this property is
/// always set to `screenshot`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ScreenshotType {
    #[default]
    #[serde(rename = "screenshot")]
    Screenshot,
}

impl ScreenshotType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Screenshot => "screenshot",
        }
    }
}

/// A scroll action.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Scroll {
    /// The horizontal scroll distance.
    pub scroll_x: i64,
    /// The vertical scroll distance.
    pub scroll_y: i64,
    /// Specifies the event type. For a scroll action, this property is
    /// always set to `scroll`.
    #[serde(rename = "type")]
    pub r#type: ScrollType,
    /// The x-coordinate where the scroll occurred.
    pub x: i64,
    /// The y-coordinate where the scroll occurred.
    pub y: i64,
}

/// Specifies the event type. For a scroll action, this property is
/// always set to `scroll`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ScrollType {
    #[default]
    #[serde(rename = "scroll")]
    Scroll,
}

impl ScrollType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Scroll => "scroll",
        }
    }
}

/// Specifies the processing type used for serving the request.
///   - If set to 'auto', then the request will be processed with the service tier configured in the Project settings. Unless otherwise configured, the Project will use 'default'.
///   - If set to 'default', then the request will be processed with the standard pricing and performance for the selected model.
///   - If set to '[flex](/docs/guides/flex-processing)' or 'priority', then the request will be processed with the corresponding service tier. [Contact sales](https://openai.com/contact-sales) to learn more about Priority processing.
///   - When not set, the default behavior is 'auto'.
///
///   When the `service_tier` parameter is set, the response body will include the `service_tier` value based on the processing mode actually used to serve the request. This response value may be different from the value set in the parameter.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ServiceTier {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "flex")]
    Flex,
    #[serde(rename = "scale")]
    Scale,
    #[serde(rename = "priority")]
    Priority,
}

impl ServiceTier {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Auto => "auto",
            Self::Default => "default",
            Self::Flex => "flex",
            Self::Scale => "scale",
            Self::Priority => "priority",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum Status {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}

impl Status {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Incomplete => "incomplete",
        }
    }
}

/// An object specifying the format that the model must output.
///
/// Configuring `{ "type": "json_schema" }` enables Structured Outputs,
/// which ensures the model will match your supplied JSON schema. Learn more in the
/// [Structured Outputs guide](/docs/guides/structured-outputs).
///
/// The default format is `{ "type": "text" }` with no additional options.
///
/// **Not recommended for gpt-4o and newer models:**
///
/// Setting to `{ "type": "json_object" }` enables the older JSON mode, which
/// ensures the message the model generates is valid JSON. Using `json_schema`
/// is preferred for models that support it.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum TextResponseFormatConfiguration {
    #[serde(untagged)]
    ResponseFormatText(ResponseFormatText),
    #[serde(untagged)]
    TextResponseFormatJsonSchema(TextResponseFormatJsonSchema),
    #[serde(untagged)]
    ResponseFormatJsonObject(ResponseFormatJsonObject),
}

impl From<ResponseFormatJsonObject> for TextResponseFormatConfiguration {
    fn from(value: ResponseFormatJsonObject) -> Self {
        TextResponseFormatConfiguration::ResponseFormatJsonObject(value)
    }
}
impl From<ResponseFormatText> for TextResponseFormatConfiguration {
    fn from(value: ResponseFormatText) -> Self {
        TextResponseFormatConfiguration::ResponseFormatText(value)
    }
}
impl From<TextResponseFormatJsonSchema> for TextResponseFormatConfiguration {
    fn from(value: TextResponseFormatJsonSchema) -> Self {
        TextResponseFormatConfiguration::TextResponseFormatJsonSchema(value)
    }
}
/// JSON Schema response format. Used to generate structured JSON responses.
/// Learn more about [Structured Outputs](/docs/guides/structured-outputs).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct TextResponseFormatJsonSchema {
    /// A description of what the response format is for, used by the model to
    /// determine how to respond in the format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the response format. Must be a-z, A-Z, 0-9, or contain
    /// underscores and dashes, with a maximum length of 64.
    pub name: String,
    pub schema: ResponseFormatJsonSchemaSchema,
    /// Whether to enable strict schema adherence when generating the output.
    /// If set to true, the model will always follow the exact schema defined
    /// in the `schema` field. Only a subset of JSON Schema is supported when
    /// `strict` is `true`. To learn more, read the [Structured Outputs
    /// guide](/docs/guides/structured-outputs).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    /// The type of response format being defined. Always `json_schema`.
    #[serde(rename = "type")]
    pub r#type: TextResponseFormatJsonSchemaType,
}

/// The type of response format being defined. Always `json_schema`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum TextResponseFormatJsonSchemaType {
    #[default]
    #[serde(rename = "json_schema")]
    JsonSchema,
}

impl TextResponseFormatJsonSchemaType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::JsonSchema => "json_schema",
        }
    }
}

/// A tool that can be used to generate a response.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(tag = "type")]
pub enum Tool {
    #[serde(rename = "function")]
    FunctionTool {
        /// A description of the function. Used by the model to determine whether or not to call the function.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// The name of the function to call.
        name: String,
        /// A JSON schema object describing the parameters of the function.
        parameters: Option<serde_json::Map<String, serde_json::Value>>,
        /// Whether to enforce strict parameter validation. Default `true`.
        strict: Option<bool>,
    },
    #[serde(rename = "file_search")]
    FileSearchTool {
        /// A filter to apply.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filters: Option<Filters>,
        /// The maximum number of results to return. This number should be between 1 and 50 inclusive.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_num_results: Option<i64>,
        /// Ranking options for search.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        ranking_options: Option<RankingOptions>,
        /// The IDs of the vector stores to search.
        vector_store_ids: Vec<String>,
    },
    #[serde(rename = "computer_use_preview")]
    ComputerUsePreviewTool {
        /// The height of the computer display.
        display_height: i64,
        /// The width of the computer display.
        display_width: i64,
        /// The type of computer environment to control.
        environment: ComputerUsePreviewToolEnvironment,
    },
    #[serde(rename = "mcp")]
    MCPTool {
        /// List of allowed tool names or a filter object.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        allowed_tools: Option<MCPToolAllowedTools>,
        /// Optional HTTP headers to send to the MCP server. Use for authentication
        /// or other purposes.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        headers: Option<serde_json::Map<String, serde_json::Value>>,
        /// Specify which of the MCP server's tools require approval.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        require_approval: Option<MCPToolRequireApproval>,
        /// Optional description of the MCP server, used to provide more context.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        server_description: Option<String>,
        /// A label for this MCP server, used to identify it in tool calls.
        server_label: String,
        /// The URL for the MCP server.
        server_url: String,
    },
    #[serde(rename = "code_interpreter")]
    CodeInterpreterTool {
        /// The code interpreter container. Can be a container ID or an object that
        /// specifies uploaded file IDs to make available to your code.
        container: CodeInterpreterToolContainer,
    },
    #[serde(rename = "image_generation")]
    ImageGenTool {
        /// Background type for the generated image. One of `transparent`,
        /// `opaque`, or `auto`. Default: `auto`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        background: Option<ImageGenToolBackground>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        input_fidelity: Option<ImageInputFidelity>,
        /// Optional mask for inpainting. Contains `image_url`
        /// (string, optional) and `file_id` (string, optional).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        input_image_mask: Option<ImageGenToolInputImageMask>,
        /// The image generation model to use. Default: `gpt-image-1`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        model: Option<ImageGenToolModel>,
        /// Moderation level for the generated image. Default: `auto`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        moderation: Option<ImageGenToolModeration>,
        /// Compression level for the output image. Default: 100.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        output_compression: Option<i64>,
        /// The output format of the generated image. One of `png`, `webp`, or
        /// `jpeg`. Default: `png`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        output_format: Option<ImageGenToolOutputFormat>,
        /// Number of partial images to generate in streaming mode, from 0 (default value) to 3.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        partial_images: Option<i64>,
        /// The quality of the generated image. One of `low`, `medium`, `high`,
        /// or `auto`. Default: `auto`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        quality: Option<ImageGenToolQuality>,
        /// The size of the generated image. One of `1024x1024`, `1024x1536`,
        /// `1536x1024`, or `auto`. Default: `auto`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        size: Option<ImageGenToolSize>,
    },
    #[serde(rename = "local_shell")]
    LocalShellTool {},
    #[serde(untagged)]
    WebSearchPreviewTool(WebSearchPreviewTool),
}

impl From<WebSearchPreviewTool> for Tool {
    fn from(value: WebSearchPreviewTool) -> Self {
        Tool::WebSearchPreviewTool(value)
    }
}
/// Use this option to force the model to call a specific function.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ToolChoiceFunction {
    /// The name of the function to call.
    pub name: String,
    /// For function calling, the type is always `function`.
    #[serde(rename = "type")]
    pub r#type: ToolChoiceFunctionType,
}

/// For function calling, the type is always `function`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ToolChoiceFunctionType {
    #[default]
    #[serde(rename = "function")]
    Function,
}

impl ToolChoiceFunctionType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Function => "function",
        }
    }
}

/// Use this option to force the model to call a specific tool on a remote MCP server.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ToolChoiceMCP {
    /// The name of the tool to call on the server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The label of the MCP server to use.
    pub server_label: String,
    /// For MCP tools, the type is always `mcp`.
    #[serde(rename = "type")]
    pub r#type: ToolChoiceMCPType,
}

/// For MCP tools, the type is always `mcp`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum ToolChoiceMCPType {
    #[default]
    #[serde(rename = "mcp")]
    Mcp,
}

impl ToolChoiceMCPType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Mcp => "mcp",
        }
    }
}

/// Controls which (if any) tool is called by the model.
///
/// `none` means the model will not call any tool and instead generates a message.
///
/// `auto` means the model can pick between generating a message or calling one or
/// more tools.
///
/// `required` means the model must call one or more tools.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum ToolChoiceOptions {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
}

impl ToolChoiceOptions {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::Auto => "auto",
            Self::Required => "required",
        }
    }
}

/// Indicates that the model should use a built-in tool to generate a response.
/// [Learn more about built-in tools](/docs/guides/tools).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ToolChoiceTypes {
    /// The type of hosted tool the model should to use. Learn more about
    /// [built-in tools](/docs/guides/tools).
    ///
    /// Allowed values are:
    /// - `file_search`
    /// - `web_search_preview`
    /// - `computer_use_preview`
    /// - `code_interpreter`
    /// - `image_generation`
    #[serde(rename = "type")]
    pub r#type: ToolChoiceTypesType,
}

/// The type of hosted tool the model should to use. Learn more about
/// [built-in tools](/docs/guides/tools).
///
/// Allowed values are:
/// - `file_search`
/// - `web_search_preview`
/// - `computer_use_preview`
/// - `code_interpreter`
/// - `image_generation`
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum ToolChoiceTypesType {
    #[serde(rename = "file_search")]
    FileSearch,
    #[serde(rename = "web_search_preview")]
    WebSearchPreview,
    #[serde(rename = "computer_use_preview")]
    ComputerUsePreview,
    #[serde(rename = "web_search_preview_2025_03_11")]
    WebSearchPreview20250311,
    #[serde(rename = "image_generation")]
    ImageGeneration,
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
}

impl ToolChoiceTypesType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::FileSearch => "file_search",
            Self::WebSearchPreview => "web_search_preview",
            Self::ComputerUsePreview => "computer_use_preview",
            Self::WebSearchPreview20250311 => "web_search_preview_2025_03_11",
            Self::ImageGeneration => "image_generation",
            Self::CodeInterpreter => "code_interpreter",
        }
    }
}

/// The top log probability of a token.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct TopLogProb {
    pub bytes: Vec<i64>,
    pub logprob: f64,
    pub token: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct TracingConfig {
    /// The group id to attach to this trace to enable filtering and grouping in the traces dashboard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The arbitrary metadata to attach to this trace to enable filtering in the traces dashboard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
    /// The name of the workflow to attach to this trace. This is used to name the trace in the traces dashboard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum TracingConfigOrAuto {
    #[serde(rename = "auto")]
    Auto,
    #[serde(untagged)]
    TracingConfig(TracingConfig),
}

impl From<TracingConfig> for TracingConfigOrAuto {
    fn from(value: TracingConfig) -> Self {
        TracingConfigOrAuto::TracingConfig(value)
    }
}
/// Usage statistics for models billed by audio input duration.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct TranscriptTextUsageDuration {
    /// Duration of the input audio in seconds.
    pub seconds: f64,
    /// The type of the usage object. Always `duration` for this variant.
    #[serde(rename = "type")]
    pub r#type: TranscriptTextUsageDurationType,
}

/// The type of the usage object. Always `duration` for this variant.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum TranscriptTextUsageDurationType {
    #[default]
    #[serde(rename = "duration")]
    Duration,
}

impl TranscriptTextUsageDurationType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Duration => "duration",
        }
    }
}

/// Usage statistics for models billed by token usage.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct TranscriptTextUsageTokens {
    /// Details about the input tokens billed for this request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_token_details: Option<TranscriptTextUsageTokensInputTokenDetails>,
    /// Number of input tokens billed for this request.
    pub input_tokens: i64,
    /// Number of output tokens generated.
    pub output_tokens: i64,
    /// Total number of tokens used (input + output).
    pub total_tokens: i64,
    /// The type of the usage object. Always `tokens` for this variant.
    #[serde(rename = "type")]
    pub r#type: TranscriptTextUsageTokensType,
}

/// Details about the input tokens billed for this request.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct TranscriptTextUsageTokensInputTokenDetails {
    /// Number of audio tokens billed for this request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<i64>,
    /// Number of text tokens billed for this request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_tokens: Option<i64>,
}

/// The type of the usage object. Always `tokens` for this variant.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum TranscriptTextUsageTokensType {
    #[default]
    #[serde(rename = "tokens")]
    Tokens,
}

impl TranscriptTextUsageTokensType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Tokens => "tokens",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(transparent)]
pub struct TranscriptionModel(pub String);

impl From<TranscriptionModel> for String {
    fn from(value: TranscriptionModel) -> String {
        value.0
    }
}

impl From<String> for TranscriptionModel {
    fn from(value: String) -> Self {
        TranscriptionModel(value)
    }
}

impl AsRef<String> for TranscriptionModel {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl AsMut<String> for TranscriptionModel {
    fn as_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
/// An action to type in text.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Type {
    /// The text to type.
    pub text: String,
    /// Specifies the event type. For a type action, this property is
    /// always set to `type`.
    #[serde(rename = "type")]
    pub r#type: TypeType,
}

/// Specifies the event type. For a type action, this property is
/// always set to `type`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum TypeType {
    #[default]
    #[serde(rename = "type")]
    Type,
}

impl TypeType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Type => "type",
        }
    }
}

/// A citation for a web resource used to generate a model response.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct UrlCitationBody {
    /// The index of the last character of the URL citation in the message.
    pub end_index: i64,
    /// The index of the first character of the URL citation in the message.
    pub start_index: i64,
    /// The title of the web resource.
    pub title: String,
    /// The type of the URL citation. Always `url_citation`.
    #[serde(rename = "type")]
    pub r#type: UrlCitationBodyType,
    /// The URL of the web resource.
    pub url: String,
}

/// The type of the URL citation. Always `url_citation`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum UrlCitationBodyType {
    #[default]
    #[serde(rename = "url_citation")]
    UrlCitation,
}

impl UrlCitationBodyType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::UrlCitation => "url_citation",
        }
    }
}

/// Set of 16 key-value pairs that can be attached to an object. This can be
/// useful for storing additional information about the object in a structured
/// format, and querying for objects via API or the dashboard. Keys are strings
/// with a maximum length of 64 characters. Values are strings with a maximum
/// length of 512 characters, booleans, or numbers.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct VectorStoreFileAttributes {
    #[serde(flatten)]
    pub extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(transparent)]
pub struct VoiceId(pub String);

impl From<VoiceId> for String {
    fn from(value: VoiceId) -> String {
        value.0
    }
}

impl From<String> for VoiceId {
    fn from(value: String) -> Self {
        VoiceId(value)
    }
}

impl AsRef<String> for VoiceId {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl AsMut<String> for VoiceId {
    fn as_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
/// A wait action.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Wait {
    /// Specifies the event type. For a wait action, this property is
    /// always set to `wait`.
    #[serde(rename = "type")]
    pub r#type: WaitType,
}

/// Specifies the event type. For a wait action, this property is
/// always set to `wait`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum WaitType {
    #[default]
    #[serde(rename = "wait")]
    Wait,
}

impl WaitType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Wait => "wait",
        }
    }
}

/// Action type "find": Searches for a pattern within a loaded page.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct WebSearchActionFind {
    /// The pattern or text to search for within the page.
    pub pattern: String,
    /// The action type.
    #[serde(rename = "type")]
    pub r#type: WebSearchActionFindType,
    /// The URL of the page searched for the pattern.
    pub url: String,
}

/// The action type.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum WebSearchActionFindType {
    #[default]
    #[serde(rename = "find")]
    Find,
}

impl WebSearchActionFindType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Find => "find",
        }
    }
}

/// Action type "open_page" - Opens a specific URL from search results.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct WebSearchActionOpenPage {
    /// The action type.
    #[serde(rename = "type")]
    pub r#type: WebSearchActionOpenPageType,
    /// The URL opened by the model.
    pub url: String,
}

/// The action type.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum WebSearchActionOpenPageType {
    #[default]
    #[serde(rename = "open_page")]
    OpenPage,
}

impl WebSearchActionOpenPageType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::OpenPage => "open_page",
        }
    }
}

/// Action type "search" - Performs a web search query.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct WebSearchActionSearch {
    /// The search query.
    pub query: String,
    /// The action type.
    #[serde(rename = "type")]
    pub r#type: WebSearchActionSearchType,
}

/// The action type.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum WebSearchActionSearchType {
    #[default]
    #[serde(rename = "search")]
    Search,
}

impl WebSearchActionSearchType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Search => "search",
        }
    }
}

/// This tool searches the web for relevant results to use in a response. Learn more about the [web search tool](https://platform.openai.com/docs/guides/tools-web-search).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct WebSearchPreviewTool {
    /// High level guidance for the amount of context window space to use for the search. One of `low`, `medium`, or `high`. `medium` is the default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_context_size: Option<WebSearchPreviewToolSearchContextSize>,
    /// The type of the web search tool. One of `web_search_preview` or `web_search_preview_2025_03_11`.
    #[serde(rename = "type")]
    pub r#type: WebSearchPreviewToolType,
    /// The user's location.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_location: Option<ApproximateLocation>,
}

/// High level guidance for the amount of context window space to use for the search. One of `low`, `medium`, or `high`. `medium` is the default.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum WebSearchPreviewToolSearchContextSize {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}

impl WebSearchPreviewToolSearchContextSize {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}

/// The type of the web search tool. One of `web_search_preview` or `web_search_preview_2025_03_11`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum WebSearchPreviewToolType {
    #[default]
    #[serde(rename = "web_search_preview")]
    WebSearchPreview,
    #[serde(rename = "web_search_preview_2025_03_11")]
    WebSearchPreview20250311,
}

impl WebSearchPreviewToolType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::WebSearchPreview => "web_search_preview",
            Self::WebSearchPreview20250311 => "web_search_preview_2025_03_11",
        }
    }
}

/// The results of a web search tool call. See the
/// [web search guide](/docs/guides/tools-web-search) for more information.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct WebSearchToolCall {
    /// An object describing the specific action taken in this web search call.
    /// Includes details on how the model used the web (search, open_page, find).
    pub action: WebSearchToolCallAction,
    /// The unique ID of the web search tool call.
    pub id: String,
    /// The status of the web search tool call.
    pub status: WebSearchToolCallStatus,
    /// The type of the web search tool call. Always `web_search_call`.
    #[serde(rename = "type")]
    pub r#type: WebSearchToolCallType,
}

impl crate::HasId for WebSearchToolCall {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}

/// An object describing the specific action taken in this web search call.
/// Includes details on how the model used the web (search, open_page, find).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum WebSearchToolCallAction {
    #[serde(untagged)]
    WebSearchActionSearch(WebSearchActionSearch),
    #[serde(untagged)]
    WebSearchActionOpenPage(WebSearchActionOpenPage),
    #[serde(untagged)]
    WebSearchActionFind(WebSearchActionFind),
}

impl From<WebSearchActionFind> for WebSearchToolCallAction {
    fn from(value: WebSearchActionFind) -> Self {
        WebSearchToolCallAction::WebSearchActionFind(value)
    }
}
impl From<WebSearchActionOpenPage> for WebSearchToolCallAction {
    fn from(value: WebSearchActionOpenPage) -> Self {
        WebSearchToolCallAction::WebSearchActionOpenPage(value)
    }
}
impl From<WebSearchActionSearch> for WebSearchToolCallAction {
    fn from(value: WebSearchActionSearch) -> Self {
        WebSearchToolCallAction::WebSearchActionSearch(value)
    }
}
/// The status of the web search tool call.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum WebSearchToolCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "searching")]
    Searching,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}

impl WebSearchToolCallStatus {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::InProgress => "in_progress",
            Self::Searching => "searching",
            Self::Completed => "completed",
            Self::Failed => "failed",
        }
    }
}

/// The type of the web search tool call. Always `web_search_call`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
pub enum WebSearchToolCallType {
    #[default]
    #[serde(rename = "web_search_call")]
    WebSearchCall,
}

impl WebSearchToolCallType {
    /// Converts the enum to a string
    pub fn to_str(&self) -> &str {
        match self {
            Self::WebSearchCall => "web_search_call",
        }
    }
}

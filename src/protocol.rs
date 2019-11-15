#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbPFtpError {
    OperationSucceeded = 0,
    Rebooting = 1,
    TryAgain = 2,
    UnidentifiedHostError = 100,
    InvalidCommand = 101,
    InvalidParameter = 102,
    NoSuchFileOrDirectory = 103,
    DirectoryExists = 104,
    FileExists = 105,
    OperationNotPermitted = 106,
    NoSuchUser = 107,
    Timeout = 108,
    UnidentifiedDeviceError = 200,
    NotImplemented = 201,
    SystemBusy = 202,
    InvalidContent = 203,
    ChecksumFailure = 204,
    DiskFull = 205,
    PrerequisiteNotMet = 206,
    InsufficientBuffer = 207,
    WaitForIdling = 208,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpFilesystemModifiedParams {
    #[prost(enumeration="Action", required, tag="1")]
    pub action: i32,
    #[prost(string, required, tag="2")]
    pub path: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpInactivityAlert {
    #[prost(uint32, required, tag="1")]
    pub countdown: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpTrainingSessionStatus {
    #[prost(bool, required, tag="1")]
    pub inprogress: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpAutoSyncStatusParams {
    #[prost(bool, required, tag="1")]
    pub succeeded: bool,
    #[prost(string, optional, tag="2")]
    pub description: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPftpPnsDhAttribute {
    #[prost(enumeration="PbPftpPnsDhAttributeType", required, tag="1")]
    pub r#type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPftpPnsDhNotificationResponse {
    #[prost(uint32, required, tag="1")]
    pub notification_id: u32,
    #[prost(message, repeated, tag="2")]
    pub attributes: ::std::vec::Vec<PbPftpPnsDhAttribute>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPftpPnsState {
    #[prost(bool, required, tag="1")]
    pub notifications_enabled: bool,
    #[prost(bool, optional, tag="2")]
    pub preview_enabled: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpStopSyncParams {
    #[prost(bool, required, tag="1")]
    pub completed: bool,
    #[prost(string, optional, tag="2")]
    pub description: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpFactoryResetParams {
    #[prost(bool, required, tag="1")]
    pub sleep: bool,
    #[prost(bool, optional, tag="2", default="true")]
    pub do_factory_defaults: ::std::option::Option<bool>,
    #[prost(bool, optional, tag="3", default="false")]
    pub ota_fwupdate: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpStartAutosyncParams {
    #[prost(uint32, required, tag="1")]
    pub timeout: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPftpPnsHdAttribute {
    #[prost(enumeration="PbPftpPnsHdAttributeType", required, tag="1")]
    pub r#type: i32,
    #[prost(string, optional, tag="2")]
    pub data: ::std::option::Option<std::string::String>,
    #[prost(uint32, optional, tag="3")]
    pub attribute_full_size: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPftpPnsHdNotification {
    #[prost(uint32, required, tag="1")]
    pub notification_id: u32,
    #[prost(enumeration="PbPftpPnsHdCategoryId", required, tag="2")]
    pub category_id: i32,
    #[prost(enumeration="Action", required, tag="3")]
    pub action: i32,
    #[prost(message, required, tag="4")]
    pub issue_time: super::types::PbLocalDateTime,
    #[prost(uint32, optional, tag="5")]
    pub new_same_category_notifications: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub unread_same_category_notifications: ::std::option::Option<u32>,
    #[prost(message, repeated, tag="7")]
    pub attributes: ::std::vec::Vec<PbPftpPnsHdAttribute>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbPFtpDevToHostNotification {
    FilesystemModified = 0,
    InternalTestEvent = 1,
    Idling = 2,
    BatteryStatus = 3,
    InactivityAlert = 4,
    TrainingSessionStatus = 5,
    SyncRequired = 7,
    AutosyncStatus = 8,
    PnsDhNotificationResponse = 9,
    PnsSettings = 10,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    Created = 0,
    Updated = 1,
    Removed = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbPftpPnsDhAttributeType {
    UnknownAction = 1,
    PositiveAction = 2,
    NegativeAction = 3,
    ClearAction = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbPFtpHostToDevNotification {
    StartSync = 0,
    StopSync = 1,
    Reset = 2,
    LockProductionData = 3,
    TerminateSync = 4,
    KeepAlive = 5,
    StartAutosync = 6,
    PnsHdNotification = 7,
    InitializeSession = 8,
    TerminateSession = 9,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbPftpPnsHdCategoryId {
    CategoryIdOther = 0,
    CategoryIdPolar = 1,
    CategoryIdIncomingcall = 2,
    CategoryIdMissedcall = 3,
    CategoryIdVoicemail = 4,
    CategoryIdSocial = 5,
    CategoryIdSchedule = 6,
    CategoryIdEmail = 7,
    CategoryIdNews = 8,
    CategoryIdHealthandfitness = 9,
    CategoryIdBusinessandfinance = 10,
    CategoryIdLocation = 11,
    CategoryIdEntertainment = 12,
    CategoryIdAlarm = 13,
    CategoryIdPromo = 14,
    CategoryIdRecommendation = 15,
    CategoryIdStatus = 16,
    CategoryIdTransport = 17,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbPftpPnsHdAttributeType {
    Title = 0,
    Subtitle = 1,
    Message = 2,
    PositiveActionLabel = 3,
    NegativeActionLabel = 4,
    ApplicationName = 5,
    ClearActionLabel = 6,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpEntry {
    #[prost(string, required, tag="1")]
    pub name: std::string::String,
    #[prost(uint64, required, tag="2")]
    pub size: u64,
    #[prost(message, optional, tag="3")]
    pub created: ::std::option::Option<super::types::PbSystemDateTime>,
    #[prost(message, optional, tag="4")]
    pub modified: ::std::option::Option<super::types::PbSystemDateTime>,
    #[prost(message, optional, tag="5")]
    pub touched: ::std::option::Option<super::types::PbSystemDateTime>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpDirectory {
    /// [packed=true];
    #[prost(message, repeated, tag="1")]
    pub entries: ::std::vec::Vec<PbPFtpEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpIdentifyDeviceResult {
    #[prost(string, required, tag="1")]
    pub device_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpGetSystemTimeResult {
    #[prost(message, required, tag="1")]
    pub date: super::types::PbDate,
    #[prost(message, required, tag="2")]
    pub time: super::types::PbTime,
    #[prost(bool, required, tag="3")]
    pub trusted: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpGetLocalTimeResult {
    #[prost(message, required, tag="1")]
    pub date: super::types::PbDate,
    #[prost(message, required, tag="2")]
    pub time: super::types::PbTime,
    #[prost(int32, optional, tag="3")]
    pub tz_offset: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpDiskSpaceResult {
    #[prost(uint32, required, tag="1")]
    pub fragment_size: u32,
    #[prost(uint64, required, tag="2")]
    pub total_fragments: u64,
    #[prost(uint64, required, tag="3")]
    pub free_fragments: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpGenerateChallengeTokenResult {
    #[prost(bytes, required, tag="1")]
    pub token: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpBatteryStatusResult {
    #[prost(uint32, required, tag="1")]
    pub battery_status: u32,
    #[prost(bool, optional, tag="2")]
    pub charging: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpGetInactivityPreAlertResult {
    #[prost(bool, required, tag="1")]
    pub inactivity_pre_alert_on: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpOperation {
    #[prost(enumeration="pb_p_ftp_operation::Command", required, tag="1")]
    pub command: i32,
    #[prost(string, required, tag="2")]
    pub path: std::string::String,
}
pub mod pb_p_ftp_operation {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Command {
        Get = 0,
        Put = 1,
        Merge = 2,
        Remove = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpSetSystemTimeParams {
    #[prost(message, required, tag="1")]
    pub date: super::types::PbDate,
    #[prost(message, required, tag="2")]
    pub time: super::types::PbTime,
    #[prost(bool, required, tag="3")]
    pub trusted: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpSetLocalTimeParams {
    #[prost(message, required, tag="1")]
    pub date: super::types::PbDate,
    #[prost(message, required, tag="2")]
    pub time: super::types::PbTime,
    #[prost(int32, optional, tag="3")]
    pub tz_offset: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpGenerateChallengeTokenParams {
    #[prost(uint32, required, tag="1")]
    pub user_id: u32,
    #[prost(bytes, required, tag="2")]
    pub nonse: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpSetAdbModeParams {
    #[prost(bool, required, tag="1")]
    pub enable: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPFtpCleanupDiskSpaceParams {
    #[prost(uint64, required, tag="1")]
    pub required_bytes: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbPFtpQuery {
    IdentifyDevice = 0,
    SetSystemTime = 1,
    GetSystemTime = 2,
    SetLocalTime = 3,
    GetLocalTime = 4,
    GetDiskSpace = 5,
    GenerateChallengeToken = 6,
    SetInternalTest = 7,
    GetBatteryStatus = 8,
    SetAdbMode = 9,
    CleanupDiskSpace = 10,
    GetInactivityPreAlert = 11,
}

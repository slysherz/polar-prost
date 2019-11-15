#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbRangeOptions {
    #[prost(int32, optional, tag="1")]
    pub min_value: ::std::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub max_value: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbDate {
    #[prost(uint32, required, tag="1")]
    pub year: u32,
    #[prost(uint32, required, tag="2")]
    pub month: u32,
    #[prost(uint32, required, tag="3")]
    pub day: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbTime {
    #[prost(uint32, required, tag="1")]
    pub hour: u32,
    #[prost(uint32, required, tag="2")]
    pub minute: u32,
    #[prost(uint32, required, tag="3")]
    pub seconds: u32,
    #[prost(uint32, optional, tag="4", default="0")]
    pub millis: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSystemDateTime {
    #[prost(message, required, tag="1")]
    pub date: PbDate,
    #[prost(message, required, tag="2")]
    pub time: PbTime,
    #[prost(bool, required, tag="3")]
    pub trusted: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLocalDateTime {
    #[prost(message, required, tag="1")]
    pub date: PbDate,
    #[prost(message, required, tag="2")]
    pub time: PbTime,
    #[prost(bool, required, tag="3")]
    pub obsolete_trusted: bool,
    #[prost(int32, optional, tag="4")]
    pub time_zone_offset: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbDuration {
    #[prost(uint32, optional, tag="1", default="0")]
    pub hours: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="2", default="0")]
    pub minutes: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="3", default="0")]
    pub seconds: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="4", default="0")]
    pub millis: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLocation {
    #[prost(double, required, tag="1")]
    pub latitude: f64,
    #[prost(double, required, tag="2")]
    pub longitude: f64,
    #[prost(message, optional, tag="3")]
    pub timestamp: ::std::option::Option<PbSystemDateTime>,
    #[prost(enumeration="pb_location::Fix", optional, tag="4", default="None")]
    pub fix: ::std::option::Option<i32>,
    #[prost(uint32, optional, tag="5", default="0")]
    pub satellites: ::std::option::Option<u32>,
}
pub mod pb_location {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Fix {
        None = 0,
        Fix2d = 1,
        Fix3d = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSensorOffline {
    #[prost(uint32, required, tag="1")]
    pub start_index: u32,
    #[prost(uint32, required, tag="2")]
    pub stop_index: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbVolume {
    #[prost(uint32, required, tag="1")]
    pub volume: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbStrideSensorCalibSettings {
    #[prost(float, required, tag="1")]
    pub running_factor: f32,
    #[prost(enumeration="pb_stride_sensor_calib_settings::PbStrideCalibType", required, tag="2")]
    pub calib_type: i32,
    #[prost(enumeration="pb_stride_sensor_calib_settings::PbRunningFactorSource", optional, tag="3", default="RunningFactorSourceDefault")]
    pub running_factor_source: ::std::option::Option<i32>,
}
pub mod pb_stride_sensor_calib_settings {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbStrideCalibType {
        StrideCalibManual = 0,
        StrideCalibAuto = 1,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbRunningFactorSource {
        RunningFactorSourceDefault = 0,
        RunningFactorSourceAutoCalibration = 1,
        RunningFactorSourceManualCalibration = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbVolumeTarget {
    #[prost(enumeration="pb_volume_target::PbVolymeTargetType", required, tag="1")]
    pub target_type: i32,
    #[prost(message, optional, tag="2")]
    pub duration: ::std::option::Option<PbDuration>,
    #[prost(float, optional, tag="3")]
    pub distance: ::std::option::Option<f32>,
    #[prost(uint32, optional, tag="4")]
    pub calories: ::std::option::Option<u32>,
}
pub mod pb_volume_target {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbVolymeTargetType {
        VolumeTargetTypeDuration = 0,
        VolumeTargetTypeDistance = 1,
        VolumeTargetTypeCalories = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbTrainingLoad {
    #[prost(uint32, optional, tag="1")]
    pub training_load_val: ::std::option::Option<u32>,
    #[prost(message, optional, tag="2")]
    pub recovery_time: ::std::option::Option<PbDuration>,
    #[prost(uint32, optional, tag="3")]
    pub carbohydrate_consumption: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub protein_consumption: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub fat_consumption: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbHeartRateZone {
    #[prost(uint32, required, tag="1")]
    pub lower_limit: u32,
    #[prost(uint32, required, tag="2")]
    pub higher_limit: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSpeedZone {
    #[prost(float, required, tag="1")]
    pub lower_limit: f32,
    #[prost(float, required, tag="2")]
    pub higher_limit: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPowerZone {
    #[prost(uint32, required, tag="1")]
    pub lower_limit: u32,
    #[prost(uint32, required, tag="2")]
    pub higher_limit: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbZones {
    #[prost(message, repeated, tag="1")]
    pub heart_rate_zone: ::std::vec::Vec<PbHeartRateZone>,
    #[prost(message, repeated, tag="2")]
    pub speed_zone: ::std::vec::Vec<PbSpeedZone>,
    #[prost(message, repeated, tag="3")]
    pub power_zone: ::std::vec::Vec<PbPowerZone>,
    #[prost(enumeration="PbHeartRateZoneSettingSource", optional, tag="10")]
    pub heart_rate_setting_source: ::std::option::Option<i32>,
    #[prost(enumeration="PbPowerZoneSettingSource", optional, tag="11")]
    pub power_setting_source: ::std::option::Option<i32>,
    #[prost(enumeration="PbSpeedZoneSettingSource", optional, tag="12")]
    pub speed_setting_source: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbBleMac {
    #[prost(bytes, required, tag="1")]
    pub mac: std::vec::Vec<u8>,
    #[prost(enumeration="PbMacType", required, tag="2")]
    pub r#type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbBleDeviceName {
    #[prost(string, required, tag="1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbDeviceId {
    #[prost(string, required, tag="1")]
    pub device_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbRunningIndex {
    #[prost(uint32, required, tag="1")]
    pub value: u32,
    #[prost(message, optional, tag="2")]
    pub calculation_time: ::std::option::Option<PbDuration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSportIdentifier {
    #[prost(uint64, required, tag="1")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbOneLineText {
    #[prost(string, required, tag="1")]
    pub text: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbMultiLineText {
    #[prost(string, required, tag="1")]
    pub text: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLanguageId {
    #[prost(string, required, tag="1")]
    pub language: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbTrainingSessionTargetId {
    #[prost(uint64, required, tag="1")]
    pub value: u64,
    #[prost(message, optional, tag="2")]
    pub last_modified: ::std::option::Option<PbSystemDateTime>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbTrainingSessionFavoriteId {
    #[prost(uint64, required, tag="1")]
    pub value: u64,
    #[prost(message, optional, tag="2")]
    pub last_modified: ::std::option::Option<PbSystemDateTime>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbRouteId {
    #[prost(uint64, required, tag="1")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSwimmingPoolInfo {
    #[prost(float, optional, tag="1")]
    pub pool_length: ::std::option::Option<f32>,
    #[prost(enumeration="PbSwimmingPoolUnits", required, tag="2")]
    pub swimming_pool_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbTrainingProgramId {
    #[prost(uint64, required, tag="1")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbEventId {
    #[prost(uint64, required, tag="1")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbStravaSegmentTargets {
    #[prost(message, required, tag="1")]
    pub own_best: PbDuration,
    #[prost(message, required, tag="2")]
    pub kom_qom: PbDuration,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbStravaSegmentTarget {
    #[prost(enumeration="pb_strava_segment_target::PbStravaSegmentType", required, tag="1")]
    pub strava_segment_type: i32,
    #[prost(message, required, tag="2")]
    pub strava_segment_targets: PbStravaSegmentTargets,
}
pub mod pb_strava_segment_target {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbStravaSegmentType {
        StravaSegmentTypeRide = 1,
        StravaSegmentTypeRun = 2,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbDataType {
    TypeUndefined = 0,
    TypeInherited = 1,
    TypeEnum = 2,
    TypeMillis = 3,
    TypeSecond = 4,
    TypeMinute = 5,
    TypeHour = 6,
    TypeHours = 7,
    TypeDay = 8,
    TypeMonth = 9,
    TypeYear = 10,
    TypeWeight = 11,
    TypeHeight = 12,
    TypeVo2max = 13,
    TypeHeartrate = 20,
    TypeHrPercent = 21,
    TypeHrReserve = 22,
    TypeSpeed = 23,
    TypeCadence = 24,
    TypeAltitude = 25,
    TypePower = 26,
    TypePowerLrb = 27,
    TypePowerPi = 28,
    TypeTemperature = 29,
    TypeActivity = 30,
    TypeStrideLength = 31,
    TypeIncline = 32,
    TypeDecline = 33,
    TypeDistance = 52,
    TypeEnergy = 53,
    TypeFatPercents = 54,
    TypeAscent = 55,
    TypeDescent = 56,
    TypeLatitude = 57,
    TypeLongitude = 58,
    TypeHertz = 59,
    TypePercent = 60,
    TypeCumulatedActivityDay = 61,
    TypeRunningIndex = 62,
    TypeRrInterval = 63,
    TypeZIndex = 64,
    TypeExerciseTargetIndex = 65,
    TypeTimeZoneOffset = 66,
    TypeWheelSize = 67,
    TypeFitnessClass = 68,
    TypeAcceleration = 69,
    TypeCrankLength = 70,
    TypeAngleDegree = 71,
    TypeNewton = 72,
    TypeFunctionalThresholdPower = 73,
    TypeCalories = 74,
    TypeSpeedCalibrationOffset = 75,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbHeartRateView {
    HeartRateViewBpm = 1,
    HeartRateViewPercentsOfHrReserve = 2,
    HeartRateViewPercentsOfMaxHr = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbUnitSystem {
    Metric = 1,
    Imperial = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbTimeSelection {
    Time1 = 1,
    Time2 = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbTimeFormat {
    TimeFormat24h = 1,
    TimeFormat12h = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbTimeFormatSeparator {
    TimeFormatSeparatorDot = 1,
    TimeFormatSeparatorColon = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbStartDayOfWeek {
    Monday = 1,
    Saturday = 2,
    Sunday = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbDateFormatSeparator {
    Dot = 1,
    Slash = 2,
    Hyphen = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbDateFormat {
    DdMmYyyy = 1,
    MmDdYyyy = 2,
    YyyyMmDd = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbFeatureType {
    FeatureTypeHeartRate = 1,
    FeatureTypeRrInterval = 2,
    FeatureTypeSpeed = 3,
    FeatureTypeDistance = 4,
    FeatureTypeBikeCadence = 5,
    FeatureTypeBikePower = 6,
    FeatureTypeGpsLocation = 7,
    FeatureTypeRunningCadence = 8,
    FeatureTypePressTemperature = 9,
    FeatureTypeAltitude = 10,
    FeatureTypeSteps = 11,
    FeatureTypeActivity = 12,
    FeatureTypeStrideLength = 13,
    FeatureTypeRscMovingType = 14,
    FeatureTypeJumpHeigth = 15,
    FeatureTypeCompassHeading = 16,
    FeatureTypeGpsSpeed = 17,
    FeatureTypeGpsDistance = 18,
    FeatureTypeGpsAltitude = 19,
    FeatureTypeBikeWheelRevolution = 20,
    FeatureTypeBikeCrankRevolution = 21,
    FeatureTypeAsSpeed = 22,
    FeatureTypeAsCadence = 23,
    FeatureTypeAsDistance = 24,
    FeatureTypeAsSwrState = 25,
    FeatureTypeBatteryLevel = 26,
    FeatureTypeFileTransfer = 27,
    FeatureTypePushNotifications = 28,
    FeatureTypeWeightScale = 29,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbMovingType {
    Walking = 0,
    Running = 1,
    Standing = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbOperationType {
    Multiply = 1,
    Sum = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbExerciseFeedback {
    FeedbackNone = 1,
    Feedback1 = 2,
    Feedback2 = 3,
    Feedback3 = 4,
    Feedback4 = 5,
    Feedback5 = 6,
    Feedback6 = 7,
    Feedback7 = 8,
    Feedback8 = 9,
    Feedback9 = 10,
    Feedback10 = 11,
    Feedback11 = 12,
    Feedback12 = 13,
    Feedback13 = 14,
    Feedback14 = 15,
    Feedback15 = 16,
    Feedback16 = 17,
    Feedback17 = 18,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbHeartRateZoneSettingSource {
    HeartRateZoneSettingSourceDefault = 0,
    HeartRateZoneSettingSourceThreshold = 1,
    HeartRateZoneSettingSourceFree = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbPowerZoneSettingSource {
    PowerZoneSettingSourceDefault = 0,
    PowerZoneSettingSourceFree = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbSpeedZoneSettingSource {
    SpeedZoneSettingSourceDefault = 0,
    SpeedZoneSettingSourceFree = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbMacType {
    MacTypePublic = 0,
    MacTypeStatic = 1,
    MacTypePrivateNonresolvable = 2,
    MacTypePrivateResolvable = 3,
    MacTypeBtClassic = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbSwimmingStyle {
    Other = -1,
    Turn = 0,
    OtherSwimming = 10,
    Freestyle = 11,
    Breaststroke = 12,
    Backstroke = 13,
    Butterfly = 14,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbSwimmingPoolUnits {
    SwimmingPoolMeters = 0,
    SwimmingPoolYards = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbExerciseTargetType {
    ExerciseTargetTypeFree = 0,
    ExerciseTargetTypeVolume = 1,
    ExerciseTargetTypePhased = 2,
    ExerciseTargetTypeRoute = 3,
    ExerciseTargetTypeSteadyRacePace = 4,
    ExerciseTargetTypeRouteRacePace = 5,
    ExerciseTargetTypeStravaSegment = 6,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbDeviceLocation {
    DeviceLocationUndefined = 0,
    DeviceLocationOther = 1,
    DeviceLocationWristLeft = 2,
    DeviceLocationWristRight = 3,
    DeviceLocationNecklace = 4,
    DeviceLocationChest = 5,
    DeviceLocationUpperBack = 6,
    DeviceLocationFootLeft = 7,
    DeviceLocationFootRight = 8,
    DeviceLocationLowerArmLeft = 9,
    DeviceLocationLowerArmRight = 10,
    DeviceLocationUpperArmLeft = 11,
    DeviceLocationUpperArmRight = 12,
    DeviceLocationBikeMount = 13,
}

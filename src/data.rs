#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbActivityMetMinGoal {
    #[prost(float, required, tag="1")]
    pub goal: f32,
    #[prost(float, optional, tag="2")]
    pub activity_cutoff_threshold: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPolarBalanceGoal {
    #[prost(message, required, tag="1")]
    pub start_date: super::types::PbDate,
    #[prost(float, optional, tag="2")]
    pub target_weight: ::std::option::Option<f32>,
    #[prost(uint32, optional, tag="3")]
    pub goal_duration_in_weeks: ::std::option::Option<u32>,
    #[prost(float, optional, tag="4")]
    pub fraction_of_activity: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbDailyActivityGoal {
    #[prost(enumeration="pb_daily_activity_goal::PbActivityGoalType", optional, tag="3")]
    pub goal_type: ::std::option::Option<i32>,
    #[prost(message, optional, tag="1")]
    pub activity_metmin_goal: ::std::option::Option<PbActivityMetMinGoal>,
    #[prost(message, required, tag="2")]
    pub last_modified: super::types::PbSystemDateTime,
    #[prost(message, optional, tag="4")]
    pub polar_balance_goal: ::std::option::Option<PbPolarBalanceGoal>,
}
pub mod pb_daily_activity_goal {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbActivityGoalType {
        ActivityGoalDailyActivity = 1,
        ActivityGoalWeightLoss = 2,
        ActivityGoalWeightMaintain = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSportInfo {
    #[prost(float, required, tag="1")]
    pub factor: f32,
    #[prost(message, required, tag="2")]
    pub time_stamp: super::types::PbLocalDateTime,
    #[prost(uint64, optional, tag="3")]
    pub sport_profile_id: ::std::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbActivityInfo {
    #[prost(enumeration="pb_activity_info::ActivityClass", required, tag="1")]
    pub value: i32,
    #[prost(message, required, tag="2")]
    pub time_stamp: super::types::PbLocalDateTime,
    #[prost(float, optional, tag="3")]
    pub factor: ::std::option::Option<f32>,
}
pub mod pb_activity_info {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ActivityClass {
        Sleep = 1,
        Sedentary = 2,
        Light = 3,
        ContinuousModerate = 4,
        IntermittentModerate = 5,
        ContinuousVigorous = 6,
        IntermittentVigorous = 7,
        NonWear = 8,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbInActivityTriggerInfo {
    #[prost(message, required, tag="1")]
    pub time_stamp: super::types::PbLocalDateTime,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbInActivityNonWearTriggerInfo {
    #[prost(message, required, tag="1")]
    pub start_time_stamp: super::types::PbLocalDateTime,
    #[prost(message, required, tag="2")]
    pub end_time_stamp: super::types::PbLocalDateTime,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbActivitySamples {
    #[prost(message, required, tag="1")]
    pub start_time: super::types::PbLocalDateTime,
    #[prost(message, required, tag="2")]
    pub met_recording_interval: super::types::PbDuration,
    #[prost(message, required, tag="3")]
    pub steps_recording_interval: super::types::PbDuration,
    #[prost(float, repeated, packed="false", tag="4")]
    pub met_samples: ::std::vec::Vec<f32>,
    #[prost(uint32, repeated, tag="5")]
    pub steps_samples: ::std::vec::Vec<u32>,
    #[prost(message, repeated, tag="6")]
    pub sport_info: ::std::vec::Vec<PbSportInfo>,
    #[prost(message, repeated, tag="7")]
    pub activity_info: ::std::vec::Vec<PbActivityInfo>,
    #[prost(message, repeated, tag="8")]
    pub inactivity_trigger: ::std::vec::Vec<PbInActivityTriggerInfo>,
    #[prost(message, repeated, tag="9")]
    pub inactivity_non_wear_trigger: ::std::vec::Vec<PbInActivityNonWearTriggerInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbBleUuid {
    #[prost(bytes, required, tag="1")]
    pub uuid: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbBleCharacteristic {
    #[prost(uint32, required, tag="1")]
    pub handle: u32,
    #[prost(message, required, tag="2")]
    pub r#type: PbBleUuid,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbBleService {
    #[prost(message, required, tag="1")]
    pub service_uuid: PbBleUuid,
    #[prost(message, repeated, tag="2")]
    pub characteristics: ::std::vec::Vec<PbBleCharacteristic>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbBleUser {
    #[prost(uint32, required, tag="1")]
    pub user_index: u32,
    #[prost(uint32, required, tag="2")]
    pub device_user_index: u32,
    #[prost(uint32, optional, tag="3")]
    pub consent: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbBleDevice {
    #[prost(message, required, tag="1")]
    pub paired: super::types::PbSystemDateTime,
    #[prost(message, required, tag="2")]
    pub last_modified: super::types::PbSystemDateTime,
    #[prost(enumeration="PbDeviceManufacturerType", required, tag="3")]
    pub manufacturer: i32,
    #[prost(message, optional, tag="5")]
    pub deleted_time_stamp: ::std::option::Option<super::types::PbSystemDateTime>,
    #[prost(message, optional, tag="6")]
    pub mac: ::std::option::Option<super::types::PbBleMac>,
    #[prost(string, optional, tag="7")]
    pub device_id: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="8")]
    pub name: ::std::option::Option<std::string::String>,
    #[prost(uint32, optional, tag="9")]
    pub battery_level: ::std::option::Option<u32>,
    #[prost(string, optional, tag="10")]
    pub manufacturer_name: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="11")]
    pub model_name: ::std::option::Option<std::string::String>,
    #[prost(bytes, optional, tag="12")]
    pub peer_ltk: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(bytes, optional, tag="13")]
    pub peer_irk: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(bytes, optional, tag="14")]
    pub peer_csrk: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(enumeration="super::types::PbFeatureType", repeated, packed="false", tag="15")]
    pub available_features: ::std::vec::Vec<i32>,
    #[prost(message, repeated, tag="16")]
    pub services: ::std::vec::Vec<PbBleService>,
    #[prost(bytes, optional, tag="17")]
    pub peer_rand: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="18")]
    pub peer_ediv: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="19")]
    pub encr_key_size: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="20")]
    pub distributed_keys: ::std::option::Option<u32>,
    #[prost(bool, optional, tag="21")]
    pub authenticated: ::std::option::Option<bool>,
    #[prost(enumeration="pb_ble_device::PbSensorLocation", optional, tag="22")]
    pub sensor_location: ::std::option::Option<i32>,
    #[prost(string, optional, tag="23")]
    pub software_version: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="24")]
    pub secondary_software_version: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="25")]
    pub serial_number: ::std::option::Option<std::string::String>,
    #[prost(bytes, optional, tag="26")]
    pub local_ltk: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(bytes, optional, tag="27")]
    pub local_rand: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="28")]
    pub local_ediv: ::std::option::Option<u32>,
    #[prost(message, repeated, tag="29")]
    pub user_data: ::std::vec::Vec<PbBleUser>,
}
pub mod pb_ble_device {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbBleKeyType {
        BlePeerEncryptionKey = 1,
        BlePeerIdentificationKey = 2,
        BlePeerSigningKey = 4,
        BleLocalEncryptionKey = 8,
        BleLocalIdentificationKey = 16,
        BleLocalSigningKey = 32,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbSensorLocation {
        SensorLocationOther = 0,
        SensorLocationTopOfShoe = 1,
        SensorLocationInShoe = 2,
        SensorLocationHip = 3,
        SensorLocationFrontWheel = 4,
        SensorLocationLeftCrank = 5,
        SensorLocationRightCrank = 6,
        SensorLocationLeftPedal = 7,
        SensorLocationRightPedal = 8,
        SensorLocationFrontHub = 9,
        SensorLocationRearDropout = 10,
        SensorLocationChainstay = 11,
        SensorLocationRearWheel = 12,
        SensorLocationRearHub = 13,
        SensorLocationChest = 14,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbDeviceManufacturerType {
    ManufacturerPolar = 1,
    ManufacturerOther = 2,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbActivityGoalSummary {
    #[prost(float, required, tag="1")]
    pub activity_goal: f32,
    #[prost(float, required, tag="2")]
    pub achieved_activity: f32,
    #[prost(message, optional, tag="3")]
    pub time_to_go_up: ::std::option::Option<super::types::PbDuration>,
    #[prost(message, optional, tag="4")]
    pub time_to_go_walk: ::std::option::Option<super::types::PbDuration>,
    #[prost(message, optional, tag="5")]
    pub time_to_go_jog: ::std::option::Option<super::types::PbDuration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbActivityClassTimes {
    #[prost(message, required, tag="1")]
    pub time_non_wear: super::types::PbDuration,
    #[prost(message, required, tag="2")]
    pub time_sleep: super::types::PbDuration,
    #[prost(message, required, tag="3")]
    pub time_sedentary: super::types::PbDuration,
    #[prost(message, required, tag="4")]
    pub time_light_activity: super::types::PbDuration,
    #[prost(message, required, tag="5")]
    pub time_continuous_moderate: super::types::PbDuration,
    #[prost(message, required, tag="6")]
    pub time_intermittent_moderate: super::types::PbDuration,
    #[prost(message, required, tag="7")]
    pub time_continuous_vigorous: super::types::PbDuration,
    #[prost(message, required, tag="8")]
    pub time_intermittent_vigorous: super::types::PbDuration,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbDailySummary {
    #[prost(message, required, tag="1")]
    pub date: super::types::PbDate,
    #[prost(uint32, optional, tag="2")]
    pub steps: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub activity_calories: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub training_calories: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub bmr_calories: ::std::option::Option<u32>,
    #[prost(message, optional, tag="6")]
    pub activity_goal_summary: ::std::option::Option<PbActivityGoalSummary>,
    #[prost(message, optional, tag="7")]
    pub activity_class_times: ::std::option::Option<PbActivityClassTimes>,
    #[prost(float, optional, tag="8")]
    pub activity_distance: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbVersion {
    #[prost(uint32, required, tag="1")]
    pub major: u32,
    #[prost(uint32, required, tag="2")]
    pub minor: u32,
    #[prost(uint32, required, tag="3")]
    pub patch: u32,
    #[prost(string, optional, tag="4")]
    pub specifier: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbDeviceInfo {
    #[prost(message, optional, tag="1")]
    pub bootloader_version: ::std::option::Option<PbVersion>,
    #[prost(message, optional, tag="2")]
    pub platform_version: ::std::option::Option<PbVersion>,
    #[prost(message, optional, tag="3")]
    pub device_version: ::std::option::Option<PbVersion>,
    #[prost(uint32, optional, tag="4")]
    pub svn_rev: ::std::option::Option<u32>,
    #[prost(string, optional, tag="5")]
    pub electrical_serial_number: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="6")]
    pub device_id: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="7")]
    pub model_name: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="8")]
    pub hardware_code: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="9")]
    pub product_color: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="10")]
    pub product_design: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="11")]
    pub system_id: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbConstraintViolation {
    #[prost(string, required, tag="1")]
    pub value_name: std::string::String,
    #[prost(string, required, tag="2")]
    pub violation_reason: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbErrors {
    #[prost(string, required, tag="1")]
    pub message: std::string::String,
    #[prost(message, repeated, tag="2")]
    pub violations: ::std::vec::Vec<PbConstraintViolation>,
    #[prost(string, repeated, tag="3")]
    pub errors: ::std::vec::Vec<std::string::String>,
    #[prost(string, repeated, tag="4")]
    pub stack_trace: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbExerciseCounters {
    #[prost(uint32, optional, tag="1")]
    pub sprint_count: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbExerciseBase {
    #[prost(message, required, tag="1")]
    pub start: super::types::PbLocalDateTime,
    #[prost(message, required, tag="2")]
    pub duration: super::types::PbDuration,
    #[prost(message, required, tag="3")]
    pub sport: super::types::PbSportIdentifier,
    #[prost(float, optional, tag="4")]
    pub distance: ::std::option::Option<f32>,
    #[prost(uint32, optional, tag="5")]
    pub calories: ::std::option::Option<u32>,
    #[prost(message, optional, tag="6")]
    pub training_load: ::std::option::Option<super::types::PbTrainingLoad>,
    #[prost(enumeration="super::types::PbFeatureType", repeated, packed="false", tag="7")]
    pub available_sensor_features: ::std::vec::Vec<i32>,
    #[prost(message, optional, tag="9")]
    pub running_index: ::std::option::Option<super::types::PbRunningIndex>,
    #[prost(float, optional, tag="10")]
    pub ascent: ::std::option::Option<f32>,
    #[prost(float, optional, tag="11")]
    pub descent: ::std::option::Option<f32>,
    #[prost(double, optional, tag="12")]
    pub latitude: ::std::option::Option<f64>,
    #[prost(double, optional, tag="13")]
    pub longitude: ::std::option::Option<f64>,
    #[prost(string, optional, tag="14")]
    pub place: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag="16")]
    pub exercise_counters: ::std::option::Option<PbExerciseCounters>,
    #[prost(float, optional, tag="17", default="0")]
    pub speed_calibration_offset: ::std::option::Option<f32>,
    #[prost(float, optional, tag="18")]
    pub walking_distance: ::std::option::Option<f32>,
    #[prost(message, optional, tag="19")]
    pub walking_duration: ::std::option::Option<super::types::PbDuration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLapHeader {
    #[prost(message, required, tag="1")]
    pub split_time: super::types::PbDuration,
    #[prost(message, required, tag="2")]
    pub duration: super::types::PbDuration,
    #[prost(float, optional, tag="3")]
    pub distance: ::std::option::Option<f32>,
    #[prost(float, optional, tag="4")]
    pub ascent: ::std::option::Option<f32>,
    #[prost(float, optional, tag="5")]
    pub descent: ::std::option::Option<f32>,
    #[prost(enumeration="pb_lap_header::PbAutolapType", optional, tag="6")]
    pub autolap_type: ::std::option::Option<i32>,
}
pub mod pb_lap_header {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbAutolapType {
        AutolapTypeDistance = 1,
        AutolapTypeDuration = 2,
        AutolapTypeLocation = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLapSwimmingStatistics {
    #[prost(uint32, optional, tag="1")]
    pub lap_strokes: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub pool_count: ::std::option::Option<u32>,
    #[prost(float, optional, tag="3")]
    pub avg_duration_of_pool: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLapHeartRateStatistics {
    #[prost(uint32, optional, tag="1")]
    pub average: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub maximum: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub minimum: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLapSpeedStatistics {
    #[prost(float, optional, tag="1")]
    pub average: ::std::option::Option<f32>,
    #[prost(float, optional, tag="2")]
    pub maximum: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLapCadenceStatistics {
    #[prost(uint32, optional, tag="1")]
    pub average: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub maximum: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLapPowerStatistics {
    #[prost(int32, optional, tag="1")]
    pub average: ::std::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub maximum: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLapLrBalanceStatistics {
    #[prost(float, optional, tag="1")]
    pub average: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLapPedalingIndexStatistics {
    #[prost(uint32, optional, tag="1")]
    pub average: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLapPedalingEfficiencyStatistics {
    #[prost(uint32, optional, tag="1")]
    pub average: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLapInclineStatistics {
    #[prost(float, optional, tag="1")]
    pub max: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLapStrideLengthStatistics {
    #[prost(uint32, optional, tag="1")]
    pub average: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLapStatistics {
    #[prost(message, optional, tag="1")]
    pub heart_rate: ::std::option::Option<PbLapHeartRateStatistics>,
    #[prost(message, optional, tag="2")]
    pub speed: ::std::option::Option<PbLapSpeedStatistics>,
    #[prost(message, optional, tag="3")]
    pub cadence: ::std::option::Option<PbLapCadenceStatistics>,
    #[prost(message, optional, tag="4")]
    pub power: ::std::option::Option<PbLapPowerStatistics>,
    #[prost(message, optional, tag="5")]
    pub obsolete_pedaling_index: ::std::option::Option<PbLapPedalingIndexStatistics>,
    #[prost(message, optional, tag="6")]
    pub incline: ::std::option::Option<PbLapInclineStatistics>,
    #[prost(message, optional, tag="7")]
    pub stride_length: ::std::option::Option<PbLapStrideLengthStatistics>,
    #[prost(message, optional, tag="8")]
    pub swimming_statistics: ::std::option::Option<PbLapSwimmingStatistics>,
    #[prost(message, optional, tag="9")]
    pub left_right_balance: ::std::option::Option<PbLapLrBalanceStatistics>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLap {
    #[prost(message, required, tag="1")]
    pub header: PbLapHeader,
    #[prost(message, optional, tag="2")]
    pub statistics: ::std::option::Option<PbLapStatistics>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLapSummary {
    #[prost(message, optional, tag="1")]
    pub best_lap_duration: ::std::option::Option<super::types::PbDuration>,
    #[prost(message, optional, tag="2")]
    pub average_lap_duration: ::std::option::Option<super::types::PbDuration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLaps {
    #[prost(message, repeated, tag="1")]
    pub laps: ::std::vec::Vec<PbLap>,
    #[prost(message, optional, tag="2")]
    pub summary: ::std::option::Option<PbLapSummary>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbAutoLaps {
    #[prost(message, repeated, tag="1")]
    pub auto_laps: ::std::vec::Vec<PbLap>,
    #[prost(message, optional, tag="2")]
    pub summary: ::std::option::Option<PbLapSummary>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPhaseGoal {
    #[prost(enumeration="pb_phase_goal::PhaseGoalType", required, tag="1")]
    pub goal_type: i32,
    #[prost(message, optional, tag="2")]
    pub duration: ::std::option::Option<super::types::PbDuration>,
    #[prost(float, optional, tag="3")]
    pub distance: ::std::option::Option<f32>,
    #[prost(uint32, optional, tag="4")]
    pub heart_rate: ::std::option::Option<u32>,
}
pub mod pb_phase_goal {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PhaseGoalType {
        PhaseGoalOff = 0,
        PhaseGoalTime = 1,
        PhaseGoalDistance = 2,
        PhaseGoalIncreasingHr = 3,
        PhaseGoalDecreasingHr = 4,
        PhaseGoalRacePace = 5,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPhaseIntensity {
    #[prost(enumeration="pb_phase_intensity::PhaseIntensityType", required, tag="1")]
    pub intensity_type: i32,
    #[prost(message, optional, tag="2")]
    pub heart_rate_zone: ::std::option::Option<pb_phase_intensity::IntensityZone>,
    #[prost(message, optional, tag="3")]
    pub speed_zone: ::std::option::Option<pb_phase_intensity::IntensityZone>,
    #[prost(message, optional, tag="4")]
    pub power_zone: ::std::option::Option<pb_phase_intensity::IntensityZone>,
}
pub mod pb_phase_intensity {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IntensityZone {
        #[prost(uint32, required, tag="1")]
        pub lower: u32,
        #[prost(uint32, required, tag="2")]
        pub upper: u32,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PhaseIntensityType {
        PhaseIntensityFree = 0,
        PhaseIntensitySportzone = 1,
        PhaseIntensitySpeedZone = 2,
        PhaseIntensityPowerZone = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPhase {
    #[prost(message, required, tag="1")]
    pub name: super::types::PbOneLineText,
    #[prost(enumeration="pb_phase::PbPhaseChangeType", required, tag="2")]
    pub change: i32,
    #[prost(message, required, tag="3")]
    pub goal: PbPhaseGoal,
    #[prost(message, required, tag="4")]
    pub intensity: PbPhaseIntensity,
    #[prost(uint32, optional, tag="5")]
    pub repeat_count: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub jump_index: ::std::option::Option<u32>,
}
pub mod pb_phase {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbPhaseChangeType {
        ChangeManual = 0,
        ChangeAutomatic = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPhases {
    #[prost(message, repeated, tag="1")]
    pub phase: ::std::vec::Vec<PbPhase>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSwimmingStyleStatistics {
    #[prost(float, required, tag="1")]
    pub distance: f32,
    #[prost(uint32, required, tag="2")]
    pub stroke_count: u32,
    #[prost(message, optional, tag="3")]
    pub swimming_time_total: ::std::option::Option<super::types::PbDuration>,
    #[prost(uint32, optional, tag="4")]
    pub average_heartrate: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub maximum_heartrate: ::std::option::Option<u32>,
    #[prost(float, optional, tag="6")]
    pub average_swolf: ::std::option::Option<f32>,
    #[prost(message, optional, tag="7")]
    pub pool_time_min: ::std::option::Option<super::types::PbDuration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSwimmingStatistics {
    #[prost(float, required, tag="1")]
    pub swimming_distance: f32,
    #[prost(message, optional, tag="2")]
    pub freestyle_statistics: ::std::option::Option<PbSwimmingStyleStatistics>,
    #[prost(message, optional, tag="3")]
    pub backstroke_statistics: ::std::option::Option<PbSwimmingStyleStatistics>,
    #[prost(message, optional, tag="4")]
    pub breaststroke_statistics: ::std::option::Option<PbSwimmingStyleStatistics>,
    #[prost(message, optional, tag="5")]
    pub butterfly_statistics: ::std::option::Option<PbSwimmingStyleStatistics>,
    #[prost(uint32, optional, tag="6")]
    pub total_stroke_count: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub number_of_pools_swimmed: ::std::option::Option<u32>,
    #[prost(message, optional, tag="8")]
    pub swimming_pool: ::std::option::Option<super::types::PbSwimmingPoolInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbHeartRateStatistics {
    #[prost(uint32, optional, tag="1")]
    pub minimum: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub average: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub maximum: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSpeedStatistics {
    #[prost(float, optional, tag="1")]
    pub average: ::std::option::Option<f32>,
    #[prost(float, optional, tag="2")]
    pub maximum: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbCadenceStatistics {
    #[prost(uint32, optional, tag="1")]
    pub average: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub maximum: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbAltitudeStatistics {
    #[prost(float, optional, tag="1")]
    pub minimum: ::std::option::Option<f32>,
    #[prost(float, optional, tag="2")]
    pub average: ::std::option::Option<f32>,
    #[prost(float, optional, tag="3")]
    pub maximum: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPowerStatistics {
    #[prost(int32, optional, tag="1")]
    pub average: ::std::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub maximum: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbCyclingEfficiencyStatistics {
    #[prost(uint32, optional, tag="1")]
    pub average: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPedalingEfficiencyStatistics {
    #[prost(uint32, optional, tag="1")]
    pub average: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLrBalanceStatistics {
    #[prost(float, optional, tag="1")]
    pub average: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbTemperatureStatistics {
    #[prost(float, optional, tag="1")]
    pub minimum: ::std::option::Option<f32>,
    #[prost(float, optional, tag="2")]
    pub average: ::std::option::Option<f32>,
    #[prost(float, optional, tag="3")]
    pub maximum: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbStrideLengthStatistics {
    #[prost(uint32, optional, tag="1")]
    pub average: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub maximum: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbInclineStatistics {
    #[prost(float, optional, tag="1")]
    pub average: ::std::option::Option<f32>,
    #[prost(float, optional, tag="2")]
    pub maximum: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbDeclineStatistics {
    #[prost(float, optional, tag="1")]
    pub average: ::std::option::Option<f32>,
    #[prost(float, optional, tag="2")]
    pub maximum: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbActivityStatistics {
    #[prost(float, optional, tag="1")]
    pub average: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbExerciseStatistics {
    #[prost(message, optional, tag="1")]
    pub heart_rate: ::std::option::Option<PbHeartRateStatistics>,
    #[prost(message, optional, tag="2")]
    pub speed: ::std::option::Option<PbSpeedStatistics>,
    #[prost(message, optional, tag="3")]
    pub cadence: ::std::option::Option<PbCadenceStatistics>,
    #[prost(message, optional, tag="4")]
    pub altitude: ::std::option::Option<PbAltitudeStatistics>,
    #[prost(message, optional, tag="5")]
    pub power: ::std::option::Option<PbPowerStatistics>,
    #[prost(message, optional, tag="6")]
    pub left_right_balance: ::std::option::Option<PbLrBalanceStatistics>,
    #[prost(message, optional, tag="7")]
    pub temperature: ::std::option::Option<PbTemperatureStatistics>,
    #[prost(message, optional, tag="8")]
    pub activity: ::std::option::Option<PbActivityStatistics>,
    #[prost(message, optional, tag="9")]
    pub stride_length: ::std::option::Option<PbStrideLengthStatistics>,
    #[prost(message, optional, tag="10")]
    pub incline: ::std::option::Option<PbInclineStatistics>,
    #[prost(message, optional, tag="11")]
    pub decline: ::std::option::Option<PbDeclineStatistics>,
    #[prost(message, optional, tag="12")]
    pub swimming: ::std::option::Option<PbSwimmingStatistics>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPhaseHeartRateStatistics {
    #[prost(uint32, optional, tag="1")]
    pub average: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub maximum: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPhaseStrideLengthStatistics {
    #[prost(uint32, optional, tag="1")]
    pub average: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPhaseRepetition {
    #[prost(uint32, required, tag="1")]
    pub index: u32,
    #[prost(message, required, tag="2")]
    pub split_time: super::types::PbDuration,
    #[prost(message, required, tag="3")]
    pub duration: super::types::PbDuration,
    #[prost(bool, optional, tag="4")]
    pub phase_finished: ::std::option::Option<bool>,
    #[prost(float, optional, tag="5")]
    pub split_distance: ::std::option::Option<f32>,
    #[prost(float, optional, tag="6")]
    pub distance: ::std::option::Option<f32>,
    #[prost(message, optional, tag="7")]
    pub in_target_zone: ::std::option::Option<super::types::PbDuration>,
    #[prost(message, optional, tag="8")]
    pub heart_rate: ::std::option::Option<PbPhaseHeartRateStatistics>,
    #[prost(message, optional, tag="9")]
    pub speed: ::std::option::Option<PbSpeedStatistics>,
    #[prost(message, optional, tag="10")]
    pub cadence: ::std::option::Option<PbCadenceStatistics>,
    #[prost(message, optional, tag="11")]
    pub power: ::std::option::Option<PbPowerStatistics>,
    #[prost(message, optional, tag="12")]
    pub left_right_balance: ::std::option::Option<PbLrBalanceStatistics>,
    #[prost(message, optional, tag="13")]
    pub stride_length: ::std::option::Option<PbPhaseStrideLengthStatistics>,
    #[prost(uint32, optional, tag="14")]
    pub stroke_count: ::std::option::Option<u32>,
    #[prost(float, optional, tag="15")]
    pub average_swolf: ::std::option::Option<f32>,
    #[prost(uint32, optional, tag="16")]
    pub strokes_per_min: ::std::option::Option<u32>,
    #[prost(float, optional, tag="17")]
    pub ascent: ::std::option::Option<f32>,
    #[prost(float, optional, tag="18")]
    pub descent: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPhaseRepetitions {
    #[prost(message, repeated, tag="1")]
    pub phase: ::std::vec::Vec<PbPhaseRepetition>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbExerciseRouteSamples {
    #[prost(uint32, repeated, tag="1")]
    pub duration: ::std::vec::Vec<u32>,
    #[prost(double, repeated, tag="2")]
    pub latitude: ::std::vec::Vec<f64>,
    #[prost(double, repeated, tag="3")]
    pub longitude: ::std::vec::Vec<f64>,
    #[prost(sint32, repeated, tag="4")]
    pub gps_altitude: ::std::vec::Vec<i32>,
    #[prost(uint32, repeated, tag="5")]
    pub satellite_amount: ::std::vec::Vec<u32>,
    #[prost(bool, repeated, tag="6")]
    pub obsolete_fix: ::std::vec::Vec<bool>,
    #[prost(message, repeated, tag="7")]
    pub obsolete_gps_offline: ::std::vec::Vec<super::types::PbSensorOffline>,
    #[prost(message, repeated, tag="8")]
    pub obsolete_gps_date_time: ::std::vec::Vec<super::types::PbSystemDateTime>,
    #[prost(message, optional, tag="9")]
    pub first_location_time: ::std::option::Option<super::types::PbSystemDateTime>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbRrOffline {
    #[prost(message, required, tag="1")]
    pub start_time: super::types::PbDuration,
    #[prost(message, required, tag="2")]
    pub time_interval: super::types::PbDuration,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbExerciseRrIntervals {
    #[prost(uint32, repeated, tag="1")]
    pub rr_intervals: ::std::vec::Vec<u32>,
    #[prost(message, repeated, tag="2")]
    pub rr_sensor_offline: ::std::vec::Vec<PbRrOffline>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPowerMeasurements {
    #[prost(int32, required, tag="1")]
    pub current_power: i32,
    #[prost(uint32, optional, tag="2")]
    pub cumulative_crank_revolutions: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub cumulative_timestamp: ::std::option::Option<u32>,
    #[prost(sint32, optional, tag="4")]
    pub force_magnitude_min: ::std::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub force_magnitude_max: ::std::option::Option<i32>,
    #[prost(uint32, optional, tag="6")]
    pub force_magnitude_min_angle: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub force_magnitude_max_angle: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="8")]
    pub bottom_dead_spot_angle: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub top_dead_spot_angle: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbCalibrationValue {
    #[prost(uint32, required, tag="1")]
    pub start_index: u32,
    #[prost(float, required, tag="2")]
    pub value: f32,
    #[prost(enumeration="super::types::PbOperationType", required, tag="3")]
    pub operation: i32,
    #[prost(enumeration="super::types::PbMovingType", optional, tag="4")]
    pub cause: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbExerciseSamples {
    #[prost(message, required, tag="1")]
    pub recording_interval: super::types::PbDuration,
    #[prost(uint32, repeated, tag="2")]
    pub heart_rate_samples: ::std::vec::Vec<u32>,
    #[prost(message, repeated, tag="3")]
    pub heart_rate_offline: ::std::vec::Vec<super::types::PbSensorOffline>,
    #[prost(uint32, repeated, tag="4")]
    pub cadence_samples: ::std::vec::Vec<u32>,
    #[prost(message, repeated, tag="5")]
    pub cadence_offline: ::std::vec::Vec<super::types::PbSensorOffline>,
    #[prost(float, repeated, tag="6")]
    pub altitude_samples: ::std::vec::Vec<f32>,
    #[prost(message, repeated, tag="18")]
    pub altitude_offline: ::std::vec::Vec<super::types::PbSensorOffline>,
    #[prost(message, repeated, tag="7")]
    pub altitude_calibration: ::std::vec::Vec<PbCalibrationValue>,
    #[prost(float, repeated, tag="8")]
    pub temperature_samples: ::std::vec::Vec<f32>,
    #[prost(message, repeated, tag="19")]
    pub temperature_offline: ::std::vec::Vec<super::types::PbSensorOffline>,
    #[prost(float, repeated, tag="9")]
    pub speed_samples: ::std::vec::Vec<f32>,
    #[prost(message, repeated, tag="10")]
    pub speed_offline: ::std::vec::Vec<super::types::PbSensorOffline>,
    #[prost(float, repeated, tag="11")]
    pub distance_samples: ::std::vec::Vec<f32>,
    #[prost(message, repeated, tag="12")]
    pub distance_offline: ::std::vec::Vec<super::types::PbSensorOffline>,
    #[prost(uint32, repeated, packed="false", tag="13")]
    pub stride_length_samples: ::std::vec::Vec<u32>,
    #[prost(message, repeated, tag="14")]
    pub stride_length_offline: ::std::vec::Vec<super::types::PbSensorOffline>,
    #[prost(message, repeated, tag="15")]
    pub stride_calibration: ::std::vec::Vec<PbCalibrationValue>,
    #[prost(float, repeated, packed="false", tag="16")]
    pub forward_acceleration: ::std::vec::Vec<f32>,
    #[prost(message, repeated, tag="20")]
    pub forward_acceleration_offline: ::std::vec::Vec<super::types::PbSensorOffline>,
    #[prost(enumeration="super::types::PbMovingType", repeated, packed="false", tag="17")]
    pub moving_type_samples: ::std::vec::Vec<i32>,
    #[prost(message, repeated, tag="21")]
    pub moving_type_offline: ::std::vec::Vec<super::types::PbSensorOffline>,
    #[prost(message, repeated, tag="22")]
    pub left_pedal_power_samples: ::std::vec::Vec<PbPowerMeasurements>,
    #[prost(message, repeated, tag="23")]
    pub left_pedal_power_offline: ::std::vec::Vec<super::types::PbSensorOffline>,
    #[prost(message, repeated, tag="24")]
    pub right_pedal_power_samples: ::std::vec::Vec<PbPowerMeasurements>,
    #[prost(message, repeated, tag="25")]
    pub right_pedal_power_offline: ::std::vec::Vec<super::types::PbSensorOffline>,
    #[prost(message, repeated, tag="26")]
    pub left_power_calibration: ::std::vec::Vec<PbCalibrationValue>,
    #[prost(message, repeated, tag="27")]
    pub right_power_calibration: ::std::vec::Vec<PbCalibrationValue>,
    #[prost(message, optional, tag="28")]
    pub rr_samples: ::std::option::Option<PbExerciseRrIntervals>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbExerciseSensor {
    #[prost(message, required, tag="1")]
    pub mac: super::types::PbBleMac,
    #[prost(message, optional, tag="2")]
    pub device_id: ::std::option::Option<super::types::PbDeviceId>,
    #[prost(message, optional, tag="3")]
    pub device_name: ::std::option::Option<super::types::PbBleDeviceName>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbExerciseSensors {
    #[prost(message, repeated, tag="1")]
    pub sensors: ::std::vec::Vec<PbExerciseSensor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSteadyRacePace {
    #[prost(message, required, tag="1")]
    pub duration: super::types::PbDuration,
    #[prost(float, required, tag="2")]
    pub distance: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbExerciseTarget {
    #[prost(enumeration="super::types::PbExerciseTargetType", required, tag="1")]
    pub target_type: i32,
    #[prost(message, optional, tag="2")]
    pub sport_id: ::std::option::Option<super::types::PbSportIdentifier>,
    #[prost(message, optional, tag="3")]
    pub volume_target: ::std::option::Option<super::types::PbVolumeTarget>,
    #[prost(message, optional, tag="4")]
    pub phases: ::std::option::Option<PbPhases>,
    #[prost(message, optional, tag="5")]
    pub route: ::std::option::Option<super::types::PbRouteId>,
    #[prost(message, optional, tag="6")]
    pub steady_race_pace: ::std::option::Option<PbSteadyRacePace>,
    #[prost(message, optional, tag="7")]
    pub strava_segment_target: ::std::option::Option<super::types::PbStravaSegmentTarget>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbTrainingSessionTarget {
    #[prost(message, required, tag="2")]
    pub name: super::types::PbOneLineText,
    #[prost(message, optional, tag="3")]
    pub sport_id: ::std::option::Option<super::types::PbSportIdentifier>,
    #[prost(message, optional, tag="4")]
    pub start_time: ::std::option::Option<super::types::PbLocalDateTime>,
    #[prost(message, optional, tag="5")]
    pub description: ::std::option::Option<super::types::PbMultiLineText>,
    #[prost(message, repeated, tag="6")]
    pub exercise_target: ::std::vec::Vec<PbExerciseTarget>,
    #[prost(bool, optional, tag="7")]
    pub target_done: ::std::option::Option<bool>,
    #[prost(message, optional, tag="8")]
    pub duration: ::std::option::Option<super::types::PbDuration>,
    #[prost(message, optional, tag="9")]
    pub training_program_id: ::std::option::Option<super::types::PbTrainingProgramId>,
    #[prost(message, optional, tag="10")]
    pub event_id: ::std::option::Option<super::types::PbEventId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSteadyRacePaceResult {
    #[prost(message, required, tag="1")]
    pub completed_time: super::types::PbDuration,
    #[prost(uint32, optional, tag="2")]
    pub average_heartrate: ::std::option::Option<u32>,
    #[prost(float, optional, tag="3")]
    pub average_speed: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbExerciseTargetInfo {
    #[prost(enumeration="super::types::PbExerciseTargetType", required, tag="1")]
    pub target_type: i32,
    #[prost(uint32, required, tag="2")]
    pub index: u32,
    #[prost(message, optional, tag="3")]
    pub name: ::std::option::Option<super::types::PbOneLineText>,
    #[prost(bool, optional, tag="4")]
    pub target_reached: ::std::option::Option<bool>,
    #[prost(message, optional, tag="5")]
    pub end_time: ::std::option::Option<super::types::PbDuration>,
    #[prost(message, optional, tag="6")]
    pub sport_id: ::std::option::Option<super::types::PbSportIdentifier>,
    #[prost(message, optional, tag="7")]
    pub volume_target: ::std::option::Option<super::types::PbVolumeTarget>,
    #[prost(message, optional, tag="8")]
    pub phases: ::std::option::Option<PbPhases>,
    #[prost(message, optional, tag="9")]
    pub route: ::std::option::Option<super::types::PbRouteId>,
    #[prost(message, optional, tag="10")]
    pub steady_race_pace: ::std::option::Option<PbSteadyRacePace>,
    #[prost(message, optional, tag="11")]
    pub steady_race_pace_result: ::std::option::Option<PbSteadyRacePaceResult>,
    #[prost(message, optional, tag="12")]
    pub strava_segment_target: ::std::option::Option<super::types::PbStravaSegmentTarget>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbRecordedHeartRateZone {
    #[prost(message, required, tag="1")]
    pub zone_limits: super::types::PbHeartRateZone,
    #[prost(message, required, tag="2")]
    pub in_zone: super::types::PbDuration,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbRecordedPowerZone {
    #[prost(message, required, tag="1")]
    pub zone_limits: super::types::PbPowerZone,
    #[prost(message, required, tag="2")]
    pub in_zone: super::types::PbDuration,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbRecordedFatFitZones {
    #[prost(uint32, required, tag="1")]
    pub fatfit_limit: u32,
    #[prost(message, required, tag="2")]
    pub fat_time: super::types::PbDuration,
    #[prost(message, required, tag="3")]
    pub fit_time: super::types::PbDuration,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbRecordedSpeedZone {
    #[prost(message, required, tag="1")]
    pub zone_limits: super::types::PbSpeedZone,
    #[prost(message, optional, tag="2")]
    pub time_in_zone: ::std::option::Option<super::types::PbDuration>,
    #[prost(float, optional, tag="3")]
    pub distance_in_zone: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbRecordedZones {
    #[prost(message, repeated, tag="1")]
    pub heart_rate_zone: ::std::vec::Vec<PbRecordedHeartRateZone>,
    #[prost(message, repeated, tag="2")]
    pub power_zone: ::std::vec::Vec<PbRecordedPowerZone>,
    #[prost(message, optional, tag="3")]
    pub fatfit_zones: ::std::option::Option<PbRecordedFatFitZones>,
    #[prost(message, repeated, tag="4")]
    pub speed_zone: ::std::vec::Vec<PbRecordedSpeedZone>,
    #[prost(enumeration="super::types::PbHeartRateZoneSettingSource", optional, tag="10")]
    pub heart_rate_setting_source: ::std::option::Option<i32>,
    #[prost(enumeration="super::types::PbPowerZoneSettingSource", optional, tag="11")]
    pub power_setting_source: ::std::option::Option<i32>,
    #[prost(enumeration="super::types::PbSpeedZoneSettingSource", optional, tag="12")]
    pub speed_setting_source: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbFitnessTestResult {
    #[prost(message, required, tag="1")]
    pub start_time: super::types::PbLocalDateTime,
    #[prost(uint32, required, tag="2")]
    pub fitness: u32,
    /// some type or level, te be confirmed. 7 would be "Elite" 
    #[prost(uint32, required, tag="3")]
    pub unknonw: u32,
    /// not sure -- to be confirmed 
    #[prost(uint32, required, tag="4")]
    pub hr_avg: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbGpsAlmanacInfo {
    #[prost(message, required, tag="1")]
    pub end_time: super::types::PbSystemDateTime,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbIdentifier {
    #[prost(uint64, required, tag="1")]
    pub ecosystem_id: u64,
    #[prost(message, required, tag="2")]
    pub created: super::types::PbSystemDateTime,
    #[prost(message, required, tag="3")]
    pub last_modified: super::types::PbSystemDateTime,
    #[prost(bool, optional, tag="4")]
    pub deleted: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbJump {
    #[prost(message, required, tag="1")]
    pub flight_time: super::types::PbDuration,
    #[prost(message, optional, tag="2")]
    pub contact_time: ::std::option::Option<super::types::PbDuration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbJumpTest {
    #[prost(enumeration="pb_jump_test::PbJumpTestType", required, tag="1")]
    pub test_type: i32,
    #[prost(message, required, tag="2")]
    pub start_time: super::types::PbLocalDateTime,
    #[prost(message, repeated, tag="3")]
    pub jump: ::std::vec::Vec<PbJump>,
    #[prost(message, optional, tag="4")]
    pub cont_jump_duration: ::std::option::Option<super::types::PbDuration>,
}
pub mod pb_jump_test {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbJumpTestType {
        JumpTestTypeSquat = 0,
        JumpTestTypeCounter = 1,
        JumpTestTypeContinuous = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbMapLocation {
    #[prost(double, required, tag="1")]
    pub latitude: f64,
    #[prost(double, required, tag="2")]
    pub longitude: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbMapInformation {
    #[prost(message, required, tag="1")]
    pub centre_point: PbMapLocation,
    #[prost(message, optional, tag="2")]
    pub data_timestamp: ::std::option::Option<super::types::PbSystemDateTime>,
    #[prost(bool, optional, tag="3", default="false")]
    pub updated: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NanoPbOptions {
    #[prost(int32, optional, tag="1")]
    pub max_size: ::std::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub max_count: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbOrthostaticTestResult {
    #[prost(message, required, tag="1")]
    pub start_time: super::types::PbLocalDateTime,
    #[prost(message, required, tag="2")]
    pub reset_time: super::types::PbLocalDateTime,
    #[prost(uint32, required, tag="3")]
    pub rr_avg_supine: u32,
    #[prost(uint32, required, tag="4")]
    pub rr_long_term_avg_of_supine: u32,
    #[prost(uint32, required, tag="5")]
    pub rr_min_after_standup: u32,
    #[prost(uint32, required, tag="6")]
    pub rr_long_term_avg_of_min_after_standup: u32,
    #[prost(uint32, required, tag="7")]
    pub rr_avg_stand: u32,
    #[prost(uint32, required, tag="8")]
    pub rr_long_term_avg_of_stand: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPersonalBest {
    #[prost(float, optional, tag="1")]
    pub distance: ::std::option::Option<f32>,
    #[prost(float, optional, tag="2")]
    pub average_speed: ::std::option::Option<f32>,
    #[prost(uint32, optional, tag="3")]
    pub calories: ::std::option::Option<u32>,
    #[prost(float, optional, tag="4")]
    pub ascent: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbStravaSegmentPort {
    #[prost(message, required, tag="1")]
    pub left_location: super::types::PbLocation,
    #[prost(message, required, tag="2")]
    pub right_location: super::types::PbLocation,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbRoutePoint {
    #[prost(sint32, required, tag="1")]
    pub x_offset: i32,
    #[prost(sint32, required, tag="2")]
    pub y_offset: i32,
    #[prost(uint32, optional, tag="3")]
    pub time_offset: ::std::option::Option<u32>,
    #[prost(sint32, optional, tag="4")]
    pub z_offset: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPlannedRoute {
    #[prost(message, required, tag="1")]
    pub route_id: super::types::PbRouteId,
    #[prost(message, required, tag="2")]
    pub name: super::types::PbOneLineText,
    #[prost(float, optional, tag="3")]
    pub length: ::std::option::Option<f32>,
    #[prost(message, optional, tag="4")]
    pub start_location: ::std::option::Option<super::types::PbLocation>,
    #[prost(float, optional, tag="5")]
    pub start_altitude: ::std::option::Option<f32>,
    #[prost(message, repeated, tag="6")]
    pub point: ::std::vec::Vec<PbRoutePoint>,
    #[prost(message, optional, tag="7")]
    pub segment_start_port: ::std::option::Option<PbStravaSegmentPort>,
    #[prost(message, optional, tag="8")]
    pub segment_end_port: ::std::option::Option<PbStravaSegmentPort>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPointOfInterest {
    #[prost(message, required, tag="1")]
    pub location: super::types::PbLocation,
    #[prost(uint64, optional, tag="2")]
    pub point_id: ::std::option::Option<u64>,
    #[prost(message, optional, tag="3")]
    pub name: ::std::option::Option<super::types::PbMultiLineText>,
    #[prost(bool, optional, tag="4")]
    pub alarm: ::std::option::Option<bool>,
    #[prost(message, optional, tag="100")]
    pub created: ::std::option::Option<super::types::PbSystemDateTime>,
    #[prost(message, optional, tag="101")]
    pub last_modified: ::std::option::Option<super::types::PbSystemDateTime>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPointOfInterests {
    #[prost(message, repeated, tag="1")]
    pub point: ::std::vec::Vec<PbPointOfInterest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbRecoveryTimes {
    #[prost(message, required, tag="1")]
    pub start_of_times: super::types::PbLocalDateTime,
    #[prost(float, repeated, tag="2")]
    pub recovery_times: ::std::vec::Vec<f32>,
    #[prost(float, optional, tag="3")]
    pub end_glycogen_left_percent: ::std::option::Option<f32>,
    #[prost(float, optional, tag="4")]
    pub end_carbo_consumption: ::std::option::Option<f32>,
    #[prost(float, optional, tag="5")]
    pub end_protein_consumption: ::std::option::Option<f32>,
    #[prost(float, optional, tag="6")]
    pub end_cumulative_mechanical_stimulus: ::std::option::Option<f32>,
    #[prost(float, optional, tag="7")]
    pub last_half_hour_avg_met: ::std::option::Option<f32>,
    #[prost(float, optional, tag="8")]
    pub exercise_calories: ::std::option::Option<f32>,
    #[prost(float, optional, tag="9")]
    pub activity_calories: ::std::option::Option<f32>,
    #[prost(float, optional, tag="10")]
    pub bmr_calories: ::std::option::Option<f32>,
    #[prost(uint32, optional, tag="11")]
    pub steps: ::std::option::Option<u32>,
    #[prost(float, optional, tag="12")]
    pub accumulated_activity: ::std::option::Option<f32>,
    #[prost(uint32, optional, tag="13")]
    pub number_of_exercise_half_hours: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbRrRecordingTestResult {
    #[prost(message, required, tag="1")]
    pub start_time: super::types::PbLocalDateTime,
    #[prost(message, required, tag="2")]
    pub end_time: super::types::PbLocalDateTime,
    #[prost(uint32, required, tag="3")]
    pub hr_avg: u32,
    #[prost(uint32, required, tag="4")]
    pub hr_min: u32,
    #[prost(uint32, required, tag="5")]
    pub hr_max: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSportTranslation {
    #[prost(message, required, tag="1")]
    pub id: super::types::PbLanguageId,
    #[prost(message, required, tag="2")]
    pub text: super::types::PbOneLineText,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSport {
    #[prost(message, required, tag="1")]
    pub identifier: super::types::PbSportIdentifier,
    #[prost(message, required, tag="2")]
    pub parent_identifier: super::types::PbSportIdentifier,
    #[prost(message, repeated, tag="3")]
    pub translation: ::std::vec::Vec<PbSportTranslation>,
    #[prost(float, optional, tag="4")]
    pub factor: ::std::option::Option<f32>,
    #[prost(message, repeated, tag="5")]
    pub stages: ::std::vec::Vec<super::types::PbSportIdentifier>,
    #[prost(enumeration="pb_sport::PbSportType", optional, tag="6", default="SportTypeSingleSport")]
    pub sport_type: ::std::option::Option<i32>,
    #[prost(bool, optional, tag="7", default="false")]
    pub speed_zones_enabled: ::std::option::Option<bool>,
    #[prost(message, optional, tag="100")]
    pub created: ::std::option::Option<super::types::PbSystemDateTime>,
    #[prost(message, optional, tag="101")]
    pub last_modified: ::std::option::Option<super::types::PbSystemDateTime>,
}
pub mod pb_sport {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbSportType {
        SportTypeSingleSport = 1,
        SportTypeMultiSport = 2,
        SportTypeSubSport = 3,
        SportTypeFreeMultiSport = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbAceSportProfileSettings {
    #[prost(enumeration="pb_ace_sport_profile_settings::PbHeartTouch", optional, tag="1")]
    pub heart_touch: ::std::option::Option<i32>,
    #[prost(bool, optional, tag="4")]
    pub auto_start: ::std::option::Option<bool>,
    #[prost(message, optional, tag="6")]
    pub stride_sensor_calib_settings: ::std::option::Option<super::types::PbStrideSensorCalibSettings>,
}
pub mod pb_ace_sport_profile_settings {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbHeartTouch {
        HeartTouchOff = 1,
        HeartTouchActivateBacklight = 2,
        HeartTouchShowPreviousLap = 3,
        HeartTouchShowTimeOfDay = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbArcherSportProfileSettings {
    #[prost(enumeration="pb_archer_sport_profile_settings::PbHeartTouch", optional, tag="1")]
    pub heart_touch: ::std::option::Option<i32>,
    #[prost(bool, optional, tag="4")]
    pub auto_start: ::std::option::Option<bool>,
}
pub mod pb_archer_sport_profile_settings {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbHeartTouch {
        HeartTouchOff = 1,
        HeartTouchActivateBacklight = 2,
        HeartTouchShowPreviousLap = 3,
        HeartTouchShowTimeOfDay = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbAstraSportProfileSettings {
    #[prost(bool, optional, tag="3")]
    pub vibration: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbAvalonSportProfileSettings {
    #[prost(enumeration="pb_avalon_sport_profile_settings::PbHeartTouch", optional, tag="1")]
    pub heart_touch: ::std::option::Option<i32>,
    #[prost(bool, optional, tag="3")]
    pub vibration: ::std::option::Option<bool>,
    #[prost(bool, optional, tag="4")]
    pub auto_start: ::std::option::Option<bool>,
}
pub mod pb_avalon_sport_profile_settings {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbHeartTouch {
        HeartTouchOff = 1,
        HeartTouchActivateBacklight = 2,
        HeartTouchShowPreviousLap = 3,
        HeartTouchShowTimeOfDay = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbGuitarSportProfileSettings {
    #[prost(enumeration="pb_guitar_sport_profile_settings::PbHeartTouch", optional, tag="1")]
    pub heart_touch: ::std::option::Option<i32>,
    #[prost(enumeration="pb_guitar_sport_profile_settings::PbTapButtonAction", optional, tag="2")]
    pub tap_button_action: ::std::option::Option<i32>,
    #[prost(bool, optional, tag="3")]
    pub vibration: ::std::option::Option<bool>,
    #[prost(bool, optional, tag="4")]
    pub auto_start: ::std::option::Option<bool>,
    #[prost(bool, optional, tag="5")]
    pub auto_scrolling: ::std::option::Option<bool>,
    #[prost(message, optional, tag="6")]
    pub stride_sensor_calib_settings: ::std::option::Option<super::types::PbStrideSensorCalibSettings>,
    #[prost(uint32, optional, tag="7")]
    pub sprint_display_activation: ::std::option::Option<u32>,
    #[prost(enumeration="pb_guitar_sport_profile_settings::PbSportTapButtonSensitivity", optional, tag="8", default="SportTapButtonSensitivityMedium")]
    pub sport_tap_button_sensitivity: ::std::option::Option<i32>,
    #[prost(message, optional, tag="9")]
    pub swimming_pool: ::std::option::Option<super::types::PbSwimmingPoolInfo>,
}
pub mod pb_guitar_sport_profile_settings {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbHeartTouch {
        HeartTouchOff = 1,
        HeartTouchActivateBacklight = 2,
        HeartTouchShowPreviousLap = 3,
        HeartTouchShowTimeOfDay = 4,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbTapButtonAction {
        TapButtonOff = 1,
        TapButtonTakeLap = 2,
        TapButtonChangeTrainingView = 3,
        TapButtonActivateBacklight = 4,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbSportTapButtonSensitivity {
        SportTapButtonSensitivityOff = 1,
        SportTapButtonSensitivityVeryLow = 5,
        SportTapButtonSensitivityLow = 2,
        SportTapButtonSensitivityMedium = 3,
        SportTapButtonSensitivityHigh = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbMaseratiSportProfileSettings {
    #[prost(enumeration="pb_maserati_sport_profile_settings::PbHeartTouch", optional, tag="1")]
    pub heart_touch: ::std::option::Option<i32>,
    #[prost(enumeration="pb_maserati_sport_profile_settings::PbTapButtonAction", optional, tag="2")]
    pub tap_button_action: ::std::option::Option<i32>,
    #[prost(bool, optional, tag="3")]
    pub vibration: ::std::option::Option<bool>,
    #[prost(bool, optional, tag="4")]
    pub auto_start: ::std::option::Option<bool>,
}
pub mod pb_maserati_sport_profile_settings {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbHeartTouch {
        HeartTouchOff = 1,
        HeartTouchActivateBacklight = 2,
        HeartTouchShowPreviousLap = 3,
        HeartTouchShowTimeOfDay = 4,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbTapButtonAction {
        TapButtonOff = 1,
        TapButtonTakeLap = 2,
        TapButtonChangeTrainingView = 3,
        TapButtonActivateBacklight = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbMcLarenSportProfileSettings {
    #[prost(bool, required, tag="4")]
    pub auto_start: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSirius2TrainingDisplay {
    #[prost(enumeration="PbTrainingDisplayItem", repeated, packed="false", tag="1")]
    pub item: ::std::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSirius2DisplaySettings {
    #[prost(message, repeated, tag="1")]
    pub display: ::std::vec::Vec<PbSirius2TrainingDisplay>,
    #[prost(uint32, optional, tag="2")]
    pub last_shown_display: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub added_default_displays: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbAutoLapSettings {
    #[prost(enumeration="pb_auto_lap_settings::PbAutomaticLap", required, tag="1")]
    pub automatic_lap: i32,
    #[prost(float, optional, tag="2")]
    pub automatic_lap_distance: ::std::option::Option<f32>,
    #[prost(message, optional, tag="3")]
    pub automatic_lap_duration: ::std::option::Option<super::types::PbDuration>,
}
pub mod pb_auto_lap_settings {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbAutomaticLap {
        AutomaticLapOff = 1,
        AutomaticLapDistance = 2,
        AutomaticLapDuration = 3,
        AutomaticLapLocation = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbTrainingReminder {
    #[prost(enumeration="pb_training_reminder::PbTrainingReminderType", required, tag="1")]
    pub reminder_type: i32,
    #[prost(message, optional, tag="2")]
    pub reminder_text: ::std::option::Option<super::types::PbOneLineText>,
    #[prost(uint32, optional, tag="3")]
    pub calorie_reminder_value: ::std::option::Option<u32>,
    #[prost(message, optional, tag="4")]
    pub time_reminder_value: ::std::option::Option<super::types::PbDuration>,
    #[prost(float, optional, tag="5")]
    pub distance_reminder_value: ::std::option::Option<f32>,
}
pub mod pb_training_reminder {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbTrainingReminderType {
        TrainingReminderOff = 1,
        TrainingReminderCaloriesBased = 2,
        TrainingReminderDistanceBased = 3,
        TrainingReminderTimeBased = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSportProfileSettings {
    #[prost(message, optional, tag="1")]
    pub volume: ::std::option::Option<super::types::PbVolume>,
    #[prost(enumeration="pb_sport_profile_settings::PbSpeedView", optional, tag="2")]
    pub speed_view: ::std::option::Option<i32>,
    #[prost(enumeration="pb_sport_profile_settings::PbZoneOptimizerSetting", optional, tag="3")]
    pub zone_optimizer_setting: ::std::option::Option<i32>,
    #[prost(enumeration="super::types::PbHeartRateView", optional, tag="4")]
    pub heart_rate_view: ::std::option::Option<i32>,
    #[prost(bool, optional, tag="5")]
    pub sensor_broadcasting_hr: ::std::option::Option<bool>,
    #[prost(message, optional, tag="6")]
    pub zone_limits: ::std::option::Option<super::types::PbZones>,
    #[prost(message, optional, tag="7")]
    pub training_reminder: ::std::option::Option<PbTrainingReminder>,
    #[prost(bool, optional, tag="8")]
    pub voice_guidance: ::std::option::Option<bool>,
    #[prost(enumeration="pb_sport_profile_settings::PbGpsSetting", optional, tag="9")]
    pub gps_setting: ::std::option::Option<i32>,
    #[prost(message, optional, tag="10")]
    pub autolap_settings: ::std::option::Option<PbAutoLapSettings>,
    #[prost(enumeration="pb_sport_profile_settings::PbAltitudeSetting", optional, tag="11")]
    pub altitude_setting: ::std::option::Option<i32>,
    #[prost(enumeration="pb_sport_profile_settings::PbPowerView", optional, tag="12")]
    pub power_view: ::std::option::Option<i32>,
    #[prost(enumeration="pb_sport_profile_settings::PbStrideSpeedSource", optional, tag="13", default="StrideSpeedSourceStride")]
    pub stride_speed_source: ::std::option::Option<i32>,
    #[prost(enumeration="pb_sport_profile_settings::PbRemoteButtonAction", repeated, packed="false", tag="14")]
    pub remote_button_actions: ::std::vec::Vec<i32>,
    #[prost(bool, optional, tag="15")]
    pub hr_zone_lock_available: ::std::option::Option<bool>,
    #[prost(bool, optional, tag="16")]
    pub speed_zone_lock_available: ::std::option::Option<bool>,
    #[prost(bool, optional, tag="17")]
    pub power_zone_lock_available: ::std::option::Option<bool>,
    #[prost(enumeration="pb_sport_profile_settings::PbSwimmingUnits", optional, tag="18")]
    pub swimming_units: ::std::option::Option<i32>,
}
pub mod pb_sport_profile_settings {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbSpeedView {
        SpeedViewPace = 1,
        SpeedViewSpeed = 2,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbZoneOptimizerSetting {
        ZoneoptimizerOff = 1,
        ZoneoptimizerModifiedOff = 2,
        ZoneoptimizerDefault = 3,
        ZoneoptimizerModified = 4,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbGpsSetting {
        GpsOff = 0,
        GpsOnNormal = 1,
        GpsOnLong = 2,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbAltitudeSetting {
        AltitudeOff = 0,
        AltitudeOn = 1,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbPowerView {
        PowerViewWatt = 1,
        PowerViewWattPerKg = 2,
        PowerViewFtpPercent = 3,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbStrideSpeedSource {
        StrideSpeedSourceStride = 1,
        StrideSpeedSourceGps = 2,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbRemoteButtonAction {
        RemoteButtonRingBell = 1,
        RemoteButtonActivateBacklight = 2,
        RemoteButtonChangeTrainingView = 3,
        RemoteButtonTakeLap = 4,
        RemoteButtonActivateSafetyLight = 5,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbSwimmingUnits {
        SwimmingMeters = 0,
        SwimmingYards = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbAutoPause {
    #[prost(enumeration="pb_auto_pause::PbAutoPauseTrigger", required, tag="1")]
    pub trigger: i32,
    #[prost(float, optional, tag="2")]
    pub speed_threshold: ::std::option::Option<f32>,
}
pub mod pb_auto_pause {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbAutoPauseTrigger {
        AutoPauseOff = 0,
        AutoPauseTriggerSpeed = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSportProfile {
    #[prost(uint64, optional, tag="1")]
    pub identifier: ::std::option::Option<u64>,
    #[prost(message, required, tag="2")]
    pub sport_identifier: super::types::PbSportIdentifier,
    #[prost(message, optional, tag="3")]
    pub settings: ::std::option::Option<PbSportProfileSettings>,
    #[prost(message, optional, tag="4")]
    pub sirius2_display_settings: ::std::option::Option<PbSirius2DisplaySettings>,
    #[prost(float, optional, tag="5")]
    pub sport_factor: ::std::option::Option<f32>,
    #[prost(uint32, optional, tag="6")]
    pub aerobic_threshold: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub anaerobic_threshold: ::std::option::Option<u32>,
    #[prost(message, required, tag="8")]
    pub last_modified: super::types::PbSystemDateTime,
    #[prost(float, optional, tag="9")]
    pub sprint_threshold: ::std::option::Option<f32>,
    #[prost(message, optional, tag="10")]
    pub auto_pause: ::std::option::Option<PbAutoPause>,
    #[prost(message, optional, tag="200")]
    pub guitar_settings: ::std::option::Option<PbGuitarSportProfileSettings>,
    #[prost(message, optional, tag="201")]
    pub mclaren_settings: ::std::option::Option<PbMcLarenSportProfileSettings>,
    #[prost(message, optional, tag="202")]
    pub ace_settings: ::std::option::Option<PbAceSportProfileSettings>,
    #[prost(message, optional, tag="203")]
    pub avalon_settings: ::std::option::Option<PbAvalonSportProfileSettings>,
    #[prost(message, optional, tag="204")]
    pub archer_settings: ::std::option::Option<PbArcherSportProfileSettings>,
    #[prost(message, optional, tag="205")]
    pub astra_settings: ::std::option::Option<PbAstraSportProfileSettings>,
    #[prost(message, optional, tag="206")]
    pub maserati_settings: ::std::option::Option<PbMaseratiSportProfileSettings>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbTrainingDisplayItem {
    TimeOfDay = 2,
    Stopwatch = 3,
    CurrentLapTime = 6,
    LastLapTime = 7,
    LastAutomaticLapTime = 8,
    Altitude = 10,
    Ascent = 11,
    Descent = 12,
    Inclinometer = 13,
    Temperature = 15,
    CurrentLapAscent = 16,
    CurrentLapDescent = 17,
    CurrentLapVam = 18,
    CurrentHeartRate = 20,
    AverageHeartRate = 21,
    MaximumHeartRate = 22,
    CurrentLapAverageHeartRate = 24,
    CurrentLapMaxHeartRate = 25,
    PreviousLapAverageHeartRate = 26,
    PreviousLapMaxHeartRate = 28,
    Calories = 27,
    ZonePointer = 32,
    TimeInZone = 33,
    RrVariation = 35,
    Distance = 37,
    CurrentLapDistance = 38,
    PreviousLapDistance = 39,
    SpeedOrPace = 41,
    SpeedOrPaceAverage = 42,
    SpeedOrPaceMaximum = 43,
    CurrentLapSpeedOrPace = 44,
    SpeedZonePointer = 45,
    TimeInSpeedZone = 46,
    CurrentLapMaxPaceOrSpeed = 47,
    PreviousLapMaxPaceOrSpeed = 48,
    PreviousLapSpeedOrPace = 220,
    VerticalSpeedMovingAverage = 221,
    Cadence = 49,
    AverageCadence = 50,
    MaximumCadence = 240,
    CurrentLapCadence = 51,
    CurrentLapMaxCadence = 52,
    PreviousLapCadence = 53,
    StrideLength = 54,
    AverageStrideLength = 55,
    CurrentPower = 56,
    CurrentPowerLeftRightBalance = 57,
    MaximumForce = 58,
    PowerZonePointer = 59,
    AveragePower = 60,
    MaximumPower = 61,
    AveragePowerLeftRightBalance = 62,
    CurrentLapAveragePower = 63,
    CurrentLapMaximumPower = 64,
    CurrentLapAveragePowerLrBalance = 65,
    TimeInPowerZone = 66,
    PreviousLapAveragePower = 67,
    PreviousLapMaximumPower = 68,
    PreviousLapAveragePowerLrBalance = 230,
    RestTime = 69,
    PoolCounter = 70,
    MultisportDuration = 88,
    MultisportDistance = 89,
    MultisportCalories = 90,
    MultisportAscent = 91,
    MultisportDescent = 92,
    HeartRateZones = 100,
    MultisportHeartRateZones = 101,
    LocationGuide = 102,
    PowerZones = 103,
    ForceGraph = 104,
    TimeBasedSpeedZones = 105,
    CurrentAlapAverageHeartRate = 200,
    CurrentAlapTime = 201,
    CurrentAlapAveragePower = 202,
    CurrentAlapMaximumPower = 203,
    CurrentAlapSpeedOrPace = 204,
    CurrentAlapDistance = 205,
    CurrentAlapAscent = 206,
    CurrentAlapDescent = 207,
    CurrentAlapCadence = 208,
    CurrentAlapAveragePowerLrBalance = 209,
    CurrentAlapMaxHeartRate = 210,
    CurrentAlapMaxSpeed = 211,
    CurrentAlapMaxCadence = 212,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSwimmingStyleChange {
    #[prost(enumeration="super::types::PbSwimmingStyle", required, tag="1")]
    pub style: i32,
    #[prost(message, required, tag="2")]
    pub timestamp: super::types::PbDuration,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSwimmingPoolMetric {
    #[prost(message, required, tag="1")]
    pub start_offset: super::types::PbDuration,
    #[prost(message, required, tag="2")]
    pub duration: super::types::PbDuration,
    #[prost(enumeration="super::types::PbSwimmingStyle", optional, tag="3")]
    pub style: ::std::option::Option<i32>,
    #[prost(uint32, optional, tag="4")]
    pub strokes: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSwimmingSamples {
    #[prost(message, required, tag="1")]
    pub start: super::types::PbLocalDateTime,
    #[prost(message, repeated, tag="3")]
    pub pool_metric: ::std::vec::Vec<PbSwimmingPoolMetric>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSyncInfo {
    #[prost(message, required, tag="1")]
    pub last_modified: super::types::PbSystemDateTime,
    #[prost(string, repeated, tag="2")]
    pub changed_path: ::std::vec::Vec<std::string::String>,
    #[prost(message, optional, tag="3")]
    pub last_synchronized: ::std::option::Option<super::types::PbSystemDateTime>,
    #[prost(bool, optional, tag="4", default="true")]
    pub full_sync_required: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbTeamMember {
    #[prost(uint64, required, tag="1")]
    pub team_identifier: u64,
    #[prost(uint32, optional, tag="2")]
    pub player_number: ::std::option::Option<u32>,
    #[prost(message, optional, tag="3")]
    pub player_role: ::std::option::Option<super::types::PbOneLineText>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSessionHeartRateStatistics {
    #[prost(uint32, optional, tag="1")]
    pub average: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub maximum: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbTrainingSession {
    #[prost(message, required, tag="1")]
    pub start: super::types::PbLocalDateTime,
    #[prost(message, optional, tag="20")]
    pub end: ::std::option::Option<super::types::PbLocalDateTime>,
    #[prost(uint32, required, tag="2")]
    pub exercise_count: u32,
    #[prost(string, optional, tag="3")]
    pub device_id: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="4")]
    pub model_name: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag="5")]
    pub duration: ::std::option::Option<super::types::PbDuration>,
    #[prost(float, optional, tag="6")]
    pub distance: ::std::option::Option<f32>,
    #[prost(uint32, optional, tag="7")]
    pub calories: ::std::option::Option<u32>,
    #[prost(message, optional, tag="8")]
    pub heart_rate: ::std::option::Option<PbSessionHeartRateStatistics>,
    #[prost(message, repeated, tag="9")]
    pub heart_rate_zone_duration: ::std::vec::Vec<super::types::PbDuration>,
    #[prost(message, optional, tag="10")]
    pub training_load: ::std::option::Option<super::types::PbTrainingLoad>,
    #[prost(message, optional, tag="11")]
    pub session_name: ::std::option::Option<super::types::PbOneLineText>,
    #[prost(float, optional, tag="12")]
    pub feeling: ::std::option::Option<f32>,
    #[prost(message, optional, tag="13")]
    pub note: ::std::option::Option<super::types::PbMultiLineText>,
    #[prost(message, optional, tag="14")]
    pub place: ::std::option::Option<super::types::PbOneLineText>,
    #[prost(double, optional, tag="15")]
    pub latitude: ::std::option::Option<f64>,
    #[prost(double, optional, tag="16")]
    pub longitude: ::std::option::Option<f64>,
    #[prost(enumeration="super::types::PbExerciseFeedback", optional, tag="17")]
    pub benefit: ::std::option::Option<i32>,
    #[prost(message, optional, tag="18")]
    pub sport: ::std::option::Option<super::types::PbSportIdentifier>,
    #[prost(message, optional, tag="19")]
    pub training_session_target_id: ::std::option::Option<super::types::PbTrainingSessionTargetId>,
    #[prost(message, optional, tag="21")]
    pub training_session_favorite_id: ::std::option::Option<super::types::PbTrainingSessionFavoriteId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserDb {
    #[prost(uint32, required, tag="1")]
    pub current_user_index: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserDeviceGeneralSettings {
    #[prost(enumeration="super::types::PbTimeSelection", optional, tag="1")]
    pub obsolete_time_selection: ::std::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub obsolete_time2_offset: ::std::option::Option<i32>,
    #[prost(enumeration="pb_user_device_general_settings::PbWatchFace", optional, tag="3")]
    pub watch_face: ::std::option::Option<i32>,
    #[prost(enumeration="pb_user_device_general_settings::PbButtonLockMode", optional, tag="4")]
    pub button_lock_mode: ::std::option::Option<i32>,
    #[prost(message, optional, tag="5")]
    pub button_sound_volume: ::std::option::Option<super::types::PbVolume>,
    #[prost(bool, optional, tag="7")]
    pub vibration_mode: ::std::option::Option<bool>,
    #[prost(enumeration="pb_user_device_general_settings::PbHandedness", optional, tag="8")]
    pub handedness: ::std::option::Option<i32>,
    #[prost(bool, optional, tag="9")]
    pub exeview_inverted: ::std::option::Option<bool>,
    #[prost(enumeration="pb_user_device_general_settings::PbTapButtonSensitivity", optional, tag="10")]
    pub tap_button_sensitivity: ::std::option::Option<i32>,
    #[prost(enumeration="pb_user_device_general_settings::PbInactivityAlert", optional, tag="11")]
    pub inactivity_alert: ::std::option::Option<i32>,
    #[prost(bool, optional, tag="12")]
    pub ble_connect_mode_enable: ::std::option::Option<bool>,
    #[prost(enumeration="pb_user_device_general_settings::PbWatchFace", optional, tag="13")]
    pub backup_watch_face: ::std::option::Option<i32>,
    #[prost(enumeration="pb_user_device_general_settings::PbFlightMode", optional, tag="14")]
    pub flightmode: ::std::option::Option<i32>,
    #[prost(enumeration="super::types::PbDeviceLocation", optional, tag="15")]
    pub device_location: ::std::option::Option<i32>,
    #[prost(int32, optional, tag="16")]
    pub watch_face_color: ::std::option::Option<i32>,
}
pub mod pb_user_device_general_settings {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbFlightMode {
        FlightmodeOff = 1,
        FlightmodeOn = 2,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbWatchFace {
        Basic = 1,
        Award = 2,
        UserName = 3,
        Event = 4,
        Analog = 5,
        Big = 6,
        Activity = 7,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbButtonLockMode {
        Manual = 1,
        Auto = 2,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbHandedness {
        WuInLeftHand = 1,
        WuInRightHand = 2,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbTapButtonSensitivity {
        TapButtonSensitivityOff = 1,
        TapButtonSensitivityVeryLow = 5,
        TapButtonSensitivityLow = 2,
        TapButtonSensitivityMedium = 3,
        TapButtonSensitivityHigh = 4,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbInactivityAlert {
        InactivityAlertOff = 1,
        InactivityAlertOn = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserDeviceAlarmSettings {
    #[prost(enumeration="pb_user_device_alarm_settings::PbAlarmMode", required, tag="1")]
    pub alarm_mode: i32,
    #[prost(message, required, tag="2")]
    pub alarm_time: super::types::PbTime,
}
pub mod pb_user_device_alarm_settings {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbAlarmMode {
        AlarmModeOff = 1,
        AlarmModeOnce = 2,
        AlarmModeMonToFri = 3,
        AlarmModeEveryDay = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserDeviceCountdownSettings {
    #[prost(message, required, tag="1")]
    pub countdown_time: super::types::PbDuration,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserDeviceJumpTestSettings {
    #[prost(message, required, tag="1")]
    pub cont_jump_duration: super::types::PbDuration,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbIntervalTimerValue {
    #[prost(enumeration="pb_interval_timer_value::PbIntervalTimerType", required, tag="1")]
    pub interval_timer_type: i32,
    #[prost(message, optional, tag="2")]
    pub interval_timer_duration: ::std::option::Option<super::types::PbDuration>,
    #[prost(float, optional, tag="3")]
    pub interval_timer_distance: ::std::option::Option<f32>,
}
pub mod pb_interval_timer_value {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbIntervalTimerType {
        IntervalTimerTypeDuration = 1,
        IntervalTimerTypeDistance = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserIntervalTimerSettings {
    #[prost(message, repeated, tag="1")]
    pub interval_timer_value: ::std::vec::Vec<PbIntervalTimerValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserEndTimeEstimatorSettings {
    #[prost(float, optional, tag="1")]
    pub end_time_estimator_target: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserDeviceResearchSettings {
    #[prost(bool, optional, tag="1")]
    pub accelerometer_raw_data_enable: ::std::option::Option<bool>,
    #[prost(bool, optional, tag="2")]
    pub gyroscope_raw_data_enable: ::std::option::Option<bool>,
    #[prost(bool, optional, tag="3")]
    pub magnetometer_raw_data_enable: ::std::option::Option<bool>,
    #[prost(bool, optional, tag="4")]
    pub linear_acceleration_data_enable: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserSafetyLightSettings {
    #[prost(enumeration="pb_user_safety_light_settings::PbSafetyLightMode", required, tag="1")]
    pub mode: i32,
    #[prost(enumeration="pb_user_safety_light_settings::PbSafetyLightActivationLevel", optional, tag="2")]
    pub activation_level: ::std::option::Option<i32>,
    #[prost(enumeration="pb_user_safety_light_settings::PbSafetyLightBlinkRate", optional, tag="3")]
    pub blink_rate: ::std::option::Option<i32>,
}
pub mod pb_user_safety_light_settings {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbSafetyLightMode {
        SafetyLightManual = 1,
        SafetyLightAutomatic = 2,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbSafetyLightActivationLevel {
        ActivationLevelDark = 1,
        ActivationLevelDusk = 2,
        ActivationLevelLight = 3,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbSafetyLightBlinkRate {
        BlinkRateOff = 1,
        BlinkRateSlow = 2,
        BlinkRateFast = 3,
        BlinkRateVeryFast = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbDoNotDisturbSettings {
    #[prost(bool, required, tag="1")]
    pub enabled: bool,
    #[prost(message, optional, tag="2")]
    pub start: ::std::option::Option<super::types::PbTime>,
    #[prost(message, optional, tag="3")]
    pub end: ::std::option::Option<super::types::PbTime>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserSmartWatchNotificationSettings {
    #[prost(bool, required, tag="1")]
    pub enabled: bool,
    #[prost(bool, optional, tag="2")]
    pub preview_enabled: ::std::option::Option<bool>,
    #[prost(message, optional, tag="3")]
    pub do_not_disturb_settings: ::std::option::Option<PbDoNotDisturbSettings>,
    #[prost(bool, optional, tag="4")]
    pub sounds_enabled: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserMapSettings {
    #[prost(enumeration="pb_user_map_settings::PbMapTopDirection", required, tag="1")]
    pub map_top_direction: i32,
    #[prost(bool, optional, tag="2")]
    pub bike_route_data_enabled: ::std::option::Option<bool>,
    #[prost(bool, optional, tag="3")]
    pub altitude_data_enabled: ::std::option::Option<bool>,
}
pub mod pb_user_map_settings {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbMapTopDirection {
        TopDirectionNorth = 1,
        TopDirectionHeading = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserDeviceRinseDryMessageSettings {
    #[prost(int32, required, tag="1")]
    pub message_count: i32,
    #[prost(message, optional, tag="2")]
    pub last_modified: ::std::option::Option<super::types::PbSystemDateTime>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserDeviceMassStorageSettings {
    #[prost(bool, required, tag="1")]
    pub enabled: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserDeviceDoNotDisturbSettings {
    #[prost(bool, required, tag="1")]
    pub do_not_disturb_on: bool,
    #[prost(enumeration="pb_user_device_do_not_disturb_settings::PbDoNotDisturbSettingSource", optional, tag="2")]
    pub setting_source: ::std::option::Option<i32>,
}
pub mod pb_user_device_do_not_disturb_settings {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PbDoNotDisturbSettingSource {
        SourceUser = 0,
        SourceTimed = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserDeviceAutoSyncSettings {
    #[prost(bool, required, tag="1")]
    pub enabled: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserDeviceAutomaticSampleSettings {
    #[prost(bool, required, tag="1")]
    pub ohr247_enabled: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserDeviceStravaSegmentsSettings {
    #[prost(bool, required, tag="1")]
    pub enabled: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserDeviceDaylightSaving {
    #[prost(message, required, tag="1")]
    pub next_daylight_saving_time: super::types::PbSystemDateTime,
    #[prost(int32, required, tag="2")]
    pub offset: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserDeviceSettings {
    #[prost(message, required, tag="1")]
    pub general_settings: PbUserDeviceGeneralSettings,
    #[prost(message, optional, tag="2")]
    pub alarm_settings: ::std::option::Option<PbUserDeviceAlarmSettings>,
    #[prost(message, optional, tag="3")]
    pub countdown_settings: ::std::option::Option<PbUserDeviceCountdownSettings>,
    #[prost(message, optional, tag="4")]
    pub jumptest_settings: ::std::option::Option<PbUserDeviceJumpTestSettings>,
    #[prost(message, optional, tag="5")]
    pub interval_timer_settings: ::std::option::Option<PbUserIntervalTimerSettings>,
    #[prost(message, optional, tag="6")]
    pub end_time_estimator_settings: ::std::option::Option<PbUserEndTimeEstimatorSettings>,
    #[prost(message, optional, tag="7")]
    pub research_settings: ::std::option::Option<PbUserDeviceResearchSettings>,
    #[prost(message, optional, tag="8")]
    pub safety_light_settings: ::std::option::Option<PbUserSafetyLightSettings>,
    #[prost(message, optional, tag="9")]
    pub smart_watch_notification_settings: ::std::option::Option<PbUserSmartWatchNotificationSettings>,
    #[prost(message, optional, tag="10")]
    pub map_settings: ::std::option::Option<PbUserMapSettings>,
    #[prost(message, optional, tag="11")]
    pub rinse_dry_message_settings: ::std::option::Option<PbUserDeviceRinseDryMessageSettings>,
    #[prost(message, optional, tag="12")]
    pub mass_storage_settings: ::std::option::Option<PbUserDeviceMassStorageSettings>,
    #[prost(message, optional, tag="13")]
    pub do_not_disturb_settings: ::std::option::Option<PbUserDeviceDoNotDisturbSettings>,
    #[prost(message, optional, tag="14")]
    pub auto_sync_settings: ::std::option::Option<PbUserDeviceAutoSyncSettings>,
    #[prost(message, optional, tag="15")]
    pub automatic_sample_settings: ::std::option::Option<PbUserDeviceAutomaticSampleSettings>,
    #[prost(message, optional, tag="16")]
    pub strava_segments_settings: ::std::option::Option<PbUserDeviceStravaSegmentsSettings>,
    #[prost(message, optional, tag="17")]
    pub daylight_saving: ::std::option::Option<PbUserDeviceDaylightSaving>,
    #[prost(message, required, tag="101")]
    pub last_modified: super::types::PbSystemDateTime,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPasswordToken {
    #[prost(bytes, required, tag="1")]
    pub token: std::vec::Vec<u8>,
    #[prost(bool, required, tag="2")]
    pub encrypted: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserIdentifier {
    #[prost(uint64, optional, tag="1")]
    pub master_identifier: ::std::option::Option<u64>,
    #[prost(string, optional, tag="2")]
    pub email: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag="3")]
    pub password_token: ::std::option::Option<PbPasswordToken>,
    #[prost(string, optional, tag="4")]
    pub nickname: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="5")]
    pub first_name: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="6")]
    pub last_name: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag="100")]
    pub user_id_last_modified: ::std::option::Option<super::types::PbSystemDateTime>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserBirthday {
    #[prost(message, required, tag="1")]
    pub value: super::types::PbDate,
    #[prost(message, required, tag="2")]
    pub last_modified: super::types::PbSystemDateTime,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserGender {
    #[prost(enumeration="pb_user_gender::Gender", required, tag="1")]
    pub value: i32,
    #[prost(message, required, tag="2")]
    pub last_modified: super::types::PbSystemDateTime,
}
pub mod pb_user_gender {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Gender {
        Male = 1,
        Female = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserHrAttribute {
    #[prost(uint32, required, tag="1")]
    pub value: u32,
    #[prost(message, required, tag="2")]
    pub last_modified: super::types::PbSystemDateTime,
    #[prost(enumeration="pb_user_hr_attribute::HrSettingSource", optional, tag="3")]
    pub setting_source: ::std::option::Option<i32>,
}
pub mod pb_user_hr_attribute {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HrSettingSource {
        SourceDefault = 0,
        SourceAgeBased = 1,
        SourceUser = 2,
        SourceMeasured = 3,
        SourceKeep = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserWeight {
    #[prost(float, required, tag="1")]
    pub value: f32,
    #[prost(message, required, tag="2")]
    pub last_modified: super::types::PbSystemDateTime,
    #[prost(enumeration="pb_user_weight::WeightSettingSource", optional, tag="3")]
    pub setting_source: ::std::option::Option<i32>,
}
pub mod pb_user_weight {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum WeightSettingSource {
        SourceDefault = 0,
        SourceUser = 2,
        SourceMeasured = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserHeight {
    #[prost(float, required, tag="1")]
    pub value: f32,
    #[prost(message, required, tag="2")]
    pub last_modified: super::types::PbSystemDateTime,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserVo2Max {
    #[prost(uint32, required, tag="1")]
    pub value: u32,
    #[prost(message, required, tag="2")]
    pub last_modified: super::types::PbSystemDateTime,
    #[prost(enumeration="pb_user_vo2_max::Vo2MaxSettingSource", optional, tag="3")]
    pub setting_source: ::std::option::Option<i32>,
}
pub mod pb_user_vo2_max {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Vo2MaxSettingSource {
        SourceDefault = 0,
        SourceEstimate = 1,
        SourceUser = 2,
        SourceFitnesstest = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserTrainingBackground {
    #[prost(enumeration="pb_user_training_background::TrainingBackground", required, tag="1")]
    pub value: i32,
    #[prost(message, required, tag="2")]
    pub last_modified: super::types::PbSystemDateTime,
}
pub mod pb_user_training_background {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TrainingBackground {
        Occasional = 10,
        Regular = 20,
        Frequent = 30,
        Heavy = 40,
        SemiPro = 50,
        Pro = 60,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserTypicalDay {
    #[prost(enumeration="pb_user_typical_day::TypicalDay", required, tag="1")]
    pub value: i32,
    #[prost(message, required, tag="2")]
    pub last_modified: super::types::PbSystemDateTime,
}
pub mod pb_user_typical_day {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TypicalDay {
        MostlySitting = 1,
        MostlyStanding = 2,
        MostlyMoving = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbWeeklyRecoveryTimeSum {
    #[prost(float, required, tag="1")]
    pub value: f32,
    #[prost(message, required, tag="2")]
    pub last_modified: super::types::PbSystemDateTime,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSpeedCalibrationOffset {
    #[prost(float, required, tag="1")]
    pub value: f32,
    #[prost(message, required, tag="2")]
    pub last_modified: super::types::PbSystemDateTime,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserFunctionalThresholdPower {
    #[prost(uint32, required, tag="1")]
    pub value: u32,
    #[prost(message, required, tag="2")]
    pub last_modified: super::types::PbSystemDateTime,
    #[prost(enumeration="pb_user_functional_threshold_power::FtpSettingSource", optional, tag="3")]
    pub setting_source: ::std::option::Option<i32>,
}
pub mod pb_user_functional_threshold_power {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FtpSettingSource {
        SourceDefault = 0,
        SourceEstimate = 1,
        SourceUser = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserPhysData {
    #[prost(message, required, tag="1")]
    pub birthday: PbUserBirthday,
    #[prost(message, required, tag="2")]
    pub gender: PbUserGender,
    #[prost(message, optional, tag="3")]
    pub weight: ::std::option::Option<PbUserWeight>,
    #[prost(message, optional, tag="4")]
    pub height: ::std::option::Option<PbUserHeight>,
    #[prost(message, optional, tag="5")]
    pub maximum_heartrate: ::std::option::Option<PbUserHrAttribute>,
    #[prost(message, optional, tag="6")]
    pub resting_heartrate: ::std::option::Option<PbUserHrAttribute>,
    #[prost(message, optional, tag="7")]
    pub obsolete_sitting_heartrate: ::std::option::Option<PbUserHrAttribute>,
    #[prost(message, optional, tag="8")]
    pub aerobic_threshold: ::std::option::Option<PbUserHrAttribute>,
    #[prost(message, optional, tag="9")]
    pub anaerobic_threshold: ::std::option::Option<PbUserHrAttribute>,
    #[prost(message, optional, tag="10")]
    pub vo2max: ::std::option::Option<PbUserVo2Max>,
    #[prost(message, optional, tag="11")]
    pub training_background: ::std::option::Option<PbUserTrainingBackground>,
    #[prost(message, optional, tag="12")]
    pub typical_day: ::std::option::Option<PbUserTypicalDay>,
    #[prost(message, optional, tag="13")]
    pub weekly_recovery_time_sum: ::std::option::Option<PbWeeklyRecoveryTimeSum>,
    #[prost(message, optional, tag="14")]
    pub speed_calibration_offset: ::std::option::Option<PbSpeedCalibrationOffset>,
    #[prost(message, optional, tag="15")]
    pub functional_threshold_power: ::std::option::Option<PbUserFunctionalThresholdPower>,
    #[prost(message, optional, tag="100")]
    pub last_modified: ::std::option::Option<super::types::PbSystemDateTime>,
    #[prost(message, optional, tag="101")]
    pub snapshot_start_time: ::std::option::Option<super::types::PbLocalDateTime>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbLocalizationPreferences {
    #[prost(message, optional, tag="1")]
    pub language: ::std::option::Option<super::types::PbLanguageId>,
    #[prost(enumeration="super::types::PbUnitSystem", optional, tag="2")]
    pub unit_system: ::std::option::Option<i32>,
    #[prost(enumeration="super::types::PbTimeFormat", optional, tag="3")]
    pub time_format: ::std::option::Option<i32>,
    #[prost(enumeration="super::types::PbTimeFormatSeparator", optional, tag="4")]
    pub time_format_separator: ::std::option::Option<i32>,
    #[prost(enumeration="super::types::PbDateFormat", optional, tag="5")]
    pub date_format: ::std::option::Option<i32>,
    #[prost(enumeration="super::types::PbDateFormatSeparator", optional, tag="6")]
    pub date_format_separator: ::std::option::Option<i32>,
    #[prost(enumeration="super::types::PbStartDayOfWeek", optional, tag="7")]
    pub firstday_of_week: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbTrainingPreferences {
    #[prost(uint32, optional, tag="1")]
    pub obsolete_heart_rate_zone_lock: ::std::option::Option<u32>,
    #[prost(enumeration="super::types::PbHeartRateView", optional, tag="2")]
    pub heart_rate_view: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbActivityGoalPreferences {
    #[prost(bool, required, tag="1")]
    pub visible: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbGeneralPreferences {
    #[prost(message, optional, tag="1")]
    pub localization_preferences: ::std::option::Option<PbLocalizationPreferences>,
    #[prost(message, optional, tag="2")]
    pub training_preferences: ::std::option::Option<PbTrainingPreferences>,
    #[prost(message, optional, tag="3")]
    pub activity_goal_preferences: ::std::option::Option<PbActivityGoalPreferences>,
    #[prost(message, required, tag="101")]
    pub last_modified: super::types::PbSystemDateTime,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUserTestPreferences {
    #[prost(message, optional, tag="1")]
    pub orthostatic_test_reset: ::std::option::Option<super::types::PbLocalDateTime>,
    #[prost(message, required, tag="101")]
    pub last_modified: super::types::PbSystemDateTime,
}

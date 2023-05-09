#![crate_name="asicamera_sdk_v2"]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[doc(inline)]

use std::{mem, ffi::CStr};

#[path = "asicamera2_bindings.rs"] mod ZWOASICamera2;

#[repr(i32)]
pub enum BayerPattern {
    RG = ZWOASICamera2::ASI_BAYER_PATTERN_ASI_BAYER_RG,
    BG = ZWOASICamera2::ASI_BAYER_PATTERN_ASI_BAYER_BG,
    GR = ZWOASICamera2::ASI_BAYER_PATTERN_ASI_BAYER_GR,
    GB = ZWOASICamera2::ASI_BAYER_PATTERN_ASI_BAYER_GB,
}

#[repr(i32)]
pub enum ImageType {
    RAW8 = ZWOASICamera2::ASI_IMG_TYPE_ASI_IMG_RAW8,
    RAW16 = ZWOASICamera2::ASI_IMG_TYPE_ASI_IMG_RAW16,
    RGB24 = ZWOASICamera2::ASI_IMG_TYPE_ASI_IMG_RGB24,
    Y8 = ZWOASICamera2::ASI_IMG_TYPE_ASI_IMG_Y8,
}

#[repr(i32)]
pub enum GuideDirection {
    North = ZWOASICamera2::ASI_GUIDE_DIRECTION_ASI_GUIDE_NORTH,
    South = ZWOASICamera2::ASI_GUIDE_DIRECTION_ASI_GUIDE_SOUTH,
    East = ZWOASICamera2::ASI_GUIDE_DIRECTION_ASI_GUIDE_EAST,
    West = ZWOASICamera2::ASI_GUIDE_DIRECTION_ASI_GUIDE_WEST,
}

#[repr(i32)]
pub enum FlipStatus {
    Horizontally = ZWOASICamera2::ASI_FLIP_STATUS_ASI_FLIP_HORIZ,
    Vertically = ZWOASICamera2::ASI_FLIP_STATUS_ASI_FLIP_VERT,
    Both = ZWOASICamera2::ASI_FLIP_STATUS_ASI_FLIP_BOTH,
}

#[repr(i32)]
pub enum CameraMode {
    Normal = ZWOASICamera2::ASI_CAMERA_MODE_ASI_MODE_NORMAL,
    EdgeTrigger_Soft = ZWOASICamera2::ASI_CAMERA_MODE_ASI_MODE_TRIG_SOFT_EDGE,
    EdgeTrigger_Rise = ZWOASICamera2::ASI_CAMERA_MODE_ASI_MODE_TRIG_RISE_EDGE,
    EdgeTrigger_Fall = ZWOASICamera2::ASI_CAMERA_MODE_ASI_MODE_TRIG_FALL_EDGE,
    LevelTrigger_Soft = ZWOASICamera2::ASI_CAMERA_MODE_ASI_MODE_TRIG_SOFT_LEVEL,
    LevelTrigger_High = ZWOASICamera2::ASI_CAMERA_MODE_ASI_MODE_TRIG_HIGH_LEVEL,
    LevelTrigger_Low = ZWOASICamera2::ASI_CAMERA_MODE_ASI_MODE_TRIG_LOW_LEVEL,
}

#[repr(i32)]
pub enum TrigOutput {
    PinA = ZWOASICamera2::ASI_TRIG_OUTPUT_ASI_TRIG_OUTPUT_PINA,
    PinB = ZWOASICamera2::ASI_TRIG_OUTPUT_ASI_TRIG_OUTPUT_PINB,
}

#[repr(i32)]
pub enum ErrorCode {
    Success = ZWOASICamera2::ASI_ERROR_CODE_ASI_SUCCESS,
    InvalidIndex = ZWOASICamera2::ASI_ERROR_CODE_ASI_ERROR_INVALID_INDEX,
    InvalidId = ZWOASICamera2::ASI_ERROR_CODE_ASI_ERROR_INVALID_ID,
    InvalidControlType = ZWOASICamera2::ASI_ERROR_CODE_ASI_ERROR_INVALID_CONTROL_TYPE,
    CameraClosed = ZWOASICamera2::ASI_ERROR_CODE_ASI_ERROR_CAMERA_CLOSED,
    CameraRemoved = ZWOASICamera2::ASI_ERROR_CODE_ASI_ERROR_CAMERA_REMOVED,
    InvalidPath = ZWOASICamera2::ASI_ERROR_CODE_ASI_ERROR_INVALID_PATH,
    InvalidFileFormat = ZWOASICamera2::ASI_ERROR_CODE_ASI_ERROR_INVALID_FILEFORMAT,
    InvalidSize = ZWOASICamera2::ASI_ERROR_CODE_ASI_ERROR_INVALID_SIZE,
    InvalidImageType = ZWOASICamera2::ASI_ERROR_CODE_ASI_ERROR_INVALID_IMGTYPE,
    OutOfBounds = ZWOASICamera2::ASI_ERROR_CODE_ASI_ERROR_OUTOF_BOUNDARY,
    Timeout = ZWOASICamera2::ASI_ERROR_CODE_ASI_ERROR_TIMEOUT,
    InvalidSequence = ZWOASICamera2::ASI_ERROR_CODE_ASI_ERROR_INVALID_SEQUENCE,
    BufferTooSmall = ZWOASICamera2::ASI_ERROR_CODE_ASI_ERROR_BUFFER_TOO_SMALL,
    VideoModeActive = ZWOASICamera2::ASI_ERROR_CODE_ASI_ERROR_VIDEO_MODE_ACTIVE,
    ExposureInProgress = ZWOASICamera2::ASI_ERROR_CODE_ASI_ERROR_EXPOSURE_IN_PROGRESS,
    GeneralError = ZWOASICamera2::ASI_ERROR_CODE_ASI_ERROR_GENERAL_ERROR,
    InvalidMode = ZWOASICamera2::ASI_ERROR_CODE_ASI_ERROR_INVALID_MODE,
}

#[repr(i32)]
pub enum ControlType {
    Gain = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_GAIN,
    Exposure = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_EXPOSURE,
    Gamma = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_GAMMA,
    WhiteBalance_B = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_WB_B,
    WhiteBalance_R = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_WB_R,
    Offset = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_OFFSET,
    BandwidthOvld = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_BANDWIDTHOVERLOAD,
    Overclock = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_OVERCLOCK,
    Temperature = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_TEMPERATURE,
    Flip = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_FLIP,
    AutoExposure_MaxGain = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_AUTO_MAX_GAIN,
    AutoExposure_MaxExp = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_AUTO_MAX_EXP,
    AutoExposure_TgtBrightness = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_AUTO_TARGET_BRIGHTNESS,
    HWBin = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_HARDWARE_BIN,
    HighSpeedMode = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_HIGH_SPEED_MODE,
    CoolPowerPct = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_COOLER_POWER_PERC,
    TargetTemp = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_TARGET_TEMP,
    CoolerOn = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_COOLER_ON,
    MonoBin = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_MONO_BIN,
    FanOn = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_FAN_ON,
    PatternAdjust = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_PATTERN_ADJUST,
    AntiDewHeater = ZWOASICamera2::ASI_CONTROL_TYPE_ASI_ANTI_DEW_HEATER,
}

#[repr(i32)]
pub enum ExposureStatus {
    Idle = ZWOASICamera2::ASI_EXPOSURE_STATUS_ASI_EXP_IDLE,
    Working = ZWOASICamera2::ASI_EXPOSURE_STATUS_ASI_EXP_WORKING,
    Success = ZWOASICamera2::ASI_EXPOSURE_STATUS_ASI_EXP_SUCCESS,
    Failed = ZWOASICamera2::ASI_EXPOSURE_STATUS_ASI_EXP_FAILED,
}

pub struct CameraInfo {
    pub name: String,
    pub camera_id: i32,
    pub max_height: i32,
    pub max_width: i32,
    pub is_color_camera: bool,
    pub bayer_pattern: BayerPattern,
    pub supported_bins: Vec<i32>,
    pub supported_video_format: Vec<ImageType>,
    pub pixel_size: f64,
    pub has_mechanical_shutter: bool,
    pub has_st4_port: bool,
    pub has_cooler: bool,
    pub is_usb3_host: bool,
    pub is_usb3_camera: bool,
    pub e_per_adu: f64,
    pub bit_depth: i32,
    pub is_trigger_camera: bool,   
}

pub struct ControlCaps {
    pub name: String,
    pub description: String,
    pub control_type: ControlType,
    pub min_value: i64,
    pub max_value: i64,
    pub default_value: i64,
    pub is_auto_supported: bool,
    pub is_writable: bool,
}

pub type SupportedModes = Vec<CameraMode>;



// Copy-Paste from https://github.com/pololu/jrk-g2-arduino/blob/master/JrkG2.h
#[allow(dead_code)]
pub enum VarOffset {
    Input = 0x00,              // u16
    Target = 0x02,             // u16
    Feedback = 0x04,           // u16
    ScaledFeedback = 0x06,     // u16
    Integral = 0x08,           // i16
    DutyCycleTarget = 0x0A,    // i16
    DutyCycle = 0x0C,          // i16
    CurrentLowRes = 0x0E,      // u8
    PIDPeriodExceeded = 0x0F,  // u8
    PIDPeriodCount = 0x10,     // u16
    ErrorFlagsHalting = 0x12,  // u16
    ErrorFlagsOccurred = 0x14, // u16

    FlagByte1 = 0x16,  // u8
    VinVoltage = 0x17, // u16
    Current = 0x19,    // u16

    // variables above can be read with single-byte commands (GetVariable)
    // variables below must be read with segment read (GetVariables)
    DeviceReset = 0x1F,                     // u8
    UpTime = 0x20,                          // u32
    RCPulseWidth = 0x24,                    // u16
    FBTReading = 0x26,                      // u16
    AnalogReadingSDA = 0x28,                // u16
    AnalogReadingFBA = 0x2A,                // u16
    DigitalReadings = 0x2C,                 // u8
    RawCurrent = 0x2D,                      // u16
    EncodedHardCurrentLimit = 0x2F,         // u16
    LastDutyCycle = 0x31,                   // i16
    CurrentChoppingConsecutiveCount = 0x33, // u8
    CurrentChoppingOccurrenceCount = 0x34,  // u8; read with dedicated command
}

#[allow(dead_code)]
pub enum SettingOffset {
    OptionsByte1 = 0x01,                        // u8
    OptionsByte2 = 0x02,                        // u8
    InputMode = 0x03,                           // u8
    InputErrorMinimum = 0x04,                   // u16,
    InputErrorMaximum = 0x06,                   // u16,
    InputMinimum = 0x08,                        // u16,
    InputMaximum = 0x0A,                        // u16,
    InputNeutralMinimum = 0x0C,                 // u16,
    InputNeutralMaximum = 0x0E,                 // u16,
    OutputMinimum = 0x10,                       // u16,
    OutputNeutral = 0x12,                       // u16,
    OutputMaximum = 0x14,                       // u16,
    InputScalingDegree = 0x16,                  // u8,
    InputAnalogSamplesExponent = 0x17,          // u8,
    FeedbackMode = 0x18,                        // u8,
    FeedbackErrorMinimum = 0x19,                // u16,
    FeedbackErrorMaximum = 0x1B,                // u16,
    FeedbackMinimum = 0x1D,                     // u16,
    FeedbackMaximum = 0x1F,                     // u16,
    FeedbackDeadZone = 0x21,                    // u8,
    FeedbackAnalogSamplesExponent = 0x22,       // u8,
    SerialMode = 0x23,                          // u8,
    SerialBaudRateGenerator = 0x24,             // u16,
    SerialTimeout = 0x26,                       // u16,
    SerialDeviceNumber = 0x28,                  // u16,
    ErrorEnable = 0x2A,                         // u16
    ErrorLatch = 0x2C,                          // u16
    ErrorHard = 0x2E,                           // u16
    VinCalibration = 0x30,                      // u16
    PwmFrequency = 0x32,                        // u8
    CurrentSamplesExponent = 0x33,              // u8
    HardOvercurrentThreshold = 0x34,            // u8
    CurrentOffsetCalibration = 0x35,            // u16
    CurrentScaleCalibration = 0x37,             // u16
    FBTMethod = 0x39,                           // u8
    FBTOptions = 0x3A,                          // u8
    FBTTimingTimeout = 0x3B,                    // u16
    FBTSamples = 0x3D,                          // u8
    FBTDividerExponent = 0x3E,                  // u8
    IntegralDividerExponent = 0x3F,             // u8
    SoftCurrentRegulationLevelForward = 0x40,   // u16
    SoftCurrentRegulationLevelReverse = 0x42,   // u16
    OptionsByte3 = 0x50,                        // u8
    ProportionalMultiplier = 0x51,              // u16
    ProportionalExponent = 0x53,                // u8
    IntegralMultiplier = 0x54,                  // u16
    IntegralExponent = 0x56,                    // u8
    DerivativeMultiplier = 0x57,                // u16
    DerivativeExponent = 0x59,                  // u8
    PIDPeriod = 0x5A,                           // u16
    IntegralLimit = 0x5C,                       // u16
    MaxDutyCycleWhileFeedbackOutOfRange = 0x5E, // u16
    MaxAccelerationForward = 0x60,              // u16
    MaxAccelerationReverse = 0x62,              // u16
    MaxDecelerationForward = 0x64,              // u16
    MaxDecelerationReverse = 0x66,              // u16
    MaxDutyCycleForward = 0x68,                 // u16
    MaxDutyCycleReverse = 0x6A,                 // u16
    EncodedHardCurrentLimitForward = 0x6C,      // u16
    EncodedHardCurrentLimitReverse = 0x6E,      // u16
    BrakeDurationForward = 0x70,                // u8
    BrakeDurationReverse = 0x71,                // u8
    SoftCurrentLimitForward = 0x72,             // u16
    SoftCurrentLimitReverse = 0x74,             // u16
}

#[allow(dead_code)]
pub enum JrkG2Error {
    AwaitingCommand = 0,
    NoPower = 1,
    MotorDriver = 2,
    InputInvalid = 3,
    InputDisconnect = 4,
    FeedbackDisconnect = 5,
    SoftOvercurrent = 6,
    SerialSignal = 7,
    SerialOverrun = 8,
    SerialBufferFull = 9,
    SerialCrc = 10,
    SerialProtocol = 11,
    SerialTimeout = 12,
    HardOvercurrent = 13,
}

/// This enum defines the Jrk G2 command bytes which are used for its serial and
/// I2C interfaces.  These bytes are used by the library and you should not need
/// to use them.
#[allow(dead_code)]
pub enum JrkG2Command {
    SetTarget = 0xC0,
    SetTargetLowResRev = 0xE0,
    SetTargetLowResFwd = 0xE1,
    ForceDutyCycleTarget = 0xF2,
    ForceDutyCycle = 0xF4,
    MotorOff = 0xFF,
    GetVariable8 = 0x80,
    GetVariable16 = 0xA0,
    GetEEPROMSettings = 0xE3,
    GetVariables = 0xE5,
    SetRAMSettings = 0xE6,
    GetRAMSettings = 0xEA,
    GetCurrentChoppingOccurrenceCount = 0xEC,
}

/// This enum defines the modes in which the Jrk G2's duty cycle target or duty
/// cycle, normally derived from the output of its PID algorithm, can be
/// overridden with a forced value.
///
/// See JrkG2Base::getForceMode(), JrkG2Base::forceDutyCycleTarget(), and
/// JrkG2Base::forceDutyCycle().
#[allow(dead_code)]
pub enum JrkG2ForceMode {
    None = 0,
    DutyCycleTarget = 1,
    DutyCycle = 2,
}

/// This enum defines the possible causes of a full microcontroller reset for
/// the Jrk G2.
///
/// See JrkG2Base::getDeviceReset().
#[allow(dead_code)]
pub enum JrkG2Reset {
    PowerUp = 0,
    Brownout = 1,
    ResetLine = 2,
    Watchdog = 4,
    Software = 8,
    StackOverflow = 16,
    StackUnderflow = 32,
}

/// This enum defines the Jrk G2's control and feedback pins.
#[allow(dead_code)]
pub enum JrkG2Pin {
    SCL = 0,
    SDA = 1,
    TX = 2,
    RX = 3,
    RC = 4,
    AUX = 5,
    FBA = 6,
    FBT = 7,
}

/// This enum defines the bits in the Jrk G2's Options Byte 3 register.  You
/// should not need to use this directly.  See JrkG2Base::setResetIntegral(),
/// JrkG2Base::getResetIntegral(), JrkG2Base::setCoastWhenOff(), and
/// JrkG2Base::getCoastWhenOff().
#[allow(dead_code)]
pub enum JrkG2OptionsByte3 {
    ResetIntegral = 0,
    CoastWhenOff = 1,
}

enum VarOffset {
    Input = 0x00,              // uint16_t
    Target = 0x02,             // uint16_t
    Feedback = 0x04,           // uint16_t
    ScaledFeedback = 0x06,     // uint16_t
    Integral = 0x08,           // int16_t
    DutyCycleTarget = 0x0A,    // int16_t
    DutyCycle = 0x0C,          // int16_t
    CurrentLowRes = 0x0E,      // uint8_t
    PIDPeriodExceeded = 0x0F,  // uint8_t
    PIDPeriodCount = 0x10,     // uint16_t
    ErrorFlagsHalting = 0x12,  // uint16_t
    ErrorFlagsOccurred = 0x14, // uint16_t

    FlagByte1 = 0x16,  // uint8_t
    VinVoltage = 0x17, // uint16_t
    Current = 0x19,    // uint16_t

    // variables above can be read with single-byte commands (GetVariable)
    // variables below must be read with segment read (GetVariables)
    DeviceReset = 0x1F,                     // uint8_t
    UpTime = 0x20,                          // uint32_t
    RCPulseWidth = 0x24,                    // uint16_t
    FBTReading = 0x26,                      // uint16_t
    AnalogReadingSDA = 0x28,                // uint16_t
    AnalogReadingFBA = 0x2A,                // uint16_t
    DigitalReadings = 0x2C,                 // uint8_t
    RawCurrent = 0x2D,                      // uint16_t
    EncodedHardCurrentLimit = 0x2F,         // uint16_t
    LastDutyCycle = 0x31,                   // int16_t
    CurrentChoppingConsecutiveCount = 0x33, // uint8_t
    CurrentChoppingOccurrenceCount = 0x34,  // uint8_t; read with dedicated command
}

enum SettingOffset {
    OptionsByte1 = 0x01,                        // uint8_t
    OptionsByte2 = 0x02,                        // uint8_t
    InputMode = 0x03,                           // uint8_t
    InputErrorMinimum = 0x04,                   // uint16_t,
    InputErrorMaximum = 0x06,                   // uint16_t,
    InputMinimum = 0x08,                        // uint16_t,
    InputMaximum = 0x0A,                        // uint16_t,
    InputNeutralMinimum = 0x0C,                 // uint16_t,
    InputNeutralMaximum = 0x0E,                 // uint16_t,
    OutputMinimum = 0x10,                       // uint16_t,
    OutputNeutral = 0x12,                       // uint16_t,
    OutputMaximum = 0x14,                       // uint16_t,
    InputScalingDegree = 0x16,                  // uint8_t,
    InputAnalogSamplesExponent = 0x17,          // uint8_t,
    FeedbackMode = 0x18,                        // uint8_t,
    FeedbackErrorMinimum = 0x19,                // uint16_t,
    FeedbackErrorMaximum = 0x1B,                // uint16_t,
    FeedbackMinimum = 0x1D,                     // uint16_t,
    FeedbackMaximum = 0x1F,                     // uint16_t,
    FeedbackDeadZone = 0x21,                    // uint8_t,
    FeedbackAnalogSamplesExponent = 0x22,       // uint8_t,
    SerialMode = 0x23,                          // uint8_t,
    SerialBaudRateGenerator = 0x24,             // uint16_t,
    SerialTimeout = 0x26,                       // uint16_t,
    SerialDeviceNumber = 0x28,                  // uint16_t,
    ErrorEnable = 0x2A,                         // uint16_t
    ErrorLatch = 0x2C,                          // uint16_t
    ErrorHard = 0x2E,                           // uint16_t
    VinCalibration = 0x30,                      // uint16_t
    PwmFrequency = 0x32,                        // uint8_t
    CurrentSamplesExponent = 0x33,              // uint8_t
    HardOvercurrentThreshold = 0x34,            // uint8_t
    CurrentOffsetCalibration = 0x35,            // uint16_t
    CurrentScaleCalibration = 0x37,             // uint16_t
    FBTMethod = 0x39,                           // uint8_t
    FBTOptions = 0x3A,                          // uint8_t
    FBTTimingTimeout = 0x3B,                    // uint16_t
    FBTSamples = 0x3D,                          // uint8_t
    FBTDividerExponent = 0x3E,                  // uint8_t
    IntegralDividerExponent = 0x3F,             // uint8_t
    SoftCurrentRegulationLevelForward = 0x40,   // uint16_t
    SoftCurrentRegulationLevelReverse = 0x42,   // uint16_t
    OptionsByte3 = 0x50,                        // uint8_t
    ProportionalMultiplier = 0x51,              // uint16_t
    ProportionalExponent = 0x53,                // uint8_t
    IntegralMultiplier = 0x54,                  // uint16_t
    IntegralExponent = 0x56,                    // uint8_t
    DerivativeMultiplier = 0x57,                // uint16_t
    DerivativeExponent = 0x59,                  // uint8_t
    PIDPeriod = 0x5A,                           // uint16_t
    IntegralLimit = 0x5C,                       // uint16_t
    MaxDutyCycleWhileFeedbackOutOfRange = 0x5E, // uint16_t
    MaxAccelerationForward = 0x60,              // uint16_t
    MaxAccelerationReverse = 0x62,              // uint16_t
    MaxDecelerationForward = 0x64,              // uint16_t
    MaxDecelerationReverse = 0x66,              // uint16_t
    MaxDutyCycleForward = 0x68,                 // uint16_t
    MaxDutyCycleReverse = 0x6A,                 // uint16_t
    EncodedHardCurrentLimitForward = 0x6C,      // uint16_t
    EncodedHardCurrentLimitReverse = 0x6E,      // uint16_t
    BrakeDurationForward = 0x70,                // uint8_t
    BrakeDurationReverse = 0x71,                // uint8_t
    SoftCurrentLimitForward = 0x72,             // uint16_t
    SoftCurrentLimitReverse = 0x74,             // uint16_t
}

enum JrkG2Error {
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
enum JrkG2Command {
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
enum JrkG2ForceMode {
    None = 0,
    DutyCycleTarget = 1,
    DutyCycle = 2,
}

/// This enum defines the possible causes of a full microcontroller reset for
/// the Jrk G2.
///
/// See JrkG2Base::getDeviceReset().
enum JrkG2Reset {
    PowerUp = 0,
    Brownout = 1,
    ResetLine = 2,
    Watchdog = 4,
    Software = 8,
    StackOverflow = 16,
    StackUnderflow = 32,
}

/// This enum defines the Jrk G2's control and feedback pins.
enum JrkG2Pin {
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
enum JrkG2OptionsByte3 {
    ResetIntegral = 0,
    CoastWhenOff = 1,
}

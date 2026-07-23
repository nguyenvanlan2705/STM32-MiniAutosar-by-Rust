# ADC

## Purpose

The ADC module is the first analog input path in the project.

It is currently used to validate a basic sensor flow:

```text
PB0 / ADC1_IN8
    |
    v
Register ADC
    |
    v
MCAL ADC
    |
    v
IoHwAb Sensor
    |
    v
IoIf RX PDU 0x101
    |
    v
Application temperature runnable
```

## Current Hardware Mapping

Current ADC test channel:

| Logical role | STM32 pin | ADC channel | Config |
|---|---:|---:|---|
| LM35 sensor draft | PB0 | ADC1_IN8 | `ADC_CHANNEL_8` |

Important mapping lesson:

```text
ADC channel 8 is PB0, not PA8.
```

For STM32F411, common ADC1 mappings include:

```text
ADC_CHANNEL_0 -> PA0
ADC_CHANNEL_1 -> PA1
ADC_CHANNEL_2 -> PA2
ADC_CHANNEL_3 -> PA3
ADC_CHANNEL_4 -> PA4
ADC_CHANNEL_5 -> PA5
ADC_CHANNEL_6 -> PA6
ADC_CHANNEL_7 -> PA7
ADC_CHANNEL_8 -> PB0
ADC_CHANNEL_9 -> PB1
```

## Register Flow

Basic single-conversion ADC flow:

```text
1. Enable GPIO clock for the analog pin
2. Configure the GPIO pin as analog mode
3. Disable pull-up/pull-down on the analog pin
4. Enable ADC1 clock
   RCC_APB2ENR.ADC1EN = 1
5. Configure ADC sample time
   ADC_SMPR1/SMPR2
6. Configure regular sequence rank
   ADC_SQR3 first conversion = channel
7. Configure resolution
   ADC_CR1.RES
8. Configure data alignment
   ADC_CR2.ALIGN
9. Enable ADC
   ADC_CR2.ADON = 1
10. Start software conversion
   ADC_CR2.SWSTART = 1
11. Check EOC
   ADC_SR.EOC = 1
12. Read result
   ADC_DR
```

Current low-level ADC1 base address:

```text
ADC1 = 0x4001_2000
```

## MCAL ADC APIs

The MCAL ADC layer currently has both old blocking helper style and the newer non-blocking split.

Current non-blocking direction:

```rust
adc_start_conversion(channel);
adc_is_conversion_complete();
adc_read_conversion_result(&mut data);
```

Important distinction:

```text
adc_is_conversion_complete()
    checks EOC once and returns.
    It is non-blocking.

adc_wait_for_conversion_complete()
    loops until EOC or timeout.
    It is blocking.
```

The blocking helper can still be useful for very small bring-up tests, but it should not be used inside cooperative scheduler runnables.

## IoHwAb Sensor State Machine

The ADC sensor path is now handled through `iohwab_sensor_mainfunction()`.

Current state flow:

```text
SENSOR_IDLE
    |
    v
start ADC conversion
    |
    v
SENSOR_CONVERTING
    |
    +-- EOC not set -> remain CONVERTING and return
    |
    +-- EOC set -> read ADC_DR, save cached value
                    |
                    v
              SENSOR_COMPLETE
                    |
                    v
IoIf/App reads latest value
                    |
                    v
SENSOR_IDLE
```

This is intentionally non-blocking. Each scheduler call only performs one small step.

## IoIf RX Path

The LM35 sensor is exposed through IoIf RX:

```text
IoIf RX PDU ID: 0x101
Peripheral: ADC
Channel: SENSOR_LM35
Mode: POLLING
```

Current read flow:

```text
temperature_measurement_app_1ms()
    |
    v
ioif_read_rx_value(0x101, &mut adc_raw_value)
    |
    v
IoIf maps SENSOR_LM35 to IoHwAb SensorType::LM35
    |
    v
iohwab_sensor_read_latest_value()
    |
    v
return cached ADC value if SENSOR_COMPLETE
```

If the sensor conversion is not complete yet, IoIf returns `IOIF_E_NOT_OK`. This currently means "no fresh data available" for the ADC path, not necessarily a hardware error.

## LM35 Conversion

LM35 output is approximately:

```text
10 mV / degC
```

ADC raw to voltage:

```rust
voltage = raw as f32 * vref / 4095.0;
```

Voltage to temperature:

```rust
temperature_c = voltage * 100.0;
```

With `VDDA/VREF` around 3.3 V:

```rust
temperature_c = raw as f32 * 3.3 / 4095.0 * 100.0;
```

Important:

```text
The ADC reference is VDDA/VREF, not the LM35 supply voltage.
Even if LM35 is powered from 5 V, the ADC conversion formula should use the ADC reference voltage.
```

## Hardware Validation Notes

The ADC driver was sanity-checked with direct pin tests:

```text
PB0 -> GND   expected raw near 0
PB0 -> 3.3V  expected raw near 4095
```

These tests indicate the ADC channel, base address, and conversion path are basically working.

LM35 validation remains hardware-pending because the observed module output appeared unstable/high and there was a mild shock sensation when touching the sensor. Do not keep debugging code when the hardware feels electrically unsafe.

Recommended hardware checks before trusting LM35 readings:

```text
1. Confirm LM35 GND is common with STM32 GND
2. Confirm LM35 pinout
3. Measure LM35 Vout with a multimeter
4. Measure PB0-GND directly
5. Optionally add 100 nF from Vout to GND and 100 nF from Vcc to GND near the sensor
```

## Scheduler Rule

Do not place blocking ADC waits inside scheduler runnables.

Good:

```text
Task calls sensor mainfunction
sensor mainfunction checks EOC once
returns immediately if conversion is not complete
```

Bad:

```text
Task starts ADC
Task loops until EOC is set
```

The project currently uses the good direction through `iohwab_sensor_mainfunction()`.

## Current Limitations

- Only one sensor, `SensorType::LM35`, is configured.
- ADC is still effectively one global ADC conversion resource.
- Multiple ADC sensors will need an ownership rule so one conversion is not overwritten by another.
- The old blocking MCAL ADC helper still exists and should be treated as bring-up/test-only.
- IoIf ADC "not ready yet" currently returns `IOIF_E_NOT_OK`; a clearer pending/no-new-data status can be added later.

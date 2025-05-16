//! This module contains the definitions of the registers for the UART peripheral.
use bitstuff::ints::u6;
use core::num::{NonZeroU8, NonZeroU16};

/// The UARTDR Register; the data register.
///
/// For words to be transmitted:
///  - if the FIFOs are enabled, data written to this location is pushed onto the transmit FIFO
///  - if the FIFOs are not enabled, data is stored in the transmitter holding register (the bottom word of the transmit FIFO).
/// The write operation initiates transmission from the UART. The data is prefixed with a start bit, appended with the appropriate parity bit (if parity is enabled), and a stop bit. The resultant word is then transmitted.
///
/// For received words:
///  - if the FIFOs are enabled, the data byte and the 4-bit status (break, frame, parity, and overrun) is pushed onto the 12-bit wide receive FIFO
///  - if the FIFOs are not enabled, the data byte and status are stored in the receiving holding register (the bottom word of the receive FIFO).
///
/// The received data byte is read by performing reads from the UARTDR Register along with the corresponding status information. The status information can also be read by a read of the UARTRSR/UARTECR Register.
#[bitstuff::stuff(u32)]
#[derive(Default)]
pub struct DataRegister {
    /// This bit is set to 1 if data is received and the receive FIFO is already full.
    /// This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it.
    #[bitstuff(bit = 11)]
    overrun_error: bool,
    /// This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits).
    ///
    /// In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO.
    /// The next character is only enabled after the receive data input goes to a 1 (marking state), and the next valid start bit is received.
    #[bitstuff(bit = 10)]
    break_error: bool,
    ///  When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H select.
    /// In FIFO mode, this error is associated with the character at the top of the FIFO.
    #[bitstuff(bit = 9)]
    parity_error: bool,
    /// When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1).
    /// In FIFO mode, this error is associated with the character at the top of the FIFO.
    #[bitstuff(bit = 8)]
    framing_error: bool,
    /// Receive (read) data character.
    /// Transmit (write) data character.
    #[bitstuff(bits = 0..=7)]
    data: u8,
}

/// The UARTRSR/UARTECR Register; the receive status register/error clear register.
///
/// Receive status can also be read from the UARTRSR Register. If the status is read from this register, then the status information for break, framing and parity corresponds to the data character read from the Data Register, UARTDR prior to reading the UARTRSR Register. The status information for overrun is set immediately when an overrun condition occurs.
///
/// A write to this register clears the framing, parity, break, and overrun errors. The data value is not important. All the bits are cleared to 0 on reset.
#[bitstuff::stuff(u32)]
#[derive(Default)]
pub struct ReceiveStatusRegister {
    /// This bit is set to 1 if data is received and the FIFO is already full.
    ///
    /// This bit is cleared to 0 by a write to this register.
    ///
    /// The FIFO contents remain valid because no more data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must now read the data, to empty the FIFO.
    #[bitstuff(bit = 3)]
    overrun_error: bool,
    /// This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity, and stop bits).
    ///
    /// This bit is cleared to 0 after a write to this register.
    ///
    /// In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received.
    #[bitstuff(bit = 2)]
    break_error: bool,
    /// When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H select.
    ///
    /// This bit is cleared to 0 by a write to this register.
    ///
    /// In FIFO mode, this error is associated with the character at the top of the FIFO.
    #[bitstuff(bit = 1)]
    parity_error: bool,
    /// When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1).
    ///
    /// This bit is cleared to 0 by a write to this register.
    ///
    /// In FIFO mode, this error is associated with the character at the top of the FIFO.
    #[bitstuff(bit = 0)]
    framing_error: bool,
}

//note: read only, could do without the "with" functions but they can be useful for testing i suppose
/// The UARTFR Register; the flag register.
///
/// After reset TXFF, RXFF, and BUSY are 0, and TXFE and RXFE are 1.
#[bitstuff::stuff(u32)]
#[derive(Default)]
pub struct FlagRegister {
    /// This bit is the complement of the UART ring indicator, nUARTRI, modem status input. That is, the bit is 1 when nUARTRI is LOW.
    #[bitstuff(bit = 8)]
    ring_indicator: bool,
    /// The meaning of this bit depends on the state of the FEN bit in the Line Control Register, UARTLCR_H.
    ///
    /// If the FIFO is disabled, this bit is set when the transmit holding register is empty.
    ///
    /// If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty.
    ///
    /// This bit does not indicate if there is data in the transmit shift register.
    #[bitstuff(bit = 7)]
    transmit_fifo_empty: bool,
    /// The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register.
    ///
    /// If the FIFO is disabled, this bit is set when the receive holding register is full.
    ///
    /// If the FIFO is enabled, the RXFF bit is set when the receive FIFO is full.
    #[bitstuff(bit = 6)]
    receive_fifo_full: bool,
    /// The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register.
    ///
    /// If the FIFO is disabled, this bit is set when the transmit holding register is full.
    ///
    /// If the FIFO is enabled, the TXFF bit is set when the transmit FIFO is full.
    #[bitstuff(bit = 5)]
    transmit_fifo_full: bool,
    /// Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register.
    ///
    /// If the FIFO is disabled, this bit is set when the receive holding register is empty.
    ///
    /// If the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty.
    #[bitstuff(bit = 4)]
    receive_fifo_empty: bool,
    /// If this bit is set to 1, the UART is busy transmitting data. This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register.
    ///
    /// This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether the UART is enabled or not.
    #[bitstuff(bit = 3)]
    uart_busy: bool,
    /// This bit is the complement of the UART data carrier detect, nUARTDCD, modem status input. That is, the bit is 1 when nUARTDCD is LOW.
    #[bitstuff(bit = 2)]
    data_carrier_detect: bool,
    /// This bit is the complement of the UART data set ready, nUARTDSR, modem status input. That is, the bit is 1 when nUARTDSR is LOW.
    #[bitstuff(bit = 1)]
    data_set_ready: bool,
    /// Clear to send. This bit is the complement of the UART clear to send, nUARTCTS, modem status input. That is, the bit is 1 when nUARTCTS is LOW.
    #[bitstuff(bit = 0)]
    clear_to_send: bool,
}

//note: since this is a wrapper around a single field. Maybe we should extend the macro to support Struct(u8) or something like that
/// The UARTILPR Register; the IrDA low-power counter register.
///
/// This is an 8-bit read/write register that stores the low-power counter divisor value used to generate the IrLPBaud16 signal by dividing down of UARTCLK.
/// The IrLPBaud16 signal is generated by dividing down the UARTCLK signal according to the low-power divisor value written to the UARTILPR Register.
///
/// The low-power divisor value is calculated as follows:
///
/// low-power divisor (ILPDVSR) = (FUARTCLK / FIrLPBaud16)
///
/// where FIrLPBaud16 is nominally 1.8432MHz.
///
/// You must select the divisor so that 1.42MHz < FIrLPBaud16 < 2.12MHz, results in a low-power pulse duration of 1.41 - 2.11µs (three times the period of IrLPBaud16).
///
/// Note: In low-power IrDA mode the UART rejects random noise on the received serial data input by ignoring SIRIN pulses that are less than 3 periods of IrLPBaud16.
#[bitstuff::stuff(u8)]
// todo: fix default being zero here
#[derive(Default)]
pub struct IrDALowPowerRegister {
    /// These bits are cleared to 0 at reset.
    #[bitstuff(bits = 0..=7, falliable)]
    low_power_divisor_value: NonZeroU8,
}

/// The UARTIBRD Register; the integer baud rate divisor register.
///
/// The baud rate divisor is calculated as follows:
///
///     Baud rate divisor BAUDDIV = (FUARTCLK / (16 x Baud rate))
///     where FUARTCLK is the UART reference clock frequency.
///
/// The BAUDDIV is comprised of the integer value (BAUD DIVINT) and the fractional value (BAUD DIVFRAC).
///
/// The contents of the UARTIBRD and UARTFBRD registers are not updated until transmission or reception of the current character is complete.
///
/// The minimum divide ratio possible is 1 and the maximum is 65535(216 - 1). That is, UARTIBRD = 0 is invalid and UARTFBRD is ignored when this is the case.
///
/// Similarly, when UARTIBRD = 65535 (that is 0xFFFF), then UARTFBRD must not be greater than zero. If this is exceeded it results in an aborted transmission or reception.
#[bitstuff::stuff(u16)]
#[derive(Default)]
pub struct IntegerBaudRateDivisorRegister {
    /// These bits are cleared to 0 at reset.
    #[bitstuff(bits = 0..=15, falliable)]
    integer_baud_rate_divisor: NonZeroU16,
}

/// The UARTFBRD Register; the fractional part of the baud rate divisor value.
///
/// The baud rate divisor is calculated as follows:
///
///     Baud rate divisor BAUDDIV = (FUARTCLK / (16 x Baud rate))
///     where FUARTCLK is the UART reference clock frequency.
///
/// The BAUDDIV is comprised of the integer value (BAUD DIVINT) and the fractional value (BAUD DIVFRAC).
///
/// The contents of the UARTIBRD and UARTFBRD registers are not updated until transmission or reception of the current character is complete.
///
/// The minimum divide ratio possible is 1 and the maximum is 65535(216 - 1). That is, UARTIBRD = 0 is invalid and UARTFBRD is ignored when this is the case.
///
/// Similarly, when UARTIBRD = 65535 (that is 0xFFFF), then UARTFBRD must not be greater than zero. If this is exceeded it results in an aborted transmission or reception.

#[bitstuff::stuff(u8)]
#[derive(Default)]
pub struct FractionalBaudRateDivisorRegister {
    /// These bits are cleared to 0 at reset.
    #[bitstuff(bits = 0..=5)]
    fractional_baud_rate_divisor: u6,
}

/// the number of data bits transmitted or received in a frame
#[derive(Debug, Default)]
#[bitstuff::stuff]
pub enum WordLength {
    #[default]
    FiveBits = 0b00,
    SixBits = 0b01,
    SevenBits = 0b10,
    EightBits = 0b11,
}

/// The UARTLCR_H Register; the line control register.
///
/// This register accesses bits 29 to 22 of the UART Line Control Register, UARTLCR.
/// All the bits are cleared to 0 when reset.
#[bitstuff::stuff(u16)]
#[derive(Default)]
pub struct LineControlRegister {
    /// `false` = stick parity is disabled
    ///
    /// `true` = either:
    ///  - if the EPS bit is false then the parity bit is transmitted and checked as a 1
    ///  - if the EPS bit is true then the parity bit is transmitted and checked as a 0.
    #[bitstuff(bit = 7)]
    stick_parity: bool,
    ///Indicates the number of data bits transmitted or received in a frame
    #[bitstuff(bits = 5..=6)]
    word_length: WordLength,
    /// Enable FIFOs:
    ///  - `false` = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers
    ///  - `true` = transmit and receive FIFO buffers are enabled (FIFO mode).
    #[bitstuff(bit = 4)]
    enable_fifos: bool,
    //note: we could use an stop bits enum here, just for printing/clarity
    /// Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame.
    /// The receive logic does not check for two stop bits being received.
    #[bitstuff(bit = 3)]
    two_stop_bits_select: bool,
    //note: we could use and EvenOdd enum here
    /// Controls the type of parity the UART uses during transmission and reception:
    ///  - `false` = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits.
    ///  - `true` = even parity. The UART generates or checks for an even number of 1s in the data and parity bits.
    /// This bit has no effect when the PEN bit disables parity checking and generation.
    #[bitstuff(bit = 2)]
    even_parity_select: bool,
    /// Parity enable:
    /// - `false` = parity is disabled and no parity bit added to the data frame
    /// - `true` = parity checking and generation is enabled.
    #[bitstuff(bit = 1)]
    parity_enable: bool,
    /// Send break. If this bit is set to `true`, a low-level is continually output on the UARTTXD output,
    /// after completing transmission of the current character.
    ///
    /// For the proper execution of the break command, the software must set this bit for at least two complete frames.
    ///
    /// For normal use, this bit must be cleared to 0.
    #[bitstuff(bit = 0)]
    send_break: bool,
}

/// The UARTCR Register; the control register.
///
/// All the bits are cleared to 0 on reset except for bits 9 and 8 that are set to 1.
#[bitstuff::stuff(u16)]
#[derive(Default)]
pub struct ControlRegister {
    /// If this bit is set to `true`, CTS hardware flow control is enabled. Data is only transmitted when the nUARTCTS signal is asserted.
    #[bitstuff(bit = 15)]
    #[allow(non_snake_case)]
    CTS_hardware_flow_control_enable: bool,
    /// If this bit is set to `true`, RTS hardware flow control is enabled. Data is only requested when there is space in the receive FIFO for it to be received.
    #[bitstuff(bit = 14)]
    #[allow(non_snake_case)]
    RTS_hardware_flow_control_enable: bool,
    /// This bit is the complement of the UART Out2 (nUARTOut2) modem status output. That is, when the bit is programmed to `true`, the output is 0.
    /// For DTE this can be used as Ring Indicator (RI).
    #[bitstuff(bit = 13)]
    out2: bool,
    /// This bit is the complement of the UART Out1 (nUARTOut1) modem status output. That is, when the bit is programmed to `true`, the output is 0.
    /// For DTE this can be used as Data Carrier Detect (DCD).
    #[bitstuff(bit = 12)]
    out1: bool,
    /// This bit is the complement of the UART request to send, nUARTRTS, modem status output. That is, when the bit is programmed to a `true` then nUARTRTS is LOW.
    #[bitstuff(bit = 11)]
    request_to_send: bool,
    /// Data transmit ready. This bit is the complement of the UART data transmit ready, nUARTDTR, modem status output. That is, when the bit is programmed to a `true` then nUARTDTR is LOW.
    #[bitstuff(bit = 10)]
    data_transmit_ready: bool,
    ///  If this bit is set to `true`, the receive section of the UART is enabled. Data reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of reception, it completes the current character before stopping.
    #[bitstuff(bit = 9)]
    receive_enable: bool,
    /// Transmit enable. If this bit is set to `true`, the transmit section of the UART is enabled. Data transmission occurs for either UART signals, or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of transmission, it completes the current character before stopping.
    #[bitstuff(bit = 8)]
    transmit_enable: bool,
    /// If this bit is set to 1 and the SIREN bit is set to 1 and the SIRTEST bit in the Test Control Register, UARTTCR is set to 1, then the nSIROUT path is inverted, and fed through to the SIRIN path. The SIRTEST bit in the test register must be set to 1 to override the normal half-duplex SIR operation. This must be the requirement for accessing the test registers during normal operation, and SIRTEST must be cleared to 0 when loopback testing is finished. This feature reduces the amount of external coupling required during system test.
    ///
    /// If this bit is set to 1, and the SIRTEST bit is set to 0, the UARTTXD path is fed through to the UARTRXD path.
    /// In either SIR mode or UART mode, when this bit is set, the modem outputs are also fed through to the modem inputs.
    /// This bit is cleared to 0 on reset, to disable loopback.
    #[bitstuff(bit = 7)]
    loopback_enable: bool,
    /// SIR enable:
    /// - `false` = IrDA SIR ENDEC is disabled. nSIROUT remains LOW (no light pulse generated), and signal transitions on SIRIN have no effect.
    /// - `true` = IrDA SIR ENDEC is enabled. Data is transmitted and received on nSIROUT and SIRIN. UARTTXD remains HIGH, in the marking state. Signal transitions on UARTRXD or modem status inputs have no effect.
    ///
    /// This bit has no effect if the UARTEN bit disables the UART.
    #[bitstuff(bit = 1)]
    #[allow(non_snake_case)]
    SIR_enable: bool,
    /// UART enable:
    /// - `false` = UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping.
    /// - `true` = the UART is enabled. Data transmission and reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit.
    #[bitstuff(bit = 0)]
    #[allow(non_snake_case)]
    UART_enable: bool,
}

/// Receive and transmit interrupt FIFO level select trigger points.
#[derive(Debug, Default)]
#[bitstuff::stuff]
pub enum FIFOLevelSelect {
    /// Receive FIFO becomes ≥ 1/8 full
    /// or
    /// Transmit FIFO becomes ≤ 1/8 full
    OneEighth = 0b000,
    /// Receive FIFO becomes ≥ 1/4 full
    /// or
    /// Transmit FIFO becomes ≤ 1/4 full
    OneFourth = 0b001,
    /// Receive FIFO becomes ≥ 1/2 full
    /// or
    /// Transmit FIFO becomes ≤ 1/2 full
    #[default]
    OneHalf = 0b010,
    /// Receive FIFO becomes ≥ 3/4 full
    /// or
    /// Transmit FIFO becomes ≤ 3/4 full
    ThreeFourth = 0b011,
    /// Receive FIFO becomes ≥ 7/8 full
    /// or
    /// Transmit FIFO becomes ≤ 7/8 full
    SevenEighth = 0b100,
}

/// The UARTIFLS Register; the interrupt FIFO level select register.
///
/// You can use this register to define the FIFO level that triggers the assertion of UARTTXINTR and UARTRXINTR.
///
/// The interrupts are generated based on a transition through a level rather than being based on the level. That is, the interrupts are generated when the fill level progresses through the trigger level.
///
/// The bits are reset so that the trigger level is when the FIFOs are at the half-way mark.
#[bitstuff::stuff(u16)]
pub struct InterruptFIFOLevelSelectRegister {
    /// Receive FIFO interrupt level select.
    #[bitstuff(bits = 3..=5, falliable)]
    #[allow(non_snake_case)]
    receive_interrupt_FIFO_level_select: FIFOLevelSelect,
    /// Transmit FIFO interrupt level select.
    #[bitstuff(bits = 0..=2, falliable)]
    #[allow(non_snake_case)]
    transmit_interrupt_FIFO_level_select: FIFOLevelSelect,
}

/// The UARTIMSC Register; the interrupt mask set/clear register.
///
/// It is a read/write register.
///
/// On a read this register returns the current value of the mask on the relevant interrupt.
/// On a write of 1 to the particular bit, it sets the corresponding mask of that interrupt.
/// A write of 0 clears the corresponding mask.
///
/// All the bits are cleared to 0 when reset.
#[bitstuff::stuff(u16)]
pub struct InterruptMaskSetClearRegister {
    #[bitstuff(bit = 10)]
    overrun_error_interrupt_mask: bool,
    #[bitstuff(bit = 9)]
    break_error_interrupt_mask: bool,
    #[bitstuff(bit = 8)]
    parity_error_interrupt_mask: bool,
    #[bitstuff(bit = 7)]
    framing_error_interrupt_mask: bool,
    #[bitstuff(bit = 6)]
    receive_timeout_interrupt_mask: bool,
    #[bitstuff(bit = 5)]
    transmit_interrupt_mask: bool,
    #[bitstuff(bit = 4)]
    receive_interrupt_mask: bool,
    #[bitstuff(bit = 3)]
    #[allow(non_snake_case)]
    nUARTDSR_modem_interrupt_mask: bool,
    #[bitstuff(bit = 2)]
    #[allow(non_snake_case)]
    nUARTDCD_modem_interrupt_mask: bool,
    #[bitstuff(bit = 1)]
    #[allow(non_snake_case)]
    nUARTCTS_modem_interrupt_mask: bool,
    #[bitstuff(bit = 0)]
    #[allow(non_snake_case)]
    nUARTRI_modem_interrupt_mask: bool,
}

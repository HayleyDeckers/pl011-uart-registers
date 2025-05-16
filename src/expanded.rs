#![feature(prelude_import)]
//! this crate implements the registers of the [ARM PrimeCell UART (PL011)](https://developer.arm.com/documentation/ddi0183/g) peripheral
//! it doesn't implement the full peripheral, just the registers. Making it easier to write your own drivers :)
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use bitstuff::{FromBits, TryFromBits};
pub mod registrers {
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
    pub struct DataRegister(u32);
    #[automatically_derived]
    impl ::core::default::Default for DataRegister {
        #[inline]
        fn default() -> DataRegister {
            DataRegister(::core::default::Default::default())
        }
    }
    impl DataRegister {
        /// This bit is set to 1 if data is received and the receive FIFO is already full.
        /// This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it.
        #[inline(always)]
        pub fn overrun_error(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 11) as _),
            )
        }
        #[inline(always)]
        pub fn with_overrun_error(mut self, value: bool) -> Self {
            let value: u32 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b100000000000) | (value << 11u32);
            self
        }
        /// This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits).
        ///
        /// In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO.
        /// The next character is only enabled after the receive data input goes to a 1 (marking state), and the next valid start bit is received.
        #[inline(always)]
        pub fn break_error(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 10) as _),
            )
        }
        #[inline(always)]
        pub fn with_break_error(mut self, value: bool) -> Self {
            let value: u32 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b10000000000) | (value << 10u32);
            self
        }
        ///  When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H select.
        /// In FIFO mode, this error is associated with the character at the top of the FIFO.
        #[inline(always)]
        pub fn parity_error(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 9) as _),
            )
        }
        #[inline(always)]
        pub fn with_parity_error(mut self, value: bool) -> Self {
            let value: u32 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b1000000000) | (value << 9u32);
            self
        }
        /// When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1).
        /// In FIFO mode, this error is associated with the character at the top of the FIFO.
        #[inline(always)]
        pub fn framing_error(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 8) as _),
            )
        }
        #[inline(always)]
        pub fn with_framing_error(mut self, value: bool) -> Self {
            let value: u32 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b100000000) | (value << 8u32);
            self
        }
        /// Receive (read) data character.
        /// Transmit (write) data character.
        #[inline(always)]
        pub fn data(&self) -> u8 {
            <u8 as ::bitstuff::FromBits>::from_bits((self.0 >> 0) as _)
        }
        #[inline(always)]
        pub fn with_data(mut self, value: u8) -> Self {
            let value: u32 = <u8 as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b11111111) | (value << 0u32);
            self
        }
    }
    impl ::core::fmt::Debug for DataRegister {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("DataRegister")
                .field("overrun_error", &self.overrun_error())
                .field("break_error", &self.break_error())
                .field("parity_error", &self.parity_error())
                .field("framing_error", &self.framing_error())
                .field("data", &self.data())
                .finish()
        }
    }
    /// The UARTRSR/UARTECR Register; the receive status register/error clear register.
    ///
    /// Receive status can also be read from the UARTRSR Register. If the status is read from this register, then the status information for break, framing and parity corresponds to the data character read from the Data Register, UARTDR prior to reading the UARTRSR Register. The status information for overrun is set immediately when an overrun condition occurs.
    ///
    /// A write to this register clears the framing, parity, break, and overrun errors. The data value is not important. All the bits are cleared to 0 on reset.
    pub struct ReceiveStatusRegister(u32);
    #[automatically_derived]
    impl ::core::default::Default for ReceiveStatusRegister {
        #[inline]
        fn default() -> ReceiveStatusRegister {
            ReceiveStatusRegister(::core::default::Default::default())
        }
    }
    impl ReceiveStatusRegister {
        /// This bit is set to 1 if data is received and the FIFO is already full.
        ///
        /// This bit is cleared to 0 by a write to this register.
        ///
        /// The FIFO contents remain valid because no more data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must now read the data, to empty the FIFO.
        #[inline(always)]
        pub fn overrun_error(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 3) as _),
            )
        }
        #[inline(always)]
        pub fn with_overrun_error(mut self, value: bool) -> Self {
            let value: u32 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b1000) | (value << 3u32);
            self
        }
        /// This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity, and stop bits).
        ///
        /// This bit is cleared to 0 after a write to this register.
        ///
        /// In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received.
        #[inline(always)]
        pub fn break_error(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 2) as _),
            )
        }
        #[inline(always)]
        pub fn with_break_error(mut self, value: bool) -> Self {
            let value: u32 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b100) | (value << 2u32);
            self
        }
        /// When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H select.
        ///
        /// This bit is cleared to 0 by a write to this register.
        ///
        /// In FIFO mode, this error is associated with the character at the top of the FIFO.
        #[inline(always)]
        pub fn parity_error(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 1) as _),
            )
        }
        #[inline(always)]
        pub fn with_parity_error(mut self, value: bool) -> Self {
            let value: u32 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b10) | (value << 1u32);
            self
        }
        /// When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1).
        ///
        /// This bit is cleared to 0 by a write to this register.
        ///
        /// In FIFO mode, this error is associated with the character at the top of the FIFO.
        #[inline(always)]
        pub fn framing_error(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 0) as _),
            )
        }
        #[inline(always)]
        pub fn with_framing_error(mut self, value: bool) -> Self {
            let value: u32 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b1) | (value << 0u32);
            self
        }
    }
    impl ::core::fmt::Debug for ReceiveStatusRegister {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("ReceiveStatusRegister")
                .field("overrun_error", &self.overrun_error())
                .field("break_error", &self.break_error())
                .field("parity_error", &self.parity_error())
                .field("framing_error", &self.framing_error())
                .finish()
        }
    }
    /// The UARTFR Register; the flag register.
    ///
    /// After reset TXFF, RXFF, and BUSY are 0, and TXFE and RXFE are 1.
    pub struct FlagRegister(u32);
    #[automatically_derived]
    impl ::core::default::Default for FlagRegister {
        #[inline]
        fn default() -> FlagRegister {
            FlagRegister(::core::default::Default::default())
        }
    }
    impl FlagRegister {
        /// This bit is the complement of the UART ring indicator, nUARTRI, modem status input. That is, the bit is 1 when nUARTRI is LOW.
        #[inline(always)]
        pub fn ring_indicator(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 8) as _),
            )
        }
        #[inline(always)]
        pub fn with_ring_indicator(mut self, value: bool) -> Self {
            let value: u32 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b100000000) | (value << 8u32);
            self
        }
        /// The meaning of this bit depends on the state of the FEN bit in the Line Control Register, UARTLCR_H.
        ///
        /// If the FIFO is disabled, this bit is set when the transmit holding register is empty.
        ///
        /// If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty.
        ///
        /// This bit does not indicate if there is data in the transmit shift register.
        #[inline(always)]
        pub fn transmit_fifo_empty(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 7) as _),
            )
        }
        #[inline(always)]
        pub fn with_transmit_fifo_empty(mut self, value: bool) -> Self {
            let value: u32 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b10000000) | (value << 7u32);
            self
        }
        /// The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register.
        ///
        /// If the FIFO is disabled, this bit is set when the receive holding register is full.
        ///
        /// If the FIFO is enabled, the RXFF bit is set when the receive FIFO is full.
        #[inline(always)]
        pub fn receive_fifo_full(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 6) as _),
            )
        }
        #[inline(always)]
        pub fn with_receive_fifo_full(mut self, value: bool) -> Self {
            let value: u32 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b1000000) | (value << 6u32);
            self
        }
        /// The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register.
        ///
        /// If the FIFO is disabled, this bit is set when the transmit holding register is full.
        ///
        /// If the FIFO is enabled, the TXFF bit is set when the transmit FIFO is full.
        #[inline(always)]
        pub fn transmit_fifo_full(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 5) as _),
            )
        }
        #[inline(always)]
        pub fn with_transmit_fifo_full(mut self, value: bool) -> Self {
            let value: u32 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b100000) | (value << 5u32);
            self
        }
        /// Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register.
        ///
        /// If the FIFO is disabled, this bit is set when the receive holding register is empty.
        ///
        /// If the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty.
        #[inline(always)]
        pub fn receive_fifo_empty(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 4) as _),
            )
        }
        #[inline(always)]
        pub fn with_receive_fifo_empty(mut self, value: bool) -> Self {
            let value: u32 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b10000) | (value << 4u32);
            self
        }
        /// If this bit is set to 1, the UART is busy transmitting data. This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register.
        ///
        /// This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether the UART is enabled or not.
        #[inline(always)]
        pub fn uart_busy(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 3) as _),
            )
        }
        #[inline(always)]
        pub fn with_uart_busy(mut self, value: bool) -> Self {
            let value: u32 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b1000) | (value << 3u32);
            self
        }
        /// This bit is the complement of the UART data carrier detect, nUARTDCD, modem status input. That is, the bit is 1 when nUARTDCD is LOW.
        #[inline(always)]
        pub fn data_carrier_detect(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 2) as _),
            )
        }
        #[inline(always)]
        pub fn with_data_carrier_detect(mut self, value: bool) -> Self {
            let value: u32 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b100) | (value << 2u32);
            self
        }
        /// This bit is the complement of the UART data set ready, nUARTDSR, modem status input. That is, the bit is 1 when nUARTDSR is LOW.
        #[inline(always)]
        pub fn data_set_ready(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 1) as _),
            )
        }
        #[inline(always)]
        pub fn with_data_set_ready(mut self, value: bool) -> Self {
            let value: u32 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b10) | (value << 1u32);
            self
        }
        /// Clear to send. This bit is the complement of the UART clear to send, nUARTCTS, modem status input. That is, the bit is 1 when nUARTCTS is LOW.
        #[inline(always)]
        pub fn clear_to_send(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 0) as _),
            )
        }
        #[inline(always)]
        pub fn with_clear_to_send(mut self, value: bool) -> Self {
            let value: u32 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b1) | (value << 0u32);
            self
        }
    }
    impl ::core::fmt::Debug for FlagRegister {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("FlagRegister")
                .field("ring_indicator", &self.ring_indicator())
                .field("transmit_fifo_empty", &self.transmit_fifo_empty())
                .field("receive_fifo_full", &self.receive_fifo_full())
                .field("transmit_fifo_full", &self.transmit_fifo_full())
                .field("receive_fifo_empty", &self.receive_fifo_empty())
                .field("uart_busy", &self.uart_busy())
                .field("data_carrier_detect", &self.data_carrier_detect())
                .field("data_set_ready", &self.data_set_ready())
                .field("clear_to_send", &self.clear_to_send())
                .finish()
        }
    }
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
    pub struct IrDALowPowerRegister(u8);
    #[automatically_derived]
    impl ::core::default::Default for IrDALowPowerRegister {
        #[inline]
        fn default() -> IrDALowPowerRegister {
            IrDALowPowerRegister(::core::default::Default::default())
        }
    }
    impl IrDALowPowerRegister {
        /// These bits are cleared to 0 at reset.
        #[inline(always)]
        pub fn low_power_divisor_value(
            &self,
        ) -> ::core::result::Result<NonZeroU8, ::core::primitive::u8> {
            <NonZeroU8 as ::bitstuff::TryFromBits>::try_from_bits((self.0 >> 0) as _)
        }
        #[inline(always)]
        pub fn with_low_power_divisor_value(mut self, value: NonZeroU8) -> Self {
            let value: u8 = <NonZeroU8 as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b11111111) | (value << 0u32);
            self
        }
    }
    impl ::core::fmt::Debug for IrDALowPowerRegister {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("IrDALowPowerRegister")
                .field("low_power_divisor_value", &self.low_power_divisor_value())
                .finish()
        }
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
    pub struct IntegerBaudRateDivisorRegister(u16);
    #[automatically_derived]
    impl ::core::default::Default for IntegerBaudRateDivisorRegister {
        #[inline]
        fn default() -> IntegerBaudRateDivisorRegister {
            IntegerBaudRateDivisorRegister(::core::default::Default::default())
        }
    }
    impl IntegerBaudRateDivisorRegister {
        /// These bits are cleared to 0 at reset.
        #[inline(always)]
        pub fn integer_baud_rate_divisor(
            &self,
        ) -> ::core::result::Result<NonZeroU16, ::core::primitive::u16> {
            <NonZeroU16 as ::bitstuff::TryFromBits>::try_from_bits((self.0 >> 0) as _)
        }
        #[inline(always)]
        pub fn with_integer_baud_rate_divisor(mut self, value: NonZeroU16) -> Self {
            let value: u16 = <NonZeroU16 as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b1111111111111111) | (value << 0u32);
            self
        }
    }
    impl ::core::fmt::Debug for IntegerBaudRateDivisorRegister {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("IntegerBaudRateDivisorRegister")
                .field("integer_baud_rate_divisor", &self.integer_baud_rate_divisor())
                .finish()
        }
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
    pub struct FractionalBaudRateDivisorRegister(u8);
    #[automatically_derived]
    impl ::core::default::Default for FractionalBaudRateDivisorRegister {
        #[inline]
        fn default() -> FractionalBaudRateDivisorRegister {
            FractionalBaudRateDivisorRegister(::core::default::Default::default())
        }
    }
    impl FractionalBaudRateDivisorRegister {
        /// These bits are cleared to 0 at reset.
        #[inline(always)]
        pub fn fractional_baud_rate_divisor(&self) -> u6 {
            <u6 as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u6::trimmed_new((self.0 >> 0) as _),
            )
        }
        #[inline(always)]
        pub fn with_fractional_baud_rate_divisor(mut self, value: u6) -> Self {
            let value: u8 = <u6 as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b111111) | (value << 0u32);
            self
        }
    }
    impl ::core::fmt::Debug for FractionalBaudRateDivisorRegister {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("FractionalBaudRateDivisorRegister")
                .field(
                    "fractional_baud_rate_divisor",
                    &self.fractional_baud_rate_divisor(),
                )
                .finish()
        }
    }
    /// the number of data bits transmitted or received in a frame
    pub enum WordLength {
        #[default]
        FiveBits = 0b00,
        SixBits = 0b01,
        SevenBits = 0b10,
        EightBits = 0b11,
    }
    impl ::bitstuff::BitRepr for WordLength {
        type BitRepr = ::bitstuff::ints::u2;
    }
    impl ::bitstuff::FromBits for WordLength {
        fn from_bits(value: ::bitstuff::ints::u2) -> Self {
            match u8::from(value) {
                0 => Self::FiveBits,
                1 => Self::SixBits,
                2 => Self::SevenBits,
                3 => Self::EightBits,
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
    }
    impl ::bitstuff::ToBits for WordLength {
        fn to_bits(self) -> ::bitstuff::ints::u2 {
            ::bitstuff::ints::u2::trimmed_new(
                match self {
                    Self::FiveBits => 0,
                    Self::SixBits => 1,
                    Self::SevenBits => 2,
                    Self::EightBits => 3,
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for WordLength {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    WordLength::FiveBits => "FiveBits",
                    WordLength::SixBits => "SixBits",
                    WordLength::SevenBits => "SevenBits",
                    WordLength::EightBits => "EightBits",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for WordLength {
        #[inline]
        fn default() -> WordLength {
            Self::FiveBits
        }
    }
    /// The UARTLCR_H Register; the line control register.
    ///
    /// This register accesses bits 29 to 22 of the UART Line Control Register, UARTLCR.
    /// All the bits are cleared to 0 when reset.
    pub struct LineControlRegister(u16);
    #[automatically_derived]
    impl ::core::default::Default for LineControlRegister {
        #[inline]
        fn default() -> LineControlRegister {
            LineControlRegister(::core::default::Default::default())
        }
    }
    impl LineControlRegister {
        /// `false` = stick parity is disabled
        ///
        /// `true` = either:
        ///  - if the EPS bit is false then the parity bit is transmitted and checked as a 1
        ///  - if the EPS bit is true then the parity bit is transmitted and checked as a 0.
        #[inline(always)]
        pub fn stick_parity(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 7) as _),
            )
        }
        #[inline(always)]
        pub fn with_stick_parity(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b10000000) | (value << 7u32);
            self
        }
        ///Indicates the number of data bits transmitted or received in a frame
        #[inline(always)]
        pub fn word_length(&self) -> WordLength {
            <WordLength as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u2::trimmed_new((self.0 >> 5) as _),
            )
        }
        #[inline(always)]
        pub fn with_word_length(mut self, value: WordLength) -> Self {
            let value: u16 = <WordLength as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b1100000) | (value << 5u32);
            self
        }
        /// Enable FIFOs:
        ///  - `false` = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers
        ///  - `true` = transmit and receive FIFO buffers are enabled (FIFO mode).
        #[inline(always)]
        pub fn enable_fifos(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 4) as _),
            )
        }
        #[inline(always)]
        pub fn with_enable_fifos(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b10000) | (value << 4u32);
            self
        }
        /// Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame.
        /// The receive logic does not check for two stop bits being received.
        #[inline(always)]
        pub fn two_stop_bits_select(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 3) as _),
            )
        }
        #[inline(always)]
        pub fn with_two_stop_bits_select(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b1000) | (value << 3u32);
            self
        }
        /// Controls the type of parity the UART uses during transmission and reception:
        ///  - `false` = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits.
        ///  - `true` = even parity. The UART generates or checks for an even number of 1s in the data and parity bits.
        /// This bit has no effect when the PEN bit disables parity checking and generation.
        #[inline(always)]
        pub fn even_parity_select(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 2) as _),
            )
        }
        #[inline(always)]
        pub fn with_even_parity_select(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b100) | (value << 2u32);
            self
        }
        /// Parity enable:
        /// - `false` = parity is disabled and no parity bit added to the data frame
        /// - `true` = parity checking and generation is enabled.
        #[inline(always)]
        pub fn parity_enable(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 1) as _),
            )
        }
        #[inline(always)]
        pub fn with_parity_enable(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b10) | (value << 1u32);
            self
        }
        /// Send break. If this bit is set to `true`, a low-level is continually output on the UARTTXD output,
        /// after completing transmission of the current character.
        ///
        /// For the proper execution of the break command, the software must set this bit for at least two complete frames.
        ///
        /// For normal use, this bit must be cleared to 0.
        #[inline(always)]
        pub fn send_break(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 0) as _),
            )
        }
        #[inline(always)]
        pub fn with_send_break(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b1) | (value << 0u32);
            self
        }
    }
    impl ::core::fmt::Debug for LineControlRegister {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("LineControlRegister")
                .field("stick_parity", &self.stick_parity())
                .field("word_length", &self.word_length())
                .field("enable_fifos", &self.enable_fifos())
                .field("two_stop_bits_select", &self.two_stop_bits_select())
                .field("even_parity_select", &self.even_parity_select())
                .field("parity_enable", &self.parity_enable())
                .field("send_break", &self.send_break())
                .finish()
        }
    }
    /// The UARTCR Register; the control register.
    ///
    /// All the bits are cleared to 0 on reset except for bits 9 and 8 that are set to 1.
    pub struct ControlRegister(u16);
    #[automatically_derived]
    impl ::core::default::Default for ControlRegister {
        #[inline]
        fn default() -> ControlRegister {
            ControlRegister(::core::default::Default::default())
        }
    }
    impl ControlRegister {
        /// If this bit is set to `true`, CTS hardware flow control is enabled. Data is only transmitted when the nUARTCTS signal is asserted.
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn CTS_hardware_flow_control_enable(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 15) as _),
            )
        }
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn with_CTS_hardware_flow_control_enable(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b1000000000000000) | (value << 15u32);
            self
        }
        /// If this bit is set to `true`, RTS hardware flow control is enabled. Data is only requested when there is space in the receive FIFO for it to be received.
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn RTS_hardware_flow_control_enable(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 14) as _),
            )
        }
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn with_RTS_hardware_flow_control_enable(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b100000000000000) | (value << 14u32);
            self
        }
        /// This bit is the complement of the UART Out2 (nUARTOut2) modem status output. That is, when the bit is programmed to `true`, the output is 0.
        /// For DTE this can be used as Ring Indicator (RI).
        #[inline(always)]
        pub fn out2(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 13) as _),
            )
        }
        #[inline(always)]
        pub fn with_out2(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b10000000000000) | (value << 13u32);
            self
        }
        /// This bit is the complement of the UART Out1 (nUARTOut1) modem status output. That is, when the bit is programmed to `true`, the output is 0.
        /// For DTE this can be used as Data Carrier Detect (DCD).
        #[inline(always)]
        pub fn out1(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 12) as _),
            )
        }
        #[inline(always)]
        pub fn with_out1(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b1000000000000) | (value << 12u32);
            self
        }
        /// This bit is the complement of the UART request to send, nUARTRTS, modem status output. That is, when the bit is programmed to a `true` then nUARTRTS is LOW.
        #[inline(always)]
        pub fn request_to_send(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 11) as _),
            )
        }
        #[inline(always)]
        pub fn with_request_to_send(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b100000000000) | (value << 11u32);
            self
        }
        /// Data transmit ready. This bit is the complement of the UART data transmit ready, nUARTDTR, modem status output. That is, when the bit is programmed to a `true` then nUARTDTR is LOW.
        #[inline(always)]
        pub fn data_transmit_ready(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 10) as _),
            )
        }
        #[inline(always)]
        pub fn with_data_transmit_ready(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b10000000000) | (value << 10u32);
            self
        }
        ///  If this bit is set to `true`, the receive section of the UART is enabled. Data reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of reception, it completes the current character before stopping.
        #[inline(always)]
        pub fn receive_enable(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 9) as _),
            )
        }
        #[inline(always)]
        pub fn with_receive_enable(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b1000000000) | (value << 9u32);
            self
        }
        /// Transmit enable. If this bit is set to `true`, the transmit section of the UART is enabled. Data transmission occurs for either UART signals, or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of transmission, it completes the current character before stopping.
        #[inline(always)]
        pub fn transmit_enable(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 8) as _),
            )
        }
        #[inline(always)]
        pub fn with_transmit_enable(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b100000000) | (value << 8u32);
            self
        }
        /// If this bit is set to 1 and the SIREN bit is set to 1 and the SIRTEST bit in the Test Control Register, UARTTCR is set to 1, then the nSIROUT path is inverted, and fed through to the SIRIN path. The SIRTEST bit in the test register must be set to 1 to override the normal half-duplex SIR operation. This must be the requirement for accessing the test registers during normal operation, and SIRTEST must be cleared to 0 when loopback testing is finished. This feature reduces the amount of external coupling required during system test.
        ///
        /// If this bit is set to 1, and the SIRTEST bit is set to 0, the UARTTXD path is fed through to the UARTRXD path.
        /// In either SIR mode or UART mode, when this bit is set, the modem outputs are also fed through to the modem inputs.
        /// This bit is cleared to 0 on reset, to disable loopback.
        #[inline(always)]
        pub fn loopback_enable(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 7) as _),
            )
        }
        #[inline(always)]
        pub fn with_loopback_enable(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b10000000) | (value << 7u32);
            self
        }
        /// SIR enable:
        /// - `false` = IrDA SIR ENDEC is disabled. nSIROUT remains LOW (no light pulse generated), and signal transitions on SIRIN have no effect.
        /// - `true` = IrDA SIR ENDEC is enabled. Data is transmitted and received on nSIROUT and SIRIN. UARTTXD remains HIGH, in the marking state. Signal transitions on UARTRXD or modem status inputs have no effect.
        ///
        /// This bit has no effect if the UARTEN bit disables the UART.
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn SIR_enable(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 1) as _),
            )
        }
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn with_SIR_enable(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b10) | (value << 1u32);
            self
        }
        /// UART enable:
        /// - `false` = UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping.
        /// - `true` = the UART is enabled. Data transmission and reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit.
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn UART_enable(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 0) as _),
            )
        }
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn with_UART_enable(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b1) | (value << 0u32);
            self
        }
    }
    impl ::core::fmt::Debug for ControlRegister {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("ControlRegister")
                .field(
                    "CTS_hardware_flow_control_enable",
                    &self.CTS_hardware_flow_control_enable(),
                )
                .field(
                    "RTS_hardware_flow_control_enable",
                    &self.RTS_hardware_flow_control_enable(),
                )
                .field("out2", &self.out2())
                .field("out1", &self.out1())
                .field("request_to_send", &self.request_to_send())
                .field("data_transmit_ready", &self.data_transmit_ready())
                .field("receive_enable", &self.receive_enable())
                .field("transmit_enable", &self.transmit_enable())
                .field("loopback_enable", &self.loopback_enable())
                .field("SIR_enable", &self.SIR_enable())
                .field("UART_enable", &self.UART_enable())
                .finish()
        }
    }
    /// Receive and transmit interrupt FIFO level select trigger points.
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
    impl ::bitstuff::TryFromBits for FIFOLevelSelect {
        fn try_from_bits(
            value: ::bitstuff::ints::u3,
        ) -> ::core::result::Result<Self, ::bitstuff::ints::u3> {
            match u8::from(value) {
                0 => Ok(Self::OneEighth),
                1 => Ok(Self::OneFourth),
                2 => Ok(Self::OneHalf),
                3 => Ok(Self::ThreeFourth),
                4 => Ok(Self::SevenEighth),
                _ => Err(value),
            }
        }
    }
    impl ::bitstuff::ToBits for FIFOLevelSelect {
        fn to_bits(self) -> ::bitstuff::ints::u3 {
            ::bitstuff::ints::u3::trimmed_new(
                match self {
                    Self::OneEighth => 0,
                    Self::OneFourth => 1,
                    Self::OneHalf => 2,
                    Self::ThreeFourth => 3,
                    Self::SevenEighth => 4,
                },
            )
        }
    }
    impl ::bitstuff::BitRepr for FIFOLevelSelect {
        type BitRepr = ::bitstuff::ints::u3;
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FIFOLevelSelect {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    FIFOLevelSelect::OneEighth => "OneEighth",
                    FIFOLevelSelect::OneFourth => "OneFourth",
                    FIFOLevelSelect::OneHalf => "OneHalf",
                    FIFOLevelSelect::ThreeFourth => "ThreeFourth",
                    FIFOLevelSelect::SevenEighth => "SevenEighth",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for FIFOLevelSelect {
        #[inline]
        fn default() -> FIFOLevelSelect {
            Self::OneHalf
        }
    }
    /// The UARTIFLS Register; the interrupt FIFO level select register.
    ///
    /// You can use this register to define the FIFO level that triggers the assertion of UARTTXINTR and UARTRXINTR.
    ///
    /// The interrupts are generated based on a transition through a level rather than being based on the level. That is, the interrupts are generated when the fill level progresses through the trigger level.
    ///
    /// The bits are reset so that the trigger level is when the FIFOs are at the half-way mark.
    pub struct InterruptFIFOLevelSelectRegister(u16);
    impl InterruptFIFOLevelSelectRegister {
        /// Receive FIFO interrupt level select.
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn receive_interrupt_FIFO_level_select(
            &self,
        ) -> ::core::result::Result<FIFOLevelSelect, ::bitstuff::ints::u3> {
            <FIFOLevelSelect as ::bitstuff::TryFromBits>::try_from_bits(
                ::bitstuff::ints::u3::trimmed_new((self.0 >> 3) as _),
            )
        }
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn with_receive_interrupt_FIFO_level_select(
            mut self,
            value: FIFOLevelSelect,
        ) -> Self {
            let value: u16 = <FIFOLevelSelect as ::bitstuff::ToBits>::to_bits(value)
                .into();
            self.0 = (self.0 & !0b111000) | (value << 3u32);
            self
        }
        /// Transmit FIFO interrupt level select.
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn transmit_interrupt_FIFO_level_select(
            &self,
        ) -> ::core::result::Result<FIFOLevelSelect, ::bitstuff::ints::u3> {
            <FIFOLevelSelect as ::bitstuff::TryFromBits>::try_from_bits(
                ::bitstuff::ints::u3::trimmed_new((self.0 >> 0) as _),
            )
        }
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn with_transmit_interrupt_FIFO_level_select(
            mut self,
            value: FIFOLevelSelect,
        ) -> Self {
            let value: u16 = <FIFOLevelSelect as ::bitstuff::ToBits>::to_bits(value)
                .into();
            self.0 = (self.0 & !0b111) | (value << 0u32);
            self
        }
    }
    impl ::core::fmt::Debug for InterruptFIFOLevelSelectRegister {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("InterruptFIFOLevelSelectRegister")
                .field(
                    "receive_interrupt_FIFO_level_select",
                    &self.receive_interrupt_FIFO_level_select(),
                )
                .field(
                    "transmit_interrupt_FIFO_level_select",
                    &self.transmit_interrupt_FIFO_level_select(),
                )
                .finish()
        }
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
    pub struct InterruptMaskSetClearRegister(u16);
    impl InterruptMaskSetClearRegister {
        #[inline(always)]
        pub fn overrun_error_interrupt_mask(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 10) as _),
            )
        }
        #[inline(always)]
        pub fn with_overrun_error_interrupt_mask(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b10000000000) | (value << 10u32);
            self
        }
        #[inline(always)]
        pub fn break_error_interrupt_mask(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 9) as _),
            )
        }
        #[inline(always)]
        pub fn with_break_error_interrupt_mask(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b1000000000) | (value << 9u32);
            self
        }
        #[inline(always)]
        pub fn parity_error_interrupt_mask(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 8) as _),
            )
        }
        #[inline(always)]
        pub fn with_parity_error_interrupt_mask(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b100000000) | (value << 8u32);
            self
        }
        #[inline(always)]
        pub fn framing_error_interrupt_mask(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 7) as _),
            )
        }
        #[inline(always)]
        pub fn with_framing_error_interrupt_mask(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b10000000) | (value << 7u32);
            self
        }
        #[inline(always)]
        pub fn receive_timeout_interrupt_mask(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 6) as _),
            )
        }
        #[inline(always)]
        pub fn with_receive_timeout_interrupt_mask(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b1000000) | (value << 6u32);
            self
        }
        #[inline(always)]
        pub fn transmit_interrupt_mask(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 5) as _),
            )
        }
        #[inline(always)]
        pub fn with_transmit_interrupt_mask(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b100000) | (value << 5u32);
            self
        }
        #[inline(always)]
        pub fn receive_interrupt_mask(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 4) as _),
            )
        }
        #[inline(always)]
        pub fn with_receive_interrupt_mask(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b10000) | (value << 4u32);
            self
        }
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn nUARTDSR_modem_interrupt_mask(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 3) as _),
            )
        }
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn with_nUARTDSR_modem_interrupt_mask(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b1000) | (value << 3u32);
            self
        }
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn nUARTDCD_modem_interrupt_mask(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 2) as _),
            )
        }
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn with_nUARTDCD_modem_interrupt_mask(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b100) | (value << 2u32);
            self
        }
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn nUARTCTS_modem_interrupt_mask(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 1) as _),
            )
        }
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn with_nUARTCTS_modem_interrupt_mask(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b10) | (value << 1u32);
            self
        }
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn nUARTRI_modem_interrupt_mask(&self) -> bool {
            <bool as ::bitstuff::FromBits>::from_bits(
                ::bitstuff::ints::u1::trimmed_new((self.0 >> 0) as _),
            )
        }
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn with_nUARTRI_modem_interrupt_mask(mut self, value: bool) -> Self {
            let value: u16 = <bool as ::bitstuff::ToBits>::to_bits(value).into();
            self.0 = (self.0 & !0b1) | (value << 0u32);
            self
        }
    }
    impl ::core::fmt::Debug for InterruptMaskSetClearRegister {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("InterruptMaskSetClearRegister")
                .field(
                    "overrun_error_interrupt_mask",
                    &self.overrun_error_interrupt_mask(),
                )
                .field("break_error_interrupt_mask", &self.break_error_interrupt_mask())
                .field(
                    "parity_error_interrupt_mask",
                    &self.parity_error_interrupt_mask(),
                )
                .field(
                    "framing_error_interrupt_mask",
                    &self.framing_error_interrupt_mask(),
                )
                .field(
                    "receive_timeout_interrupt_mask",
                    &self.receive_timeout_interrupt_mask(),
                )
                .field("transmit_interrupt_mask", &self.transmit_interrupt_mask())
                .field("receive_interrupt_mask", &self.receive_interrupt_mask())
                .field(
                    "nUARTDSR_modem_interrupt_mask",
                    &self.nUARTDSR_modem_interrupt_mask(),
                )
                .field(
                    "nUARTDCD_modem_interrupt_mask",
                    &self.nUARTDCD_modem_interrupt_mask(),
                )
                .field(
                    "nUARTCTS_modem_interrupt_mask",
                    &self.nUARTCTS_modem_interrupt_mask(),
                )
                .field(
                    "nUARTRI_modem_interrupt_mask",
                    &self.nUARTRI_modem_interrupt_mask(),
                )
                .finish()
        }
    }
}
trait BaseAddress: Copy {
    fn base_address(self) -> usize;
}
impl BaseAddress for usize {
    fn base_address(self) -> usize {
        self
    }
}
struct FixedAddress<const BASE: usize>;
#[automatically_derived]
impl<const BASE: usize> ::core::fmt::Debug for FixedAddress<BASE> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "FixedAddress")
    }
}
#[automatically_derived]
impl<const BASE: usize> ::core::clone::Clone for FixedAddress<BASE> {
    #[inline]
    fn clone(&self) -> FixedAddress<BASE> {
        *self
    }
}
#[automatically_derived]
impl<const BASE: usize> ::core::marker::Copy for FixedAddress<BASE> {}
impl<const BASE: usize> BaseAddress for FixedAddress<BASE> {
    fn base_address(self) -> usize {
        BASE
    }
}
struct UART<T: BaseAddress> {
    base: T,
}
#[automatically_derived]
impl<T: ::core::fmt::Debug + BaseAddress> ::core::fmt::Debug for UART<T> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "UART",
            "base",
            &&self.base,
        )
    }
}
#[automatically_derived]
impl<T: ::core::clone::Clone + BaseAddress> ::core::clone::Clone for UART<T> {
    #[inline]
    fn clone(&self) -> UART<T> {
        UART {
            base: ::core::clone::Clone::clone(&self.base),
        }
    }
}
#[automatically_derived]
impl<T: ::core::marker::Copy + BaseAddress> ::core::marker::Copy for UART<T> {}
impl<T: BaseAddress> UART<T> {
    fn new(base: T) -> Self {
        UART { base }
    }
    fn read_register<R>(self, offset: usize) -> R
    where
        R: FromBits,
    {
        unsafe {
            let raw = (self.base.base_address() as *const u8).add(offset)
                as *const R::BitRepr;
            R::from_bits(raw.read_volatile())
        }
    }
    fn try_read_register<R>(self, offset: usize) -> Result<R, R::BitRepr>
    where
        R: TryFromBits,
    {
        unsafe {
            let raw = (self.base.base_address() as *const u8).add(offset)
                as *const R::BitRepr;
            R::try_from_bits(raw.read_volatile())
        }
    }
    fn data_register(&self) -> registrers::DataRegister {
        self.read_register(0x00)
    }
}

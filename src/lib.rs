//! this crate implements the registers of the [ARM PrimeCell UART (PL011)](https://developer.arm.com/documentation/ddi0183/g) peripheral
//! it doesn't implement the full peripheral, just the registers. Making it easier to write your own drivers :)

pub mod registrers;

/// This trait is used to get the base address of the peripheral.
/// by using a trait it can be a constant or a runtime value.
pub trait BaseAddress: Copy {
    fn base_address(self) -> usize;
}

// for a runtime value, we can use a usize
impl BaseAddress for usize {
    fn base_address(self) -> usize {
        self
    }
}

/// for a fixed address, we can use a struct with a const generic parameter
/// this is a zero-sized type, allowing is to use zero-sized and type-safe register blocks
#[derive(Debug, Clone, Copy)]
pub struct FixedAddress<const BASE: usize>;
impl<const BASE: usize> BaseAddress for FixedAddress<BASE> {
    fn base_address(self) -> usize {
        BASE
    }
}

/// This is the main struct for the UART peripheral.
///
/// It takes a base address as a generic parameter.
/// This allows us to use a fixed address or a runtime value.
/// When `T = FixedAddress<BASE>`, the base address is a constant and this is a zero-sized type.
/// When `T = usize`, the base address is a runtime value and this struct is the size of a pointer.
#[derive(Debug, Clone, Copy)]
pub struct UART<T: BaseAddress> {
    base: T,
}

//ideally we generate this with a macro
// whose input looks a little like this
// ```
// #[register_block]
// struct UART {
//     #[register(0x00, RW)]
//     data_register: DataRegister,
//     #[register(0x04, RO)]
//     receive_status_register: ReceiveStatusRegister,
//     #[register(0x04, ClearAll)]
//     error_clear_register: u32, //needs a size, but doesn't take an arg because it's a clear-all value
//     #[register(0x18, RO)]
//     flag_register: FlagRegister,
//     #[register(0x20, RW)]
//     irda_low_power_register: IrDALowPowerRegister,
// }
// ```

impl<T: BaseAddress> UART<T> {
    pub const fn new(base: T) -> Self {
        UART { base }
    }

    unsafe fn read_register<R>(self, offset: usize) -> R {
        unsafe {
            let raw = (self.base.base_address() as *const u8).add(offset) as *const R;
            raw.read_volatile()
        }
    }

    unsafe fn write_register<R>(self, offset: usize, value: R) {
        unsafe {
            let raw = (self.base.base_address() as *mut u8).add(offset) as *mut R;
            raw.write_volatile(value);
        }
    }

    unsafe fn update_register<R, F>(self, offset: usize, f: F)
    where
        F: FnOnce(R) -> R,
    {
        unsafe { self.write_register::<R>(offset, f(self.read_register::<R>(offset))) };
    }

    /// data register, read/write, offset 0x00
    pub fn read_data_register(&self) -> registrers::DataRegister {
        unsafe { self.read_register(0x00) }
    }

    pub fn write_data_register(&self, value: registrers::DataRegister) {
        unsafe { self.write_register(0x00, value) }
    }

    pub fn update_data_register<F>(&self, f: F)
    where
        F: FnOnce(registrers::DataRegister) -> registrers::DataRegister,
    {
        unsafe { self.update_register(0x00, f) };
    }

    /// Receive Status Register, read-only, offset 0x04
    pub fn read_receive_status_register(&self) -> registrers::ReceiveStatusRegister {
        unsafe { self.read_register(0x04) }
    }

    /// Error clear register, write-only, offset 0x04
    // no value, just a clear-all but it does need to know the size of the register
    pub fn write_error_clear_register(&self) {
        unsafe { self.write_register(0x04, 0u32) }
    }

    /// Flag register, read-only, offset 0x18
    pub fn read_flag_register(&self) -> registrers::FlagRegister {
        unsafe { self.read_register(0x18) }
    }

    /// irda low power register, read/write, offset 0x20
    pub fn read_irda_low_power_register(&self) -> registrers::IrDALowPowerRegister {
        unsafe { self.read_register(0x20) }
    }
    pub fn write_irda_low_power_register(&self, value: registrers::IrDALowPowerRegister) {
        unsafe { self.write_register(0x20, value) }
    }
    pub fn update_irda_low_power_register<F>(&self, f: F)
    where
        F: FnOnce(registrers::IrDALowPowerRegister) -> registrers::IrDALowPowerRegister,
    {
        unsafe { self.update_register(0x20, f) };
    }

    /// Integer Baud Rate Divisor Register, read/write, offset 0x24
    pub fn read_integer_baud_rate_divisor_register(
        &self,
    ) -> registrers::IntegerBaudRateDivisorRegister {
        unsafe { self.read_register(0x24) }
    }
    pub fn write_integer_baud_rate_divisor_register(
        &self,
        value: registrers::IntegerBaudRateDivisorRegister,
    ) {
        unsafe { self.write_register(0x24, value) }
    }
    pub fn update_integer_baud_rate_divisor_register<F>(&self, f: F)
    where
        F: FnOnce(
            registrers::IntegerBaudRateDivisorRegister,
        ) -> registrers::IntegerBaudRateDivisorRegister,
    {
        unsafe { self.update_register(0x24, f) };
    }

    /// Fractional Baud Rate Divisor Register, read/write, offset 0x28
    pub fn read_fractional_baud_rate_divisor_register(
        &self,
    ) -> registrers::FractionalBaudRateDivisorRegister {
        unsafe { self.read_register(0x28) }
    }
    pub fn write_fractional_baud_rate_divisor_register(
        &self,
        value: registrers::FractionalBaudRateDivisorRegister,
    ) {
        unsafe { self.write_register(0x28, value) }
    }
    pub fn update_fractional_baud_rate_divisor_register<F>(&self, f: F)
    where
        F: FnOnce(
            registrers::FractionalBaudRateDivisorRegister,
        ) -> registrers::FractionalBaudRateDivisorRegister,
    {
        unsafe { self.update_register(0x28, f) };
    }

    /// Line Control Register, read/write, offset 0x2C
    pub fn read_line_control_register(&self) -> registrers::LineControlRegister {
        unsafe { self.read_register(0x2C) }
    }
    pub fn write_line_control_register(&self, value: registrers::LineControlRegister) {
        unsafe { self.write_register(0x2C, value) }
    }
    pub fn update_line_control_register<F>(&self, f: F)
    where
        F: FnOnce(registrers::LineControlRegister) -> registrers::LineControlRegister,
    {
        unsafe { self.update_register(0x2C, f) };
    }

    /// Control Register, read/write, offset 0x30
    pub fn read_control_register(&self) -> registrers::ControlRegister {
        unsafe { self.read_register(0x30) }
    }
    pub fn write_control_register(&self, value: registrers::ControlRegister) {
        unsafe { self.write_register(0x30, value) }
    }
    pub fn update_control_register<F>(&self, f: F)
    where
        F: FnOnce(registrers::ControlRegister) -> registrers::ControlRegister,
    {
        unsafe { self.update_register(0x30, f) };
    }

    /// Interrupt FIFO Level Select Register, read/write, offset 0x34
    pub fn read_interrupt_fifo_level_select_register(
        &self,
    ) -> registrers::InterruptFIFOLevelSelectRegister {
        unsafe { self.read_register(0x34) }
    }
    pub fn write_interrupt_fifo_level_select_register(
        &self,
        value: registrers::InterruptFIFOLevelSelectRegister,
    ) {
        unsafe { self.write_register(0x34, value) }
    }
    pub fn update_interrupt_fifo_level_select_register<F>(&self, f: F)
    where
        F: FnOnce(
            registrers::InterruptFIFOLevelSelectRegister,
        ) -> registrers::InterruptFIFOLevelSelectRegister,
    {
        unsafe { self.update_register(0x34, f) };
    }

    /// Interrupt Mask Set/Clear Register, read/write, offset 0x38
    pub fn read_interrupt_mask_set_clear_register(
        &self,
    ) -> registrers::InterruptMaskSetClearRegister {
        unsafe { self.read_register(0x38) }
    }
    pub fn write_interrupt_mask_set_clear_register(
        &self,
        value: registrers::InterruptMaskSetClearRegister,
    ) {
        unsafe { self.write_register(0x38, value) }
    }
    pub fn update_interrupt_mask_set_clear_register<F>(&self, f: F)
    where
        F: FnOnce(
            registrers::InterruptMaskSetClearRegister,
        ) -> registrers::InterruptMaskSetClearRegister,
    {
        unsafe { self.update_register(0x38, f) };
    }

    /// Raw Interrupt Status Register, read-only, offset 0x3C
    pub fn read_raw_interrupt_status_register(&self) -> registrers::RawInterruptStatusRegister {
        unsafe { self.read_register(0x3C) }
    }

    /// Masked Interrupt Status Register, read-only, offset 0x40
    pub fn read_masked_interrupt_status_register(
        &self,
    ) -> registrers::MaskedInterruptStatusRegister {
        unsafe { self.read_register(0x40) }
    }

    /// Interrupt Clear Register, write-only, offset 0x44
    pub fn write_interrupt_clear_register(&self, value: registrers::InterruptClearRegister) {
        unsafe { self.write_register(0x44, value) }
    }

    /// DMA Control Register, read/write, offset 0x48
    pub fn read_dma_control_register(&self) -> registrers::DMAControlRegister {
        unsafe { self.read_register(0x48) }
    }
    pub fn write_dma_control_register(&self, value: registrers::DMAControlRegister) {
        unsafe { self.write_register(0x48, value) }
    }
    pub fn update_dma_control_register<F>(&self, f: F)
    where
        F: FnOnce(registrers::DMAControlRegister) -> registrers::DMAControlRegister,
    {
        unsafe { self.update_register(0x48, f) };
    }
}

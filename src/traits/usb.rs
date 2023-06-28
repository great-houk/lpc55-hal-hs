use core::ops::Deref;

#[derive(Copy, Clone)]
pub enum UsbPeripheral {
    USB0,
    USB1,
}

pub trait Usb<State>: Deref<Target = crate::raw::usb1::RegisterBlock> + Sync {
    const USB: UsbPeripheral;
}

extern crate core_foundation;
extern crate core_foundation_sys;
extern crate libc;

use std::mem;


use core_foundation::base::{CFType, TCFType};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef, CFMutableDictionary};
use core_foundation::number::CFNumber;
use core_foundation::string::CFString;
use core_foundation_sys::base::kCFAllocatorDefault;
use core_foundation_sys::dictionary::CFMutableDictionaryRef;
use core_foundation_sys::mach_port::CFAllocatorRef;
use libc::c_char;
use mach::kern_return::{self, kern_return_t};
use mach::port::{mach_port_t, MACH_PORT_NULL};
use IOKit_sys::{io_iterator_t, io_object_t, io_registry_entry_t, IOOptionBits};

extern "C" {
    pub static kIOMasterPortDefault: mach_port_t;

    pub fn IOMasterPort(bootstrapPort: mach_port_t, masterPort: *mut mach_port_t) -> kern_return_t;

    pub fn IOServiceMatching(name: *const c_char) -> CFMutableDictionaryRef;

    pub fn IOServiceGetMatchingServices(
        masterPort: mach_port_t,
        matching: CFDictionaryRef,
        existing: *mut io_iterator_t,
    ) -> kern_return_t;

    pub fn IOObjectRelease(object: io_object_t) -> kern_return_t;

    pub fn IOIteratorNext(iterator: io_iterator_t) -> io_object_t;

    pub fn IORegistryEntryCreateCFProperties(
        entry: io_registry_entry_t,
        properties: *mut CFMutableDictionaryRef,
        allocator: CFAllocatorRef,
        options: IOOptionBits,
    ) -> kern_return::kern_return_t;

}

fn main() {
    let mut master_port: mach_port_t = MACH_PORT_NULL;
    unsafe {
        // TODO: Handle the possible error
        let _result = IOMasterPort(kIOMasterPortDefault, &mut master_port);

        let match_dict = IOServiceMatching(b"IOPMPowerSource\0".as_ptr() as *const c_char);
        let mut iterator: io_iterator_t = mem::uninitialized();

        // TODO: Handle the possible error
        let _result = IOServiceGetMatchingServices(master_port, match_dict, &mut iterator);

        let battery_obj = IOIteratorNext(iterator);

        let mut props: CFMutableDictionaryRef = mem::uninitialized();

        let _result =
            IORegistryEntryCreateCFProperties(battery_obj, &mut props, kCFAllocatorDefault, 0);

        let properties: CFDictionary<CFString, CFType> =
            CFMutableDictionary::wrap_under_create_rule(props).to_immutable();
        let key = CFString::from_static_string("CurrentCapacity");

        let current_capacity = properties
            .find(key)
            .and_then(|value_ref| value_ref.downcast::<CFNumber>())
            .and_then(|value| value.to_i32())
            .expect("Unable to find the Current Capacity");

        println!("{}", current_capacity)

        //properties.show();
    }
}

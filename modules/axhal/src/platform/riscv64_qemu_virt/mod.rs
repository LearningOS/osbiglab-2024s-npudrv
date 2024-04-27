mod boot;

pub mod console;
pub mod mem;
pub mod misc;
pub mod time;

#[cfg(feature = "irq")]
pub mod irq;

#[cfg(feature = "smp")]
pub mod mp;

extern "C" {
    fn trap_vector_base();
    fn rust_main(cpu_id: usize, dtb: usize);
    fn main(); // somewhere, could be the application
    #[cfg(feature = "smp")]
    fn rust_main_secondary(cpu_id: usize);
}

unsafe extern "C" fn rust_entry(cpu_id: usize, _dtb: usize) {
    crate::mem::clear_bss();
    crate::cpu::init_primary(cpu_id);
    crate::arch::set_trap_vector_base(trap_vector_base as usize);
    // we might just skip initializations in
    // axruntime/src/lib.rs rust_main
    // and directly calls the app.
    // rust_main(cpu_id, dtb);
    main();
    // this is what rust_main does for uni-program OS.
    self::misc::terminate();
}

#[cfg(feature = "smp")]
unsafe extern "C" fn rust_entry_secondary(cpu_id: usize) {
    crate::arch::set_trap_vector_base(trap_vector_base as usize);
    crate::cpu::init_secondary(cpu_id);
    rust_main_secondary(cpu_id);
}

/// Initializes the platform devices for the primary CPU.
///
/// For example, the interrupt controller and the timer.
pub fn platform_init() {
    #[cfg(feature = "irq")]
    self::irq::init_percpu();
    self::time::init_percpu();
}

/// Initializes the platform devices for secondary CPUs.
#[cfg(feature = "smp")]
pub fn platform_init_secondary() {
    #[cfg(feature = "irq")]
    self::irq::init_percpu();
    self::time::init_percpu();
}

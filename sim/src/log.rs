// // https://stackoverflow.com/a/34539114

// use crate::cpu;

// #[cfg(debug_assertions)]
// #[macro_export]
// macro_rules! info {
//     ($($arg:tt)+) => {
//         ::log::info!($($arg)+);
//     };
// }

// #[cfg(debug_assertions)]
// #[macro_export]
// macro_rules! debug {
//     ($($arg:tt)+) => {
//         ::log::debug!($($arg)+);
//     };
// }

// #[cfg(debug_assertions)]
// #[macro_export]
// macro_rules! error {
//     ($($arg:tt)+) => {
//         ::log::error!($($arg)+);
//     };
// }

// #[cfg(debug_assertions)]
// #[macro_export]
// macro_rules! trace {
//     ($($arg:tt)+) => {
//         ::log::trace!($($arg)+);
//     };
// }

// #[cfg(debug_assertions)]
// #[macro_export]
// macro_rules! warn {
//     ($($arg:tt)+) => {
//         ::log::warn!($($arg)+);
//     };
// }

// #[cfg(debug_assertions)]
// #[inline(always)]
// pub fn trace_insn(pc: u64, bits: u64, name: &str, form: cpu::InsnType) {
//     crate::trace!(
//         "core   0: {:#016x} ({:#08x}) {}     {}",
//         pc,
//         bits as u32,
//         name,
//         form
//     );
// }

// // non debug

// #[cfg(not(debug_assertions))]
// #[macro_export]
// macro_rules! info {
//     ($($arg:tt)+) => {};
// }

// #[cfg(not(debug_assertions))]
// #[macro_export]
// macro_rules! debug {
//     ($($arg:tt)+) => {};
// }

// #[cfg(not(debug_assertions))]
// #[macro_export]
// macro_rules! error {
//     ($($arg:tt)+) => {};
// }

// #[cfg(not(debug_assertions))]
// #[macro_export]
// macro_rules! trace {
//     ($($arg:tt)+) => {};
// }

// #[cfg(not(debug_assertions))]
// #[macro_export]
// macro_rules! warn {
//     ($($arg:tt)+) => {};
// }

// #[cfg(not(debug_assertions))]
// #[macro_export]
// pub fn trace_insn(name: &str, form: cpu::InsnType) {}

// https://stackoverflow.com/a/34539114

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {
        ::log::info!($($arg)+);
    };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => {
        ::log::debug!($($arg)+);
    };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => {
        ::log::error!($($arg)+);
    };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => {
        ::log::trace!($($arg)+);
    };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => {
        ::log::warn!($($arg)+);
    };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! trace_insn {
    ($insn:expr $(, $argname:ident = $argval:expr)* $(,)?) => {{
        crate::trace!(
            concat!(
                "[{}]",
                $(
                    " ", stringify!($argname), "=", "{}",
                )*
            ),
            $insn
            $(, $argval)*
        );
    }};
}

// non debug

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => {};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => {};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => {};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => {};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! trace_insn {
    ($insn:expr, $($argname:ident = $argval:expr),* $(,)?) => {};
}

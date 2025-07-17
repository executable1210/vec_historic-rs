#[macro_export]
macro_rules! vec_historic {
    () => {
        $crate::VecHistoric::from_data($crate::gap_buffer![])
    };
    ($($elem:expr),+ $(,)?) => {{
        $crate::VecHistoric::from_data($crate::gap_buffer![$($elem),*])
    }};
}
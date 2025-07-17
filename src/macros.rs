#[macro_export]
macro_rules! vec_historic {
    () => {
        VecHistoric::from_data(gapbuf::gap_buffer![])
    };
    ($($elem:expr),+ $(,)?) => {{
        VecHistoric::from_data(gapbuf::gap_buffer![$($elem),*])
    }};
}

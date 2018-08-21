#[macro_export]
macro_rules! environment(
    { $($key:expr => $value:expr),* } => {
        {
            use env::EnvValues;
            use procedure::ProcedureData;
            let mut m = EnvValues::new();
            $(m.insert($key.to_string(), ProcedureData::new_primitive($value));)*
            m
        }
    };
);

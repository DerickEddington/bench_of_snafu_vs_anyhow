#![macro_use] // Make this module's macros visible to all following modules.


#[rustfmt::skip]
macro_rules! impl_StdError
{
    ($ty:ty) =>
    {
        impl std::error::Error for $ty {}

        impl std::fmt::Display for $ty {
            fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { unreachable!() }
        }
    };
}


macro_rules! module_group
{
    (name = $name:literal; benches = $( $bench:path ),+ $(,)?) =>
    {
        const MODULE_GROUP_NAME: &str = $name;

        criterion_group! {
            name = group;
            config = criterion_config();
            targets = $( $bench ),+
        }
    }
}


#[rustfmt::skip]
macro_rules! bench_group_named
{
    (($manager:ident) name = $name:literal; ($group:ident)
     log-scale = $log_scale:literal;) =>
    {
        let group_name = format!("{}::{}", MODULE_GROUP_NAME, $name);
        let mut $group = $manager.benchmark_group(group_name);
        if $log_scale {
            $group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
        }
    }
}


macro_rules! bench_group
{
    (($manager:ident) name = $name:literal;
     routines = [$( $id:literal = $func:expr ),* $(,)? ];
     log-scale = $log_scale:literal;) =>
    {
        bench_group_named!(($manager) name = $name; (group) log-scale = $log_scale;);
        $( group.bench_function($id, |bencher| bencher.iter($func)); )+
    };

    (($manager:ident) name = $name:literal;
     routines = [$( $id:literal = $func:expr ),* $(,)? ];) =>
    {
        bench_group! { ($manager) name = $name;
            routines = [$( $id = $func ),*];
            log-scale = false;
        }
    };

    (($manager:ident) name = $name:literal;) =>
    {
        bench_group! { ($manager) name = $name;
            routines = ["Anyhow" = anyhow, "Snafu" = snafu];
        }
    }
}

use {
    anyhow::anyhow,
    criterion::{
        criterion_group,
        criterion_main,
        AxisScale,
        BenchmarkId,
        Criterion,
        PlotConfiguration,
    },
    my_whatever::MyWhatever,
    std::{
        error::Error as StdError,
        hint::black_box as bb,
        mem::size_of,
    },
};

mod helpers; // Must be before all modules that use its macros.

mod create_type;

mod dyn_trait_obj;


fn criterion_config() -> Criterion
{
    #[cfg(feature = "noisy_PC")]
    {
        // Note: Don't use this for comparing changes to the source-code (to either these benches
        // or the libs) that are expected to have only minor effects on performance.  Only use for
        // comparing successive runs of the same source-code, or for changes expected to have
        // major effects, on your PC that is "noisy" due to various other processes running, due
        // to your variable CPU-frequency and fan management, etc.
        Criterion::default().sample_size(1_000).noise_threshold(0.10).significance_level(0.01)
    }
    #[cfg(not(feature = "noisy_PC"))]
    {
        Criterion::default()
    }
}


criterion_main!(create_type::group, dyn_trait_obj::group);

use super::*;

module_group! {
    name = "Dynamic-Trait-Object";
    benches = core, snafu_fat_vs_anyhow_thin
}


#[inline(never)]
fn more<T>(x: T) -> T
{
    bb(x)
}


/// Fundamental uses of passing (as return value or as argument value) fat `dyn Trait` type
/// vs. thin static type, without use of the error-helping libraries.
fn core(c: &mut Criterion)
{
    bench_group! { (c) name = "Core";
        routines = [
            "Thin" = || drop(bb(thin())),
            "Fat" = || drop(bb(fat())),
            "Thin-More" = || drop(bb(thin_more())),
            "Fat-More" = || drop(bb(fat_more())),
        ];
    }

    #[derive(Debug)]
    struct MyError;
    impl_StdError! { MyError }

    assert!(size_of::<MyError>() == 0); // So Box::new(MyError) won't allocate.
    assert!(size_of::<Box<MyError>>() == size_of::<usize>()); // It's still a pointer.
    assert!(size_of::<Box<dyn StdError>>() == 2 * size_of::<usize>()); // Still fat.

    macro_rules! e {
        () => {
            bb(Box::new(bb(MyError)))
        };
    }

    fn thin() -> Result<(), Box<MyError>>
    {
        Err(e!())
    }

    fn fat() -> Result<(), Box<dyn StdError>>
    {
        Err(e!())
    }

    fn thin_more() -> Result<(), Box<MyError>>
    {
        bb(more(bb(Err(bb(more::<Box<MyError>>(e!()))))))
    }

    fn fat_more() -> Result<(), Box<dyn StdError>>
    {
        bb(more(bb(Err(bb(more::<Box<dyn StdError>>(e!()))))))
    }
}


/// Passing (as return value or as argument value) SNAFU's fat `Box<dyn StdError>` type in its
/// fatter `Whatever` type vs. Anyhow's thin-box type.
fn snafu_fat_vs_anyhow_thin(c: &mut Criterion)
{
    // These avoid needing creation in the first two bench'ed routines.
    let mut ae: Option<anyhow::Error> = Some(anyhow!(""));
    #[allow(clippy::useless_format)]
    let mut se: Option<MyWhatever> = Some(snafu::FromString::without_source(format!("")));

    bench_group! { (c) name = "Snafu-Fat-Vs-Anyhow-Thin";
        routines = [
            "Anyhow" = || ae = bb(anyhow(bb(ae.take().unwrap()))).err(),
            "Snafu" = || se = bb(snafu(bb(se.take().unwrap()))).err(),
            "Anyhow-Create" = || drop(bb(anyhow_create())),
            "Snafu-Create" = || drop(bb(snafu_create())),
        ];
    }

    assert!(size_of::<anyhow::Error>() == size_of::<usize>());

    // Comparable to `Thin-More`.
    fn anyhow(e: anyhow::Error) -> Result<(), anyhow::Error>
    {
        bb(more(bb(Err(bb(more(bb(e)))))))
    }

    fn anyhow_create() -> Result<(), anyhow::Error>
    {
        let e: anyhow::Error = anyhow!("");
        anyhow(e)
    }

    assert!(size_of::<MyWhatever>() >= 2 * size_of::<usize>());

    // Comparable to `Fat-More`.
    fn snafu(e: MyWhatever) -> Result<(), MyWhatever>
    {
        bb(more(bb(Err(bb(more(bb(e)))))))
    }

    fn snafu_create() -> Result<(), MyWhatever>
    {
        #[allow(clippy::useless_format)]
        let e: MyWhatever = snafu::FromString::without_source(format!(""));
        snafu(e)
    }
}

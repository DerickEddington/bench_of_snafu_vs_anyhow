//! Bench various ways of constructing a value of each library's "dyn Error"-holding type, as
//! similarly as possible, like a user would use via their high-level.

use super::*;

module_group! {
    name = "Create-Type";
    benches = empty_msg, lit_msg, fmt_msg, auto_return, chain
}


fn empty_msg(c: &mut Criterion)
{
    bench_group! { (c) name = "Empty-Message"; }

    fn anyhow()
    {
        // `anyhow!` has a disadvantage in that it boxes (moves to a new heap allocation) the
        // message value as the error value.
        let e: anyhow::Error = anyhow!("");
        bb(e);
    }

    fn snafu()
    {
        // This `format!("")` creates an empty `String` and that doesn't allocate.
        #[allow(clippy::useless_format)]
        let e: MyWhatever = snafu::FromString::without_source(format!(""));
        bb(e);
    }
}


fn lit_msg(c: &mut Criterion)
{
    bench_group! { (c) name = "Literal-Message"; }

    fn anyhow()
    {
        // `anyhow!` has an advantage in that it can avoid allocating a `String` for a literal
        // message that doesn't need formatting.  But it has a disadvantage in that it boxes
        // (moves to a new heap allocation) the message value as the error value.
        let e: anyhow::Error = anyhow!("blah");
        bb(e);
    }

    fn snafu()
    {
        // This `format!` does allocate a non-empty `String`.
        #[allow(clippy::useless_format)]
        let e: MyWhatever = snafu::FromString::without_source(format!("blah"));
        bb(e);
    }
}


fn fmt_msg(c: &mut Criterion)
{
    bench_group_named! { (c) name = "Format-Message"; (group) log-scale = true; }

    let powers = (0 ..= 9).step_by(9);
    let powers_of_ten = powers.map(|p| 10_u32.checked_pow(p).unwrap());

    fn anyhow(i: &u32)
    {
        let e: anyhow::Error = anyhow!("i: {}", i);
        bb(e);
    }

    fn snafu(i: &u32)
    {
        let e: MyWhatever = snafu::FromString::without_source(format!("i: {}", i));
        bb(e);
    }

    #[allow(clippy::unit_arg)]
    for i in powers_of_ten {
        group.bench_with_input(BenchmarkId::new("Anyhow", i), &i, |b, i| b.iter(|| anyhow(i)));
        group.bench_with_input(BenchmarkId::new("Snafu", i), &i, |b, i| b.iter(|| snafu(i)));
    }
}


fn auto_return(c: &mut Criterion)
{
    bench_group! { (c) name = "Automatic-Return";
        routines = ["Anyhow" = || drop(bb(anyhow())), "Snafu" = || drop(bb(snafu()))];
    }

    fn anyhow() -> Result<(), anyhow::Error>
    {
        anyhow::bail!("blah");
    }

    fn snafu() -> Result<(), MyWhatever>
    {
        snafu::whatever!("blah");
    }
}


fn chain(c: &mut Criterion)
{
    bench_group! { (c) name = "Chain"; }

    #[derive(Debug)]
    struct MyError(#[allow(dead_code)] i32);
    impl_StdError! { MyError }

    const E: Result<(), MyError> = Err(MyError(123));

    fn anyhow()
    {
        use anyhow::Context as _;

        let e: Result<(), anyhow::Error> = E.context("blah");
        let _ = bb(e);
    }

    fn snafu()
    {
        use snafu::ResultExt as _;

        let e: Result<(), MyWhatever> = E.whatever_context("blah");
        let _ = bb(e);
    }
}

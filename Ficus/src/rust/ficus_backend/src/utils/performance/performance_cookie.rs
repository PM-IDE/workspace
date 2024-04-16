use stopwatch::Stopwatch;

pub trait PerformanceLogger<TErr> {
    fn log(&self, message: String) -> Result<(), TErr>;
}

pub fn performance_cookie<TErr>(
    activity_name: &str,
    logger: &impl PerformanceLogger<TErr>,
    action: &mut impl FnMut() -> Result<(), TErr>,
) -> Result<(), TErr> {
    let mut stopwatch = Stopwatch::start_new();
    logger.log(format!("Started activity {}", activity_name))?;
    (action)()?;

    stopwatch.stop();
    let elapsed = stopwatch.elapsed_ms();

    logger.log(format!("Activity {} finished in {}ms", activity_name, elapsed))?;
    Ok(())
}

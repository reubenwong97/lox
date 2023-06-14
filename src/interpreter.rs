use std::{error::Error, path::Path};

pub fn run_file<P>(path: P) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    Ok(())
}

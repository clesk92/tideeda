use bitblast::Aig;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn write_aig(_aig: &Aig, _path: &Path) -> Result<(), ExportError> {
    // Placeholder for AIG export
    Ok(())
}

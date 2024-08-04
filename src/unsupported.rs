use crate::DisplayInfo;
use anyhow::Result;

pub type ScreenRawHandle = ();

pub fn get_all() -> Result<Vec<DisplayInfo>> {
    Err(anyhow::anyhow!("Platform not supported"))
}

pub fn get_from_point(_x: i32, _y: i32) -> Result<DisplayInfo> {
    Err(anyhow::anyhow!("Platform not supported"))
}

//! * Dummy Driver for the LSM9DS1 accelerometer
//!
//! This is just a placeholder so the the docs build without RTIMULib.

use super::ImuData;
use std::marker::PhantomData;

#[derive(Debug)]
pub enum Error {
    RTIMULibError,
}

pub(crate) struct Lsm9ds1<'a> {
    phantom: PhantomData<&'a u32>,
}

impl<'a> Lsm9ds1<'a> {
    /// Uses the `RTIMULib` library.
    pub(crate) fn new() -> Result<Lsm9ds1<'a>, Error> {
        Ok(Lsm9ds1 {
            phantom: PhantomData,
        })
    }

    /// Make the IMU do some work. When this function returns true, the IMU
    /// has data we can fetch with `get_imu_data()`.
    pub(crate) fn imu_read(&mut self) -> bool {
        false
    }

    pub(crate) fn set_fusion(&mut self) {}

    pub(crate) fn set_compass_only(&mut self) {}

    pub(crate) fn set_gyro_only(&mut self) {}

    pub(crate) fn set_accel_only(&mut self) {}

    pub(crate) fn get_imu_data(&mut self) -> Result<ImuData, Error> {
        Err(Error::RTIMULibError)
    }
}

// Normally, you would include the protos with this:
// `tonic::include_proto!("ciao.time_tracking");`
// But since we use a custom out_dir (see build.rs), we need to use
// `include!()` with the path (relative to this file)

include!("../../target/generated/ciao/ciao.devices.rs");

impl Device {
    /// Creates an empty, unknown [Device] with a `time_zone_id`
    ///
    /// # Example
    /// ```
    /// let now = &chrono::Utc::now().naive_utc();
    /// let device = ciao_rs::ciao::devices::Device::with_time_zone("Etc/UTC");
    /// // FIXME DEVICES-24 we need to think about, if ths is needed!
    /// // assert_eq!(0, device.get_time_zone_offset(now).unwrap_or_default());
    /// ```
    pub fn with_time_zone(time_zone_id: &str) -> Device {
        Device {
            id: "".to_string(),
            external_id: "".to_string(),
            name: "".to_string(),
            r#type: DeviceType::Unknown.into(),
            time_zone_id: Some(time_zone_id.to_string()),
            actions: vec![],
        }
    }

    /// Returns a device action from an id
    ///
    /// # Arguments
    /// * `device_action_id`: The id of the device action
    ///
    pub fn find_action_by_device_action_id(&self, device_action_id: u8) -> Option<&DeviceAction> {
        self.actions
            .iter()
            .find(|action| action.device_action_id == device_action_id as i32)
    }

    // FIXME DEVICES-24 we need to think about, if ths is needed!

    // /// Returns the offset in seconds to the given time
    // ///
    // /// # Example
    // /// The offset during DST in Europe/Berlin must be 7200 seconds
    // /// ```
    // /// let summer = chrono::NaiveDateTime::new(
    // ///     chrono::NaiveDate::from_ymd_opt(2024, 06, 01).unwrap(),
    // ///     chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
    // /// );
    // /// let device = ciao_rs::ciao::devices::Device::with_time_zone("Europe/Berlin");
    // /// assert_eq!(7200, device.get_time_zone_offset(&summer).unwrap_or_default());
    // /// ```
    // ///
    // pub fn get_time_zone_offset(&self, time: &NaiveDateTime) -> Option<i64> {
    //     if let Some(ref time_zone_id) = self.time_zone_id {
    //         return match time_zone_id.parse::<Tz>() {
    //             Ok(tz) => {
    //                 let local_time = &tz.from_utc_datetime(time);
    //                 let offset = local_time.offset();
    //                 Some(offset.base_utc_offset().num_seconds() + offset.dst_offset().num_seconds())
    //             }
    //             Err(e) => {
    //                 error!("Cannot parse {} to time zone id: {}", time_zone_id, e);
    //                 None
    //             }
    //         };
    //     }
    //     None
    // }

    // /// Adds the offset of the time zone of this device to the timestamp of the [ClockRecord]
    // /// The offset is only added, if the device has a `time_zone_id`
    // ///
    // /// <div class="warning">If the clock_record already has an offset, it will be overwriten</div>
    // ///
    // pub fn fix_timestamp(&self, clock_record: &mut ClockRecord) {
    //     if let Some(offset) = self.get_time_zone_offset(&Utc::now().naive_utc()) {
    //         // Device has a time zone id. Fix the offset of the clock record
    //         if let Some(ref mut timestamp) = clock_record.timestamp {
    //             let offset = Duration {
    //                 seconds: offset,
    //                 nanos: 0,
    //             };

    //             // If the device has an offset, we must subtract it from the terminal time,
    //             // because the backend will add it again. Since the terminal does not send TZ
    //             // info, we assume it is UTC
    //             //
    //             // Subtract offset, if timestamp.time_utc has a value
    //             timestamp.time_utc = timestamp.time_utc.map(|t| timestamp_minus(&t, &offset));
    //             timestamp.offset = Some(offset);
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests;

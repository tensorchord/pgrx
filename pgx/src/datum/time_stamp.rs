use crate::datum::time::USECS_PER_SEC;
use crate::{direct_function_call_as_datum, pg_sys, FromDatum, IntoDatum, TimestampWithTimeZone};
use std::ops::{Deref, DerefMut};
use time::PrimitiveDateTime;

pub struct Timestamp(time::PrimitiveDateTime);
impl FromDatum<Timestamp> for Timestamp {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, typoid: u32) -> Option<Timestamp> {
        let ts: Option<TimestampWithTimeZone> =
            TimestampWithTimeZone::from_datum(datum, is_null, typoid);
        match ts {
            None => None,
            Some(ts) => {
                let date = ts.date();
                let time = ts.time();

                Some(Timestamp(PrimitiveDateTime::new(date, time)))
            }
        }
    }
}
impl IntoDatum<Timestamp> for Timestamp {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let year = self.year();
        let month = self.month() as i32;
        let mday = self.day() as i32;
        let hour = self.hour() as i32;
        let minute = self.minute() as i32;
        let second = self.second() as f64 + (self.microsecond() as f64 / USECS_PER_SEC as f64);

        direct_function_call_as_datum(
            pg_sys::make_timestamp,
            vec![
                year.into_datum(),
                month.into_datum(),
                mday.into_datum(),
                hour.into_datum(),
                minute.into_datum(),
                second.into_datum(),
            ],
        )
    }
}
impl Timestamp {
    pub fn new(timestamp: time::PrimitiveDateTime) -> Self {
        Timestamp(timestamp)
    }
}

impl Deref for Timestamp {
    type Target = time::PrimitiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Timestamp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl serde::Serialize for Timestamp {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.format("%Y-%m-%dT%T+00:00"))
    }
}

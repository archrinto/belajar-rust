// Datetime menggunakan chrono
// DateTime<Utc> DateTime<Local>
// NaiveDateTime

use chrono::prelude::*;
use std::time::{UNIX_EPOCH, Duration};

fn main() {
    let date1_utc: DateTime<Utc> = Utc::now();
    println!("utc: {date1_utc}");

    let date2_utc = Utc.with_ymd_and_hms(2024, 3, 1, 1, 2, 3).unwrap();
    println!("utc: {date2_utc}");
    
    let date3_utc = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(1524885322));
    println!("utc: {date3_utc}");
    
    let data4_utc = "2023-03-01 01:02:03 UTC".parse::<DateTime<Utc>>().unwrap();
    println!("utc: {data4_utc}");

    // NaiveDateTime
    let timestamp: u64 = 1524885322;
    let date2_in_utc = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(timestamp));
    let date2_in_local_tz = DateTime::<Local>::from(UNIX_EPOCH + Duration::from_secs(timestamp));

    let timestamp_in_ms: i64 = 1524885322000;
    let naive_date_time: NaiveDateTime = NaiveDateTime::from_timestamp_millis(timestamp_in_ms).unwrap();

    let sample_date_in_utc = Utc.from_utc_datetime(&naive_date_time);
    let sample_date_in_local_tz = Local.from_local_datetime(&naive_date_time).unwrap();

    println!("sample date 1 (in utc):      {sample_date_in_utc}");
    println!("sample date 2 (in local_tz): {sample_date_in_local_tz}");

    // Konversi datetime
    let date1_in_local_tz: DateTime<Local> = Local::now();
    println!("date (in local): {date1_in_local_tz}");

    let date_in_utc = DateTime::<Utc>::from(date1_in_local_tz);
    println!("date (in utc):   {date_in_utc}");

    let date2_in_local_tz = DateTime::<Local>::from(date_in_utc);
    println!("date (in local): {date2_in_local_tz}");

    let date_in_local_tz: DateTime<Local> = Local::now();
    println!("date: {date_in_local_tz}");
    println!("date (in second timestamp):      {}", date_in_local_tz.timestamp());
    println!("date (in milisecond timestamp):  {}", date_in_local_tz.timestamp_millis());
    println!("date (in microsecond timestamp): {}", date_in_local_tz.timestamp_micros());

    // format date
    let date1: DateTime<Local> = Local::now();
    println!("date1 (in local):  {}", date1);
    // date1 (in local):  2023-03-02 18:13:39.954831600 +07:00

    let str_from_date1 = date1.format("%Y-%m-%d %H:%M:%S %z").to_string();
    println!("date1 (in string): {}", str_from_date1);
    // date1 (in string): 2023-03-02 18:13:39 +0700

    let date1_from_str = Local.datetime_from_str(&str_from_date1, "%Y-%m-%d %H:%M:%S %z").unwrap();
    println!("date1 (in local):  {}", date1_from_str);
    // date1 (in local):  2023-03-02 18:13:39 +07:00

    let date2_from_str = Utc.datetime_from_str("03/01/2023 13:04 +0000", "%m/%d/%Y %H:%M %z").unwrap();
    println!("date2 (in utc):    {}", date2_from_str);
    // date2 (in utc):    2023-03-01 13:04:00 UTC

    let str_from_date2 = date2_from_str.format("%Y-%m-%d %H:%M:%S %z").to_string();
    println!("date2 (in string): {}", str_from_date2);
    // date2 (in string): 2023-03-01 13:04:00 +0000
}

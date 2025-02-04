use chrono::{NaiveDate, NaiveTime};
use Agenda::{calendar::Calendar, utils::{appointment::Appointment, urgency::Urgency}};

fn main() {
    let mut calendar = Calendar::new(vec![2023, 2024, 2025, 2026]);
    let year:i32 = 2025;
    let month: u32 = 2;
    let appointment = Appointment::new(
        NaiveDate::from_ymd_opt(2025, 2, 4).unwrap(), 
        "Zlobec's Meeting".to_owned(), 
        Some("University".to_owned()), 
        Some(Urgency::High), 
        NaiveTime::from_hms_milli_opt(3, 0, 0, 0)
    );
    calendar.add_appointment(appointment);
    calendar.show(year, month);
}  

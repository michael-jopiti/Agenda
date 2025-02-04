// Calendar contains years, which contain months, which contain days
// Calendar contains pointers to dates with rendez-vous
// Calendar is able to be seen in terminaé
    // lists all things to do
    // sorts them by urgence
    // sorts them by topic
    // sorts them by time to completion

use crate::utils::year::Year;
use crate::utils::appointment::Appointment;

use chrono::Datelike;

#[derive(Default, Debug)]
pub struct Calendar {
    // year is vectors of months
    // total years = vector of year
    pub years: Vec<Year>
}

impl Calendar {
    pub fn new(years: Vec<i32>) -> Self {
        Calendar {
            years: years.iter().map(|&year| Year::new(year.into())).collect(),
        }
    }

    pub fn add_appointment(&mut self, appointment: Appointment) {
        let year = appointment.date().year();
        let month = appointment.date().month();
    
        if let Some(year_obj) = self.years.iter_mut().find(|y| *y.year() == year) {
            if let Some(month_obj) = year_obj.months().iter_mut().find(|m| m.name().as_u32() == month) {
                if let Some(day_obj) = month_obj.days().iter_mut().find(|d| d.date() == appointment.date()){
                    day_obj.set_appointment(appointment);
                }
            }
        }
    }

    pub fn show(&mut self, year: i32, month: u32) {
        if let Some(year_obj) = self.years.iter_mut().find(|y| *y.year() == year) {
            if let Some(month_obj) = year_obj.months().iter_mut().find(|m| m.name().as_u32() == month) {
                println!("\n📅 {} {}\n", month_obj.name().name(), year);
    
                for day in month_obj.days() {
                    println!("📆 {}", day.day());
                    for appointment in day.get_appointments() {
                        println!("\t|-> Appointment: {:?}", appointment);
                    }
                }
            } else {
                println!("❌ Month {} not found in year {}", month, year);
            }
        } else {
            println!("❌ Year {} not found", year);
        }
    }
}
    
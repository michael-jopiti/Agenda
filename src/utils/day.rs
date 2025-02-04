use chrono::NaiveDate;
use crate::utils::appointment::Appointment;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Day{
    pub date: NaiveDate,
    pub appointments: Vec<Appointment>
}

impl Day{
    pub fn new(date: NaiveDate) -> Self{
        Day{
            date,
            appointments: Vec::new(),
        }
    }

    pub fn day(&mut self) -> &mut NaiveDate{
        &mut self.date
    }

    pub fn date(&self) -> &NaiveDate{
        &self.date
    }

    pub fn set_appointment(&mut self, appointment: Appointment){
        self.appointments.push(appointment)
    }

    pub fn get_appointments(&mut self) -> &mut Vec<Appointment> {
        &mut self.appointments
    }

    pub fn quanitity(&self) -> usize{
        self.appointments.len()
    }
}
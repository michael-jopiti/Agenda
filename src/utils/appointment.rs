use chrono::{NaiveDate, NaiveTime};
use crate::utils::urgency::Urgency;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Appointment{
    date: NaiveDate,
    title: String,
    topic: Option<String>,
    urgency: Option<Urgency>,
    delta: Option<NaiveTime>
}


impl Appointment {
    pub fn new(date: NaiveDate, title: String, topic: Option<String>, urgency: Option<Urgency>, delta: Option<NaiveTime>) -> Self {
        Appointment {
            date,
            title,
            topic,
            urgency,
            delta,
        }
    }

    pub fn date(&self) -> &NaiveDate{
        &self.date
    }

    pub fn title(&self) -> &str{
        &self.title
    } 

    pub fn topic(&self) -> &Option<String>{
        &self.topic
    }

    pub fn urgency(&self) -> &Option<Urgency>{
        &self.urgency
    }

    pub fn delta(&self) -> &Option<NaiveTime>{
        &self.delta
    }
}
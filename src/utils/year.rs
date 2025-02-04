use super::month::{Month, MonthName};

#[derive(Default, Debug, PartialEq)]
pub struct Year{
    year: i32,
    months: Vec<Month>
}

impl Year{
    pub fn new(year:i32) -> Self{
        Year{
            year, 
            months: Self::months_per_year(year),
        }
    }

    fn months_per_year(year: i32) -> Vec<Month>{
        vec![
            Month::new(MonthName::January, year),
            Month::new(MonthName::February, year),
            Month::new(MonthName::March, year),
            Month::new(MonthName::April, year),
            Month::new(MonthName::May, year),
            Month::new(MonthName::June, year),
            Month::new(MonthName::July, year),
            Month::new(MonthName::August, year),
            Month::new(MonthName::September, year),
            Month::new(MonthName::October, year),
            Month::new(MonthName::November, year),
            Month::new(MonthName::December, year),
        ]
    }

    pub fn year(&self) -> &i32{
        &self.year
    }

    pub fn months(&mut self) -> &mut Vec<Month>{
        &mut self.months
    }

}



#[derive(Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    pub weekday: WeekDay,
    pub start_time: Time,
    pub end_time: Time,
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeekDay { Mon, Tues, Wed, Thur, Fri, Sat, Sun }

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Section {
    pub number: u32,
    pub instructors: Vec<String>,
    pub time_slots: Vec<TimeSlot>,
    pub preference: Option<f32>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct ScheduleItem {
    pub id: u32,
    pub name: String,
    pub ty: String,
    pub sections: Vec<Section>,
    pub preference: Option<f32>,
}

impl ::std::cmp::PartialEq for ScheduleItem {
    fn eq(&self, other: &Self) -> bool {other.id == self.id}
}

impl ::std::cmp::Eq for ScheduleItem {}

pub type Schedule = (f32, Vec<ScheduleItem>);
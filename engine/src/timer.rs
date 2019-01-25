use std::time::SystemTime;

pub struct Timer
{
    time: SystemTime
}

impl Timer
{
    pub fn new() -> Timer
    {
        Timer { time: SystemTime::now() }
    }

    pub fn get_time(&self) -> f32
    {
        let duration = self.time.elapsed().unwrap();
        let sec = duration.as_secs();
        let nsec = duration.subsec_nanos();

        return (sec as f32) + ((nsec as f32) / 1000000000.0);
    }

    pub fn reset(&mut self)
    {
        self.time = SystemTime::now();
    }
}

pub enum Shot{
    Bullseye,
    Hit(f64),
    Miss,
}

impl Shot{
    pub fn points(self) -> i32
    {
        match self
        {
            Self::Bullseye => 5,
            Self::Hit(x) if x < 3.0 => 2,
            Self::Hit(x) => 1,
            Self::Miss => 0,
        }
    }
}
use super::bite::Bite;

#[derive(Debug)]
pub struct Carrot{
    pub percent_left: f32,
}

impl Bite for Carrot{
    fn bite(self:&mut Self)
    {
        // Eat 20% of the remaining carrot/
        self.percent_left *= 0.8;
    }
}
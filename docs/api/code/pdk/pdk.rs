pub struct ExamplePdk;

impl Pdk for ExamplePdk {
    type Layers = ExamplePdkLayers;
    type Corner = ExamplePdkCorner;
    fn corner(&self, name: &str) -> Option<Self::Corner> {
        match name {
            "tt" => Some(ExamplePdkCorner::Tt),
            "ss" => Some(ExamplePdkCorner::Ss),
            "ff" => Some(ExamplePdkCorner::Ff),
            _ => None,
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub enum ExamplePdkCorner {
    Tt,
    Ss,
    Ff,
};

impl Corner for ExamplePdkCorner {
    fn name(&self) -> arcstr::ArcStr {
        match *self {
            Self::Tt => arcstr::literal!("tt"),
            Self::Ff => arcstr::literal!("ff"),
            Self::Ss => arcstr::literal!("ss"),
        }
    }
}

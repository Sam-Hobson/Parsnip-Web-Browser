pub struct Stylesheet {
    rules: Vec<Rule>,
}

pub struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}

pub enum Selector {
    Simple(SimpleSelector),
}

pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

pub struct Declaration {
    name: String,
    value: Value,
}

pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColourValue(Colour),
}

pub enum Unit {
    Px,
}

pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        // http://www.w3.org/TR/selectors/#specificity
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        (a, b, c)
    }
}
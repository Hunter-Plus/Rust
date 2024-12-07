pub trait Tweeting {
    fn tweet(&self) -> String;

    // default implementation using non-default method
    fn loud_tweet(&self) -> String {
        format!("{}!!!", self.tweet())
    }
}

pub trait Greeting {
    fn hello(&self) -> String;
}

// default implementation
pub trait Warning {
    fn warn(&self) -> String {
        String::from("huhuhuhuhu~")
    }
}

pub struct Squirrel {
    pub name: String,
    pub legs: i32,
    pub tone: String,
}
impl Tweeting for Squirrel {
    fn tweet(&self) -> String {
        format!("{}, zhizhizhi", self.tone)
    }
}
impl Greeting for Squirrel {
    fn hello(&self) -> String {
        format!("zhizhi, {}, gugu, {}", self.name, self.tone)
    }
}
impl Warning for Squirrel {
    // default impl
}

pub struct ParirieDog {
    pub name: String,
    pub tail: i32,
    pub tone: String,
}

impl Tweeting for ParirieDog {
    fn tweet(&self) -> String {
        format!("{}!{}!{}!", self.tone, self.tone, self.tone)
    }
}

impl Greeting for ParirieDog {
    fn hello(&self) -> String {
        format!("Yaho!{}, {}", self.name, self.tone)
    }
}

impl Warning for ParirieDog {
    // overriding the default impl
    fn warn(&self) -> String {
        String::from("Ahhh!Ahhh!Ahhh!")
    }
}

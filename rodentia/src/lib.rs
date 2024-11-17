mod rodent_services {
    pub mod bird_feeder {
        use crate::raining;

        pub fn add_food() {}

        fn spinning() {}
    }
    // using super
    fn call_for_rain() {
        super::raining();
    }
}

fn raining() {}

pub mod water {
    use crate::rodent_services;

    fn hydrating() {}

    fn skating() {}

    fn using_paths() {
        // absolute path
        crate::rodent_services::bird_feeder::add_food();
        // relative path
        rodent_services::bird_feeder::add_food();
    }
}

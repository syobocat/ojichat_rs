use std::{collections::HashMap, sync::LazyLock};
pub static TAG_TYPE: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    let mut tag_type = HashMap::new();
    tag_type.insert("first_person", "FIRST_PERSON");
    tag_type.insert("day_of_week", "DAY_OF_WEEK");
    tag_type.insert("location", "LOCATION");
    tag_type.insert("restaurant", "RESTAURANT");
    tag_type.insert("food", "FOOD");
    tag_type.insert("weather", "WEATHER");
    tag_type.insert("nanchatte", "NANCHATTE");
    tag_type.insert("hotel", "HOTEL");
    tag_type.insert("date", "DATE");
    tag_type.insert("metaphor", "METAPHOR");
    tag_type
});

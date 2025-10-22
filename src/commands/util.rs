use itertools::Itertools;
use std::fmt::Display;
use std::ops::Deref;

pub fn channels_to_string<I, T>(channels: I) -> String
where
    I: Iterator<Item = T>,
    T: Display + Deref<Target = i64>,
{
    channels.map(|c| format!("<#{c}>")).join(" ")
}

pub fn roles_to_string<I, T>(roles: I) -> String
where
    I: Iterator<Item = T>,
    T: Display + Deref<Target = i64>,
{
    roles.map(|c| format!("<@&{c}>")).join(" ")
}

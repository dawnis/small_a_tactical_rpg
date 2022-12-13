use nannou::prelude::*;
use crate::soots::arthropods::Wasp;

pub struct SFactory<'a> {
    api: Option<&'a Draw>,
}

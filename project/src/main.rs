use std::io::Read;

use splat::segment::*;
use splat::*;

pub struct RustSegment {
    data: Vec<u8>,
}

impl Segment for RustSegment {
    fn output(&self) -> Vec<u8> {
        self.data
            .iter()
            .map(|e| format!("{e:02X}"))
            .collect::<Vec<_>>()
            .join(" ")
            .into_bytes()
    }
}

pub struct RustSegmentBuilder;

impl SegmentBuilder for RustSegmentBuilder {
    fn parse<T: Read>(reader: &mut T) -> impl Segment
    where
        Self: Sized,
    {
        let mut data = vec![];
        let _ = reader.read_to_end(&mut data);
        RustSegment { data }
    }

    fn name(&self) -> String {
        "rust".into()
    }
}

fn main() {
    let mut segments = default_segments();
    segments.push(&RustSegmentBuilder);

    split(&segments);
}

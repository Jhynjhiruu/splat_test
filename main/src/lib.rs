use segment::SegmentBuilder;

pub mod segment;

pub fn split(segments: &[&dyn SegmentBuilder]) {
    for seg in segments {
        println!("{}", seg.name());
    }
}

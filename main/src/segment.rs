use std::io::Read;

pub trait Segment {
    fn output(&self) -> Vec<u8>;
}

pub trait SegmentBuilder {
    fn parse<T: Read>(reader: &mut T) -> impl Segment
    where
        Self: Sized;
    fn name(&self) -> String;
}

pub struct AsmSegment {
    data: Vec<u8>,
}

impl Segment for AsmSegment {
    fn output(&self) -> Vec<u8> {
        self.data
            .iter()
            .map(|e| format!("{e:02X}"))
            .collect::<Vec<_>>()
            .join(" ")
            .into_bytes()
    }
}

pub struct AsmSegmentBuilder;

impl SegmentBuilder for AsmSegmentBuilder {
    fn parse<T: Read>(reader: &mut T) -> impl Segment
    where
        Self: Sized,
    {
        let mut data = vec![];
        let _ = reader.read_to_end(&mut data);
        AsmSegment { data }
    }

    fn name(&self) -> String {
        "asm".into()
    }
}

pub struct CSegment {
    data: Vec<u8>,
}

impl Segment for CSegment {
    fn output(&self) -> Vec<u8> {
        self.data
            .iter()
            .map(|e| format!("{e:02X}"))
            .collect::<Vec<_>>()
            .join(" ")
            .into_bytes()
    }
}

pub struct CSegmentBuilder;

impl SegmentBuilder for CSegmentBuilder {
    fn parse<T: Read>(reader: &mut T) -> impl Segment
    where
        Self: Sized,
    {
        let mut data = vec![];
        let _ = reader.read_to_end(&mut data);
        CSegment { data }
    }

    fn name(&self) -> String {
        "c".into()
    }
}

pub fn default_segments() -> Vec<&'static dyn SegmentBuilder> {
    vec![&AsmSegmentBuilder, &CSegmentBuilder]
}

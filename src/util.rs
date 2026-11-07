//! ffmpeg model — multimedia framework.

#[derive(Debug, Clone)]
pub struct Stream {
    pub type_: String,
    pub codec: String,
}

#[derive(Debug, Clone)]
pub struct Conversion {
    pub input: String,
    pub output: String,
    pub streams: Vec<Stream>,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub conversions: Vec<Conversion>,
}

impl Default for Model {
    fn default() -> Self { Self { conversions: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn convert(&mut self, input: &str, output: &str, streams: Vec<Stream>) -> bool {
        self.conversions.push(Conversion { input: input.into(), output: output.into(), streams, status: "queued".into() });
        true
    }

    pub fn probe(&self, file: &str) -> Option<&Conversion> {
        self.conversions.iter().find(|c| c.input == file || c.output == file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn convert_probe() {
        let mut f = Model::new();
        assert!(f.convert("in.mp4", "out.mkv", vec![Stream { type_: "video".into(), codec: "h264".into() }]));
        assert!(f.probe("in.mp4").is_some());
    }
}

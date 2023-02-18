use crate::dimensions::Dimensions;

#[derive(Debug)]
pub struct TranscoderJob {
    input_uri: String,
    output_uri: String,
    new_dimensions: Dimensions,
    audio_codec: Option<String>,
}

impl TranscoderJob {
    pub fn new(
        input_uri: String,
        output_uri: String,
        new_dimensions: Dimensions,
        audio_codec: Option<String>,
    ) -> TranscoderJob {
        TranscoderJob {
            input_uri,
            output_uri,
            new_dimensions,
            audio_codec,
        }
    }

    pub fn input_uri(&self) -> &String {
        &self.input_uri
    }

    pub fn output_uri(&self) -> &String {
        &self.output_uri
    }

    pub fn new_dimensions(&self) -> &Dimensions {
        &self.new_dimensions
    }

    pub fn audio_codec(&self) -> &Option<String> {
        &self.audio_codec
    }
}

use anyhow::{anyhow, Result};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    js_sys::ArrayBuffer, AudioBuffer, AudioBufferSourceNode, AudioContext, AudioDestinationNode,
    AudioNode,
};

pub fn creat_audio_context() -> Result<AudioContext> {
    AudioContext::new().map_err(|err| anyhow!("Could not create audio context: {:#?}", err))
}

fn create_buffer_source(context: &AudioContext) -> Result<AudioBufferSourceNode> {
    context
        .create_buffer_source()
        .map_err(|err| anyhow!("Error creating buffer source {:#?}", err))
}

fn connect_with_audio_node(
    buffer_source: &AudioBufferSourceNode,
    destination: &AudioDestinationNode,
) -> Result<AudioNode> {
    buffer_source
        .connect_with_audio_node(&destination)
        .map_err(|err| anyhow!("Error connecting audio source to destination {:#?}", err))
}

pub fn create_track_source(
    context: &AudioContext,
    buffer: &AudioBuffer,
) -> Result<AudioBufferSourceNode> {
    let track_source = create_buffer_source(context)?;
    track_source.set_buffer(Some(&buffer));
    connect_with_audio_node(&track_source, &context.destination())?;
    Ok(track_source)
}
pub enum LOOPING {
    NO,
    YES,
}

pub fn play_sound(
    context: &AudioContext,
    buffer: &AudioBuffer,
    looping: LOOPING,
) -> Result<AudioBufferSourceNode> {
    let track_source = create_track_source(context, buffer)?;
    if matches!(looping, LOOPING::YES) {
        track_source.set_loop(true);
    }

    track_source
        .start()
        .map_err(|err| anyhow!("Could not start sound!{:#?}", err))?;
    Ok(track_source)
}

pub async fn decode_auto_data(
    context: &AudioContext,
    array_buffer: &ArrayBuffer,
) -> Result<AudioBuffer> {
    JsFuture::from(
        context
            .decode_audio_data(&array_buffer)
            .map_err(|err| anyhow!("Could not decode audio from array buffer {:#?}", err))?,
    )
    .await
    .map_err(|err| anyhow!("Could not convert promise to future {:#?}", err))?
    .dyn_into()
    .map_err(|err| anyhow!("Could not cast into AudioBuffer {:#?}", err))
}

extern crate sawblade;
use sawblade::audio::AudioContext;

fn main() {
    let mut audio_context = AudioContext::new();
    audio_context.load("test.ogg", "test");
    audio_context.play_on_new_thread("test").unwrap();

}
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

pub struct AudioPlayer {
    sink: Arc<Mutex<Sink>>,
    _stream: OutputStream,
    victory_played: Arc<Mutex<bool>>, 
}

impl AudioPlayer {
    pub fn new(music_file: &str) -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let file = BufReader::new(File::open(music_file).unwrap());
        let source = Decoder::new(file).unwrap();
        
        let looped_source = source.repeat_infinite();
        sink.append(looped_source);
        sink.set_volume(0.5);

        AudioPlayer {
            sink: Arc::new(Mutex::new(sink)),
            _stream: stream,
            victory_played: Arc::new(Mutex::new(false)), 
        }
    }

    pub fn play(&self) {
        self.sink.lock().unwrap().play();
    }

    pub fn stop(&self) {
        self.sink.lock().unwrap().stop();
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.lock().unwrap().set_volume(volume);
    }

    pub fn play_sound_effect(&self, sound_file: &str, volume: f32) {
        let victory_played = *self.victory_played.lock().unwrap();
        if !victory_played {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
    
            let file = BufReader::new(File::open(sound_file).unwrap());
            let source = Decoder::new(file).unwrap();
    
            let louder_source = source.amplify(volume);
            sink.append(louder_source);
            
            sink.sleep_until_end();
    
            *self.victory_played.lock().unwrap() = true;
        }
    }
    

    pub fn reset_victory(&self) {
        *self.victory_played.lock().unwrap() = false;
    }
}

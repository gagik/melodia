extern crate alsa;
extern crate rust_gpiozero;

use alsa::mixer::{Mixer, Selem, SelemChannelId, SelemId};

struct Sound<'a> {
    selem: Selem<'a>,
    default_channel: SelemChannelId,
    range: (f64, f64)
}

impl<'a> Sound<'a> {
    fn new(mixer: &'a Mixer, selem_name: &'a str, channel: SelemChannelId) -> Sound<'a> {
        let selem_id = SelemId::new(selem_name, 0);
        let selem = match mixer.find_selem(&selem_id) {
            Some(selem) => selem,
            None => panic!("No salem found."),
        };
        let range = selem.get_playback_volume_range();
        let range = (range.0 as f64, range.1 as f64);
        
        Sound {
            selem,
            range,
            default_channel: channel,
        }
    }
    fn get_volume(&self) -> i64 {
        match self.selem.get_playback_volume(self.default_channel) {
            Ok(vol) => vol,
            Err(err) => panic!("Volume access error. {}", err),
        }
    }
    fn get_volume_percent(&self) -> f64 {
        let volume = self.get_volume();
        self.range.0 + volume as f64 / self.range.1 * 100.0
    }
    fn set_volume(&self, volume: i64) -> () {
        match self.selem.set_playback_volume_all(volume) {
            Ok(_) => (),
            Err(err) => panic!("Volume set error. {}", err),
        }
    }
    fn set_volume_percent(&self, volume: f64) -> () {
        let absolute_volume: f64 = (volume * self.range.1 / 100.0) - self.range.0;
        // This might rounddown, causing some inaccuracy but nothing catastrophic.
        self.set_volume(absolute_volume as i64)
    }
}

fn main() {
    let mixer = match Mixer::new("hw:0", false) {
        Ok(mixer) => mixer,
        Err(err) => panic!(err),
    };
    let speaker = Sound::new(&mixer, "Speaker", SelemChannelId::FrontLeft);

    speaker.set_volume(0);

    print!("Current volume: {} or {}%\n", speaker.get_volume(), speaker.get_volume_percent());

    let new_volume = 12;
    print!("Changing volume to: {}\n", new_volume);

    speaker.set_volume(new_volume);
    speaker.set_volume_percent(speaker.get_volume_percent());

    print!("New volume: {} or {}%\n", speaker.get_volume(), speaker.get_volume_percent());
}

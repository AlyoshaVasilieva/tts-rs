#[cfg(target_os = "android")]
use std::sync::Mutex;

use lazy_static::lazy_static;
use log::info;

use crate::{Backend, BackendId, Error, Features, UtteranceId, CALLBACKS};

lazy_static! {
    static ref NEXT_BACKEND_ID: Mutex<u64> = Mutex::new(0);
}

#[derive(Clone, Debug)]
pub(crate) struct Android(BackendId);

impl Android {
    pub(crate) fn new() -> Self {
        info!("Initializing Android backend");
        let mut backend_id = NEXT_BACKEND_ID.lock().unwrap();
        let bid = BackendId::Android(*backend_id);
        *backend_id += 1;
        Self(bid)
    }
}

impl Backend for Android {
    fn id(&self) -> Option<BackendId> {
        Some(self.0)
    }

    fn supported_features(&self) -> Features {
        Features {
            stop: false,
            rate: false,
            pitch: false,
            volume: false,
            is_speaking: false,
            utterance_callbacks: false,
        }
    }

    fn speak(&mut self, text: &str, interrupt: bool) -> Result<Option<UtteranceId>, Error> {
        println!("Speaking {}, {:?}", text, interrupt);
        Ok(None)
    }

    fn stop(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn min_rate(&self) -> f32 {
        todo!()
    }

    fn max_rate(&self) -> f32 {
        todo!()
    }

    fn normal_rate(&self) -> f32 {
        todo!()
    }

    fn get_rate(&self) -> Result<f32, Error> {
        todo!()
    }

    fn set_rate(&mut self, rate: f32) -> Result<(), Error> {
        todo!()
    }

    fn min_pitch(&self) -> f32 {
        todo!()
    }

    fn max_pitch(&self) -> f32 {
        todo!()
    }

    fn normal_pitch(&self) -> f32 {
        todo!()
    }

    fn get_pitch(&self) -> Result<f32, Error> {
        todo!()
    }

    fn set_pitch(&mut self, pitch: f32) -> Result<(), Error> {
        todo!()
    }

    fn min_volume(&self) -> f32 {
        todo!()
    }

    fn max_volume(&self) -> f32 {
        todo!()
    }

    fn normal_volume(&self) -> f32 {
        todo!()
    }

    fn get_volume(&self) -> Result<f32, Error> {
        todo!()
    }

    fn set_volume(&mut self, volume: f32) -> Result<(), Error> {
        todo!()
    }

    fn is_speaking(&self) -> Result<bool, Error> {
        todo!()
    }
}
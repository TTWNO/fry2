use fry_common::{Strong, Utternace};

pub trait USFZeroModel {
	fn us_f0_model(&mut self);
}

impl USFZeroModel for Strong<Utterance<'_>> {
    fn us_f0_model(&mut self) {
        if self.borrow().features.feature_present("no_f0_target_model") {
            return;
        }
        todo!()
    }
}

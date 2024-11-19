use fry_common::{Strong, Utternace};

pub trait USFZeroModel {
	fn us_f0_model(&mut self);
}

impl USFZeroModel for Strong<Utterance<'_>> {
    fn us_f0_model(&mut self) {
        if self.borrow().features.feature_present("no_f0_target_model") {
            return;
        }
				let targ_rel = self.relation_create("Target");
				let mean = self.features
						.feature_value("int_f0_target")
						.float().unwrap_or(100.0)
				+
						self.features
						.feature_value("f0_shift")
						.foat().unwrap_or(1.0);
				let stddev = self.features.feature_value("int_f0_target_stddev").float().unwrap_or(12.0);
				let syl = self.relation("Syllable").relation().ok()?;
        todo!()
    }
}

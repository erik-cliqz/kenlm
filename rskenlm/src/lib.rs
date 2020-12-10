#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![crate_type = "lib"]

mod bindings;
pub mod kenlm;

#[cfg(test)]
mod tests {
    use kenlm::KenLM;

    #[test]
    fn lm_load() {
        let kenlm_model = KenLM::from_file("src/test.arpa").unwrap();
        println! {"Score : {:?}", kenlm_model.perplexity("screening a little")};
    }
}

use std::collections::HashMap;

#[derive(Default)]
pub struct CodonsInfo<'a> {
    info: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        for (key, values) in self.info.iter() {
            for val in values {
                if val == &codon {
                    return Some(key);
                }
            }
        }

        return None;
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut proteins: Vec<&str> = vec![];
        let mut rnas = vec![];
        let mut rna = rna;
        let mut flag = false;

        while rna.len() > 3 {
            let (first, second) = rna.split_at(3);

            rnas.push(first);

            if second.len() == 3 {
                rnas.push(second);
            }

            rna = second;
        }

        println!("{rnas:?}");

        for (key, values) in self.info.iter() {
            for (index, rna_val) in rnas.clone().iter().enumerate() {
                if *rna_val == "UAA" {
                    let (left, _right) = rnas.split_at(index);
                    rnas = left.to_vec();
                    flag = true;
                }
                if values.contains(&rna_val) {
                    proteins.push(*key);
                }
            }
        }

        println!("{proteins:?}");

        if proteins.is_empty() || (!flag && rna.len() > 0)   {
            return None;
        }

        proteins.sort_by(|&a, &b| a.cmp(b));

        return Some(proteins);
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for (codon, rna) in pairs {
        map.entry(rna)
            .and_modify(|codons| codons.push(codon))
            .or_insert(vec![codon]);
    }

    CodonsInfo { info: map }
}

#[cfg(test)]
mod tests {
    use crate::protein_translation as proteins;

    #[test]
    fn test_methionine() {
        let info = proteins::parse(make_pairs());
        assert_eq!(info.name_for("AUG"), Some("methionine"));
    }

    #[test]
    fn test_cysteine_tgt() {
        let info = proteins::parse(make_pairs());
        assert_eq!(info.name_for("UGU"), Some("cysteine"));
    }

    #[test]
    fn test_stop() {
        let info = proteins::parse(make_pairs());
        assert_eq!(info.name_for("UAA"), Some("stop codon"));
    }

    #[test]
    fn test_valine() {
        let info = proteins::parse(make_pairs());
        assert_eq!(info.name_for("GUU"), Some("valine"));
    }

    #[test]
    fn test_isoleucine() {
        let info = proteins::parse(make_pairs());
        assert_eq!(info.name_for("AUU"), Some("isoleucine"));
    }

    #[test]
    fn test_arginine_name() {
        let info = proteins::parse(make_pairs());
        assert_eq!(info.name_for("CGA"), Some("arginine"));
        assert_eq!(info.name_for("AGA"), Some("arginine"));
        assert_eq!(info.name_for("AGG"), Some("arginine"));
    }

    #[test]
    fn empty_is_invalid() {
        let info = proteins::parse(make_pairs());
        assert!(info.name_for("").is_none());
    }

    #[test]
    fn x_is_not_shorthand_so_is_invalid() {
        let info = proteins::parse(make_pairs());
        assert!(info.name_for("VWX").is_none());
    }

    #[test]
    fn too_short_is_invalid() {
        let info = proteins::parse(make_pairs());
        assert!(info.name_for("AU").is_none());
    }

    #[test]
    fn too_long_is_invalid() {
        let info = proteins::parse(make_pairs());
        assert!(info.name_for("ATTA").is_none());
    }

    #[test]
    fn test_translates_rna_strand_into_correct_protein() {
        let info = proteins::parse(make_pairs());
        assert_eq!(
            info.of_rna("AUGUUUUGG"),
            Some(vec!["methionine", "phenylalanine", "tryptophan"])
        );
    }

    #[test]
    fn test_stops_translation_if_stop_codon_present() {
        let info = proteins::parse(make_pairs());
        assert_eq!(
            info.of_rna("AUGUUUUAA"),
            Some(vec!["methionine", "phenylalanine"])
        );
    }

    #[test]
    fn test_stops_translation_of_longer_strand() {
        let info = proteins::parse(make_pairs());
        assert_eq!(
            info.of_rna("UGGUGUUAUUAAUGGUUU"),
            Some(vec!["tryptophan", "cysteine", "tyrosine"])
        );
    }

    #[test]
    fn test_invalid_codons() {
        let info = proteins::parse(make_pairs());
        assert!(info.of_rna("CARROT").is_none());
    }
    #[test]
    fn test_invalid_length() {
        let info = proteins::parse(make_pairs());
        assert!(info.of_rna("AUGUA").is_none());
    }
    #[test]
    fn test_valid_stopped_rna() {
        let info = proteins::parse(make_pairs());
        assert_eq!(info.of_rna("AUGUAAASDF"), Some(vec!["methionine"]));
    }
    // The input data constructor. Returns a list of codon, name pairs.
    fn make_pairs() -> Vec<(&'static str, &'static str)> {
        let grouped = vec![
            ("isoleucine", vec!["AUU", "AUC", "AUA"]),
            ("valine", vec!["GUU", "GUC", "GUA", "GUG"]),
            ("phenylalanine", vec!["UUU", "UUC"]),
            ("methionine", vec!["AUG"]),
            ("cysteine", vec!["UGU", "UGC"]),
            ("alanine", vec!["GCU", "GCC", "GCA", "GCG"]),
            ("glycine", vec!["GGU", "GGC", "GGA", "GGG"]),
            ("proline", vec!["CCU", "CCC", "CCA", "CCG"]),
            ("threonine", vec!["ACU", "ACC", "ACA", "ACG"]),
            ("serine", vec!["AGU", "AGC"]),
            ("tyrosine", vec!["UAU", "UAC"]),
            ("tryptophan", vec!["UGG"]),
            ("glutamine", vec!["CAA", "CAG"]),
            ("asparagine", vec!["AAU", "AAC"]),
            ("histidine", vec!["CAU", "CAC"]),
            ("glutamic acid", vec!["GAA", "GAG"]),
            ("aspartic acid", vec!["GAU", "GAC"]),
            ("lysine", vec!["AAA", "AAG"]),
            ("arginine", vec!["CGU", "CGC", "CGA", "CGG", "AGA", "AGG"]),
            ("stop codon", vec!["UAA", "UAG", "UGA"]),
        ];
        let mut pairs = Vec::<(&'static str, &'static str)>::new();
        for (name, codons) in grouped.into_iter() {
            for codon in codons {
                pairs.push((codon, name));
            }
        }
        pairs.sort_by(|&(_, a), &(_, b)| a.cmp(b));
        pairs
    }
}

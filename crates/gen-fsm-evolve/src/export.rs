use gen_fsm::FsmDna;
use serde_json;

pub struct DnaExporter;

impl DnaExporter {
    pub fn to_json<const S: usize, const C: usize>(dna: &FsmDna<S, C>) -> String {
        let flat = dna.to_flat_vec();

        let mut contexts: Vec<Vec<Vec<f32>>> = Vec::new();
        for ctx in 0..C {
            let mut matrix: Vec<Vec<f32>> = Vec::new();
            for row in 0..S {
                let start = ctx * S * S + row * S;
                let end = start + S;
                matrix.push(flat[start..end].to_vec());
            }
            contexts.push(matrix);
        }

        serde_json::to_string_pretty(&contexts).unwrap_or_default()
    }

    pub fn to_binary<const S: usize, const C: usize>(dna: &FsmDna<S, C>) -> Vec<u8> {
        let flat = dna.to_flat_vec();
        let mut bytes = Vec::with_capacity(flat.len() * 4);
        for val in &flat {
            bytes.extend_from_slice(&val.to_le_bytes());
        }
        bytes
    }

    pub fn to_rust_const<const S: usize, const C: usize>(
        dna: &FsmDna<S, C>,
        const_name: &str,
    ) -> String {
        let flat = dna.to_flat_vec();
        let mut out = String::new();

        out.push_str("use gen_fsm::FsmDna;\n\n");
        out.push_str(&format!(
            "pub const {}: [f32; {}] = [\n",
            const_name,
            flat.len()
        ));

        for ctx in 0..C {
            for row in 0..S {
                out.push_str("    ");
                for col in 0..S {
                    let idx = ctx * S * S + row * S + col;
                    out.push_str(&format!("{:>8.6}, ", flat[idx]));
                }
                out.push_str("\n");
            }
            out.push('\n');
        }

        out.push_str("];\n");
        out
    }

    pub fn to_c_header<const S: usize, const C: usize>(
        dna: &FsmDna<S, C>,
        array_name: &str,
    ) -> String {
        let flat = dna.to_flat_vec();
        let mut out = String::new();

        out.push_str("#ifndef FSM_DNA_H\n#define FSM_DNA_H\n\n");
        out.push_str(&format!("#define FSM_STATES {}\n", S));
        out.push_str(&format!("#define FSM_CONTEXTS {}\n", C));
        out.push_str(&format!(
            "#define FSM_DNA_SIZE {}\n\n",
            flat.len()
        ));
        out.push_str(&format!(
            "static const float {}[{}] = {{\n",
            array_name,
            flat.len()
        ));

        for ctx in 0..C {
            for row in 0..S {
                out.push_str("    ");
                for col in 0..S {
                    let idx = ctx * S * S + row * S + col;
                    out.push_str(&format!("{:.6}f, ", flat[idx]));
                }
                out.push('\n');
            }
            out.push('\n');
        }

        out.push_str("};\n\n#endif\n");
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_roundtrip() {
        let dna = FsmDna::<3, 2>::uniform();
        let json = DnaExporter::to_json(&dna);
        assert!(json.contains("0.333"));
    }

    #[test]
    fn binary_size_correct() {
        let dna = FsmDna::<4, 3>::uniform();
        let bytes = DnaExporter::to_binary(&dna);
        assert_eq!(bytes.len(), 3 * 16 * 4);
    }

    #[test]
    fn rust_const_contains_name() {
        let dna = FsmDna::<2, 1>::uniform();
        let code = DnaExporter::to_rust_const(&dna, "DRONE_DNA");
        assert!(code.contains("DRONE_DNA"));
        assert!(code.contains("gen_fsm"));
    }

    #[test]
    fn c_header_valid() {
        let dna = FsmDna::<2, 2>::uniform();
        let header = DnaExporter::to_c_header(&dna, "fsm_dna");
        assert!(header.contains("#define FSM_STATES 2"));
        assert!(header.contains("#define FSM_CONTEXTS 2"));
        assert!(header.contains("fsm_dna"));
    }
}

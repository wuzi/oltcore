use oltcore::parser::{extract_ont_id, parse_optical_info};
use oltcore::{parse_ont_autofind, parse_ont_info, parse_service_ports, ServicePort};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn autofind_never_returns_zero_number(input in ".*") {
        let entries = parse_ont_autofind(&input);
        prop_assert!(entries.iter().all(|entry| entry.number > 0));
    }

    #[test]
    fn extract_ont_id_roundtrip(id in 0u32..100000u32) {
        let output = format!("ONTID :{}", id);
        prop_assert_eq!(extract_ont_id(&output), Some(id));
    }

    #[test]
    fn service_ports_roundtrip(entries in proptest::collection::vec((1u32..100000u32, 1u32..4094u32), 0..10)) {
        let mut output = String::new();
        for (index, vlan) in &entries {
            output.push_str(&format!("     {:4} {:4} common   gpon 0/9 /2  0    20    vlan  20         10   10   up\n", index, vlan));
        }
        let ports = parse_service_ports(&output);
        let expected: Vec<ServicePort> = entries
            .into_iter()
            .map(|(index, vlan)| ServicePort { index, vlan })
            .collect();
        prop_assert_eq!(ports, expected);
    }

    #[test]
    fn ont_info_not_found_always_none(prefix in ".*", suffix in ".*") {
        let output = format!("{}The required ONT does not exist{}", prefix, suffix);
        prop_assert!(parse_ont_info(&output).is_none());
    }

    #[test]
    fn optical_info_minimal_has_data(port in "[0-9/]{1,8}") {
        let output = format!("ONU NNI port ID: {}\nVendor name: VendorX\n", port);
        let info = parse_optical_info(&output);
        prop_assert!(info.is_some());
    }
}

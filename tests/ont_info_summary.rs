use oltcore::{parse_ont_info_summary, Fsp};

#[test]
fn parse_ont_info_summary_fixture() {
    let output = include_str!("fixtures/ont_info_summary.txt");
    let summary = parse_ont_info_summary(output);
    assert!(!summary.ports.is_empty());

    let port = summary
        .ports
        .iter()
        .find(|p| p.fsp == Fsp {
            frame: 0,
            slot: 5,
            port: 0,
        })
        .expect("expected port 0/5/0");

    assert_eq!(port.total_onts, 76);
    assert_eq!(port.online_onts, 72);

    let state = port
        .states
        .iter()
        .find(|s| s.id == 23)
        .expect("expected state for ONT 23");
    assert_eq!(state.run_state, "online");
    assert_eq!(state.last_down_cause, "LOFi");

    let ont = port
        .onts
        .iter()
        .find(|o| o.id == 0)
        .expect("expected ont 0 details");
    assert_eq!(ont.sn, "4D4B5047B4BABE7C");
    assert_eq!(ont.ont_type, "GONUMINI3");
    assert_eq!(ont.distance_m, Some(13407));
    assert!(matches!(ont.rx_power, Some(v) if (v + 26.77).abs() < 0.01));
    assert!(matches!(ont.tx_power, Some(v) if (v - 2.31).abs() < 0.01));
    assert_eq!(ont.description, "rozenilda.lins@gserv.");

    let missing = port
        .onts
        .iter()
        .find(|o| o.id == 1)
        .expect("expected ont 1 details");
    assert_eq!(missing.distance_m, None);
    assert_eq!(missing.rx_power, None);
    assert_eq!(missing.tx_power, None);
}

#[test]
fn parse_ont_info_summary_other_ports() {
    let output = include_str!("fixtures/ont_info_summary.txt");
    let summary = parse_ont_info_summary(output);

    let port = summary
        .ports
        .iter()
        .find(|p| p.fsp == Fsp {
            frame: 0,
            slot: 5,
            port: 4,
        })
        .expect("expected port 0/5/4");

    assert_eq!(port.total_onts, 46);
    assert_eq!(port.online_onts, 38);

    let port = summary
        .ports
        .iter()
        .find(|p| p.fsp == Fsp {
            frame: 0,
            slot: 5,
            port: 1,
        })
        .expect("expected port 0/5/1");

    let ont = port
        .onts
        .iter()
        .find(|o| o.id == 5)
        .expect("expected ont 5 details");
    assert_eq!(ont.distance_m, None);
    assert_eq!(ont.rx_power, None);
    assert_eq!(ont.tx_power, None);
    assert_eq!(ont.description, "libertelecom-29899-25");
}

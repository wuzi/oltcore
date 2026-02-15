use oltcore::{parse_active_alarms_detail, parse_active_alarms_list, Fsp};

#[test]
fn parse_active_alarms_list_fixture() {
    let output = include_str!("fixtures/display_alarm_active_all_list.txt");
    let alarms = parse_active_alarms_list(output);

    assert!(!alarms.alarms.is_empty());

    let alarm = alarms
        .alarms
        .iter()
        .find(|a| a.serial_number == 2_431_665)
        .expect("expected alarm 2431665");
    assert_eq!(alarm.timestamp, "2026-02-14 22:48:13+08:00");
    assert!(alarm.alarm_name.contains("loss of GEM channel"));
    assert!(alarm.alarm_name.contains("LCDGi"));
    assert!(alarm.alarm_name.contains("FrameID: 0"));
    assert!(alarm.alarm_name.contains("ONT ID: 23"));
}

#[test]
fn parse_active_alarms_list_multiple() {
    let output = include_str!("fixtures/display_alarm_active_all_list.txt");
    let alarms = parse_active_alarms_list(output);

    let alarm_2431646 = alarms
        .alarms
        .iter()
        .find(|a| a.serial_number == 2_431_646)
        .expect("expected alarm 2431646");
    assert!(alarm_2431646.alarm_name.contains("dying-gasp of GPON ONTi"));
}

#[test]
fn parse_active_alarms_detail_fixture() {
    let output = include_str!("fixtures/display_alarm_active_all_detail.txt");
    let alarms = parse_active_alarms_detail(output);

    assert!(!alarms.alarms.is_empty());

    let alarm = alarms
        .alarms
        .iter()
        .find(|a| a.serial_number == 2_431_665)
        .expect("expected alarm 2431665");

    assert_eq!(
        alarm.alarm_name,
        "The loss of GEM channel delineation (LCDGi) occurs"
    );
    assert_eq!(
        alarm.fsp,
        Some(Fsp {
            frame: 0,
            slot: 6,
            port: 10
        })
    );
    assert_eq!(alarm.ont_id, Some(23));
    assert_eq!(alarm.service_effect, Some("SA".to_string()));
    assert!(alarm.description.is_some());
    assert!(alarm.cause.is_some());
    assert!(alarm.advice.is_some());
}

#[test]
fn parse_active_alarms_detail_with_equipment() {
    let output = include_str!("fixtures/display_alarm_active_all_detail.txt");
    let alarms = parse_active_alarms_detail(output);

    let alarm = alarms
        .alarms
        .iter()
        .find(|a| a.serial_number == 2_431_646)
        .expect("expected alarm 2431646");

    assert_eq!(
        alarm.alarm_name,
        "The dying-gasp of GPON ONTi (DGi) is generated"
    );
    assert_eq!(
        alarm.fsp,
        Some(Fsp {
            frame: 0,
            slot: 5,
            port: 2
        })
    );
    assert_eq!(alarm.ont_id, Some(54));
    assert_eq!(alarm.equipment_id, Some("F6201BV9.3.12".to_string()));
}

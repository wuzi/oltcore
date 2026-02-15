use oltcore::parser::{extract_ont_id, parse_optical_info};
use oltcore::{
    parse_display_board, parse_ont_autofind, parse_ont_info, parse_service_ports, Fsp, ServicePort,
};

#[test]
fn parse_ont_autofind_fixture() {
    let output = include_str!("fixtures/ont_autofind.txt");
    let entries = parse_ont_autofind(output);
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].number, 1);
    assert_eq!(
        entries[0].fsp,
        Fsp {
            frame: 0,
            slot: 6,
            port: 1
        }
    );
    assert_eq!(entries[0].serial_number, "44443732E68F3DD5");
    assert_eq!(entries[0].serial_number_readable, "DD72-E68F3DD5");
    assert_eq!(entries[0].password, "0x31323334353637380000(12345678)");
    assert_eq!(entries[1].number, 2);
    assert_eq!(
        entries[1].fsp,
        Fsp {
            frame: 0,
            slot: 6,
            port: 5
        }
    );
    assert_eq!(entries[1].serial_number, "44443136E601C966");
    assert_eq!(entries[1].serial_number_readable, "DD16-E601C966");
    assert_eq!(entries[2].number, 3);
    assert_eq!(
        entries[2].fsp,
        Fsp {
            frame: 0,
            slot: 6,
            port: 11
        }
    );
    assert_eq!(entries[2].serial_number, "48575443B6113C9D");
    assert_eq!(entries[2].serial_number_readable, "HWTC-B6113C9D");
}

#[test]
fn parse_ont_info_fixture() {
    let output = include_str!("fixtures/ont_info.txt");
    let info = parse_ont_info(output).expect("expected info");
    assert_eq!(
        info.fsp,
        Fsp {
            frame: 0,
            slot: 9,
            port: 2
        }
    );
    assert_eq!(info.id, 0);
    assert_eq!(info.run_state, "online");
    assert_eq!(info.temperature, 54);
    assert_eq!(info.sn, "48575443CB8FBDB4");
    assert_eq!(info.sn_readable, "HWTC-CB8FBDB4");
    assert_eq!(info.description, "JFTECH");
    assert_eq!(info.control_flag, "active");
    assert_eq!(info.config_state, "normal");
    assert_eq!(info.match_state, "match");
}

#[test]
fn parse_optical_info_fixture() {
    let output = include_str!("fixtures/optical_info.txt");
    let info = parse_optical_info(output).expect("expected optical info");
    assert_eq!(info.onu_nni_port_id, "0");
    assert_eq!(info.vendor_name, "HUAWEI");
    assert_eq!(info.rx_optical_power, "-15.93");
    assert_eq!(info.tx_optical_power, "2.34");
}

#[test]
fn parse_service_ports_fixture() {
    let output = include_str!("fixtures/service_ports.txt");
    let ports = parse_service_ports(output);
    assert_eq!(
        ports,
        vec![ServicePort {
            index: 68,
            vlan: 1063
        },]
    );
}

#[test]
fn extract_ont_id_fixture() {
    let output = "ONTID :123\n";
    let id = extract_ont_id(output);
    assert_eq!(id, Some(123));
}

#[test]
fn parse_display_board_fixture() {
    let output = include_str!("fixtures/display_board.txt");
    let boards = parse_display_board(output);
    assert_eq!(boards.len(), 12);

    let failed = boards.iter().find(|b| b.slot_id == 4).expect("slot 4");
    assert_eq!(failed.board_name.as_deref(), Some("H901GPHF"));
    assert_eq!(failed.status.as_deref(), Some("Failed"));
    assert_eq!(failed.online_status.as_deref(), Some("Offline"));

    let normal = boards.iter().find(|b| b.slot_id == 5).expect("slot 5");
    assert_eq!(normal.board_name.as_deref(), Some("H901GPUF"));
    assert_eq!(normal.status.as_deref(), Some("Normal"));
    assert_eq!(normal.online_status.as_deref(), None);

    let standby = boards.iter().find(|b| b.slot_id == 8).expect("slot 8");
    assert_eq!(standby.board_name.as_deref(), Some("H902MPLA"));
    assert_eq!(standby.status.as_deref(), Some("Standby_normal"));
}

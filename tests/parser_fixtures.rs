use oltcore::parser::{extract_ont_id, parse_optical_info};
use oltcore::{parse_ont_autofind, parse_ont_info, parse_service_ports, Fsp, ServicePort};

#[test]
fn parse_ont_autofind_fixture() {
    let output = include_str!("fixtures/ont_autofind.txt");
    let entries = parse_ont_autofind(output);
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].number, 1);
    assert_eq!(
        entries[0].fsp,
        Fsp {
            frame: 0,
            slot: 6,
            port: 1
        }
    );
    assert_eq!(entries[0].serial_number, "4444");
    assert_eq!(entries[0].serial_number_readable, "DD72-ABCD");
    assert_eq!(entries[1].number, 2);
    assert_eq!(
        entries[1].fsp,
        Fsp {
            frame: 0,
            slot: 6,
            port: 2
        }
    );
    assert_eq!(entries[1].serial_number, "5555");
}

#[test]
fn parse_ont_info_fixture() {
    let output = include_str!("fixtures/ont_info.txt");
    let info = parse_ont_info(output).expect("expected info");
    assert_eq!(
        info.fsp,
        Fsp {
            frame: 0,
            slot: 2,
            port: 3
        }
    );
    assert_eq!(info.id, 42);
    assert_eq!(info.run_state, "online");
    assert_eq!(info.temperature, 45);
    assert_eq!(info.sn_readable, "ABCD-1234");
}

#[test]
fn parse_optical_info_fixture() {
    let output = include_str!("fixtures/optical_info.txt");
    let info = parse_optical_info(output).expect("expected optical info");
    assert_eq!(info.onu_nni_port_id, "1/1/1");
    assert_eq!(info.vendor_name, "VendorX");
    assert_eq!(info.rx_optical_power, "-12.3");
    assert_eq!(info.tx_optical_power, "2.1");
}

#[test]
fn parse_service_ports_fixture() {
    let output = include_str!("fixtures/service_ports.txt");
    let ports = parse_service_ports(output);
    assert_eq!(
        ports,
        vec![
            ServicePort {
                index: 1234,
                vlan: 100
            },
            ServicePort {
                index: 5678,
                vlan: 200
            },
        ]
    );
}

#[test]
fn extract_ont_id_fixture() {
    let output = "ONTID :123\n";
    let id = extract_ont_id(output);
    assert_eq!(id, Some(123));
}

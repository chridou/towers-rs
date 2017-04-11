use super::*;

#[test]
fn peg_should_accept_disks_in_valid_order() {
    let disks = vec![Disk(3), Disk(2), Disk(1)];
    let mut peg = Peg::new();

    for disk in disks {
        peg.put(disk).unwrap();
    }
}
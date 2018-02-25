extern crate nine_grids_shogi_analyzer;

use nine_grids_shogi_analyzer::util::{sqrt_ceil, hand_data_to_hand_info, hand_info_to_hand_data, HandInfo};

#[test]
fn sqrt_ceil_test() {
    assert_eq!(sqrt_ceil(0), 0);
    assert_eq!(sqrt_ceil(1), 1);
    assert_eq!(sqrt_ceil(2), 2);
    assert_eq!(sqrt_ceil(3), 2);
    assert_eq!(sqrt_ceil(4), 2);
    assert_eq!(sqrt_ceil(5), 3);
    assert_eq!(sqrt_ceil(6), 3);
    assert_eq!(sqrt_ceil(7), 3);
    assert_eq!(sqrt_ceil(65535), 256);
}

#[test]
fn hand_data_to_hand_info_test() {
    assert_eq!(hand_data_to_hand_info(0, 2), HandInfo {first: 0, second: 0});
    assert_eq!(hand_data_to_hand_info(1, 2), HandInfo {first: 0, second: 1});
    assert_eq!(hand_data_to_hand_info(2, 2), HandInfo {first: 0, second: 2});
    assert_eq!(hand_data_to_hand_info(3, 2), HandInfo {first: 1, second: 0});
    assert_eq!(hand_data_to_hand_info(4, 2), HandInfo {first: 1, second: 1});
    assert_eq!(hand_data_to_hand_info(5, 2), HandInfo {first: 2, second: 0});

    assert_eq!(hand_data_to_hand_info(0, 4), HandInfo {first: 0, second: 0});
    assert_eq!(hand_data_to_hand_info(1, 4), HandInfo {first: 0, second: 1});
    assert_eq!(hand_data_to_hand_info(2, 4), HandInfo {first: 0, second: 2});
    assert_eq!(hand_data_to_hand_info(3, 4), HandInfo {first: 0, second: 3});
    assert_eq!(hand_data_to_hand_info(4, 4), HandInfo {first: 0, second: 4});
    assert_eq!(hand_data_to_hand_info(5, 4), HandInfo {first: 1, second: 0});
    assert_eq!(hand_data_to_hand_info(10, 4), HandInfo {first: 2, second: 1});
    assert_eq!(hand_data_to_hand_info(11, 4), HandInfo {first: 2, second: 2});
    assert_eq!(hand_data_to_hand_info(12, 4), HandInfo {first: 3, second: 0});
    assert_eq!(hand_data_to_hand_info(13, 4), HandInfo {first: 3, second: 1});
    assert_eq!(hand_data_to_hand_info(14, 4), HandInfo {first: 4, second: 0});

    assert_eq!(hand_data_to_hand_info(0, 7), HandInfo {first: 0, second: 0});
    assert_eq!(hand_data_to_hand_info(1, 7), HandInfo {first: 0, second: 1});
    assert_eq!(hand_data_to_hand_info(2, 7), HandInfo {first: 0, second: 2});
    assert_eq!(hand_data_to_hand_info(3, 7), HandInfo {first: 0, second: 3});
    assert_eq!(hand_data_to_hand_info(4, 7), HandInfo {first: 0, second: 4});
    assert_eq!(hand_data_to_hand_info(5, 7), HandInfo {first: 0, second: 5});
    assert_eq!(hand_data_to_hand_info(6, 7), HandInfo {first: 0, second: 6});
    assert_eq!(hand_data_to_hand_info(7, 7), HandInfo {first: 0, second: 7});
    assert_eq!(hand_data_to_hand_info(8, 7), HandInfo {first: 1, second: 0});
    assert_eq!(hand_data_to_hand_info(31, 7), HandInfo {first: 5, second: 1});
    assert_eq!(hand_data_to_hand_info(32, 7), HandInfo {first: 5, second: 2});
    assert_eq!(hand_data_to_hand_info(33, 7), HandInfo {first: 6, second: 0});
    assert_eq!(hand_data_to_hand_info(34, 7), HandInfo {first: 6, second: 1});
    assert_eq!(hand_data_to_hand_info(35, 7), HandInfo {first: 7, second: 0});
}

#[test]
fn hand_info_to_hand_data_test() {
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 0, second: 0}, 2), 0);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 0, second: 1}, 2), 1);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 0, second: 2}, 2), 2);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 1, second: 0}, 2), 3);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 1, second: 1}, 2), 4);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 2, second: 0}, 2), 5);

    assert_eq!(hand_info_to_hand_data(HandInfo {first: 0, second: 0}, 4), 0);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 0, second: 1}, 4), 1);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 0, second: 2}, 4), 2);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 0, second: 3}, 4), 3);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 0, second: 4}, 4), 4);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 1, second: 0}, 4), 5);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 2, second: 1}, 4), 10);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 2, second: 2}, 4), 11);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 3, second: 0}, 4), 12);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 3, second: 1}, 4), 13);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 4, second: 0}, 4), 14);

    assert_eq!(hand_info_to_hand_data(HandInfo {first: 0, second: 0}, 7), 0);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 0, second: 1}, 7), 1);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 0, second: 2}, 7), 2);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 0, second: 3}, 7), 3);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 0, second: 4}, 7), 4);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 0, second: 5}, 7), 5);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 0, second: 6}, 7), 6);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 0, second: 7}, 7), 7);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 1, second: 0}, 7), 8);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 5, second: 1}, 7), 31);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 5, second: 2}, 7), 32);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 6, second: 0}, 7), 33);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 6, second: 1}, 7), 34);
    assert_eq!(hand_info_to_hand_data(HandInfo {first: 7, second: 0}, 7), 35);
}

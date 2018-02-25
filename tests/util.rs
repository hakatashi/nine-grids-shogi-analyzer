extern crate nine_grids_shogi_analyzer;

use nine_grids_shogi_analyzer::util::{sqrt_ceil, hand_data_to_hand_info, HandInfo};

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

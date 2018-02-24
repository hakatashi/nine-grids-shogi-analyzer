extern crate nine_grids_shogi_analyzer;

use nine_grids_shogi_analyzer::util::{hand_data_to_hand_info, HandInfo};

#[test]
fn it_works() {
    assert_eq!(hand_data_to_hand_info(0, 2), HandInfo {first: 0, second: 0});
}

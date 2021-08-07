mod tilesheet_viewer;
use tilesheet_viewer::tilesheet_viewer;

fn main() {
    tilesheet_viewer(*include_bytes!(
        "memory dumps/Legend_of_zelda_link_awaking.dmp"
    ));
}

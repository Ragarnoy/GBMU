mod tilesheet_viewer;
use tilesheet_viewer::tilesheet_viewer;

fn main() {
    tilesheet_viewer(*include_bytes!("memory dumps/Pokemon_Bleue.dmp"));
}

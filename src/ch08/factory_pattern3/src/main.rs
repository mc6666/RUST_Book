mod gui;
mod html_gui;
mod init;
mod windows_gui;

use init::initialize;

fn main() {
    let dialog = initialize();
    dialog.render();
    dialog.refresh();
}
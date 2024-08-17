use windows::{
    core::*, Win32::UI::WindowsAndMessaging::*
};

fn main() -> Result<()> {
    unsafe {
        // 彈出式視窗
        MessageBoxA(None, s!("Ansi"), s!("Caption"), MB_OK);
        // MessageBoxA(None, w!("哈囉 !!"), w!("中文測試"), MB_OK);
        MessageBoxW(None, w!("哈囉 !!"), w!("中文測試"), MB_OK);
    }

    Ok(())
}

pub use widestring;
pub use windows;

#[macro_export]
macro_rules! formatw {
    ($($arg:tt)*) => {
        $crate::widestring::U16CString::from_str_truncate(format!($($arg)*))
    }
}

#[macro_export]
macro_rules! mb {
    {
        title: ($($title:tt)*),
        content: ($($arg:tt)*),
        raw_style: $type:expr
    } => {
        unsafe {
            $crate::windows::Win32::UI::WindowsAndMessaging::MessageBoxW(
                None,
                $crate::windows::core::PCWSTR($crate::formatw!($($arg)*).as_ptr() as _),
                $crate::windows::core::PCWSTR($crate::formatw!($($title)*).as_ptr() as _),
                $crate::windows::Win32::UI::WindowsAndMessaging::MESSAGEBOX_STYLE($type)
            ).0
        }
    };
    {
        title: ($($title:tt)*),
        content: ($($arg:tt)*),
        style: $type:expr
    } => {
        unsafe {
            $crate::windows::Win32::UI::WindowsAndMessaging::MessageBoxW(
                None,
                $crate::windows::core::PCWSTR($crate::formatw!($($arg)*).as_ptr() as _),
                $crate::windows::core::PCWSTR($crate::formatw!($($title)*).as_ptr() as _),
                $type
            )
        }
    };
    ($($arg:tt)*) => {
        unsafe {
            $crate::windows::Win32::UI::WindowsAndMessaging::MessageBoxW(
                None,
                $crate::windows::core::PCWSTR($crate::formatw!($($arg)*).as_ptr() as _),
                $crate::windows::core::w!(""),
                $crate::windows::Win32::UI::WindowsAndMessaging::MB_OK
            )
        }
    }
}

#[test]
pub fn test() {
    let _ = mb!(
        title: ("使得"),
        content: ("你好"),
        raw_style: 1
    );
    mb!("计算得到了 {}", 1 + 1);
    let my_vec = vec![1, 2, 3];
    let _ = mb!("我是说你可以这样 {:?} {:?}", my_vec, formatw!("你好谢谢"));
}

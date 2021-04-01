use std::{ffi::CString, mem::{size_of_val, zeroed}, ptr::null_mut};

use anyhow::*;
use winapi::um::wininet::{INTERNET_OPTION_PER_CONNECTION_OPTION, INTERNET_OPTION_SETTINGS_CHANGED, INTERNET_PER_CONN_FLAGS_UI, INTERNET_PER_CONN_OPTIONA, INTERNET_PER_CONN_OPTION_LISTA, INTERNET_PER_CONN_PROXY_SERVER, InternetQueryOptionA, InternetSetOptionA, PROXY_TYPE_DIRECT, PROXY_TYPE_PROXY};

pub fn enable_homo(addr: impl Into<String>) -> Result<()> {
    homo_proxy(addr.into(), true)
}

pub fn disable_homo() -> Result<()> {
    homo_proxy("", false)
}

pub fn homo_proxy(server_addr: impl Into<String>, set: bool) -> Result<()> {
    unsafe {
        let server_addr = server_addr.into();
        let mut options = [zeroed::<INTERNET_PER_CONN_OPTIONA>(), zeroed::<INTERNET_PER_CONN_OPTIONA>()];

        let mut list = zeroed::<INTERNET_PER_CONN_OPTION_LISTA>();
        list.dwSize = size_of_val(&list) as _;
        list.dwOptionCount = 1;
        list.pOptions = &mut options as _;
        options[0].dwOption = INTERNET_PER_CONN_FLAGS_UI;

        let mut list_size = size_of_val(&list) as _;
        InternetQueryOptionA(null_mut(), INTERNET_OPTION_PER_CONNECTION_OPTION, &mut list as *const _ as _, &mut list_size);

        if set && *options[0].Value.dwValue() == 0 {
            bail!("homo is already enabled!");
        }
        else if !set && *options[0].Value.dwValue() == 1 {
            bail!("homo is already disabled!");
        }

        let proxy = CString::new::<String>(server_addr).expect("CString::new failed");

        list.dwOptionCount = 2;
        *options[0].Value.dwValue_mut() |= if set {
            PROXY_TYPE_PROXY
        } else {
            PROXY_TYPE_DIRECT
        };

        options[1].dwOption = INTERNET_PER_CONN_PROXY_SERVER;
        *options[1].Value.pszValue_mut() = proxy.as_ptr() as _;

        InternetSetOptionA(null_mut(), INTERNET_OPTION_PER_CONNECTION_OPTION, &mut list as *const _ as _, size_of_val(&list) as _);
        InternetSetOptionA(null_mut(), INTERNET_OPTION_SETTINGS_CHANGED, null_mut(), 0);

        Ok(())
    }
}

/// XC MISC extension requests.

macro_rules! declare_requests {
    ($($request:ident),+) => {
        $(
            mod $request;
            pub use self::$request::*;
        )+
    }
}

declare_requests! {
    xc_misc_get_version,
    xc_misc_get_xid_range,
    xc_misc_get_xid_list
}

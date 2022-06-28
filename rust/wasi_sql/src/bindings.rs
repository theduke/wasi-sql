pub mod sql_v1_alpha1 {
    #[derive(Clone)]
    pub enum ValueParam<'a> {
        Null,
        Boolean(bool),
        Int8(i8),
        Int16(i16),
        Int32(i32),
        Int64(i64),
        Uint8(u8),
        Uint16(u16),
        Uint32(u32),
        Uint64(u64),
        Floating32(f32),
        Floating64(f64),
        Str(&'a str),
        Bytes(&'a [u8]),
        Raw(&'a [u8]),
    }
    impl<'a> std::fmt::Debug for ValueParam<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ValueParam::Null => f.debug_tuple("ValueParam::Null").finish(),
                ValueParam::Boolean(e) => f.debug_tuple("ValueParam::Boolean").field(e).finish(),
                ValueParam::Int8(e) => f.debug_tuple("ValueParam::Int8").field(e).finish(),
                ValueParam::Int16(e) => f.debug_tuple("ValueParam::Int16").field(e).finish(),
                ValueParam::Int32(e) => f.debug_tuple("ValueParam::Int32").field(e).finish(),
                ValueParam::Int64(e) => f.debug_tuple("ValueParam::Int64").field(e).finish(),
                ValueParam::Uint8(e) => f.debug_tuple("ValueParam::Uint8").field(e).finish(),
                ValueParam::Uint16(e) => f.debug_tuple("ValueParam::Uint16").field(e).finish(),
                ValueParam::Uint32(e) => f.debug_tuple("ValueParam::Uint32").field(e).finish(),
                ValueParam::Uint64(e) => f.debug_tuple("ValueParam::Uint64").field(e).finish(),
                ValueParam::Floating32(e) => {
                    f.debug_tuple("ValueParam::Floating32").field(e).finish()
                }
                ValueParam::Floating64(e) => {
                    f.debug_tuple("ValueParam::Floating64").field(e).finish()
                }
                ValueParam::Str(e) => f.debug_tuple("ValueParam::Str").field(e).finish(),
                ValueParam::Bytes(e) => f.debug_tuple("ValueParam::Bytes").field(e).finish(),
                ValueParam::Raw(e) => f.debug_tuple("ValueParam::Raw").field(e).finish(),
            }
        }
    }
    #[derive(Clone)]
    pub enum ValueResult {
        Null,
        Boolean(bool),
        Int8(i8),
        Int16(i16),
        Int32(i32),
        Int64(i64),
        Uint8(u8),
        Uint16(u16),
        Uint32(u32),
        Uint64(u64),
        Floating32(f32),
        Floating64(f64),
        Str(String),
        Bytes(Vec<u8>),
        Raw(Vec<u8>),
    }
    impl std::fmt::Debug for ValueResult {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ValueResult::Null => f.debug_tuple("ValueResult::Null").finish(),
                ValueResult::Boolean(e) => f.debug_tuple("ValueResult::Boolean").field(e).finish(),
                ValueResult::Int8(e) => f.debug_tuple("ValueResult::Int8").field(e).finish(),
                ValueResult::Int16(e) => f.debug_tuple("ValueResult::Int16").field(e).finish(),
                ValueResult::Int32(e) => f.debug_tuple("ValueResult::Int32").field(e).finish(),
                ValueResult::Int64(e) => f.debug_tuple("ValueResult::Int64").field(e).finish(),
                ValueResult::Uint8(e) => f.debug_tuple("ValueResult::Uint8").field(e).finish(),
                ValueResult::Uint16(e) => f.debug_tuple("ValueResult::Uint16").field(e).finish(),
                ValueResult::Uint32(e) => f.debug_tuple("ValueResult::Uint32").field(e).finish(),
                ValueResult::Uint64(e) => f.debug_tuple("ValueResult::Uint64").field(e).finish(),
                ValueResult::Floating32(e) => {
                    f.debug_tuple("ValueResult::Floating32").field(e).finish()
                }
                ValueResult::Floating64(e) => {
                    f.debug_tuple("ValueResult::Floating64").field(e).finish()
                }
                ValueResult::Str(e) => f.debug_tuple("ValueResult::Str").field(e).finish(),
                ValueResult::Bytes(e) => f.debug_tuple("ValueResult::Bytes").field(e).finish(),
                ValueResult::Raw(e) => f.debug_tuple("ValueResult::Raw").field(e).finish(),
            }
        }
    }
    #[derive(Clone)]
    pub struct ColumnMeta {
        pub type_name: String,
        pub name: String,
    }
    impl std::fmt::Debug for ColumnMeta {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ColumnMeta")
                .field("type-name", &self.type_name)
                .field("name", &self.name)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct ValueMapItemParam<'a> {
        pub key: &'a str,
        pub value: ValueParam<'a>,
    }
    impl<'a> std::fmt::Debug for ValueMapItemParam<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ValueMapItemParam")
                .field("key", &self.key)
                .field("value", &self.value)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct ValueMapItemResult {
        pub key: String,
        pub value: ValueResult,
    }
    impl std::fmt::Debug for ValueMapItemResult {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ValueMapItemResult")
                .field("key", &self.key)
                .field("value", &self.value)
                .finish()
        }
    }
    pub type ValueMapParam<'a> = &'a [ValueMapItemParam<'a>];
    pub type ValueMapResult = Vec<ValueMapItemResult>;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PoolOptions {
        pub max_connections: Option<u32>,
        pub idle_timeout: Option<u32>,
    }
    impl std::fmt::Debug for PoolOptions {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PoolOptions")
                .field("max-connections", &self.max_connections)
                .field("idle-timeout", &self.idle_timeout)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct SqlError {
        pub code_numeric: Option<i32>,
        pub code_text: Option<String>,
        pub message: String,
        pub extra: Option<ValueMapResult>,
    }
    impl std::fmt::Debug for SqlError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("SqlError")
                .field("code-numeric", &self.code_numeric)
                .field("code-text", &self.code_text)
                .field("message", &self.message)
                .field("extra", &self.extra)
                .finish()
        }
    }
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Driver(i32);
    impl Driver {
        pub unsafe fn from_raw(raw: i32) -> Self {
            Self(raw)
        }

        pub fn into_raw(self) -> i32 {
            let ret = self.0;
            core::mem::forget(self);
            return ret;
        }

        pub fn as_raw(&self) -> i32 {
            self.0
        }
    }
    impl Drop for Driver {
        fn drop(&mut self) {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_drop_driver"]
                fn close(fd: i32);
            }
            unsafe {
                close(self.0);
            }
        }
    }
    impl Clone for Driver {
        fn clone(&self) -> Self {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_clone_driver"]
                fn clone(val: i32) -> i32;
            }
            unsafe { Self(clone(self.0)) }
        }
    }
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Db(i32);
    impl Db {
        pub unsafe fn from_raw(raw: i32) -> Self {
            Self(raw)
        }

        pub fn into_raw(self) -> i32 {
            let ret = self.0;
            core::mem::forget(self);
            return ret;
        }

        pub fn as_raw(&self) -> i32 {
            self.0
        }
    }
    impl Drop for Db {
        fn drop(&mut self) {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_drop_db"]
                fn close(fd: i32);
            }
            unsafe {
                close(self.0);
            }
        }
    }
    impl Clone for Db {
        fn clone(&self) -> Self {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_clone_db"]
                fn clone(val: i32) -> i32;
            }
            unsafe { Self(clone(self.0)) }
        }
    }
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Connection(i32);
    impl Connection {
        pub unsafe fn from_raw(raw: i32) -> Self {
            Self(raw)
        }

        pub fn into_raw(self) -> i32 {
            let ret = self.0;
            core::mem::forget(self);
            return ret;
        }

        pub fn as_raw(&self) -> i32 {
            self.0
        }
    }
    impl Drop for Connection {
        fn drop(&mut self) {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_drop_connection"]
                fn close(fd: i32);
            }
            unsafe {
                close(self.0);
            }
        }
    }
    impl Clone for Connection {
        fn clone(&self) -> Self {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_clone_connection"]
                fn clone(val: i32) -> i32;
            }
            unsafe { Self(clone(self.0)) }
        }
    }
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Rows(i32);
    impl Rows {
        pub unsafe fn from_raw(raw: i32) -> Self {
            Self(raw)
        }

        pub fn into_raw(self) -> i32 {
            let ret = self.0;
            core::mem::forget(self);
            return ret;
        }

        pub fn as_raw(&self) -> i32 {
            self.0
        }
    }
    impl Drop for Rows {
        fn drop(&mut self) {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_drop_rows"]
                fn close(fd: i32);
            }
            unsafe {
                close(self.0);
            }
        }
    }
    impl Clone for Rows {
        fn clone(&self) -> Self {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_clone_rows"]
                fn clone(val: i32) -> i32;
            }
            unsafe { Self(clone(self.0)) }
        }
    }
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Row(i32);
    impl Row {
        pub unsafe fn from_raw(raw: i32) -> Self {
            Self(raw)
        }

        pub fn into_raw(self) -> i32 {
            let ret = self.0;
            core::mem::forget(self);
            return ret;
        }

        pub fn as_raw(&self) -> i32 {
            self.0
        }
    }
    impl Drop for Row {
        fn drop(&mut self) {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_drop_row"]
                fn close(fd: i32);
            }
            unsafe {
                close(self.0);
            }
        }
    }
    impl Clone for Row {
        fn clone(&self) -> Self {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_clone_row"]
                fn clone(val: i32) -> i32;
            }
            unsafe { Self(clone(self.0)) }
        }
    }
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct PreparedStatement(i32);
    impl PreparedStatement {
        pub unsafe fn from_raw(raw: i32) -> Self {
            Self(raw)
        }

        pub fn into_raw(self) -> i32 {
            let ret = self.0;
            core::mem::forget(self);
            return ret;
        }

        pub fn as_raw(&self) -> i32 {
            self.0
        }
    }
    impl Drop for PreparedStatement {
        fn drop(&mut self) {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_drop_prepared-statement"]
                fn close(fd: i32);
            }
            unsafe {
                close(self.0);
            }
        }
    }
    impl Clone for PreparedStatement {
        fn clone(&self) -> Self {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_clone_prepared-statement"]
                fn clone(val: i32) -> i32;
            }
            unsafe { Self(clone(self.0)) }
        }
    }
    pub fn load_driver(driver: &str) -> Result<Driver, SqlError> {
        unsafe {
            let vec0 = driver;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let ptr1 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
            #[link(wasm_import_module = "sql_v1_alpha1")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "load-driver")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "sql_v1_alpha1_load-driver")]
                fn wit_import(_: i32, _: i32, _: i32);
            }
            wit_import(ptr0, len0, ptr1);
            match i32::from(*((ptr1 + 0) as *const u8)) {
                0 => Ok(Driver(*((ptr1 + 4) as *const i32))),
                1 => Err({
                    let len3 = *((ptr1 + 28) as *const i32) as usize;

                    SqlError {
                        code_numeric: match i32::from(*((ptr1 + 4) as *const u8)) {
                            0 => None,
                            1 => Some(*((ptr1 + 8) as *const i32)),
                            _ => panic!("invalid enum discriminant"),
                        },
                        code_text: match i32::from(*((ptr1 + 12) as *const u8)) {
                            0 => None,
                            1 => Some({
                                let len2 = *((ptr1 + 20) as *const i32) as usize;

                                String::from_utf8(Vec::from_raw_parts(
                                    *((ptr1 + 16) as *const i32) as *mut _,
                                    len2,
                                    len2,
                                ))
                                .unwrap()
                            }),
                            _ => panic!("invalid enum discriminant"),
                        },
                        message: String::from_utf8(Vec::from_raw_parts(
                            *((ptr1 + 24) as *const i32) as *mut _,
                            len3,
                            len3,
                        ))
                        .unwrap(),
                        extra: match i32::from(*((ptr1 + 32) as *const u8)) {
                            0 => None,
                            1 => Some({
                                let base8 = *((ptr1 + 36) as *const i32);
                                let len8 = *((ptr1 + 40) as *const i32);
                                let mut result8 = Vec::with_capacity(len8 as usize);
                                for i in 0..len8 {
                                    let base = base8 + i * 24;
                                    result8.push({
                                        let len4 = *((base + 4) as *const i32) as usize;

                                        ValueMapItemResult {
                                            key: String::from_utf8(Vec::from_raw_parts(
                                                *((base + 0) as *const i32) as *mut _,
                                                len4,
                                                len4,
                                            ))
                                            .unwrap(),
                                            value: match i32::from(*((base + 8) as *const u8)) {
                                                0 => ValueResult::Null,
                                                1 => ValueResult::Boolean(
                                                    match i32::from(*((base + 16) as *const u8)) {
                                                        0 => false,
                                                        1 => true,
                                                        _ => panic!("invalid bool discriminant"),
                                                    },
                                                ),
                                                2 => ValueResult::Int8(i32::from(
                                                    *((base + 16) as *const i8),
                                                )
                                                    as i8),
                                                3 => ValueResult::Int16(i32::from(
                                                    *((base + 16) as *const i16),
                                                )
                                                    as i16),
                                                4 => {
                                                    ValueResult::Int32(*((base + 16) as *const i32))
                                                }
                                                5 => {
                                                    ValueResult::Int64(*((base + 16) as *const i64))
                                                }
                                                6 => ValueResult::Uint8(i32::from(
                                                    *((base + 16) as *const u8),
                                                )
                                                    as u8),
                                                7 => ValueResult::Uint16(i32::from(
                                                    *((base + 16) as *const u16),
                                                )
                                                    as u16),
                                                8 => ValueResult::Uint32(
                                                    *((base + 16) as *const i32) as u32,
                                                ),
                                                9 => ValueResult::Uint64(
                                                    *((base + 16) as *const i64) as u64,
                                                ),
                                                10 => ValueResult::Floating32(
                                                    *((base + 16) as *const f32),
                                                ),
                                                11 => ValueResult::Floating64(
                                                    *((base + 16) as *const f64),
                                                ),
                                                12 => ValueResult::Str({
                                                    let len5 =
                                                        *((base + 20) as *const i32) as usize;

                                                    String::from_utf8(Vec::from_raw_parts(
                                                        *((base + 16) as *const i32) as *mut _,
                                                        len5,
                                                        len5,
                                                    ))
                                                    .unwrap()
                                                }),
                                                13 => ValueResult::Bytes({
                                                    let len6 =
                                                        *((base + 20) as *const i32) as usize;

                                                    Vec::from_raw_parts(
                                                        *((base + 16) as *const i32) as *mut _,
                                                        len6,
                                                        len6,
                                                    )
                                                }),
                                                14 => ValueResult::Raw({
                                                    let len7 =
                                                        *((base + 20) as *const i32) as usize;

                                                    Vec::from_raw_parts(
                                                        *((base + 16) as *const i32) as *mut _,
                                                        len7,
                                                        len7,
                                                    )
                                                }),
                                                _ => panic!("invalid enum discriminant"),
                                            },
                                        }
                                    });
                                }
                                std::alloc::dealloc(
                                    base8 as *mut _,
                                    std::alloc::Layout::from_size_align_unchecked(
                                        (len8 as usize) * 24,
                                        8,
                                    ),
                                );

                                result8
                            }),
                            _ => panic!("invalid enum discriminant"),
                        },
                    }
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    impl Driver {
        pub fn connect_uri(&self, uri: &str) -> Result<Connection, SqlError> {
            unsafe {
                let vec0 = uri;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let ptr1 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "driver::connect-uri")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_driver::connect-uri"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32, _: i32);
                }
                wit_import(self.0, ptr0, len0, ptr1);
                match i32::from(*((ptr1 + 0) as *const u8)) {
                    0 => Ok(Connection(*((ptr1 + 4) as *const i32))),
                    1 => Err({
                        let len3 = *((ptr1 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr1 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr1 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr1 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len2 = *((ptr1 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr1 + 16) as *const i32) as *mut _,
                                        len2,
                                        len2,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr1 + 24) as *const i32) as *mut _,
                                len3,
                                len3,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr1 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base8 = *((ptr1 + 36) as *const i32);
                                    let len8 = *((ptr1 + 40) as *const i32);
                                    let mut result8 = Vec::with_capacity(len8 as usize);
                                    for i in 0..len8 {
                                        let base = base8 + i * 24;
                                        result8.push({
                                            let len4 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len4,
                                                    len4,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len7 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len7,
                                                            len7,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base8 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len8 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result8
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Driver {
        pub fn connect_args(&self, args: ValueMapParam<'_>) -> Result<Connection, SqlError> {
            unsafe {
                let vec5 = args;
                let len5 = vec5.len() as i32;
                let layout5 = core::alloc::Layout::from_size_align_unchecked(vec5.len() * 24, 8);
                let result5 = std::alloc::alloc(layout5);
                if result5.is_null() {
                    std::alloc::handle_alloc_error(layout5);
                }
                for (i, e) in vec5.into_iter().enumerate() {
                    let base = result5 as i32 + (i as i32) * 24;
                    {
                        let ValueMapItemParam {
                            key: key0,
                            value: value0,
                        } = e;
                        let vec1 = key0;
                        let ptr1 = vec1.as_ptr() as i32;
                        let len1 = vec1.len() as i32;
                        *((base + 4) as *mut i32) = len1;
                        *((base + 0) as *mut i32) = ptr1;
                        match value0 {
                            ValueParam::Null => {
                                let e = ();
                                {
                                    *((base + 8) as *mut u8) = (0i32) as u8;
                                    let () = e;
                                }
                            }
                            ValueParam::Boolean(e) => {
                                *((base + 8) as *mut u8) = (1i32) as u8;
                                *((base + 16) as *mut u8) = (match e {
                                    true => 1,
                                    false => 0,
                                })
                                    as u8;
                            }
                            ValueParam::Int8(e) => {
                                *((base + 8) as *mut u8) = (2i32) as u8;
                                *((base + 16) as *mut u8) = (wit_bindgen_rust::rt::as_i32(e)) as u8;
                            }
                            ValueParam::Int16(e) => {
                                *((base + 8) as *mut u8) = (3i32) as u8;
                                *((base + 16) as *mut u16) =
                                    (wit_bindgen_rust::rt::as_i32(e)) as u16;
                            }
                            ValueParam::Int32(e) => {
                                *((base + 8) as *mut u8) = (4i32) as u8;
                                *((base + 16) as *mut i32) = wit_bindgen_rust::rt::as_i32(e);
                            }
                            ValueParam::Int64(e) => {
                                *((base + 8) as *mut u8) = (5i32) as u8;
                                *((base + 16) as *mut i64) = wit_bindgen_rust::rt::as_i64(e);
                            }
                            ValueParam::Uint8(e) => {
                                *((base + 8) as *mut u8) = (6i32) as u8;
                                *((base + 16) as *mut u8) = (wit_bindgen_rust::rt::as_i32(e)) as u8;
                            }
                            ValueParam::Uint16(e) => {
                                *((base + 8) as *mut u8) = (7i32) as u8;
                                *((base + 16) as *mut u16) =
                                    (wit_bindgen_rust::rt::as_i32(e)) as u16;
                            }
                            ValueParam::Uint32(e) => {
                                *((base + 8) as *mut u8) = (8i32) as u8;
                                *((base + 16) as *mut i32) = wit_bindgen_rust::rt::as_i32(e);
                            }
                            ValueParam::Uint64(e) => {
                                *((base + 8) as *mut u8) = (9i32) as u8;
                                *((base + 16) as *mut i64) = wit_bindgen_rust::rt::as_i64(e);
                            }
                            ValueParam::Floating32(e) => {
                                *((base + 8) as *mut u8) = (10i32) as u8;
                                *((base + 16) as *mut f32) = wit_bindgen_rust::rt::as_f32(e);
                            }
                            ValueParam::Floating64(e) => {
                                *((base + 8) as *mut u8) = (11i32) as u8;
                                *((base + 16) as *mut f64) = wit_bindgen_rust::rt::as_f64(e);
                            }
                            ValueParam::Str(e) => {
                                *((base + 8) as *mut u8) = (12i32) as u8;
                                let vec2 = e;
                                let ptr2 = vec2.as_ptr() as i32;
                                let len2 = vec2.len() as i32;
                                *((base + 20) as *mut i32) = len2;
                                *((base + 16) as *mut i32) = ptr2;
                            }
                            ValueParam::Bytes(e) => {
                                *((base + 8) as *mut u8) = (13i32) as u8;
                                let vec3 = e;
                                let ptr3 = vec3.as_ptr() as i32;
                                let len3 = vec3.len() as i32;
                                *((base + 20) as *mut i32) = len3;
                                *((base + 16) as *mut i32) = ptr3;
                            }
                            ValueParam::Raw(e) => {
                                *((base + 8) as *mut u8) = (14i32) as u8;
                                let vec4 = e;
                                let ptr4 = vec4.as_ptr() as i32;
                                let len4 = vec4.len() as i32;
                                *((base + 20) as *mut i32) = len4;
                                *((base + 16) as *mut i32) = ptr4;
                            }
                        };
                    }
                }
                let ptr6 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "driver::connect-args")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_driver::connect-args"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32, _: i32);
                }
                wit_import(self.0, result5 as i32, len5, ptr6);
                std::alloc::dealloc(result5, layout5);
                match i32::from(*((ptr6 + 0) as *const u8)) {
                    0 => Ok(Connection(*((ptr6 + 4) as *const i32))),
                    1 => Err({
                        let len8 = *((ptr6 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr6 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr6 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr6 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len7 = *((ptr6 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr6 + 16) as *const i32) as *mut _,
                                        len7,
                                        len7,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr6 + 24) as *const i32) as *mut _,
                                len8,
                                len8,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr6 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base13 = *((ptr6 + 36) as *const i32);
                                    let len13 = *((ptr6 + 40) as *const i32);
                                    let mut result13 = Vec::with_capacity(len13 as usize);
                                    for i in 0..len13 {
                                        let base = base13 + i * 24;
                                        result13.push({
                                            let len9 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len9,
                                                    len9,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len10 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len10,
                                                            len10,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len11 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len11,
                                                            len11,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len12 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len12,
                                                            len12,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base13 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len13 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result13
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Driver {
        pub fn open_pool_uri(
            &self,
            uri: &str,
            options: Option<PoolOptions>,
        ) -> Result<Db, SqlError> {
            unsafe {
                let vec0 = uri;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let (result4_0, result4_1, result4_2, result4_3, result4_4) = match options {
                    Some(e) => {
                        let PoolOptions {
                            max_connections: max_connections1,
                            idle_timeout: idle_timeout1,
                        } = e;
                        let (result2_0, result2_1) = match max_connections1 {
                            Some(e) => (1i32, wit_bindgen_rust::rt::as_i32(e)),
                            None => {
                                let e = ();
                                {
                                    let () = e;

                                    (0i32, 0i32)
                                }
                            }
                        };
                        let (result3_0, result3_1) = match idle_timeout1 {
                            Some(e) => (1i32, wit_bindgen_rust::rt::as_i32(e)),
                            None => {
                                let e = ();
                                {
                                    let () = e;

                                    (0i32, 0i32)
                                }
                            }
                        };
                        (1i32, result2_0, result2_1, result3_0, result3_1)
                    }
                    None => {
                        let e = ();
                        {
                            let () = e;

                            (0i32, 0i32, 0i32, 0i32, 0i32)
                        }
                    }
                };
                let ptr5 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "driver::open-pool-uri")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_driver::open-pool-uri"
                    )]
                    fn wit_import(
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                    );
                }
                wit_import(
                    self.0, ptr0, len0, result4_0, result4_1, result4_2, result4_3, result4_4, ptr5,
                );
                match i32::from(*((ptr5 + 0) as *const u8)) {
                    0 => Ok(Db(*((ptr5 + 4) as *const i32))),
                    1 => Err({
                        let len7 = *((ptr5 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr5 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr5 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr5 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len6 = *((ptr5 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr5 + 16) as *const i32) as *mut _,
                                        len6,
                                        len6,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr5 + 24) as *const i32) as *mut _,
                                len7,
                                len7,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr5 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base12 = *((ptr5 + 36) as *const i32);
                                    let len12 = *((ptr5 + 40) as *const i32);
                                    let mut result12 = Vec::with_capacity(len12 as usize);
                                    for i in 0..len12 {
                                        let base = base12 + i * 24;
                                        result12.push({
                                            let len8 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len8,
                                                    len8,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len9 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len9,
                                                            len9,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len10 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len10,
                                                            len10,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len11 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len11,
                                                            len11,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base12 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len12 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result12
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Driver {
        pub fn open_pool_args(
            &self,
            args: ValueMapParam<'_>,
            options: Option<PoolOptions>,
        ) -> Result<Db, SqlError> {
            unsafe {
                let vec5 = args;
                let len5 = vec5.len() as i32;
                let layout5 = core::alloc::Layout::from_size_align_unchecked(vec5.len() * 24, 8);
                let result5 = std::alloc::alloc(layout5);
                if result5.is_null() {
                    std::alloc::handle_alloc_error(layout5);
                }
                for (i, e) in vec5.into_iter().enumerate() {
                    let base = result5 as i32 + (i as i32) * 24;
                    {
                        let ValueMapItemParam {
                            key: key0,
                            value: value0,
                        } = e;
                        let vec1 = key0;
                        let ptr1 = vec1.as_ptr() as i32;
                        let len1 = vec1.len() as i32;
                        *((base + 4) as *mut i32) = len1;
                        *((base + 0) as *mut i32) = ptr1;
                        match value0 {
                            ValueParam::Null => {
                                let e = ();
                                {
                                    *((base + 8) as *mut u8) = (0i32) as u8;
                                    let () = e;
                                }
                            }
                            ValueParam::Boolean(e) => {
                                *((base + 8) as *mut u8) = (1i32) as u8;
                                *((base + 16) as *mut u8) = (match e {
                                    true => 1,
                                    false => 0,
                                })
                                    as u8;
                            }
                            ValueParam::Int8(e) => {
                                *((base + 8) as *mut u8) = (2i32) as u8;
                                *((base + 16) as *mut u8) = (wit_bindgen_rust::rt::as_i32(e)) as u8;
                            }
                            ValueParam::Int16(e) => {
                                *((base + 8) as *mut u8) = (3i32) as u8;
                                *((base + 16) as *mut u16) =
                                    (wit_bindgen_rust::rt::as_i32(e)) as u16;
                            }
                            ValueParam::Int32(e) => {
                                *((base + 8) as *mut u8) = (4i32) as u8;
                                *((base + 16) as *mut i32) = wit_bindgen_rust::rt::as_i32(e);
                            }
                            ValueParam::Int64(e) => {
                                *((base + 8) as *mut u8) = (5i32) as u8;
                                *((base + 16) as *mut i64) = wit_bindgen_rust::rt::as_i64(e);
                            }
                            ValueParam::Uint8(e) => {
                                *((base + 8) as *mut u8) = (6i32) as u8;
                                *((base + 16) as *mut u8) = (wit_bindgen_rust::rt::as_i32(e)) as u8;
                            }
                            ValueParam::Uint16(e) => {
                                *((base + 8) as *mut u8) = (7i32) as u8;
                                *((base + 16) as *mut u16) =
                                    (wit_bindgen_rust::rt::as_i32(e)) as u16;
                            }
                            ValueParam::Uint32(e) => {
                                *((base + 8) as *mut u8) = (8i32) as u8;
                                *((base + 16) as *mut i32) = wit_bindgen_rust::rt::as_i32(e);
                            }
                            ValueParam::Uint64(e) => {
                                *((base + 8) as *mut u8) = (9i32) as u8;
                                *((base + 16) as *mut i64) = wit_bindgen_rust::rt::as_i64(e);
                            }
                            ValueParam::Floating32(e) => {
                                *((base + 8) as *mut u8) = (10i32) as u8;
                                *((base + 16) as *mut f32) = wit_bindgen_rust::rt::as_f32(e);
                            }
                            ValueParam::Floating64(e) => {
                                *((base + 8) as *mut u8) = (11i32) as u8;
                                *((base + 16) as *mut f64) = wit_bindgen_rust::rt::as_f64(e);
                            }
                            ValueParam::Str(e) => {
                                *((base + 8) as *mut u8) = (12i32) as u8;
                                let vec2 = e;
                                let ptr2 = vec2.as_ptr() as i32;
                                let len2 = vec2.len() as i32;
                                *((base + 20) as *mut i32) = len2;
                                *((base + 16) as *mut i32) = ptr2;
                            }
                            ValueParam::Bytes(e) => {
                                *((base + 8) as *mut u8) = (13i32) as u8;
                                let vec3 = e;
                                let ptr3 = vec3.as_ptr() as i32;
                                let len3 = vec3.len() as i32;
                                *((base + 20) as *mut i32) = len3;
                                *((base + 16) as *mut i32) = ptr3;
                            }
                            ValueParam::Raw(e) => {
                                *((base + 8) as *mut u8) = (14i32) as u8;
                                let vec4 = e;
                                let ptr4 = vec4.as_ptr() as i32;
                                let len4 = vec4.len() as i32;
                                *((base + 20) as *mut i32) = len4;
                                *((base + 16) as *mut i32) = ptr4;
                            }
                        };
                    }
                }
                let (result9_0, result9_1, result9_2, result9_3, result9_4) = match options {
                    Some(e) => {
                        let PoolOptions {
                            max_connections: max_connections6,
                            idle_timeout: idle_timeout6,
                        } = e;
                        let (result7_0, result7_1) = match max_connections6 {
                            Some(e) => (1i32, wit_bindgen_rust::rt::as_i32(e)),
                            None => {
                                let e = ();
                                {
                                    let () = e;

                                    (0i32, 0i32)
                                }
                            }
                        };
                        let (result8_0, result8_1) = match idle_timeout6 {
                            Some(e) => (1i32, wit_bindgen_rust::rt::as_i32(e)),
                            None => {
                                let e = ();
                                {
                                    let () = e;

                                    (0i32, 0i32)
                                }
                            }
                        };
                        (1i32, result7_0, result7_1, result8_0, result8_1)
                    }
                    None => {
                        let e = ();
                        {
                            let () = e;

                            (0i32, 0i32, 0i32, 0i32, 0i32)
                        }
                    }
                };
                let ptr10 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "driver::open-pool-args")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_driver::open-pool-args"
                    )]
                    fn wit_import(
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                    );
                }
                wit_import(
                    self.0,
                    result5 as i32,
                    len5,
                    result9_0,
                    result9_1,
                    result9_2,
                    result9_3,
                    result9_4,
                    ptr10,
                );
                std::alloc::dealloc(result5, layout5);
                match i32::from(*((ptr10 + 0) as *const u8)) {
                    0 => Ok(Db(*((ptr10 + 4) as *const i32))),
                    1 => Err({
                        let len12 = *((ptr10 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr10 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr10 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr10 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len11 = *((ptr10 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr10 + 16) as *const i32) as *mut _,
                                        len11,
                                        len11,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr10 + 24) as *const i32) as *mut _,
                                len12,
                                len12,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr10 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base17 = *((ptr10 + 36) as *const i32);
                                    let len17 = *((ptr10 + 40) as *const i32);
                                    let mut result17 = Vec::with_capacity(len17 as usize);
                                    for i in 0..len17 {
                                        let base = base17 + i * 24;
                                        result17.push({
                                            let len13 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len13,
                                                    len13,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len14 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len14,
                                                            len14,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len15 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len15,
                                                            len15,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len16 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len16,
                                                            len16,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base17 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len17 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result17
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Db {
        pub fn connection(&self) -> Result<Connection, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "db::connection")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_db::connection"
                    )]
                    fn wit_import(_: i32, _: i32);
                }
                wit_import(self.0, ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(Connection(*((ptr0 + 4) as *const i32))),
                    1 => Err({
                        let len2 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 36) as *const i32);
                                    let len7 = *((ptr0 + 40) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Db {
        pub fn execute(&self, query: &str, args: &[ValueParam<'_>]) -> Result<(), SqlError> {
            unsafe {
                let vec0 = query;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let vec4 = args;
                let len4 = vec4.len() as i32;
                let layout4 = core::alloc::Layout::from_size_align_unchecked(vec4.len() * 16, 8);
                let result4 = std::alloc::alloc(layout4);
                if result4.is_null() {
                    std::alloc::handle_alloc_error(layout4);
                }
                for (i, e) in vec4.into_iter().enumerate() {
                    let base = result4 as i32 + (i as i32) * 16;
                    {
                        match e {
                            ValueParam::Null => {
                                let e = ();
                                {
                                    *((base + 0) as *mut u8) = (0i32) as u8;
                                    let () = e;
                                }
                            }
                            ValueParam::Boolean(e) => {
                                *((base + 0) as *mut u8) = (1i32) as u8;
                                *((base + 8) as *mut u8) = (match e {
                                    true => 1,
                                    false => 0,
                                }) as u8;
                            }
                            ValueParam::Int8(e) => {
                                *((base + 0) as *mut u8) = (2i32) as u8;
                                *((base + 8) as *mut u8) = (wit_bindgen_rust::rt::as_i32(e)) as u8;
                            }
                            ValueParam::Int16(e) => {
                                *((base + 0) as *mut u8) = (3i32) as u8;
                                *((base + 8) as *mut u16) =
                                    (wit_bindgen_rust::rt::as_i32(e)) as u16;
                            }
                            ValueParam::Int32(e) => {
                                *((base + 0) as *mut u8) = (4i32) as u8;
                                *((base + 8) as *mut i32) = wit_bindgen_rust::rt::as_i32(e);
                            }
                            ValueParam::Int64(e) => {
                                *((base + 0) as *mut u8) = (5i32) as u8;
                                *((base + 8) as *mut i64) = wit_bindgen_rust::rt::as_i64(e);
                            }
                            ValueParam::Uint8(e) => {
                                *((base + 0) as *mut u8) = (6i32) as u8;
                                *((base + 8) as *mut u8) = (wit_bindgen_rust::rt::as_i32(e)) as u8;
                            }
                            ValueParam::Uint16(e) => {
                                *((base + 0) as *mut u8) = (7i32) as u8;
                                *((base + 8) as *mut u16) =
                                    (wit_bindgen_rust::rt::as_i32(e)) as u16;
                            }
                            ValueParam::Uint32(e) => {
                                *((base + 0) as *mut u8) = (8i32) as u8;
                                *((base + 8) as *mut i32) = wit_bindgen_rust::rt::as_i32(e);
                            }
                            ValueParam::Uint64(e) => {
                                *((base + 0) as *mut u8) = (9i32) as u8;
                                *((base + 8) as *mut i64) = wit_bindgen_rust::rt::as_i64(e);
                            }
                            ValueParam::Floating32(e) => {
                                *((base + 0) as *mut u8) = (10i32) as u8;
                                *((base + 8) as *mut f32) = wit_bindgen_rust::rt::as_f32(e);
                            }
                            ValueParam::Floating64(e) => {
                                *((base + 0) as *mut u8) = (11i32) as u8;
                                *((base + 8) as *mut f64) = wit_bindgen_rust::rt::as_f64(e);
                            }
                            ValueParam::Str(e) => {
                                *((base + 0) as *mut u8) = (12i32) as u8;
                                let vec1 = e;
                                let ptr1 = vec1.as_ptr() as i32;
                                let len1 = vec1.len() as i32;
                                *((base + 12) as *mut i32) = len1;
                                *((base + 8) as *mut i32) = ptr1;
                            }
                            ValueParam::Bytes(e) => {
                                *((base + 0) as *mut u8) = (13i32) as u8;
                                let vec2 = e;
                                let ptr2 = vec2.as_ptr() as i32;
                                let len2 = vec2.len() as i32;
                                *((base + 12) as *mut i32) = len2;
                                *((base + 8) as *mut i32) = ptr2;
                            }
                            ValueParam::Raw(e) => {
                                *((base + 0) as *mut u8) = (14i32) as u8;
                                let vec3 = e;
                                let ptr3 = vec3.as_ptr() as i32;
                                let len3 = vec3.len() as i32;
                                *((base + 12) as *mut i32) = len3;
                                *((base + 8) as *mut i32) = ptr3;
                            }
                        };
                    }
                }
                let ptr5 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "db::execute")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_db::execute"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
                }
                wit_import(self.0, ptr0, len0, result4 as i32, len4, ptr5);
                std::alloc::dealloc(result4, layout4);
                match i32::from(*((ptr5 + 0) as *const u8)) {
                    0 => Ok(()),
                    1 => Err({
                        let len7 = *((ptr5 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr5 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr5 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr5 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len6 = *((ptr5 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr5 + 16) as *const i32) as *mut _,
                                        len6,
                                        len6,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr5 + 24) as *const i32) as *mut _,
                                len7,
                                len7,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr5 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base12 = *((ptr5 + 36) as *const i32);
                                    let len12 = *((ptr5 + 40) as *const i32);
                                    let mut result12 = Vec::with_capacity(len12 as usize);
                                    for i in 0..len12 {
                                        let base = base12 + i * 24;
                                        result12.push({
                                            let len8 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len8,
                                                    len8,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len9 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len9,
                                                            len9,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len10 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len10,
                                                            len10,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len11 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len11,
                                                            len11,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base12 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len12 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result12
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Db {
        pub fn query(&self, query: &str, args: &[ValueParam<'_>]) -> Result<Rows, SqlError> {
            unsafe {
                let vec0 = query;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let vec4 = args;
                let len4 = vec4.len() as i32;
                let layout4 = core::alloc::Layout::from_size_align_unchecked(vec4.len() * 16, 8);
                let result4 = std::alloc::alloc(layout4);
                if result4.is_null() {
                    std::alloc::handle_alloc_error(layout4);
                }
                for (i, e) in vec4.into_iter().enumerate() {
                    let base = result4 as i32 + (i as i32) * 16;
                    {
                        match e {
                            ValueParam::Null => {
                                let e = ();
                                {
                                    *((base + 0) as *mut u8) = (0i32) as u8;
                                    let () = e;
                                }
                            }
                            ValueParam::Boolean(e) => {
                                *((base + 0) as *mut u8) = (1i32) as u8;
                                *((base + 8) as *mut u8) = (match e {
                                    true => 1,
                                    false => 0,
                                }) as u8;
                            }
                            ValueParam::Int8(e) => {
                                *((base + 0) as *mut u8) = (2i32) as u8;
                                *((base + 8) as *mut u8) = (wit_bindgen_rust::rt::as_i32(e)) as u8;
                            }
                            ValueParam::Int16(e) => {
                                *((base + 0) as *mut u8) = (3i32) as u8;
                                *((base + 8) as *mut u16) =
                                    (wit_bindgen_rust::rt::as_i32(e)) as u16;
                            }
                            ValueParam::Int32(e) => {
                                *((base + 0) as *mut u8) = (4i32) as u8;
                                *((base + 8) as *mut i32) = wit_bindgen_rust::rt::as_i32(e);
                            }
                            ValueParam::Int64(e) => {
                                *((base + 0) as *mut u8) = (5i32) as u8;
                                *((base + 8) as *mut i64) = wit_bindgen_rust::rt::as_i64(e);
                            }
                            ValueParam::Uint8(e) => {
                                *((base + 0) as *mut u8) = (6i32) as u8;
                                *((base + 8) as *mut u8) = (wit_bindgen_rust::rt::as_i32(e)) as u8;
                            }
                            ValueParam::Uint16(e) => {
                                *((base + 0) as *mut u8) = (7i32) as u8;
                                *((base + 8) as *mut u16) =
                                    (wit_bindgen_rust::rt::as_i32(e)) as u16;
                            }
                            ValueParam::Uint32(e) => {
                                *((base + 0) as *mut u8) = (8i32) as u8;
                                *((base + 8) as *mut i32) = wit_bindgen_rust::rt::as_i32(e);
                            }
                            ValueParam::Uint64(e) => {
                                *((base + 0) as *mut u8) = (9i32) as u8;
                                *((base + 8) as *mut i64) = wit_bindgen_rust::rt::as_i64(e);
                            }
                            ValueParam::Floating32(e) => {
                                *((base + 0) as *mut u8) = (10i32) as u8;
                                *((base + 8) as *mut f32) = wit_bindgen_rust::rt::as_f32(e);
                            }
                            ValueParam::Floating64(e) => {
                                *((base + 0) as *mut u8) = (11i32) as u8;
                                *((base + 8) as *mut f64) = wit_bindgen_rust::rt::as_f64(e);
                            }
                            ValueParam::Str(e) => {
                                *((base + 0) as *mut u8) = (12i32) as u8;
                                let vec1 = e;
                                let ptr1 = vec1.as_ptr() as i32;
                                let len1 = vec1.len() as i32;
                                *((base + 12) as *mut i32) = len1;
                                *((base + 8) as *mut i32) = ptr1;
                            }
                            ValueParam::Bytes(e) => {
                                *((base + 0) as *mut u8) = (13i32) as u8;
                                let vec2 = e;
                                let ptr2 = vec2.as_ptr() as i32;
                                let len2 = vec2.len() as i32;
                                *((base + 12) as *mut i32) = len2;
                                *((base + 8) as *mut i32) = ptr2;
                            }
                            ValueParam::Raw(e) => {
                                *((base + 0) as *mut u8) = (14i32) as u8;
                                let vec3 = e;
                                let ptr3 = vec3.as_ptr() as i32;
                                let len3 = vec3.len() as i32;
                                *((base + 12) as *mut i32) = len3;
                                *((base + 8) as *mut i32) = ptr3;
                            }
                        };
                    }
                }
                let ptr5 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "db::query")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "sql_v1_alpha1_db::query")]
                    fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
                }
                wit_import(self.0, ptr0, len0, result4 as i32, len4, ptr5);
                std::alloc::dealloc(result4, layout4);
                match i32::from(*((ptr5 + 0) as *const u8)) {
                    0 => Ok(Rows(*((ptr5 + 4) as *const i32))),
                    1 => Err({
                        let len7 = *((ptr5 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr5 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr5 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr5 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len6 = *((ptr5 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr5 + 16) as *const i32) as *mut _,
                                        len6,
                                        len6,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr5 + 24) as *const i32) as *mut _,
                                len7,
                                len7,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr5 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base12 = *((ptr5 + 36) as *const i32);
                                    let len12 = *((ptr5 + 40) as *const i32);
                                    let mut result12 = Vec::with_capacity(len12 as usize);
                                    for i in 0..len12 {
                                        let base = base12 + i * 24;
                                        result12.push({
                                            let len8 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len8,
                                                    len8,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len9 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len9,
                                                            len9,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len10 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len10,
                                                            len10,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len11 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len11,
                                                            len11,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base12 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len12 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result12
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Connection {
        pub fn execute(&self, query: &str, args: &[ValueParam<'_>]) -> Result<(), SqlError> {
            unsafe {
                let vec0 = query;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let vec4 = args;
                let len4 = vec4.len() as i32;
                let layout4 = core::alloc::Layout::from_size_align_unchecked(vec4.len() * 16, 8);
                let result4 = std::alloc::alloc(layout4);
                if result4.is_null() {
                    std::alloc::handle_alloc_error(layout4);
                }
                for (i, e) in vec4.into_iter().enumerate() {
                    let base = result4 as i32 + (i as i32) * 16;
                    {
                        match e {
                            ValueParam::Null => {
                                let e = ();
                                {
                                    *((base + 0) as *mut u8) = (0i32) as u8;
                                    let () = e;
                                }
                            }
                            ValueParam::Boolean(e) => {
                                *((base + 0) as *mut u8) = (1i32) as u8;
                                *((base + 8) as *mut u8) = (match e {
                                    true => 1,
                                    false => 0,
                                }) as u8;
                            }
                            ValueParam::Int8(e) => {
                                *((base + 0) as *mut u8) = (2i32) as u8;
                                *((base + 8) as *mut u8) = (wit_bindgen_rust::rt::as_i32(e)) as u8;
                            }
                            ValueParam::Int16(e) => {
                                *((base + 0) as *mut u8) = (3i32) as u8;
                                *((base + 8) as *mut u16) =
                                    (wit_bindgen_rust::rt::as_i32(e)) as u16;
                            }
                            ValueParam::Int32(e) => {
                                *((base + 0) as *mut u8) = (4i32) as u8;
                                *((base + 8) as *mut i32) = wit_bindgen_rust::rt::as_i32(e);
                            }
                            ValueParam::Int64(e) => {
                                *((base + 0) as *mut u8) = (5i32) as u8;
                                *((base + 8) as *mut i64) = wit_bindgen_rust::rt::as_i64(e);
                            }
                            ValueParam::Uint8(e) => {
                                *((base + 0) as *mut u8) = (6i32) as u8;
                                *((base + 8) as *mut u8) = (wit_bindgen_rust::rt::as_i32(e)) as u8;
                            }
                            ValueParam::Uint16(e) => {
                                *((base + 0) as *mut u8) = (7i32) as u8;
                                *((base + 8) as *mut u16) =
                                    (wit_bindgen_rust::rt::as_i32(e)) as u16;
                            }
                            ValueParam::Uint32(e) => {
                                *((base + 0) as *mut u8) = (8i32) as u8;
                                *((base + 8) as *mut i32) = wit_bindgen_rust::rt::as_i32(e);
                            }
                            ValueParam::Uint64(e) => {
                                *((base + 0) as *mut u8) = (9i32) as u8;
                                *((base + 8) as *mut i64) = wit_bindgen_rust::rt::as_i64(e);
                            }
                            ValueParam::Floating32(e) => {
                                *((base + 0) as *mut u8) = (10i32) as u8;
                                *((base + 8) as *mut f32) = wit_bindgen_rust::rt::as_f32(e);
                            }
                            ValueParam::Floating64(e) => {
                                *((base + 0) as *mut u8) = (11i32) as u8;
                                *((base + 8) as *mut f64) = wit_bindgen_rust::rt::as_f64(e);
                            }
                            ValueParam::Str(e) => {
                                *((base + 0) as *mut u8) = (12i32) as u8;
                                let vec1 = e;
                                let ptr1 = vec1.as_ptr() as i32;
                                let len1 = vec1.len() as i32;
                                *((base + 12) as *mut i32) = len1;
                                *((base + 8) as *mut i32) = ptr1;
                            }
                            ValueParam::Bytes(e) => {
                                *((base + 0) as *mut u8) = (13i32) as u8;
                                let vec2 = e;
                                let ptr2 = vec2.as_ptr() as i32;
                                let len2 = vec2.len() as i32;
                                *((base + 12) as *mut i32) = len2;
                                *((base + 8) as *mut i32) = ptr2;
                            }
                            ValueParam::Raw(e) => {
                                *((base + 0) as *mut u8) = (14i32) as u8;
                                let vec3 = e;
                                let ptr3 = vec3.as_ptr() as i32;
                                let len3 = vec3.len() as i32;
                                *((base + 12) as *mut i32) = len3;
                                *((base + 8) as *mut i32) = ptr3;
                            }
                        };
                    }
                }
                let ptr5 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "connection::execute")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_connection::execute"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
                }
                wit_import(self.0, ptr0, len0, result4 as i32, len4, ptr5);
                std::alloc::dealloc(result4, layout4);
                match i32::from(*((ptr5 + 0) as *const u8)) {
                    0 => Ok(()),
                    1 => Err({
                        let len7 = *((ptr5 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr5 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr5 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr5 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len6 = *((ptr5 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr5 + 16) as *const i32) as *mut _,
                                        len6,
                                        len6,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr5 + 24) as *const i32) as *mut _,
                                len7,
                                len7,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr5 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base12 = *((ptr5 + 36) as *const i32);
                                    let len12 = *((ptr5 + 40) as *const i32);
                                    let mut result12 = Vec::with_capacity(len12 as usize);
                                    for i in 0..len12 {
                                        let base = base12 + i * 24;
                                        result12.push({
                                            let len8 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len8,
                                                    len8,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len9 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len9,
                                                            len9,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len10 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len10,
                                                            len10,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len11 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len11,
                                                            len11,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base12 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len12 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result12
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Connection {
        pub fn query(&self, query: &str, args: &[ValueParam<'_>]) -> Result<Rows, SqlError> {
            unsafe {
                let vec0 = query;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let vec4 = args;
                let len4 = vec4.len() as i32;
                let layout4 = core::alloc::Layout::from_size_align_unchecked(vec4.len() * 16, 8);
                let result4 = std::alloc::alloc(layout4);
                if result4.is_null() {
                    std::alloc::handle_alloc_error(layout4);
                }
                for (i, e) in vec4.into_iter().enumerate() {
                    let base = result4 as i32 + (i as i32) * 16;
                    {
                        match e {
                            ValueParam::Null => {
                                let e = ();
                                {
                                    *((base + 0) as *mut u8) = (0i32) as u8;
                                    let () = e;
                                }
                            }
                            ValueParam::Boolean(e) => {
                                *((base + 0) as *mut u8) = (1i32) as u8;
                                *((base + 8) as *mut u8) = (match e {
                                    true => 1,
                                    false => 0,
                                }) as u8;
                            }
                            ValueParam::Int8(e) => {
                                *((base + 0) as *mut u8) = (2i32) as u8;
                                *((base + 8) as *mut u8) = (wit_bindgen_rust::rt::as_i32(e)) as u8;
                            }
                            ValueParam::Int16(e) => {
                                *((base + 0) as *mut u8) = (3i32) as u8;
                                *((base + 8) as *mut u16) =
                                    (wit_bindgen_rust::rt::as_i32(e)) as u16;
                            }
                            ValueParam::Int32(e) => {
                                *((base + 0) as *mut u8) = (4i32) as u8;
                                *((base + 8) as *mut i32) = wit_bindgen_rust::rt::as_i32(e);
                            }
                            ValueParam::Int64(e) => {
                                *((base + 0) as *mut u8) = (5i32) as u8;
                                *((base + 8) as *mut i64) = wit_bindgen_rust::rt::as_i64(e);
                            }
                            ValueParam::Uint8(e) => {
                                *((base + 0) as *mut u8) = (6i32) as u8;
                                *((base + 8) as *mut u8) = (wit_bindgen_rust::rt::as_i32(e)) as u8;
                            }
                            ValueParam::Uint16(e) => {
                                *((base + 0) as *mut u8) = (7i32) as u8;
                                *((base + 8) as *mut u16) =
                                    (wit_bindgen_rust::rt::as_i32(e)) as u16;
                            }
                            ValueParam::Uint32(e) => {
                                *((base + 0) as *mut u8) = (8i32) as u8;
                                *((base + 8) as *mut i32) = wit_bindgen_rust::rt::as_i32(e);
                            }
                            ValueParam::Uint64(e) => {
                                *((base + 0) as *mut u8) = (9i32) as u8;
                                *((base + 8) as *mut i64) = wit_bindgen_rust::rt::as_i64(e);
                            }
                            ValueParam::Floating32(e) => {
                                *((base + 0) as *mut u8) = (10i32) as u8;
                                *((base + 8) as *mut f32) = wit_bindgen_rust::rt::as_f32(e);
                            }
                            ValueParam::Floating64(e) => {
                                *((base + 0) as *mut u8) = (11i32) as u8;
                                *((base + 8) as *mut f64) = wit_bindgen_rust::rt::as_f64(e);
                            }
                            ValueParam::Str(e) => {
                                *((base + 0) as *mut u8) = (12i32) as u8;
                                let vec1 = e;
                                let ptr1 = vec1.as_ptr() as i32;
                                let len1 = vec1.len() as i32;
                                *((base + 12) as *mut i32) = len1;
                                *((base + 8) as *mut i32) = ptr1;
                            }
                            ValueParam::Bytes(e) => {
                                *((base + 0) as *mut u8) = (13i32) as u8;
                                let vec2 = e;
                                let ptr2 = vec2.as_ptr() as i32;
                                let len2 = vec2.len() as i32;
                                *((base + 12) as *mut i32) = len2;
                                *((base + 8) as *mut i32) = ptr2;
                            }
                            ValueParam::Raw(e) => {
                                *((base + 0) as *mut u8) = (14i32) as u8;
                                let vec3 = e;
                                let ptr3 = vec3.as_ptr() as i32;
                                let len3 = vec3.len() as i32;
                                *((base + 12) as *mut i32) = len3;
                                *((base + 8) as *mut i32) = ptr3;
                            }
                        };
                    }
                }
                let ptr5 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "connection::query")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_connection::query"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
                }
                wit_import(self.0, ptr0, len0, result4 as i32, len4, ptr5);
                std::alloc::dealloc(result4, layout4);
                match i32::from(*((ptr5 + 0) as *const u8)) {
                    0 => Ok(Rows(*((ptr5 + 4) as *const i32))),
                    1 => Err({
                        let len7 = *((ptr5 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr5 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr5 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr5 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len6 = *((ptr5 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr5 + 16) as *const i32) as *mut _,
                                        len6,
                                        len6,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr5 + 24) as *const i32) as *mut _,
                                len7,
                                len7,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr5 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base12 = *((ptr5 + 36) as *const i32);
                                    let len12 = *((ptr5 + 40) as *const i32);
                                    let mut result12 = Vec::with_capacity(len12 as usize);
                                    for i in 0..len12 {
                                        let base = base12 + i * 24;
                                        result12.push({
                                            let len8 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len8,
                                                    len8,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len9 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len9,
                                                            len9,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len10 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len10,
                                                            len10,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len11 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len11,
                                                            len11,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base12 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len12 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result12
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Connection {
        pub fn prepare(&self, query: &str) -> Result<PreparedStatement, SqlError> {
            unsafe {
                let vec0 = query;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let ptr1 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "connection::prepare")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_connection::prepare"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32, _: i32);
                }
                wit_import(self.0, ptr0, len0, ptr1);
                match i32::from(*((ptr1 + 0) as *const u8)) {
                    0 => Ok(PreparedStatement(*((ptr1 + 4) as *const i32))),
                    1 => Err({
                        let len3 = *((ptr1 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr1 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr1 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr1 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len2 = *((ptr1 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr1 + 16) as *const i32) as *mut _,
                                        len2,
                                        len2,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr1 + 24) as *const i32) as *mut _,
                                len3,
                                len3,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr1 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base8 = *((ptr1 + 36) as *const i32);
                                    let len8 = *((ptr1 + 40) as *const i32);
                                    let mut result8 = Vec::with_capacity(len8 as usize);
                                    for i in 0..len8 {
                                        let base = base8 + i * 24;
                                        result8.push({
                                            let len4 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len4,
                                                    len4,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len7 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len7,
                                                            len7,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base8 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len8 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result8
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Rows {
        pub fn columns(&self) -> Result<Vec<ColumnMeta>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "rows::columns")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_rows::columns"
                    )]
                    fn wit_import(_: i32, _: i32);
                }
                wit_import(self.0, ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok({
                        let base3 = *((ptr0 + 4) as *const i32);
                        let len3 = *((ptr0 + 8) as *const i32);
                        let mut result3 = Vec::with_capacity(len3 as usize);
                        for i in 0..len3 {
                            let base = base3 + i * 16;
                            result3.push({
                                let len1 = *((base + 4) as *const i32) as usize;
                                let len2 = *((base + 12) as *const i32) as usize;

                                ColumnMeta {
                                    type_name: String::from_utf8(Vec::from_raw_parts(
                                        *((base + 0) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap(),
                                    name: String::from_utf8(Vec::from_raw_parts(
                                        *((base + 8) as *const i32) as *mut _,
                                        len2,
                                        len2,
                                    ))
                                    .unwrap(),
                                }
                            });
                        }
                        std::alloc::dealloc(
                            base3 as *mut _,
                            std::alloc::Layout::from_size_align_unchecked((len3 as usize) * 16, 4),
                        );

                        result3
                    }),
                    1 => Err({
                        let len5 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len4 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len4,
                                        len4,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len5,
                                len5,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base10 = *((ptr0 + 36) as *const i32);
                                    let len10 = *((ptr0 + 40) as *const i32);
                                    let mut result10 = Vec::with_capacity(len10 as usize);
                                    for i in 0..len10 {
                                        let base = base10 + i * 24;
                                        result10.push({
                                            let len6 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len6,
                                                    len6,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len7 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len7,
                                                            len7,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len8 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len8,
                                                            len8,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len9 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len9,
                                                            len9,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base10 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len10 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result10
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Rows {
        pub fn next(&self) -> Result<Option<Row>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "rows::next")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "sql_v1_alpha1_rows::next")]
                    fn wit_import(_: i32, _: i32);
                }
                wit_import(self.0, ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => None,
                        1 => Some(Row(*((ptr0 + 8) as *const i32))),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    1 => Err({
                        let len2 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 36) as *const i32);
                                    let len7 = *((ptr0 + 40) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Rows {
        pub fn next_values(&self) -> Result<Option<Vec<ValueResult>>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "rows::next-values")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_rows::next-values"
                    )]
                    fn wit_import(_: i32, _: i32);
                }
                wit_import(self.0, ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => None,
                        1 => Some({
                            let base4 = *((ptr0 + 8) as *const i32);
                            let len4 = *((ptr0 + 12) as *const i32);
                            let mut result4 = Vec::with_capacity(len4 as usize);
                            for i in 0..len4 {
                                let base = base4 + i * 16;
                                result4.push(match i32::from(*((base + 0) as *const u8)) {
                                    0 => ValueResult::Null,
                                    1 => ValueResult::Boolean(
                                        match i32::from(*((base + 8) as *const u8)) {
                                            0 => false,
                                            1 => true,
                                            _ => panic!("invalid bool discriminant"),
                                        },
                                    ),
                                    2 => ValueResult::Int8(
                                        i32::from(*((base + 8) as *const i8)) as i8
                                    ),
                                    3 => ValueResult::Int16(
                                        i32::from(*((base + 8) as *const i16)) as i16
                                    ),
                                    4 => ValueResult::Int32(*((base + 8) as *const i32)),
                                    5 => ValueResult::Int64(*((base + 8) as *const i64)),
                                    6 => ValueResult::Uint8(
                                        i32::from(*((base + 8) as *const u8)) as u8
                                    ),
                                    7 => ValueResult::Uint16(
                                        i32::from(*((base + 8) as *const u16)) as u16,
                                    ),
                                    8 => ValueResult::Uint32(*((base + 8) as *const i32) as u32),
                                    9 => ValueResult::Uint64(*((base + 8) as *const i64) as u64),
                                    10 => ValueResult::Floating32(*((base + 8) as *const f32)),
                                    11 => ValueResult::Floating64(*((base + 8) as *const f64)),
                                    12 => ValueResult::Str({
                                        let len1 = *((base + 12) as *const i32) as usize;

                                        String::from_utf8(Vec::from_raw_parts(
                                            *((base + 8) as *const i32) as *mut _,
                                            len1,
                                            len1,
                                        ))
                                        .unwrap()
                                    }),
                                    13 => ValueResult::Bytes({
                                        let len2 = *((base + 12) as *const i32) as usize;

                                        Vec::from_raw_parts(
                                            *((base + 8) as *const i32) as *mut _,
                                            len2,
                                            len2,
                                        )
                                    }),
                                    14 => ValueResult::Raw({
                                        let len3 = *((base + 12) as *const i32) as usize;

                                        Vec::from_raw_parts(
                                            *((base + 8) as *const i32) as *mut _,
                                            len3,
                                            len3,
                                        )
                                    }),
                                    _ => panic!("invalid enum discriminant"),
                                });
                            }
                            std::alloc::dealloc(
                                base4 as *mut _,
                                std::alloc::Layout::from_size_align_unchecked(
                                    (len4 as usize) * 16,
                                    8,
                                ),
                            );

                            result4
                        }),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    1 => Err({
                        let len6 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len5 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len5,
                                        len5,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len6,
                                len6,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base11 = *((ptr0 + 36) as *const i32);
                                    let len11 = *((ptr0 + 40) as *const i32);
                                    let mut result11 = Vec::with_capacity(len11 as usize);
                                    for i in 0..len11 {
                                        let base = base11 + i * 24;
                                        result11.push({
                                            let len7 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len7,
                                                    len7,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len8 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len8,
                                                            len8,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len9 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len9,
                                                            len9,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len10 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len10,
                                                            len10,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base11 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len11 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result11
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Rows {
        pub fn values(&self) -> Result<Vec<Vec<ValueResult>>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "rows::values")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_rows::values"
                    )]
                    fn wit_import(_: i32, _: i32);
                }
                wit_import(self.0, ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok({
                        let base5 = *((ptr0 + 4) as *const i32);
                        let len5 = *((ptr0 + 8) as *const i32);
                        let mut result5 = Vec::with_capacity(len5 as usize);
                        for i in 0..len5 {
                            let base = base5 + i * 8;
                            result5.push({
                                let base4 = *((base + 0) as *const i32);
                                let len4 = *((base + 4) as *const i32);
                                let mut result4 = Vec::with_capacity(len4 as usize);
                                for i in 0..len4 {
                                    let base = base4 + i * 16;
                                    result4.push(match i32::from(*((base + 0) as *const u8)) {
                                        0 => ValueResult::Null,
                                        1 => ValueResult::Boolean(
                                            match i32::from(*((base + 8) as *const u8)) {
                                                0 => false,
                                                1 => true,
                                                _ => panic!("invalid bool discriminant"),
                                            },
                                        ),
                                        2 => ValueResult::Int8(
                                            i32::from(*((base + 8) as *const i8)) as i8,
                                        ),
                                        3 => ValueResult::Int16(i32::from(
                                            *((base + 8) as *const i16),
                                        )
                                            as i16),
                                        4 => ValueResult::Int32(*((base + 8) as *const i32)),
                                        5 => ValueResult::Int64(*((base + 8) as *const i64)),
                                        6 => ValueResult::Uint8(i32::from(
                                            *((base + 8) as *const u8),
                                        )
                                            as u8),
                                        7 => ValueResult::Uint16(i32::from(
                                            *((base + 8) as *const u16),
                                        )
                                            as u16),
                                        8 => {
                                            ValueResult::Uint32(*((base + 8) as *const i32) as u32)
                                        }
                                        9 => {
                                            ValueResult::Uint64(*((base + 8) as *const i64) as u64)
                                        }
                                        10 => ValueResult::Floating32(*((base + 8) as *const f32)),
                                        11 => ValueResult::Floating64(*((base + 8) as *const f64)),
                                        12 => ValueResult::Str({
                                            let len1 = *((base + 12) as *const i32) as usize;

                                            String::from_utf8(Vec::from_raw_parts(
                                                *((base + 8) as *const i32) as *mut _,
                                                len1,
                                                len1,
                                            ))
                                            .unwrap()
                                        }),
                                        13 => ValueResult::Bytes({
                                            let len2 = *((base + 12) as *const i32) as usize;

                                            Vec::from_raw_parts(
                                                *((base + 8) as *const i32) as *mut _,
                                                len2,
                                                len2,
                                            )
                                        }),
                                        14 => ValueResult::Raw({
                                            let len3 = *((base + 12) as *const i32) as usize;

                                            Vec::from_raw_parts(
                                                *((base + 8) as *const i32) as *mut _,
                                                len3,
                                                len3,
                                            )
                                        }),
                                        _ => panic!("invalid enum discriminant"),
                                    });
                                }
                                std::alloc::dealloc(
                                    base4 as *mut _,
                                    std::alloc::Layout::from_size_align_unchecked(
                                        (len4 as usize) * 16,
                                        8,
                                    ),
                                );

                                result4
                            });
                        }
                        std::alloc::dealloc(
                            base5 as *mut _,
                            std::alloc::Layout::from_size_align_unchecked((len5 as usize) * 8, 4),
                        );

                        result5
                    }),
                    1 => Err({
                        let len7 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len6 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len6,
                                        len6,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len7,
                                len7,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base12 = *((ptr0 + 36) as *const i32);
                                    let len12 = *((ptr0 + 40) as *const i32);
                                    let mut result12 = Vec::with_capacity(len12 as usize);
                                    for i in 0..len12 {
                                        let base = base12 + i * 24;
                                        result12.push({
                                            let len8 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len8,
                                                    len8,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len9 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len9,
                                                            len9,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len10 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len10,
                                                            len10,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len11 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len11,
                                                            len11,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base12 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len12 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result12
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Rows {
        pub fn close(&self) -> Result<(), SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "rows::close")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_rows::close"
                    )]
                    fn wit_import(_: i32, _: i32);
                }
                wit_import(self.0, ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(()),
                    1 => Err({
                        let len2 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 36) as *const i32);
                                    let len7 = *((ptr0 + 40) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_bool(&self, index: u32) -> Result<bool, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-bool")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-bool"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => false,
                        1 => true,
                        _ => panic!("invalid bool discriminant"),
                    }),
                    1 => Err({
                        let len2 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 36) as *const i32);
                                    let len7 = *((ptr0 + 40) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_int8(&self, index: u32) -> Result<i8, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-int8")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-int8"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(i32::from(*((ptr0 + 4) as *const i8)) as i8),
                    1 => Err({
                        let len2 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 36) as *const i32);
                                    let len7 = *((ptr0 + 40) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_int16(&self, index: u32) -> Result<i16, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-int16")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-int16"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(i32::from(*((ptr0 + 4) as *const i16)) as i16),
                    1 => Err({
                        let len2 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 36) as *const i32);
                                    let len7 = *((ptr0 + 40) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_int32(&self, index: u32) -> Result<i32, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-int32")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-int32"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(*((ptr0 + 4) as *const i32)),
                    1 => Err({
                        let len2 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 36) as *const i32);
                                    let len7 = *((ptr0 + 40) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_int64(&self, index: u64) -> Result<i64, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-int64")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-int64"
                    )]
                    fn wit_import(_: i32, _: i64, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i64(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(*((ptr0 + 8) as *const i64)),
                    1 => Err({
                        let len2 = *((ptr0 + 32) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 8) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 12) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 16) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 24) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 20) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 28) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 36) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 40) as *const i32);
                                    let len7 = *((ptr0 + 44) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_uint8(&self, index: u32) -> Result<u8, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-uint8")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-uint8"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(i32::from(*((ptr0 + 4) as *const u8)) as u8),
                    1 => Err({
                        let len2 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 36) as *const i32);
                                    let len7 = *((ptr0 + 40) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_uint16(&self, index: u32) -> Result<u16, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-uint16")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-uint16"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(i32::from(*((ptr0 + 4) as *const u16)) as u16),
                    1 => Err({
                        let len2 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 36) as *const i32);
                                    let len7 = *((ptr0 + 40) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_uint32(&self, index: u32) -> Result<u32, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-uint32")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-uint32"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(*((ptr0 + 4) as *const i32) as u32),
                    1 => Err({
                        let len2 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 36) as *const i32);
                                    let len7 = *((ptr0 + 40) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_uint64(&self, index: u64) -> Result<u64, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-uint64")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-uint64"
                    )]
                    fn wit_import(_: i32, _: i64, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i64(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(*((ptr0 + 8) as *const i64) as u64),
                    1 => Err({
                        let len2 = *((ptr0 + 32) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 8) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 12) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 16) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 24) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 20) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 28) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 36) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 40) as *const i32);
                                    let len7 = *((ptr0 + 44) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_float32(&self, index: u32) -> Result<f32, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-float32")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-float32"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(*((ptr0 + 4) as *const f32)),
                    1 => Err({
                        let len2 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 36) as *const i32);
                                    let len7 = *((ptr0 + 40) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_float64(&self, index: u32) -> Result<f64, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-float64")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-float64"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(*((ptr0 + 8) as *const f64)),
                    1 => Err({
                        let len2 = *((ptr0 + 32) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 8) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 12) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 16) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 24) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 20) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 28) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 36) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 40) as *const i32);
                                    let len7 = *((ptr0 + 44) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_string(&self, index: u32) -> Result<String, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-string")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-string"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok({
                        let len1 = *((ptr0 + 8) as *const i32) as usize;

                        String::from_utf8(Vec::from_raw_parts(
                            *((ptr0 + 4) as *const i32) as *mut _,
                            len1,
                            len1,
                        ))
                        .unwrap()
                    }),
                    1 => Err({
                        let len3 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len2 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len2,
                                        len2,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len3,
                                len3,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base8 = *((ptr0 + 36) as *const i32);
                                    let len8 = *((ptr0 + 40) as *const i32);
                                    let mut result8 = Vec::with_capacity(len8 as usize);
                                    for i in 0..len8 {
                                        let base = base8 + i * 24;
                                        result8.push({
                                            let len4 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len4,
                                                    len4,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len7 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len7,
                                                            len7,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base8 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len8 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result8
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_bytes(&self, index: u32) -> Result<Vec<u8>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-bytes")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-bytes"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok({
                        let len1 = *((ptr0 + 8) as *const i32) as usize;

                        Vec::from_raw_parts(*((ptr0 + 4) as *const i32) as *mut _, len1, len1)
                    }),
                    1 => Err({
                        let len3 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len2 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len2,
                                        len2,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len3,
                                len3,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base8 = *((ptr0 + 36) as *const i32);
                                    let len8 = *((ptr0 + 40) as *const i32);
                                    let mut result8 = Vec::with_capacity(len8 as usize);
                                    for i in 0..len8 {
                                        let base = base8 + i * 24;
                                        result8.push({
                                            let len4 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len4,
                                                    len4,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len7 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len7,
                                                            len7,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base8 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len8 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result8
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_raw(&self, index: u32) -> Result<Vec<u8>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-raw")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-raw"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok({
                        let len1 = *((ptr0 + 8) as *const i32) as usize;

                        Vec::from_raw_parts(*((ptr0 + 4) as *const i32) as *mut _, len1, len1)
                    }),
                    1 => Err({
                        let len3 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len2 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len2,
                                        len2,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len3,
                                len3,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base8 = *((ptr0 + 36) as *const i32);
                                    let len8 = *((ptr0 + 40) as *const i32);
                                    let mut result8 = Vec::with_capacity(len8 as usize);
                                    for i in 0..len8 {
                                        let base = base8 + i * 24;
                                        result8.push({
                                            let len4 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len4,
                                                    len4,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len7 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len7,
                                                            len7,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base8 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len8 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result8
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_optional_bool(&self, index: u32) -> Result<Option<bool>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-optional-bool")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-optional-bool"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => None,
                        1 => Some(match i32::from(*((ptr0 + 5) as *const u8)) {
                            0 => false,
                            1 => true,
                            _ => panic!("invalid bool discriminant"),
                        }),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    1 => Err({
                        let len2 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 36) as *const i32);
                                    let len7 = *((ptr0 + 40) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_optional_int8(&self, index: u32) -> Result<Option<i8>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-optional-int8")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-optional-int8"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => None,
                        1 => Some(i32::from(*((ptr0 + 5) as *const i8)) as i8),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    1 => Err({
                        let len2 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 36) as *const i32);
                                    let len7 = *((ptr0 + 40) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_optional_int16(&self, index: u32) -> Result<Option<i16>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-optional-int16")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-optional-int16"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => None,
                        1 => Some(i32::from(*((ptr0 + 6) as *const i16)) as i16),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    1 => Err({
                        let len2 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 36) as *const i32);
                                    let len7 = *((ptr0 + 40) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_optional_int32(&self, index: u32) -> Result<Option<i32>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-optional-int32")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-optional-int32"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => None,
                        1 => Some(*((ptr0 + 8) as *const i32)),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    1 => Err({
                        let len2 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 36) as *const i32);
                                    let len7 = *((ptr0 + 40) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_optional_int64(&self, index: u64) -> Result<Option<i64>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-optional-int64")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-optional-int64"
                    )]
                    fn wit_import(_: i32, _: i64, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i64(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(match i32::from(*((ptr0 + 8) as *const u8)) {
                        0 => None,
                        1 => Some(*((ptr0 + 16) as *const i64)),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    1 => Err({
                        let len2 = *((ptr0 + 32) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 8) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 12) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 16) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 24) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 20) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 28) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 36) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 40) as *const i32);
                                    let len7 = *((ptr0 + 44) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_optional_uint8(&self, index: u32) -> Result<Option<u8>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-optional-uint8")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-optional-uint8"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => None,
                        1 => Some(i32::from(*((ptr0 + 5) as *const u8)) as u8),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    1 => Err({
                        let len2 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 36) as *const i32);
                                    let len7 = *((ptr0 + 40) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_optional_uint16(&self, index: u32) -> Result<Option<u16>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-optional-uint16")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-optional-uint16"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => None,
                        1 => Some(i32::from(*((ptr0 + 6) as *const u16)) as u16),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    1 => Err({
                        let len2 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 36) as *const i32);
                                    let len7 = *((ptr0 + 40) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_optional_uint32(&self, index: u32) -> Result<Option<u32>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-optional-uint32")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-optional-uint32"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => None,
                        1 => Some(*((ptr0 + 8) as *const i32) as u32),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    1 => Err({
                        let len2 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 36) as *const i32);
                                    let len7 = *((ptr0 + 40) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_optional_uint64(&self, index: u64) -> Result<Option<u64>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-optional-uint64")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-optional-uint64"
                    )]
                    fn wit_import(_: i32, _: i64, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i64(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(match i32::from(*((ptr0 + 8) as *const u8)) {
                        0 => None,
                        1 => Some(*((ptr0 + 16) as *const i64) as u64),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    1 => Err({
                        let len2 = *((ptr0 + 32) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 8) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 12) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 16) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 24) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 20) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 28) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 36) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 40) as *const i32);
                                    let len7 = *((ptr0 + 44) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_optional_float32(&self, index: u32) -> Result<Option<f32>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-optional-float32")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-optional-float32"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => None,
                        1 => Some(*((ptr0 + 8) as *const f32)),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    1 => Err({
                        let len2 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 36) as *const i32);
                                    let len7 = *((ptr0 + 40) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_optional_float64(&self, index: u32) -> Result<Option<f64>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-optional-float64")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-optional-float64"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(match i32::from(*((ptr0 + 8) as *const u8)) {
                        0 => None,
                        1 => Some(*((ptr0 + 16) as *const f64)),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    1 => Err({
                        let len2 = *((ptr0 + 32) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 8) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 12) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 16) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len1 = *((ptr0 + 24) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 20) as *const i32) as *mut _,
                                        len1,
                                        len1,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 28) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 36) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base7 = *((ptr0 + 40) as *const i32);
                                    let len7 = *((ptr0 + 44) as *const i32);
                                    let mut result7 = Vec::with_capacity(len7 as usize);
                                    for i in 0..len7 {
                                        let base = base7 + i * 24;
                                        result7.push({
                                            let len3 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len3,
                                                    len3,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len4 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len4,
                                                            len4,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base7 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len7 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result7
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_optional_string(&self, index: u32) -> Result<Option<String>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-optional-string")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-optional-string"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => None,
                        1 => Some({
                            let len1 = *((ptr0 + 12) as *const i32) as usize;

                            String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 8) as *const i32) as *mut _,
                                len1,
                                len1,
                            ))
                            .unwrap()
                        }),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    1 => Err({
                        let len3 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len2 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len2,
                                        len2,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len3,
                                len3,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base8 = *((ptr0 + 36) as *const i32);
                                    let len8 = *((ptr0 + 40) as *const i32);
                                    let mut result8 = Vec::with_capacity(len8 as usize);
                                    for i in 0..len8 {
                                        let base = base8 + i * 24;
                                        result8.push({
                                            let len4 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len4,
                                                    len4,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len7 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len7,
                                                            len7,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base8 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len8 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result8
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_optional_bytes(&self, index: u32) -> Result<Option<Vec<u8>>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-optional-bytes")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-optional-bytes"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => None,
                        1 => Some({
                            let len1 = *((ptr0 + 12) as *const i32) as usize;

                            Vec::from_raw_parts(*((ptr0 + 8) as *const i32) as *mut _, len1, len1)
                        }),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    1 => Err({
                        let len3 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len2 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len2,
                                        len2,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len3,
                                len3,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base8 = *((ptr0 + 36) as *const i32);
                                    let len8 = *((ptr0 + 40) as *const i32);
                                    let mut result8 = Vec::with_capacity(len8 as usize);
                                    for i in 0..len8 {
                                        let base = base8 + i * 24;
                                        result8.push({
                                            let len4 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len4,
                                                    len4,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len7 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len7,
                                                            len7,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base8 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len8 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result8
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_optional_raw(&self, index: u32) -> Result<Option<Vec<u8>>, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-optional-raw")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-optional-raw"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => None,
                        1 => Some({
                            let len1 = *((ptr0 + 12) as *const i32) as usize;

                            Vec::from_raw_parts(*((ptr0 + 8) as *const i32) as *mut _, len1, len1)
                        }),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    1 => Err({
                        let len3 = *((ptr0 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len2 = *((ptr0 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 16) as *const i32) as *mut _,
                                        len2,
                                        len2,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 24) as *const i32) as *mut _,
                                len3,
                                len3,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base8 = *((ptr0 + 36) as *const i32);
                                    let len8 = *((ptr0 + 40) as *const i32);
                                    let mut result8 = Vec::with_capacity(len8 as usize);
                                    for i in 0..len8 {
                                        let base = base8 + i * 24;
                                        result8.push({
                                            let len4 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len4,
                                                    len4,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len5 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len5,
                                                            len5,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len6 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len6,
                                                            len6,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len7 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len7,
                                                            len7,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base8 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len8 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result8
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Row {
        pub fn get_value(&self, index: u32) -> Result<ValueResult, SqlError> {
            unsafe {
                let ptr0 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "row::get-value")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_row::get-value"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, wit_bindgen_rust::rt::as_i32(index), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(match i32::from(*((ptr0 + 8) as *const u8)) {
                        0 => ValueResult::Null,
                        1 => ValueResult::Boolean(match i32::from(*((ptr0 + 16) as *const u8)) {
                            0 => false,
                            1 => true,
                            _ => panic!("invalid bool discriminant"),
                        }),
                        2 => ValueResult::Int8(i32::from(*((ptr0 + 16) as *const i8)) as i8),
                        3 => ValueResult::Int16(i32::from(*((ptr0 + 16) as *const i16)) as i16),
                        4 => ValueResult::Int32(*((ptr0 + 16) as *const i32)),
                        5 => ValueResult::Int64(*((ptr0 + 16) as *const i64)),
                        6 => ValueResult::Uint8(i32::from(*((ptr0 + 16) as *const u8)) as u8),
                        7 => ValueResult::Uint16(i32::from(*((ptr0 + 16) as *const u16)) as u16),
                        8 => ValueResult::Uint32(*((ptr0 + 16) as *const i32) as u32),
                        9 => ValueResult::Uint64(*((ptr0 + 16) as *const i64) as u64),
                        10 => ValueResult::Floating32(*((ptr0 + 16) as *const f32)),
                        11 => ValueResult::Floating64(*((ptr0 + 16) as *const f64)),
                        12 => ValueResult::Str({
                            let len1 = *((ptr0 + 20) as *const i32) as usize;

                            String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 16) as *const i32) as *mut _,
                                len1,
                                len1,
                            ))
                            .unwrap()
                        }),
                        13 => ValueResult::Bytes({
                            let len2 = *((ptr0 + 20) as *const i32) as usize;

                            Vec::from_raw_parts(*((ptr0 + 16) as *const i32) as *mut _, len2, len2)
                        }),
                        14 => ValueResult::Raw({
                            let len3 = *((ptr0 + 20) as *const i32) as usize;

                            Vec::from_raw_parts(*((ptr0 + 16) as *const i32) as *mut _, len3, len3)
                        }),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    1 => Err({
                        let len5 = *((ptr0 + 32) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr0 + 8) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 12) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr0 + 16) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len4 = *((ptr0 + 24) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr0 + 20) as *const i32) as *mut _,
                                        len4,
                                        len4,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 28) as *const i32) as *mut _,
                                len5,
                                len5,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr0 + 36) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base10 = *((ptr0 + 40) as *const i32);
                                    let len10 = *((ptr0 + 44) as *const i32);
                                    let mut result10 = Vec::with_capacity(len10 as usize);
                                    for i in 0..len10 {
                                        let base = base10 + i * 24;
                                        result10.push({
                                            let len6 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len6,
                                                    len6,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len7 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len7,
                                                            len7,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len8 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len8,
                                                            len8,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len9 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len9,
                                                            len9,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base10 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len10 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result10
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl PreparedStatement {
        pub fn execute(&self, args: &[ValueParam<'_>]) -> Result<(), SqlError> {
            unsafe {
                let vec3 = args;
                let len3 = vec3.len() as i32;
                let layout3 = core::alloc::Layout::from_size_align_unchecked(vec3.len() * 16, 8);
                let result3 = std::alloc::alloc(layout3);
                if result3.is_null() {
                    std::alloc::handle_alloc_error(layout3);
                }
                for (i, e) in vec3.into_iter().enumerate() {
                    let base = result3 as i32 + (i as i32) * 16;
                    {
                        match e {
                            ValueParam::Null => {
                                let e = ();
                                {
                                    *((base + 0) as *mut u8) = (0i32) as u8;
                                    let () = e;
                                }
                            }
                            ValueParam::Boolean(e) => {
                                *((base + 0) as *mut u8) = (1i32) as u8;
                                *((base + 8) as *mut u8) = (match e {
                                    true => 1,
                                    false => 0,
                                }) as u8;
                            }
                            ValueParam::Int8(e) => {
                                *((base + 0) as *mut u8) = (2i32) as u8;
                                *((base + 8) as *mut u8) = (wit_bindgen_rust::rt::as_i32(e)) as u8;
                            }
                            ValueParam::Int16(e) => {
                                *((base + 0) as *mut u8) = (3i32) as u8;
                                *((base + 8) as *mut u16) =
                                    (wit_bindgen_rust::rt::as_i32(e)) as u16;
                            }
                            ValueParam::Int32(e) => {
                                *((base + 0) as *mut u8) = (4i32) as u8;
                                *((base + 8) as *mut i32) = wit_bindgen_rust::rt::as_i32(e);
                            }
                            ValueParam::Int64(e) => {
                                *((base + 0) as *mut u8) = (5i32) as u8;
                                *((base + 8) as *mut i64) = wit_bindgen_rust::rt::as_i64(e);
                            }
                            ValueParam::Uint8(e) => {
                                *((base + 0) as *mut u8) = (6i32) as u8;
                                *((base + 8) as *mut u8) = (wit_bindgen_rust::rt::as_i32(e)) as u8;
                            }
                            ValueParam::Uint16(e) => {
                                *((base + 0) as *mut u8) = (7i32) as u8;
                                *((base + 8) as *mut u16) =
                                    (wit_bindgen_rust::rt::as_i32(e)) as u16;
                            }
                            ValueParam::Uint32(e) => {
                                *((base + 0) as *mut u8) = (8i32) as u8;
                                *((base + 8) as *mut i32) = wit_bindgen_rust::rt::as_i32(e);
                            }
                            ValueParam::Uint64(e) => {
                                *((base + 0) as *mut u8) = (9i32) as u8;
                                *((base + 8) as *mut i64) = wit_bindgen_rust::rt::as_i64(e);
                            }
                            ValueParam::Floating32(e) => {
                                *((base + 0) as *mut u8) = (10i32) as u8;
                                *((base + 8) as *mut f32) = wit_bindgen_rust::rt::as_f32(e);
                            }
                            ValueParam::Floating64(e) => {
                                *((base + 0) as *mut u8) = (11i32) as u8;
                                *((base + 8) as *mut f64) = wit_bindgen_rust::rt::as_f64(e);
                            }
                            ValueParam::Str(e) => {
                                *((base + 0) as *mut u8) = (12i32) as u8;
                                let vec0 = e;
                                let ptr0 = vec0.as_ptr() as i32;
                                let len0 = vec0.len() as i32;
                                *((base + 12) as *mut i32) = len0;
                                *((base + 8) as *mut i32) = ptr0;
                            }
                            ValueParam::Bytes(e) => {
                                *((base + 0) as *mut u8) = (13i32) as u8;
                                let vec1 = e;
                                let ptr1 = vec1.as_ptr() as i32;
                                let len1 = vec1.len() as i32;
                                *((base + 12) as *mut i32) = len1;
                                *((base + 8) as *mut i32) = ptr1;
                            }
                            ValueParam::Raw(e) => {
                                *((base + 0) as *mut u8) = (14i32) as u8;
                                let vec2 = e;
                                let ptr2 = vec2.as_ptr() as i32;
                                let len2 = vec2.len() as i32;
                                *((base + 12) as *mut i32) = len2;
                                *((base + 8) as *mut i32) = ptr2;
                            }
                        };
                    }
                }
                let ptr4 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "prepared-statement::execute")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_prepared-statement::execute"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32, _: i32);
                }
                wit_import(self.0, result3 as i32, len3, ptr4);
                std::alloc::dealloc(result3, layout3);
                match i32::from(*((ptr4 + 0) as *const u8)) {
                    0 => Ok(()),
                    1 => Err({
                        let len6 = *((ptr4 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr4 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr4 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr4 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len5 = *((ptr4 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr4 + 16) as *const i32) as *mut _,
                                        len5,
                                        len5,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr4 + 24) as *const i32) as *mut _,
                                len6,
                                len6,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr4 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base11 = *((ptr4 + 36) as *const i32);
                                    let len11 = *((ptr4 + 40) as *const i32);
                                    let mut result11 = Vec::with_capacity(len11 as usize);
                                    for i in 0..len11 {
                                        let base = base11 + i * 24;
                                        result11.push({
                                            let len7 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len7,
                                                    len7,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len8 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len8,
                                                            len8,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len9 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len9,
                                                            len9,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len10 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len10,
                                                            len10,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base11 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len11 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result11
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl PreparedStatement {
        pub fn query(&self, args: &[ValueParam<'_>]) -> Result<Rows, SqlError> {
            unsafe {
                let vec3 = args;
                let len3 = vec3.len() as i32;
                let layout3 = core::alloc::Layout::from_size_align_unchecked(vec3.len() * 16, 8);
                let result3 = std::alloc::alloc(layout3);
                if result3.is_null() {
                    std::alloc::handle_alloc_error(layout3);
                }
                for (i, e) in vec3.into_iter().enumerate() {
                    let base = result3 as i32 + (i as i32) * 16;
                    {
                        match e {
                            ValueParam::Null => {
                                let e = ();
                                {
                                    *((base + 0) as *mut u8) = (0i32) as u8;
                                    let () = e;
                                }
                            }
                            ValueParam::Boolean(e) => {
                                *((base + 0) as *mut u8) = (1i32) as u8;
                                *((base + 8) as *mut u8) = (match e {
                                    true => 1,
                                    false => 0,
                                }) as u8;
                            }
                            ValueParam::Int8(e) => {
                                *((base + 0) as *mut u8) = (2i32) as u8;
                                *((base + 8) as *mut u8) = (wit_bindgen_rust::rt::as_i32(e)) as u8;
                            }
                            ValueParam::Int16(e) => {
                                *((base + 0) as *mut u8) = (3i32) as u8;
                                *((base + 8) as *mut u16) =
                                    (wit_bindgen_rust::rt::as_i32(e)) as u16;
                            }
                            ValueParam::Int32(e) => {
                                *((base + 0) as *mut u8) = (4i32) as u8;
                                *((base + 8) as *mut i32) = wit_bindgen_rust::rt::as_i32(e);
                            }
                            ValueParam::Int64(e) => {
                                *((base + 0) as *mut u8) = (5i32) as u8;
                                *((base + 8) as *mut i64) = wit_bindgen_rust::rt::as_i64(e);
                            }
                            ValueParam::Uint8(e) => {
                                *((base + 0) as *mut u8) = (6i32) as u8;
                                *((base + 8) as *mut u8) = (wit_bindgen_rust::rt::as_i32(e)) as u8;
                            }
                            ValueParam::Uint16(e) => {
                                *((base + 0) as *mut u8) = (7i32) as u8;
                                *((base + 8) as *mut u16) =
                                    (wit_bindgen_rust::rt::as_i32(e)) as u16;
                            }
                            ValueParam::Uint32(e) => {
                                *((base + 0) as *mut u8) = (8i32) as u8;
                                *((base + 8) as *mut i32) = wit_bindgen_rust::rt::as_i32(e);
                            }
                            ValueParam::Uint64(e) => {
                                *((base + 0) as *mut u8) = (9i32) as u8;
                                *((base + 8) as *mut i64) = wit_bindgen_rust::rt::as_i64(e);
                            }
                            ValueParam::Floating32(e) => {
                                *((base + 0) as *mut u8) = (10i32) as u8;
                                *((base + 8) as *mut f32) = wit_bindgen_rust::rt::as_f32(e);
                            }
                            ValueParam::Floating64(e) => {
                                *((base + 0) as *mut u8) = (11i32) as u8;
                                *((base + 8) as *mut f64) = wit_bindgen_rust::rt::as_f64(e);
                            }
                            ValueParam::Str(e) => {
                                *((base + 0) as *mut u8) = (12i32) as u8;
                                let vec0 = e;
                                let ptr0 = vec0.as_ptr() as i32;
                                let len0 = vec0.len() as i32;
                                *((base + 12) as *mut i32) = len0;
                                *((base + 8) as *mut i32) = ptr0;
                            }
                            ValueParam::Bytes(e) => {
                                *((base + 0) as *mut u8) = (13i32) as u8;
                                let vec1 = e;
                                let ptr1 = vec1.as_ptr() as i32;
                                let len1 = vec1.len() as i32;
                                *((base + 12) as *mut i32) = len1;
                                *((base + 8) as *mut i32) = ptr1;
                            }
                            ValueParam::Raw(e) => {
                                *((base + 0) as *mut u8) = (14i32) as u8;
                                let vec2 = e;
                                let ptr2 = vec2.as_ptr() as i32;
                                let len2 = vec2.len() as i32;
                                *((base + 12) as *mut i32) = len2;
                                *((base + 8) as *mut i32) = ptr2;
                            }
                        };
                    }
                }
                let ptr4 = SQL_V1_ALPHA1_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "sql_v1_alpha1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "prepared-statement::query")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "sql_v1_alpha1_prepared-statement::query"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32, _: i32);
                }
                wit_import(self.0, result3 as i32, len3, ptr4);
                std::alloc::dealloc(result3, layout3);
                match i32::from(*((ptr4 + 0) as *const u8)) {
                    0 => Ok(Rows(*((ptr4 + 4) as *const i32))),
                    1 => Err({
                        let len6 = *((ptr4 + 28) as *const i32) as usize;

                        SqlError {
                            code_numeric: match i32::from(*((ptr4 + 4) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr4 + 8) as *const i32)),
                                _ => panic!("invalid enum discriminant"),
                            },
                            code_text: match i32::from(*((ptr4 + 12) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let len5 = *((ptr4 + 20) as *const i32) as usize;

                                    String::from_utf8(Vec::from_raw_parts(
                                        *((ptr4 + 16) as *const i32) as *mut _,
                                        len5,
                                        len5,
                                    ))
                                    .unwrap()
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                            message: String::from_utf8(Vec::from_raw_parts(
                                *((ptr4 + 24) as *const i32) as *mut _,
                                len6,
                                len6,
                            ))
                            .unwrap(),
                            extra: match i32::from(*((ptr4 + 32) as *const u8)) {
                                0 => None,
                                1 => Some({
                                    let base11 = *((ptr4 + 36) as *const i32);
                                    let len11 = *((ptr4 + 40) as *const i32);
                                    let mut result11 = Vec::with_capacity(len11 as usize);
                                    for i in 0..len11 {
                                        let base = base11 + i * 24;
                                        result11.push({
                                            let len7 = *((base + 4) as *const i32) as usize;

                                            ValueMapItemResult {
                                                key: String::from_utf8(Vec::from_raw_parts(
                                                    *((base + 0) as *const i32) as *mut _,
                                                    len7,
                                                    len7,
                                                ))
                                                .unwrap(),
                                                value: match i32::from(*((base + 8) as *const u8)) {
                                                    0 => ValueResult::Null,
                                                    1 => ValueResult::Boolean(
                                                        match i32::from(*((base + 16) as *const u8))
                                                        {
                                                            0 => false,
                                                            1 => true,
                                                            _ => {
                                                                panic!("invalid bool discriminant")
                                                            }
                                                        },
                                                    ),
                                                    2 => ValueResult::Int8(i32::from(
                                                        *((base + 16) as *const i8),
                                                    )
                                                        as i8),
                                                    3 => ValueResult::Int16(i32::from(
                                                        *((base + 16) as *const i16),
                                                    )
                                                        as i16),
                                                    4 => ValueResult::Int32(
                                                        *((base + 16) as *const i32),
                                                    ),
                                                    5 => ValueResult::Int64(
                                                        *((base + 16) as *const i64),
                                                    ),
                                                    6 => ValueResult::Uint8(i32::from(
                                                        *((base + 16) as *const u8),
                                                    )
                                                        as u8),
                                                    7 => ValueResult::Uint16(i32::from(
                                                        *((base + 16) as *const u16),
                                                    )
                                                        as u16),
                                                    8 => ValueResult::Uint32(
                                                        *((base + 16) as *const i32) as u32,
                                                    ),
                                                    9 => ValueResult::Uint64(
                                                        *((base + 16) as *const i64) as u64,
                                                    ),
                                                    10 => ValueResult::Floating32(
                                                        *((base + 16) as *const f32),
                                                    ),
                                                    11 => ValueResult::Floating64(
                                                        *((base + 16) as *const f64),
                                                    ),
                                                    12 => ValueResult::Str({
                                                        let len8 =
                                                            *((base + 20) as *const i32) as usize;

                                                        String::from_utf8(Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len8,
                                                            len8,
                                                        ))
                                                        .unwrap()
                                                    }),
                                                    13 => ValueResult::Bytes({
                                                        let len9 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len9,
                                                            len9,
                                                        )
                                                    }),
                                                    14 => ValueResult::Raw({
                                                        let len10 =
                                                            *((base + 20) as *const i32) as usize;

                                                        Vec::from_raw_parts(
                                                            *((base + 16) as *const i32) as *mut _,
                                                            len10,
                                                            len10,
                                                        )
                                                    }),
                                                    _ => panic!("invalid enum discriminant"),
                                                },
                                            }
                                        });
                                    }
                                    std::alloc::dealloc(
                                        base11 as *mut _,
                                        std::alloc::Layout::from_size_align_unchecked(
                                            (len11 as usize) * 24,
                                            8,
                                        ),
                                    );

                                    result11
                                }),
                                _ => panic!("invalid enum discriminant"),
                            },
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }

    #[repr(align(8))]
    struct RetArea([u8; 44]);
    static mut SQL_V1_ALPHA1_RET_AREA: RetArea = RetArea([0; 44]);
}

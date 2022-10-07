/// A function rarely modifying the data
mod test_1 {
    use std::borrow::Cow;

    fn remove_whitespaces(s: &str) -> Cow<str> {
        if s.contains(' ') {
            Cow::Owned(s.to_string().replace(' ', ""))
        } else {
            Cow::Borrowed(s)
        }
    }
    
    pub fn run() {
        println!("CASE 1: A function rarely modifying the data");

        let value = remove_whitespaces("Hello world!");
        println!("{}", value);

        println!();
    }
}

/// A struct optionally owning the data
mod test_2 {
    use std::borrow::Cow;

    struct User<'a> {
        first_name: Cow<'a, str>,
        last_name: Cow<'a, str>,
    }

    impl<'a> User<'a> {

        pub fn new_owned(first_name: String, last_name: String) -> User<'static> {
            User {
                first_name: Cow::Owned(first_name),
                last_name: Cow::Owned(last_name),
            }
        }
    
        pub fn new_borrowed(first_name: &'a str, last_name: &'a str) -> Self {
            Self {
                first_name: Cow::Borrowed(first_name),
                last_name: Cow::Borrowed(last_name),
            }
        }
    
    
        pub fn first_name(&self) -> &str {
            &self.first_name
        }

        pub fn last_name(&self) -> &str {
            &self.last_name
        }
    }

    pub fn run() {
        println!("CASE 2: A struct optionally owning the data");

        // Static lifetime as it owns the data
        let user: User<'static> = User::new_owned("James".to_owned(), "Bond".to_owned());
        println!("Name (1): {} {}", user.first_name(), user.last_name());

        // Static lifetime as it borrows 'static data
        let user: User<'static> = User::new_borrowed("James", "Bond");
        println!("Name (2): {} {}", user.first_name, user.last_name);

        let first_name = "James".to_owned();
        let last_name = "Bond".to_owned();

        // Non-static lifetime as it borrows the data
        let user= User::new_borrowed(&first_name, &last_name);
        println!("Name (3): {} {}", user.first_name, user.last_name);

        println!();
    }
}

/// A clone on write struct
mod test_3 {
    use std::borrow::Cow;

    struct LazyBuffer<'a> {
        data: Cow<'a, [u8]>,
    }
    
    impl<'a> LazyBuffer<'a> {
    
        pub fn new(data: &'a[u8]) -> Self {
            Self {
                data: Cow::Borrowed(data),
            }
        }
    
        pub fn data(&self) -> &[u8] {
            &self.data
        }
    
        pub fn append(&mut self, data: &[u8]) {
            self.data.to_mut().extend(data)
        }
    }

    pub fn run() {
        println!("CASE 3: A clone on write struct");

        let data = vec![0u8; 10];

        // No memory copied yet
        let mut buffer = LazyBuffer::new(&data);
        println!("Data(1): {:?}", buffer.data());

        // The data is cloned
        buffer.append(&[1, 2, 3]);
        println!("Data(2): {:?}", buffer.data());

        // The data is not cloned on further attempts
        buffer.append(&[4, 5, 6]);
        println!("Data(3): {:?}", buffer.data());

        println!();
    }
}

mod test_4 {
    use std::borrow::{Borrow, Cow};
    use std::ops::Deref;

    #[derive(Debug)]
    struct MyString {
        data: String
    }

    impl Borrow<MyStr> for MyString {
        fn borrow(&self) -> &MyStr {
            unsafe { &*(self.data.as_str() as *const str as *const MyStr) }
        }
    }

    impl Deref for MyString {
        type Target = MyStr;
    
        fn deref(&self) -> &Self::Target {
            self.borrow()
        }
    }

    #[derive(Debug)]
    #[repr(transparent)]
    struct MyStr {
        data: str,
    }

    impl ToOwned for MyStr {
        type Owned = MyString;
    
        fn to_owned(&self) -> MyString {
            MyString {
                data: self.data.to_owned()
            }
        }
    }

    pub fn run()  {
        println!("CASE 4: Keep your own type inside it");

        let data = MyString { data: "Hello world".to_owned() };
    
        let borrowed_cow: Cow<'_, MyStr> = Cow::Borrowed(&data);
        println!("Borrowed: {:?}", borrowed_cow);
    
        let owned_cow: Cow<'_, MyStr> = Cow::Owned(data);
        println!("Owned: {:?}", owned_cow);

        println!();
    }
}

mod test_5 {
    use std::borrow::{Borrow, Cow};
    use std::fmt::Debug;
    use std::ops::Deref;

    trait MyTrait: Debug {
        fn data(&self) -> &str;
    }

    #[derive(Debug)]
    struct MyString {
        data: String
    }

    impl MyTrait for MyString {
        fn data(&self) -> &str {
            &self.data
        }
    }

    impl<'a> Borrow<dyn MyTrait + 'a> for MyString {
        fn borrow(&self) -> &(dyn MyTrait + 'a) {
            self
        }
    }

    impl ToOwned for dyn MyTrait {
        type Owned = MyString;
    
        fn to_owned(&self) -> MyString {
            MyString {
                data: self.data().to_owned()
            }
        }
    }

    pub fn run()  {
        println!("CASE 5: Borrow the type as dyn Trait");

        let data = MyString { data: "Hello world".to_owned() };
    
        let borrowed_cow: Cow<'_, dyn MyTrait> = Cow::Borrowed(&data);
        println!("Borrowed: {:?}", borrowed_cow);
    
        let owned_cow: Cow<'_, dyn MyTrait> = Cow::Owned(data);
        println!("Owned: {:?}", owned_cow);

        let data = MyString { data: "Hello world".to_owned() };
        let cow1: Cow<'_, dyn MyTrait> = Cow::Borrowed(&data);
    
        let data = MyString { data: "Hello world".to_owned() };
        let cow2: Cow<'_, dyn MyTrait> = Cow::Owned(data);
    
        let mut vector: Vec<Cow<'_, dyn MyTrait>> = vec![cow1, cow2];

        println!();
    }
}

mod test_6 {
    use std::borrow::{Borrow, Cow};
    use std::fmt::{Debug, Formatter};
    use std::ops::Deref;

    struct NativeBuffer {
        pub ptr: *const u8,
        pub len: usize
    }

    impl Borrow<[u8]> for NativeBuffer {
        fn borrow(&self) -> &[u8] {
            unsafe {
                std::slice::from_raw_parts(self.ptr, self.len)
            }
        }
    }
    
    impl Deref for NativeBuffer {
        type Target = [u8];
    
        fn deref(&self) -> &Self::Target {
            self.borrow()
        }
    }
    
    impl Debug for NativeBuffer {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let data: &[u8] = self.borrow();
            write!(f, "NativeBuffer {{ data: {:?}, len: {} }}", data, self.len)
        }
    }

    #[derive(Debug)]
    struct OwnedBuffer {
        owned_data: Vec<u8>,
        native_proxy: NativeBuffer,
    }
    
    impl ToOwned for NativeBuffer {
        type Owned = OwnedBuffer;
    
        fn to_owned(&self) -> OwnedBuffer {
            let slice: &[u8] = self.borrow();
            let owned_data = slice.to_vec();
            let native_proxy = NativeBuffer {
                ptr: owned_data.as_ptr(),
                len: owned_data.len()
            };
            OwnedBuffer {
                owned_data,
                native_proxy,
            }
        }
    }

    impl Borrow<NativeBuffer> for OwnedBuffer {
        fn borrow(&self) -> &NativeBuffer {
            &self.native_proxy
        }
    }

    impl OwnedBuffer {

        pub fn append(&mut self, data: &[u8]) {
            self.owned_data.extend(data);
            self.native_proxy = NativeBuffer {
                ptr: self.owned_data.as_ptr(),
                len: self.owned_data.len()
            };
        }
    }

    pub fn run() {
        println!("CASE 6: Implement safe wrapper over FFI type");

        // Simulates the data coming across FFI (from C)
        let data = vec![1, 2, 3];
        let ptr = data.as_ptr();
        let len = data.len();
    
        let native_buffer = NativeBuffer { ptr, len};
        let mut buffer = Cow::Borrowed(&native_buffer);
        // NativeBuffer { data: [1, 2, 3], len: 3 }
        println!("{:?}", buffer);
    
        // No data cloned
        assert_eq!(buffer.ptr, ptr);
        assert_eq!(buffer.len, len);
    
        if buffer.len > 1 {
            buffer.to_mut().append(&[4, 5, 6]);
            // OwnedBuffer { owned_data: [1, 2, 3, 4, 5, 6], native_proxy: NativeBuffer { data: [1, 2, 3, 4, 5, 6], len: 6 } }
            println!("{:?}", buffer);
    
            // Data is cloned
            assert_ne!(buffer.ptr, ptr);
            assert_eq!(buffer.len, len + 3);
        }
    
        let slice: &[u8] = &buffer;
        // [1, 2, 3, 4, 5, 6]
        println!("{:?}", slice);

        println!();
    }
}

fn main() {
    test_1::run();
    test_2::run();
    test_3::run();
    test_4::run();
    test_5::run();
    test_6::run();
}

/// Little endian memory implementations.

/// This provides default implementations for the `Mem16` trait, however they are not very optimal
/// and you might get better performance from implementing them yourself.
#[macro_export]
macro_rules! impl_mem_16_little {
    {$struct:ident} => {
        impl Mem16 for $struct {
            fn read_halfword(&mut self, addr: Self::Addr) -> u16 {
                use num_traits::identities::One;

                let lo = self.read_byte(addr);
                let hi = self.read_byte(addr + Self::Addr::one());
                make16!(lo, hi)
            }

            fn write_halfword(&mut self, addr: Self::Addr, data: u16) {
                use num_traits::identities::One;

                let lo = lo16!(data);
                let hi = hi16!(data);
                self.write_byte(addr, lo);
                self.write_byte(addr + Self::Addr::one(), hi);
            }

            fn little_endian(&self) -> bool {
                true
            }
        }
    };
}

/// This provides default implementations for the `Mem32` and `Mem16` traits, however they are not very optimal
/// and you might get better performance from implementing them yourself.
#[macro_export]
macro_rules! impl_mem_32_little {
    {$struct:ident} => {
        impl_mem_16_little!{ $struct }

        impl Mem32 for $struct {
            fn read_word(&mut self, addr: Self::Addr) -> u32 {
                use num_traits::identities::One;

                let addr0 = addr;
                let addr1 = addr0 + Self::Addr::one();
                let addr2 = addr1 + Self::Addr::one();
                let addr3 = addr2 + Self::Addr::one();
                let b0 = self.read_byte(addr0);
                let b1 = self.read_byte(addr1);
                let b2 = self.read_byte(addr2);
                let b3 = self.read_byte(addr3);
                make32!(b0, b1, b2, b3)
            }

            fn write_word(&mut self, addr: Self::Addr, data: u32) {
                use num_traits::identities::One;

                let bytes = bytes32!(data);
                let addr0 = addr;
                let addr1 = addr0 + Self::Addr::one();
                let addr2 = addr1 + Self::Addr::one();
                let addr3 = addr2 + Self::Addr::one();
                self.write_byte(addr0, bytes.0);
                self.write_byte(addr1, bytes.1);
                self.write_byte(addr2, bytes.2);
                self.write_byte(addr3, bytes.3);
            }
        }
    };
}

/// This provides default implementations for the `Mem64`, `Mem32` and `Mem16` traits, however they are not very optimal
/// and you might get better performance from implementing them yourself.
#[macro_export]
macro_rules! impl_mem_64_little {
    {$struct:ident} => {
        impl_mem_32_little!{ $struct }

        impl Mem64 for $struct {
            fn read_doubleword(&mut self, addr: Self::Addr) -> u64 {
                use num_traits::identities::One;

                let addr0 = addr;
                let addr1 = addr0 + Self::Addr::one();
                let addr2 = addr1 + Self::Addr::one();
                let addr3 = addr2 + Self::Addr::one();
                let addr4 = addr3 + Self::Addr::one();
                let addr5 = addr4 + Self::Addr::one();
                let addr6 = addr5 + Self::Addr::one();
                let addr7 = addr6 + Self::Addr::one();
                let b0 = self.read_byte(addr0);
                let b1 = self.read_byte(addr1);
                let b2 = self.read_byte(addr2);
                let b3 = self.read_byte(addr3);
                let b4 = self.read_byte(addr4);
                let b5 = self.read_byte(addr5);
                let b6 = self.read_byte(addr6);
                let b7 = self.read_byte(addr7);
                make64!(b0, b1, b2, b3, b4, b5, b6, b7)
            }

            fn write_doubleword(&mut self, addr: Self::Addr, data: u64) {
                use num_traits::identities::One;

                let bytes = bytes64!(data);
                let addr0 = addr;
                let addr1 = addr0 + Self::Addr::one();
                let addr2 = addr1 + Self::Addr::one();
                let addr3 = addr2 + Self::Addr::one();
                let addr4 = addr3 + Self::Addr::one();
                let addr5 = addr4 + Self::Addr::one();
                let addr6 = addr5 + Self::Addr::one();
                let addr7 = addr6 + Self::Addr::one();
                self.write_byte(addr0, bytes.0);
                self.write_byte(addr1, bytes.1);
                self.write_byte(addr2, bytes.2);
                self.write_byte(addr3, bytes.3);
                self.write_byte(addr4, bytes.4);
                self.write_byte(addr5, bytes.5);
                self.write_byte(addr6, bytes.6);
                self.write_byte(addr7, bytes.7);
            }
        }
    };
}

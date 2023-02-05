use md5::Digest;
use hex_lit::hex;

struct MD5HashCashInput {
    // complexity in bits
    complexity: u32,
    // message to sign
    message: String,
}
impl MD5HashCashInput {
    fn createMD5HashCashInput(complexity: u32, message: String) -> Self {
        MD5HashCashInput {
            complexity,
            message,
        }
    }
}

struct MD5HashCashOutput {
    // Seed used to solve the challenge
    seed: u64,
    // hashcode found using seed + message
    hashcode: String,
}
impl MD5HashCashOutput {
    fn createMD5HashCashOutput(seed: u64, hashcode: String) -> Self {
        MD5HashCashOutput {
            seed,
            hashcode,
        }
    }

    fn solve(self){
        s.update("000000000000034Chello");
    }

    fn generate_seed(&mut self) {
        //let mut seedBinary = self.seed.to_le_bytes();
        //seedBinary = seedBinary + 1_i64.to_le_bytes();
        //self.seed = u64::from_be_bytes(seedBinary);
        self.seed = self.seed + 1;
    }
}

fn main(){
    let mut test = MD5HashCashOutput{
        seed: 0,
        hashcode: "".to_string(),
    };
    //println!("{}", test.seed);
    //test.generate_seed();
    //println!("{}", test.seed);
    //let test2 = createMD5HashCashInput(9, String::from("Hello World"));
    //let bits:[u8;8] = test.seed.to_be_bytes();
    //for bit in bits {
    //    print!("{}", bit);
    //}
    //let mut hasher = md5::Md5::new();
    //hasher.update("hello world");
    //let result = hasher.finalize();
    //for i in result.to_vec() {
//        print!("{}", i);
    //}
    //println:();
    let mut s = md5::Md5::new();
    s.update("000000000000034Chello");
    let s_bin = s.finalize();
    println!("binary : {:X}", s_bin);
    assert_eq!(s_bin[..], hex!("00441745D9BDF8E5D3C7872AC9DBB2C3"));
    //let h_bin = md5::Md5::digest(b"000000000000034Chello");
    //for i in h_bin.to_vec() {
    //    print!("{:X}", i);
    //}
}
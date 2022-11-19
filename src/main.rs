use ngmerge_sys::{self, run_rust};
use std::borrow::Borrow;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::str::from_utf8;
use lazy_static::lazy_static;

// use std::ffi::Cint;

// extern "C" {
//     pub fn findPos(
//         seq1: *mut ::std::os::raw::c_char,
//         seq2: *mut ::std::os::raw::c_char,
//         qual1: *mut ::std::os::raw::c_char,
//         qual2: *mut ::std::os::raw::c_char,
//         len1: ::std::os::raw::c_int,
//         len2: ::std::os::raw::c_int,
//         overlap: ::std::os::raw::c_int,
//         dovetail: bool,
//         doveOverlap: ::std::os::raw::c_int,
//         mismatch: f32,
//         maxLen: bool,
//         best: *mut f32,
//     ) -> ::std::os::raw::c_int;
// }

fn str2c_char(str: &[u8]) -> *mut c_char {
    let c_string = CString::new(str.to_owned()).expect("CString::new failed");
    let raw: *mut c_char = c_string.into_raw();
    return raw;
}

lazy_static! {
    static ref COMPLEMENT: [u8; 256] = {
        let mut comp = [0; 256];
        for (v, a) in comp.iter_mut().enumerate() {
            *a = v as u8;
        }
        for (&a, &b) in b"AGCTYRWSKMDVHBN".iter().zip(b"TCGARYWSMKHBDVN".iter()) {
            comp[a as usize] = b;
            comp[a as usize + 32] = b + 32;  // lowercase variants
        }
        comp
    };
}

/// assert_eq!(dna::complement(65), 84); // A → T
/// assert_eq!(dna::complement(99), 103); // c → g
/// assert_eq!(dna::complement(78), 78); // N → N
/// assert_eq!(dna::complement(89), 82); // Y → R
/// assert_eq!(dna::complement(115), 115); // s → s
/// ```
pub fn complement(a: u8) -> u8 {
    COMPLEMENT[a as usize]
}

pub fn revcomp(text: &[u8]) -> Vec<u8>{
    text.into_iter()
        .rev()
        .map(|a| complement(*a.borrow()))
        .collect()
}

fn main() {
    let r1 = b"TCGGCATTGGCGGAATCCCGAATGACCATCGTCCAGTTGTTCGTCGGCGGGTCTCCCGCAGCCGCCGGCAGCGTCGGATGGTAGCTGACAGACAGGGTTT";
    let q1 = b"IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII";
    let r2 = b"ACGGTCTCGGTGGAGTATTTCGGCAACCTCGGCAAGTCGGAAACCCTGTCGGTCAGCTACCATCCGACGCTGCCGGCGGCCGCGGGAGACCCGCCGCCAA";
    let r2 = &revcomp(r2);
    dbg!(from_utf8(r2));
    let q2 = b"IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII";
    // #![feature(cstr_from_bytes_until_nul)]
    let fastq1 = str2c_char(r1); //unsafe{CStr::from_ptr(r1).unwrap().as_ptr() as *mut c_char};
    let fastq2 = str2c_char(r2);
    let qual1 = str2c_char(q1);
    let qual2 = str2c_char(q2);
    // let fastq1 = fastq1.as_mut();
    // let res: *const c_char =
    //     unsafe { bindings::align_read_pair(al, fastq1.as_ptr(), fastq2.as_ptr()) };
    let mut x: f32 = 1.0;
    let best_pos: *mut f32 = &mut x;
    unsafe {
        let pos = ngmerge_sys::findPos(
            fastq1,
            fastq2, 
            qual1, 
            qual2, 
            79, 79, 20, false, 50, 0.5, false, best_pos,
        );
        println!("{}", pos);
        // pub fn saveQual(
        //     qualFile: *mut ::std::os::raw::c_char,
        //     maxQual: ::std::os::raw::c_int,
        //     match_: *mut *mut *mut ::std::os::raw::c_char,
        //     mism: *mut *mut *mut ::std::os::raw::c_char,
        // );
        // let qualFile = std::ptr::null();
        // let match_ = Vec<Vec<c_char>> ;// as  *mut *mut *mut c_char;
        // saveQual()

        // pub fn run_rust(
        //     maxQual: ::std::os::raw::c_int,
        //     seq1: *mut ::std::os::raw::c_char,
        //     seq2: *mut ::std::os::raw::c_char,
        //     qual1: *mut ::std::os::raw::c_char,
        //     qual2: *mut ::std::os::raw::c_char,
        //     len1: ::std::os::raw::c_int,
        //     len2: ::std::os::raw::c_int,
        //     overlap: ::std::os::raw::c_int,
        //     dovetail: bool,
        //     doveOverlap: ::std::os::raw::c_int,
        //     mismatch: f32,
        //     maxLen: bool,
        //     best: *mut f32,
        //     offset: ::std::os::raw::c_int,
        //     fjoin: bool,
        // );
        run_rust(60, fastq1, fastq2, qual1, qual2, 100, 100, 20, false, 20, 0.1, false, best_pos, 33, false);
        // let aa = CStr::from_ptr(fastq1).to_str().unwrap();
        // dbg!(aa);
        
    }
}

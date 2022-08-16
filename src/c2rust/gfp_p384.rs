#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
extern crate std;



use std::arch::asm;
extern "C" {
fn LIMBS_shl_mod(r: *mut Limb, a: *const Limb, m: *const Limb, num_limbs: size_t);
fn LIMBS_sub_mod(
r: *mut Limb,
a: *const Limb,
b: *const Limb,
m: *const Limb,
num_limbs: size_t,
);
fn LIMBS_add_mod(
r: *mut Limb,
a: *const Limb,
b: *const Limb,
m: *const Limb,
num_limbs: size_t,
);
fn LIMBS_equal(a: *const Limb, b: *const Limb, num_limbs: size_t) -> Limb;
fn LIMBS_are_zero(a: *const Limb, num_limbs: size_t) -> Limb;
fn __assert_fail(
__assertion: *const std::os::raw::c_char,
__file: *const std::os::raw::c_char,
__line: std::os::raw::c_uint,
__function: *const std::os::raw::c_char,
) -> !;
fn GFp_bn_mul_mont(
rp: *mut BN_ULONG,
ap: *const BN_ULONG,
bp: *const BN_ULONG,
np: *const BN_ULONG,
n0: *const BN_ULONG,
num: size_t,
);
fn gfp_little_endian_bytes_from_scalar(
str: *mut uint8_t,
str_len: size_t,
scalar: *const Limb,
num_limbs: size_t,
);
}
pub type size_t = u64;
pub type __uint8_t = std::os::raw::c_uchar;
pub type __uint32_t = std::os::raw::c_uint;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type crypto_word = uint32_t;
pub type Limb = crypto_word;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct P384_POINT {
pub X: [Limb; 12],
pub Y: [Limb; 12],
pub Z: [Limb; 12],
}
pub type BN_ULONG = crypto_word;
pub type Carry = Limb;
pub type DoubleLimb = uint64_t;
pub type Elem = [Limb; 12];
#[inline]
unsafe extern "C" fn value_barrier_w(mut a: crypto_word) -> crypto_word {
core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
return a;
}
#[inline]
unsafe extern "C" fn constant_time_msb_w(mut a: crypto_word) -> crypto_word {
return (0 as std::os::raw::c_uint)
.wrapping_sub(
a
>> (std::mem::size_of::<crypto_word>() as u64)
.wrapping_mul(8 as std::os::raw::c_int as u64)
.wrapping_sub(1 as std::os::raw::c_int as u64),
);
}
#[inline]
unsafe extern "C" fn constant_time_is_zero_w(mut a: crypto_word) -> crypto_word {
return constant_time_msb_w(!a & a.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint));
}
#[inline]
unsafe extern "C" fn constant_time_is_nonzero_w(mut a: crypto_word) -> crypto_word {
return !constant_time_is_zero_w(a);
}
#[inline]
unsafe extern "C" fn constant_time_eq_w(
mut a: crypto_word,
mut b: crypto_word,
) -> crypto_word {
return constant_time_is_zero_w(a ^ b);
}
#[inline]
unsafe extern "C" fn constant_time_select_w(
mut mask: crypto_word,
mut a: crypto_word,
mut b: crypto_word,
) -> crypto_word {
return value_barrier_w(mask) & a | value_barrier_w(!mask) & b;
}
#[inline]
unsafe extern "C" fn limb_adc(
mut r: *mut Limb,
mut a: Limb,
mut b: Limb,
mut carry_in: Carry,
) -> Carry {
let mut ret: Carry = 0;
let mut x: DoubleLimb = (a as DoubleLimb)
.wrapping_add(b as u64)
.wrapping_add(carry_in as u64);
*r = x as Limb;
ret = (x >> 32 as std::os::raw::c_uint) as Carry;
return ret;
}
#[inline]
unsafe extern "C" fn limb_add(mut r: *mut Limb, mut a: Limb, mut b: Limb) -> Carry {
let mut ret: Carry = 0;
let mut x: DoubleLimb = (a as DoubleLimb).wrapping_add(b as u64);
*r = x as Limb;
ret = (x >> 32 as std::os::raw::c_uint) as Carry;
return ret;
}
#[inline]
unsafe extern "C" fn limb_sbb(
mut r: *mut Limb,
mut a: Limb,
mut b: Limb,
mut borrow_in: Carry,
) -> Carry {
let mut ret: Carry = 0;
let mut x: DoubleLimb = (a as DoubleLimb)
.wrapping_sub(b as u64)
.wrapping_sub(borrow_in as u64);
*r = x as Limb;
ret = (x >> 32 as std::os::raw::c_uint & 1 as std::os::raw::c_int as u64) as Carry;
return ret;
}
#[inline]
unsafe extern "C" fn limb_sub(mut r: *mut Limb, mut a: Limb, mut b: Limb) -> Carry {
let mut ret: Carry = 0;
let mut x: DoubleLimb = (a as DoubleLimb).wrapping_sub(b as u64);
*r = x as Limb;
ret = (x >> 32 as std::os::raw::c_uint & 1 as std::os::raw::c_int as u64) as Carry;
return ret;
}
#[inline]
unsafe extern "C" fn limbs_add(
mut r: *mut Limb,
mut a: *const Limb,
mut b: *const Limb,
mut num_limbs: size_t,
) -> Carry {
if num_limbs >= 1 as std::os::raw::c_int as u64 {} else {
__assert_fail(
b"num_limbs >= 1\0" as *const u8 as *const std::os::raw::c_char,
b"crypto/fipsmodule/ec/../../limbs/limbs.inl\0" as *const u8
as *const std::os::raw::c_char,
118 as std::os::raw::c_int as std::os::raw::c_uint,
(*std::mem::transmute::<
&[u8; 60],
&[std::os::raw::c_char; 60],
>(b"Carry limbs_add(Limb *, const Limb *, const Limb *, size_t)\0"))
.as_ptr(),
);
}
let mut carry: Carry = limb_add(
&mut *r.offset(0 as std::os::raw::c_int as isize),
*a.offset(0 as std::os::raw::c_int as isize),
*b.offset(0 as std::os::raw::c_int as isize),
);
let mut i: size_t = 1 as std::os::raw::c_int as size_t;
while i < num_limbs {
carry = limb_adc(
&mut *r.offset(i as isize),
*a.offset(i as isize),
*b.offset(i as isize),
carry,
);
i = i.wrapping_add(1);
}
return carry;
}
#[inline]
unsafe extern "C" fn limbs_sub(
mut r: *mut Limb,
mut a: *const Limb,
mut b: *const Limb,
mut num_limbs: size_t,
) -> Carry {
if num_limbs >= 1 as std::os::raw::c_int as u64 {} else {
__assert_fail(
b"num_limbs >= 1\0" as *const u8 as *const std::os::raw::c_char,
b"crypto/fipsmodule/ec/../../limbs/limbs.inl\0" as *const u8
as *const std::os::raw::c_char,
129 as std::os::raw::c_int as std::os::raw::c_uint,
(*std::mem::transmute::<
&[u8; 60],
&[std::os::raw::c_char; 60],
>(b"Carry limbs_sub(Limb *, const Limb *, const Limb *, size_t)\0"))
.as_ptr(),
);
}
let mut borrow: Carry = limb_sub(
&mut *r.offset(0 as std::os::raw::c_int as isize),
*a.offset(0 as std::os::raw::c_int as isize),
*b.offset(0 as std::os::raw::c_int as isize),
);
let mut i: size_t = 1 as std::os::raw::c_int as size_t;
while i < num_limbs {
borrow = limb_sbb(
&mut *r.offset(i as isize),
*a.offset(i as isize),
*b.offset(i as isize),
borrow,
);
i = i.wrapping_add(1);
}
return borrow;
}
#[inline]
unsafe extern "C" fn limbs_copy(
mut r: *mut Limb,
mut a: *const Limb,
mut num_limbs: size_t,
) {
let mut i: size_t = 0 as std::os::raw::c_int as size_t;
while i < num_limbs {
*r.offset(i as isize) = *a.offset(i as isize);
i = i.wrapping_add(1);
}
}
#[inline]
unsafe extern "C" fn limbs_zero(mut r: *mut Limb, mut num_limbs: size_t) {
let mut i: size_t = 0 as std::os::raw::c_int as size_t;
while i < num_limbs {
*r.offset(i as isize) = 0 as std::os::raw::c_int as Limb;
i = i.wrapping_add(1);
}
}
static mut Q: [BN_ULONG; 12] = [
0xffffffff as std::os::raw::c_uint,
0 as std::os::raw::c_int as BN_ULONG,
0 as std::os::raw::c_int as BN_ULONG,
0xffffffff as std::os::raw::c_uint,
0xfffffffe as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
];
static mut N: [BN_ULONG; 12] = [
0xccc52973 as std::os::raw::c_uint,
0xecec196a as std::os::raw::c_uint,
0x48b0a77a as std::os::raw::c_int as BN_ULONG,
0x581a0db2 as std::os::raw::c_int as BN_ULONG,
0xf4372ddf as std::os::raw::c_uint,
0xc7634d81 as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
];
static mut ONE: [BN_ULONG; 12] = [
1 as std::os::raw::c_int as BN_ULONG,
0xffffffff as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
0 as std::os::raw::c_int as BN_ULONG,
1 as std::os::raw::c_int as BN_ULONG,
0 as std::os::raw::c_int as BN_ULONG,
0 as std::os::raw::c_int as BN_ULONG,
0 as std::os::raw::c_int as BN_ULONG,
0 as std::os::raw::c_int as BN_ULONG,
0 as std::os::raw::c_int as BN_ULONG,
0 as std::os::raw::c_int as BN_ULONG,
0 as std::os::raw::c_int as BN_ULONG,
];
#[inline]
unsafe extern "C" fn is_equal(mut a: *const Limb, mut b: *const Limb) -> Limb {
return LIMBS_equal(
a,
b,
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
}
#[inline]
unsafe extern "C" fn is_zero(mut a: *const BN_ULONG) -> Limb {
return LIMBS_are_zero(
a,
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
}
#[inline]
unsafe extern "C" fn copy_conditional(
mut r: *mut Limb,
mut a: *const Limb,
condition: Limb,
) {
let mut i: size_t = 0 as std::os::raw::c_int as size_t;
while i < (384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as u64 {
*r
.offset(
i as isize,
) = constant_time_select_w(
condition,
*a.offset(i as isize),
*r.offset(i as isize),
);
i = i.wrapping_add(1);
}
}
#[inline]
unsafe extern "C" fn elem_add(mut r: *mut Limb, mut a: *const Limb, mut b: *const Limb) {
LIMBS_add_mod(
r,
a,
b,
Q.as_ptr(),
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
}
#[inline]
unsafe extern "C" fn elem_sub(mut r: *mut Limb, mut a: *const Limb, mut b: *const Limb) {
LIMBS_sub_mod(
r,
a,
b,
Q.as_ptr(),
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
}
unsafe extern "C" fn elem_div_by_2(mut r: *mut Limb, mut a: *const Limb) {
let mut is_odd: Limb = constant_time_is_nonzero_w(
*a.offset(0 as std::os::raw::c_int as isize) & 1 as std::os::raw::c_int as std::os::raw::c_uint,
);
let mut carry: Limb = *a
.offset(
(384 as std::os::raw::c_uint)
.wrapping_div(32 as std::os::raw::c_uint)
.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
) & 1 as std::os::raw::c_int as std::os::raw::c_uint;
*r
.offset(
(384 as std::os::raw::c_uint)
.wrapping_div(32 as std::os::raw::c_uint)
.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
) = *a
.offset(
(384 as std::os::raw::c_uint)
.wrapping_div(32 as std::os::raw::c_uint)
.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
) >> 1 as std::os::raw::c_int;
let mut i: size_t = 1 as std::os::raw::c_int as size_t;
while i < (384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as u64 {
let mut new_carry: Limb = *a
.offset(
((384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as u64)
.wrapping_sub(i)
.wrapping_sub(1 as std::os::raw::c_int as u64) as isize,
);
*r
.offset(
((384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as u64)
.wrapping_sub(i)
.wrapping_sub(1 as std::os::raw::c_int as u64) as isize,
) = *a
.offset(
((384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as u64)
.wrapping_sub(i)
.wrapping_sub(1 as std::os::raw::c_int as u64) as isize,
) >> 1 as std::os::raw::c_int
| carry
<< (32 as std::os::raw::c_uint).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint);
carry = new_carry;
i = i.wrapping_add(1);
}
static mut Q_PLUS_1_SHR_1: Elem = [
0x80000000 as std::os::raw::c_uint,
0 as std::os::raw::c_int as Limb,
0x80000000 as std::os::raw::c_uint,
0x7fffffff as std::os::raw::c_int as Limb,
0xffffffff as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
0xffffffff as std::os::raw::c_uint,
0x7fffffff as std::os::raw::c_int as Limb,
];
let mut adjusted: Elem = [0; 12];
let mut carry2: BN_ULONG = limbs_add(
adjusted.as_mut_ptr(),
r as *const Limb,
Q_PLUS_1_SHR_1.as_ptr(),
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
copy_conditional(r, adjusted.as_mut_ptr() as *const Limb, is_odd);
}
#[inline]
unsafe extern "C" fn elem_mul_mont(
mut r: *mut Limb,
mut a: *const Limb,
mut b: *const Limb,
) {
static mut Q_N0: [BN_ULONG; 2] = [
0x1 as std::os::raw::c_int as BN_ULONG,
0x1 as std::os::raw::c_int as BN_ULONG,
];
GFp_bn_mul_mont(
r,
a,
b,
Q.as_ptr(),
Q_N0.as_ptr(),
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
}
#[inline]
unsafe extern "C" fn elem_mul_by_2(mut r: *mut Limb, mut a: *const Limb) {
LIMBS_shl_mod(
r,
a,
Q.as_ptr(),
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
}
#[inline]
unsafe extern "C" fn elem_mul_by_3(mut r: *mut Limb, mut a: *const Limb) {
let mut doubled: Elem = [0; 12];
elem_add(doubled.as_mut_ptr(), a, a);
elem_add(r, doubled.as_mut_ptr() as *const Limb, a);
}
#[inline]
unsafe extern "C" fn elem_sqr_mont(mut r: *mut Limb, mut a: *const Limb) {
elem_mul_mont(r, a, a);
}
#[no_mangle]
pub unsafe extern "C" fn GFp_p384_elem_add(
mut r: *mut Limb,
mut a: *const Limb,
mut b: *const Limb,
) {
elem_add(r, a, b);
}
#[no_mangle]
pub unsafe extern "C" fn GFp_p384_elem_sub(
mut r: *mut Limb,
mut a: *const Limb,
mut b: *const Limb,
) {
elem_sub(r, a, b);
}
#[no_mangle]
pub unsafe extern "C" fn GFp_p384_elem_div_by_2(mut r: *mut Limb, mut a: *const Limb) {
elem_div_by_2(r, a);
}
#[no_mangle]
pub unsafe extern "C" fn GFp_p384_elem_mul_mont(
mut r: *mut Limb,
mut a: *const Limb,
mut b: *const Limb,
) {
elem_mul_mont(r, a, b);
}
#[no_mangle]
pub unsafe extern "C" fn GFp_p384_elem_neg(mut r: *mut Limb, mut a: *const Limb) {
let mut is_zero_0: Limb = LIMBS_are_zero(
a,
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
let mut borrow: Carry = limbs_sub(
r,
Q.as_ptr(),
a,
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
let mut i: size_t = 0 as std::os::raw::c_int as size_t;
while i < (384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as u64 {
*r
.offset(
i as isize,
) = constant_time_select_w(
is_zero_0,
0 as std::os::raw::c_int as crypto_word,
*r.offset(i as isize),
);
i = i.wrapping_add(1);
}
}
#[no_mangle]
pub unsafe extern "C" fn GFp_p384_scalar_mul_mont(
mut r: *mut Limb,
mut a: *const Limb,
mut b: *const Limb,
) {
static mut N_N0: [BN_ULONG; 2] = [
0xe88fdc45 as std::os::raw::c_uint,
0x6ed46089 as std::os::raw::c_int as BN_ULONG,
];
GFp_bn_mul_mont(
r,
a,
b,
N.as_ptr(),
N_N0.as_ptr(),
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
}
unsafe extern "C" fn gfp_p384_point_select_w5(
mut out: *mut P384_POINT,
mut table: *const P384_POINT,
mut index: size_t,
) {
let mut x: Elem = [0; 12];
limbs_zero(
x.as_mut_ptr(),
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
let mut y: Elem = [0; 12];
limbs_zero(
y.as_mut_ptr(),
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
let mut z: Elem = [0; 12];
limbs_zero(
z.as_mut_ptr(),
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
let mut i: size_t = 0 as std::os::raw::c_int as size_t;
while i < 16 as std::os::raw::c_int as u64 {
let mut equal: crypto_word = constant_time_eq_w(
index as crypto_word,
(i as crypto_word).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint),
);
let mut j: size_t = 0 as std::os::raw::c_int as size_t;
while j < (384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as u64
{
x[j
as usize] = constant_time_select_w(
equal,
(*table.offset(i as isize)).X[j as usize],
x[j as usize],
);
y[j
as usize] = constant_time_select_w(
equal,
(*table.offset(i as isize)).Y[j as usize],
y[j as usize],
);
z[j
as usize] = constant_time_select_w(
equal,
(*table.offset(i as isize)).Z[j as usize],
z[j as usize],
);
j = j.wrapping_add(1);
}
i = i.wrapping_add(1);
}
limbs_copy(
((*out).X).as_mut_ptr(),
x.as_mut_ptr() as *const Limb,
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
limbs_copy(
((*out).Y).as_mut_ptr(),
y.as_mut_ptr() as *const Limb,
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
limbs_copy(
((*out).Z).as_mut_ptr(),
z.as_mut_ptr() as *const Limb,
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
}
#[inline]
unsafe extern "C" fn booth_recode(
mut is_negative: *mut crypto_word,
mut digit: *mut crypto_word,
mut in_0: crypto_word,
mut w: crypto_word,
) {
if w >= 2 as std::os::raw::c_int as std::os::raw::c_uint {} else {
__assert_fail(
b"w >= 2\0" as *const u8 as *const std::os::raw::c_char,
b"crypto/fipsmodule/ec/ecp_nistz.h\0" as *const u8 as *const std::os::raw::c_char,
251 as std::os::raw::c_int as std::os::raw::c_uint,
(*std::mem::transmute::<
&[u8; 74],
&[std::os::raw::c_char; 74],
>(
b"void booth_recode(crypto_word *, crypto_word *, crypto_word, crypto_word)\0",
))
.as_ptr(),
);
}
if w <= 7 as std::os::raw::c_int as std::os::raw::c_uint {} else {
__assert_fail(
b"w <= 7\0" as *const u8 as *const std::os::raw::c_char,
b"crypto/fipsmodule/ec/ecp_nistz.h\0" as *const u8 as *const std::os::raw::c_char,
252 as std::os::raw::c_int as std::os::raw::c_uint,
(*std::mem::transmute::<
&[u8; 74],
&[std::os::raw::c_char; 74],
>(
b"void booth_recode(crypto_word *, crypto_word *, crypto_word, crypto_word)\0",
))
.as_ptr(),
);
}
let mut s: crypto_word = !(in_0 >> w).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint);
let mut d: crypto_word = 0;
d = ((1 as std::os::raw::c_uint) << w.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint))
.wrapping_sub(in_0)
.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint);
d = d & s | in_0 & !s;
d = (d >> 1 as std::os::raw::c_int).wrapping_add(d & 1 as std::os::raw::c_int as std::os::raw::c_uint);
*is_negative = constant_time_is_nonzero_w(s & 1 as std::os::raw::c_int as std::os::raw::c_uint);
*digit = d;
}
#[no_mangle]
pub unsafe extern "C" fn GFp_nistz384_point_double(
mut r: *mut P384_POINT,
mut a: *const P384_POINT,
) {
let mut S: [BN_ULONG; 12] = [0; 12];
let mut M: [BN_ULONG; 12] = [0; 12];
let mut Zsqr: [BN_ULONG; 12] = [0; 12];
let mut tmp0: [BN_ULONG; 12] = [0; 12];
let mut in_x: *const BN_ULONG = ((*a).X).as_ptr();
let mut in_y: *const BN_ULONG = ((*a).Y).as_ptr();
let mut in_z: *const BN_ULONG = ((*a).Z).as_ptr();
let mut res_x: *mut BN_ULONG = ((*r).X).as_mut_ptr();
let mut res_y: *mut BN_ULONG = ((*r).Y).as_mut_ptr();
let mut res_z: *mut BN_ULONG = ((*r).Z).as_mut_ptr();
elem_mul_by_2(S.as_mut_ptr(), in_y);
elem_sqr_mont(Zsqr.as_mut_ptr(), in_z);
elem_sqr_mont(S.as_mut_ptr(), S.as_mut_ptr() as *const Limb);
elem_mul_mont(res_z, in_z, in_y);
elem_mul_by_2(res_z, res_z as *const Limb);
elem_add(M.as_mut_ptr(), in_x, Zsqr.as_mut_ptr() as *const Limb);
elem_sub(Zsqr.as_mut_ptr(), in_x, Zsqr.as_mut_ptr() as *const Limb);
elem_sqr_mont(res_y, S.as_mut_ptr() as *const Limb);
elem_div_by_2(res_y, res_y as *const Limb);
elem_mul_mont(
M.as_mut_ptr(),
M.as_mut_ptr() as *const Limb,
Zsqr.as_mut_ptr() as *const Limb,
);
elem_mul_by_3(M.as_mut_ptr(), M.as_mut_ptr() as *const Limb);
elem_mul_mont(S.as_mut_ptr(), S.as_mut_ptr() as *const Limb, in_x);
elem_mul_by_2(tmp0.as_mut_ptr(), S.as_mut_ptr() as *const Limb);
elem_sqr_mont(res_x, M.as_mut_ptr() as *const Limb);
elem_sub(res_x, res_x as *const Limb, tmp0.as_mut_ptr() as *const Limb);
elem_sub(S.as_mut_ptr(), S.as_mut_ptr() as *const Limb, res_x as *const Limb);
elem_mul_mont(
S.as_mut_ptr(),
S.as_mut_ptr() as *const Limb,
M.as_mut_ptr() as *const Limb,
);
elem_sub(res_y, S.as_mut_ptr() as *const Limb, res_y as *const Limb);
}
#[no_mangle]
pub unsafe extern "C" fn GFp_nistz384_point_add(
mut r: *mut P384_POINT,
mut a: *const P384_POINT,
mut b: *const P384_POINT,
) {
let mut U2: [BN_ULONG; 12] = [0; 12];
let mut S2: [BN_ULONG; 12] = [0; 12];
let mut U1: [BN_ULONG; 12] = [0; 12];
let mut S1: [BN_ULONG; 12] = [0; 12];
let mut Z1sqr: [BN_ULONG; 12] = [0; 12];
let mut Z2sqr: [BN_ULONG; 12] = [0; 12];
let mut H: [BN_ULONG; 12] = [0; 12];
let mut R: [BN_ULONG; 12] = [0; 12];
let mut Hsqr: [BN_ULONG; 12] = [0; 12];
let mut Rsqr: [BN_ULONG; 12] = [0; 12];
let mut Hcub: [BN_ULONG; 12] = [0; 12];
let mut res_x: [BN_ULONG; 12] = [0; 12];
let mut res_y: [BN_ULONG; 12] = [0; 12];
let mut res_z: [BN_ULONG; 12] = [0; 12];
let mut in1_x: *const BN_ULONG = ((*a).X).as_ptr();
let mut in1_y: *const BN_ULONG = ((*a).Y).as_ptr();
let mut in1_z: *const BN_ULONG = ((*a).Z).as_ptr();
let mut in2_x: *const BN_ULONG = ((*b).X).as_ptr();
let mut in2_y: *const BN_ULONG = ((*b).Y).as_ptr();
let mut in2_z: *const BN_ULONG = ((*b).Z).as_ptr();
let mut in1infty: BN_ULONG = is_zero(((*a).Z).as_ptr());
let mut in2infty: BN_ULONG = is_zero(((*b).Z).as_ptr());
elem_sqr_mont(Z2sqr.as_mut_ptr(), in2_z);
elem_sqr_mont(Z1sqr.as_mut_ptr(), in1_z);
elem_mul_mont(S1.as_mut_ptr(), Z2sqr.as_mut_ptr() as *const Limb, in2_z);
elem_mul_mont(S2.as_mut_ptr(), Z1sqr.as_mut_ptr() as *const Limb, in1_z);
elem_mul_mont(S1.as_mut_ptr(), S1.as_mut_ptr() as *const Limb, in1_y);
elem_mul_mont(S2.as_mut_ptr(), S2.as_mut_ptr() as *const Limb, in2_y);
elem_sub(
R.as_mut_ptr(),
S2.as_mut_ptr() as *const Limb,
S1.as_mut_ptr() as *const Limb,
);
elem_mul_mont(U1.as_mut_ptr(), in1_x, Z2sqr.as_mut_ptr() as *const Limb);
elem_mul_mont(U2.as_mut_ptr(), in2_x, Z1sqr.as_mut_ptr() as *const Limb);
elem_sub(
H.as_mut_ptr(),
U2.as_mut_ptr() as *const Limb,
U1.as_mut_ptr() as *const Limb,
);
let mut is_exceptional: BN_ULONG = is_equal(
U1.as_mut_ptr() as *const Limb,
U2.as_mut_ptr() as *const Limb,
) & !in1infty & !in2infty;
if is_exceptional != 0 {
if is_equal(S1.as_mut_ptr() as *const Limb, S2.as_mut_ptr() as *const Limb) != 0
{
GFp_nistz384_point_double(r, a);
} else {
limbs_zero(
((*r).X).as_mut_ptr(),
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
limbs_zero(
((*r).Y).as_mut_ptr(),
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
limbs_zero(
((*r).Z).as_mut_ptr(),
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
}
return;
}
elem_sqr_mont(Rsqr.as_mut_ptr(), R.as_mut_ptr() as *const Limb);
elem_mul_mont(res_z.as_mut_ptr(), H.as_mut_ptr() as *const Limb, in1_z);
elem_sqr_mont(Hsqr.as_mut_ptr(), H.as_mut_ptr() as *const Limb);
elem_mul_mont(res_z.as_mut_ptr(), res_z.as_mut_ptr() as *const Limb, in2_z);
elem_mul_mont(
Hcub.as_mut_ptr(),
Hsqr.as_mut_ptr() as *const Limb,
H.as_mut_ptr() as *const Limb,
);
elem_mul_mont(
U2.as_mut_ptr(),
U1.as_mut_ptr() as *const Limb,
Hsqr.as_mut_ptr() as *const Limb,
);
elem_mul_by_2(Hsqr.as_mut_ptr(), U2.as_mut_ptr() as *const Limb);
elem_sub(
res_x.as_mut_ptr(),
Rsqr.as_mut_ptr() as *const Limb,
Hsqr.as_mut_ptr() as *const Limb,
);
elem_sub(
res_x.as_mut_ptr(),
res_x.as_mut_ptr() as *const Limb,
Hcub.as_mut_ptr() as *const Limb,
);
elem_sub(
res_y.as_mut_ptr(),
U2.as_mut_ptr() as *const Limb,
res_x.as_mut_ptr() as *const Limb,
);
elem_mul_mont(
S2.as_mut_ptr(),
S1.as_mut_ptr() as *const Limb,
Hcub.as_mut_ptr() as *const Limb,
);
elem_mul_mont(
res_y.as_mut_ptr(),
R.as_mut_ptr() as *const Limb,
res_y.as_mut_ptr() as *const Limb,
);
elem_sub(
res_y.as_mut_ptr(),
res_y.as_mut_ptr() as *const Limb,
S2.as_mut_ptr() as *const Limb,
);
copy_conditional(res_x.as_mut_ptr(), in2_x, in1infty);
copy_conditional(res_y.as_mut_ptr(), in2_y, in1infty);
copy_conditional(res_z.as_mut_ptr(), in2_z, in1infty);
copy_conditional(res_x.as_mut_ptr(), in1_x, in2infty);
copy_conditional(res_y.as_mut_ptr(), in1_y, in2infty);
copy_conditional(res_z.as_mut_ptr(), in1_z, in2infty);
limbs_copy(
((*r).X).as_mut_ptr(),
res_x.as_mut_ptr() as *const Limb,
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
limbs_copy(
((*r).Y).as_mut_ptr(),
res_y.as_mut_ptr() as *const Limb,
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
limbs_copy(
((*r).Z).as_mut_ptr(),
res_z.as_mut_ptr() as *const Limb,
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
}
unsafe extern "C" fn add_precomputed_w5(
mut r: *mut P384_POINT,
mut wvalue: crypto_word,
mut table: *const P384_POINT,
) {
let mut recoded_is_negative: crypto_word = 0;
let mut recoded: crypto_word = 0;
booth_recode(
&mut recoded_is_negative,
&mut recoded,
wvalue,
5 as std::os::raw::c_int as crypto_word,
);
let mut h: P384_POINT = P384_POINT {
X: [0; 12],
Y: [0; 12],
Z: [0; 12],
};
gfp_p384_point_select_w5(&mut h, table, recoded as size_t);
let mut tmp: [BN_ULONG; 12] = [0; 12];
GFp_p384_elem_neg(tmp.as_mut_ptr(), (h.Y).as_mut_ptr() as *const Limb);
copy_conditional(
(h.Y).as_mut_ptr(),
tmp.as_mut_ptr() as *const Limb,
recoded_is_negative,
);
GFp_nistz384_point_add(r, r, &mut h);
}
#[no_mangle]
pub unsafe extern "C" fn GFp_nistz384_point_mul(
mut r: *mut P384_POINT,
mut p_scalar: *const BN_ULONG,
mut p_x: *const BN_ULONG,
mut p_y: *const BN_ULONG,
) {
static mut kWindowSize: size_t = 5 as std::os::raw::c_int as size_t;
static mut kMask: crypto_word = (((1 as std::os::raw::c_int)
<< 5 as std::os::raw::c_int + 1 as std::os::raw::c_int) - 1 as std::os::raw::c_int) as crypto_word;
let mut p_str: [uint8_t; 49] = [0; 49];
gfp_little_endian_bytes_from_scalar(
p_str.as_mut_ptr(),
(std::mem::size_of::<[uint8_t; 49]>() as u64)
.wrapping_div(std::mem::size_of::<uint8_t>() as u64),
p_scalar,
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
let mut table: [P384_POINT; 16] = [P384_POINT {
X: [0; 12],
Y: [0; 12],
Z: [0; 12],
}; 16];
let mut row: *mut P384_POINT = table.as_mut_ptr();
limbs_copy(
((*row.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize)).X).as_mut_ptr(),
p_x,
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
limbs_copy(
((*row.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize)).Y).as_mut_ptr(),
p_y,
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
limbs_copy(
((*row.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize)).Z).as_mut_ptr(),
ONE.as_ptr(),
(384 as std::os::raw::c_uint).wrapping_div(32 as std::os::raw::c_uint) as size_t,
);
GFp_nistz384_point_double(
&mut *row.offset((2 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
);
GFp_nistz384_point_add(
&mut *row.offset((3 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((2 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
);
GFp_nistz384_point_double(
&mut *row.offset((4 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((2 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
);
GFp_nistz384_point_double(
&mut *row.offset((6 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((3 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
);
GFp_nistz384_point_double(
&mut *row.offset((8 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((4 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
);
GFp_nistz384_point_double(
&mut *row.offset((12 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((6 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
);
GFp_nistz384_point_add(
&mut *row.offset((5 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((4 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
);
GFp_nistz384_point_add(
&mut *row.offset((7 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((6 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
);
GFp_nistz384_point_add(
&mut *row.offset((9 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((8 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
);
GFp_nistz384_point_add(
&mut *row.offset((13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((12 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
);
GFp_nistz384_point_double(
&mut *row.offset((14 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((7 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
);
GFp_nistz384_point_double(
&mut *row.offset((10 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((5 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
);
GFp_nistz384_point_add(
&mut *row.offset((15 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((14 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
);
GFp_nistz384_point_add(
&mut *row.offset((11 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((10 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
);
GFp_nistz384_point_double(
&mut *row.offset((16 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
&mut *row.offset((8 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize),
);
static mut START_INDEX: size_t = (384 as std::os::raw::c_int - 4 as std::os::raw::c_int) as size_t;
let mut index: size_t = START_INDEX;
let mut recoded_is_negative: BN_ULONG = 0;
let mut recoded: crypto_word = 0;
let mut wvalue: crypto_word = p_str[index
.wrapping_sub(1 as std::os::raw::c_int as u64)
.wrapping_div(8 as std::os::raw::c_int as u64) as usize] as crypto_word;
wvalue = wvalue
>> index
.wrapping_sub(1 as std::os::raw::c_int as u64)
.wrapping_rem(8 as std::os::raw::c_int as u64) & kMask;
booth_recode(
&mut recoded_is_negative,
&mut recoded,
wvalue,
5 as std::os::raw::c_int as crypto_word,
);
gfp_p384_point_select_w5(
r,
table.as_mut_ptr() as *const P384_POINT,
recoded as size_t,
);
while index >= kWindowSize {
if index != START_INDEX {
let mut off: size_t = index
.wrapping_sub(1 as std::os::raw::c_int as u64)
.wrapping_div(8 as std::os::raw::c_int as u64);
wvalue = (p_str[off as usize] as std::os::raw::c_int
| (p_str[off.wrapping_add(1 as std::os::raw::c_int as u64) as usize]
as std::os::raw::c_int) << 8 as std::os::raw::c_int) as crypto_word;
wvalue = wvalue
>> index
.wrapping_sub(1 as std::os::raw::c_int as u64)
.wrapping_rem(8 as std::os::raw::c_int as u64) & kMask;
add_precomputed_w5(r, wvalue, table.as_mut_ptr() as *const P384_POINT);
}
index = (index as u64).wrapping_sub(kWindowSize) as size_t as size_t;
GFp_nistz384_point_double(r, r);
GFp_nistz384_point_double(r, r);
GFp_nistz384_point_double(r, r);
GFp_nistz384_point_double(r, r);
GFp_nistz384_point_double(r, r);
}
wvalue = p_str[0 as std::os::raw::c_int as usize] as crypto_word;
wvalue = wvalue << 1 as std::os::raw::c_int & kMask;
add_precomputed_w5(r, wvalue, table.as_mut_ptr() as *const P384_POINT);
}

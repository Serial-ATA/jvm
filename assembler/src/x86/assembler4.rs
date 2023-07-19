use crate::Address;
use super::register::{GPRegister, XMMRegister};

pub struct Assembler {}

impl Assembler {
	pub fn adcl_m_imm(&mut self, dst: Address, imm32: i32) {
	InstructionMark im(this);
	prefix(dst);
	emit_arith_operand(0x81, rdx, dst, imm32);
	}

	pub fn adcl_m_reg(&mut self, dst: Address, src: GPRegister) {
	InstructionMark im(this);
	prefix(dst, src);
	emit_int8(0x11);
	emit_operand(src, dst, 0);
	}

	pub fn adcl_r_imm(&mut self, dst: GPRegister, imm32: i32) {
	prefix(dst);
	emit_arith(0x81, 0xD0, dst, imm32);
	}

	pub fn adcl_r_m(&mut self, dst: GPRegister, src: Address) {
	InstructionMark im(this);
	prefix(src, dst);
	emit_int8(0x13);
	emit_operand(dst, src, 0);
	}

	pub fn adcl_reg_r(&mut self,dst: GPRegister, src: GPRegister) {
	(void) prefix_and_encode(dst->encoding(), src->encoding());
	emit_arith(0x13, 0xC0, dst, src);
	}

	pub fn addl(&mut self,dst: Address, imm32: i32) {
	InstructionMark im(this);
	prefix(dst);
	emit_arith_operand(0x81, rax, dst, imm32);
	}

	pub fn addb(&mut self,dst: Address, imm8: i8) {
	InstructionMark im(this);
	prefix(dst);
	emit_int8((unsigned char)0x80);
	emit_operand(rax, dst, 1);
	emit_int8(imm8);
	}

	pub fn addw(&mut self,dst: GPRegister, src: GPRegister) {
	emit_int8(0x66);
	(void)prefix_and_encode(dst->encoding(), src->encoding());
	emit_arith(0x03, 0xC0, dst, src);
	}

	pub fn addw(&mut self,dst: Address, imm16: i16) {
	InstructionMark im(this);
	emit_int8(0x66);
	prefix(dst);
	emit_int8((unsigned char)0x81);
	emit_operand(rax, dst, 2);
	emit_int16(imm16);
	}

	pub fn addl(&mut self,dst: Address, src: GPRegister) {
	InstructionMark im(this);
	prefix(dst, src);
	emit_int8(0x01);
	emit_operand(src, dst, 0);
	}

	pub fn addl(&mut self,dst: GPRegister, imm32: i32) {
	prefix(dst);
	emit_arith(0x81, 0xC0, dst, imm32);
	}

	pub fn addl(&mut self,dst: GPRegister, src: Address) {
	InstructionMark im(this);
	prefix(src, dst);
	emit_int8(0x03);
	emit_operand(dst, src, 0);
	}

	pub fn addl(&mut self,dst: GPRegister, src: GPRegister) {
	(void) prefix_and_encode(dst->encoding(), src->encoding());
	emit_arith(0x03, 0xC0, dst, src);
	}

	pub fn addr_nop_4(&mut self,) {
	assert(UseAddressNop, "no CPU support");
	// 4 bytes: NOP DWORD PTR [EAX+0]
	emit_int32(0x0F,
	0x1F,
	0x40, // emit_rm(cbuf, 0x1, EAX_enc, EAX_enc);
	0);   // 8-bits offset (1 byte)
	}

	pub fn addr_nop_5(&mut self,) {
	assert(UseAddressNop, "no CPU support");
	// 5 bytes: NOP DWORD PTR [EAX+EAX*0+0] 8-bits offset
	emit_int32(0x0F,
	0x1F,
	0x44,  // emit_rm(cbuf, 0x1, EAX_enc, 0x4);
	0x00); // emit_rm(cbuf, 0x0, EAX_enc, EAX_enc);
	emit_int8(0);     // 8-bits offset (1 byte)
	}

	pub fn addr_nop_7(&mut self,) {
	assert(UseAddressNop, "no CPU support");
	// 7 bytes: NOP DWORD PTR [EAX+0] 32-bits offset
	emit_int24(0x0F,
	0x1F,
	(unsigned char)0x80);
	// emit_rm(cbuf, 0x2, EAX_enc, EAX_enc);
	emit_int32(0);   // 32-bits offset (4 bytes)
	}

	pub fn addr_nop_8(&mut self,) {
	assert(UseAddressNop, "no CPU support");
	// 8 bytes: NOP DWORD PTR [EAX+EAX*0+0] 32-bits offset
	emit_int32(0x0F,
	0x1F,
	(unsigned char)0x84,
	// emit_rm(cbuf, 0x2, EAX_enc, 0x4);
	0x00); // emit_rm(cbuf, 0x0, EAX_enc, EAX_enc);
	emit_int32(0);    // 32-bits offset (4 bytes)
	}

	pub fn addsd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_rex_vex_w_reverted();
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int16(0x58, (0xC0 | encode));
	}

	pub fn addsd(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
	attributes.set_rex_vex_w_reverted();
	simd_prefix(dst, dst, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int8(0x58);
	emit_operand(dst, src, 0);
	}

	pub fn addss(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16(0x58, (0xC0 | encode));
	}

	pub fn addss(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(dst, dst, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int8(0x58);
	emit_operand(dst, src, 0);
	}

	pub fn aesdec(&mut self,XMMdst: GPRegister, src: Address) {
	assert(VM_Version::supports_aes(), "");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	simd_prefix(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8((unsigned char)0xDE);
	emit_operand(dst, src, 0);
	}

	pub fn aesdec(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_aes(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16((unsigned char)0xDE, (0xC0 | encode));
	}

	pub fn vaesdec(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx512_vaes(), "");
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16((unsigned char)0xDE, (0xC0 | encode));
	}


	pub fn aesdeclast(&mut self,XMMdst: GPRegister, src: Address) {
	assert(VM_Version::supports_aes(), "");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	simd_prefix(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8((unsigned char)0xDF);
	emit_operand(dst, src, 0);
	}

	pub fn aesdeclast(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_aes(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16((unsigned char)0xDF, (0xC0 | encode));
	}

	pub fn vaesdeclast(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx512_vaes(), "");
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16((unsigned char)0xDF, (0xC0 | encode));
	}

	pub fn aesenc(&mut self,XMMdst: GPRegister, src: Address) {
	assert(VM_Version::supports_aes(), "");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	simd_prefix(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8((unsigned char)0xDC);
	emit_operand(dst, src, 0);
	}

	pub fn aesenc(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_aes(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16((unsigned char)0xDC, 0xC0 | encode);
	}

	pub fn vaesenc(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx512_vaes(), "requires vaes support/enabling");
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16((unsigned char)0xDC, (0xC0 | encode));
	}

	pub fn aesenclast(&mut self,XMMdst: GPRegister, src: Address) {
	assert(VM_Version::supports_aes(), "");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	simd_prefix(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8((unsigned char)0xDD);
	emit_operand(dst, src, 0);
	}

	pub fn aesenclast(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_aes(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16((unsigned char)0xDD, (0xC0 | encode));
	}

	pub fn vaesenclast(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx512_vaes(), "requires vaes support/enabling");
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16((unsigned char)0xDD, (0xC0 | encode));
	}

	pub fn andb(&mut self,dst: Address, src: GPRegister) {
	InstructionMark im(this);
	prefix(dst, src, true);
	emit_int8(0x20);
	emit_operand(src, dst, 0);
	}

	pub fn andw(&mut self,dst: GPRegister, src: GPRegister) {
	(void)prefix_and_encode(dst->encoding(), src->encoding());
	emit_arith(0x23, 0xC0, dst, src);
	}

	pub fn andl(&mut self,dst: Address, imm32: i32) {
	InstructionMark im(this);
	prefix(dst);
	emit_arith_operand(0x81, as_Register(4), dst, imm32);
	}

	pub fn andl(&mut self,dst: GPRegister, imm32: i32) {
	prefix(dst);
	emit_arith(0x81, 0xE0, dst, imm32);
	}

	pub fn andl(&mut self,dst: Address, src: GPRegister) {
	InstructionMark im(this);
	prefix(dst, src);
	emit_int8(0x21);
	emit_operand(src, dst, 0);
	}

	pub fn andl(&mut self,dst: GPRegister, src: Address) {
	InstructionMark im(this);
	prefix(src, dst);
	emit_int8(0x23);
	emit_operand(dst, src, 0);
	}

	pub fn andl(&mut self,dst: GPRegister, src: GPRegister) {
	(void) prefix_and_encode(dst->encoding(), src->encoding());
	emit_arith(0x23, 0xC0, dst, src);
	}

	pub fn andnl(&mut self,dst: GPRegister, src: GPRegister1, src: GPRegister2) {
	assert(VM_Version::supports_bmi1(), "bit manipulation instructions not supported");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, &attributes);
	emit_int16((unsigned char)0xF2, (0xC0 | encode));
	}

	pub fn andnl(&mut self,dst: GPRegister, src: GPRegister1, src: Address2) {
	assert(VM_Version::supports_bmi1(), "bit manipulation instructions not supported");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	vex_prefix(src2, src1->encoding(), dst->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, &attributes);
	emit_int8((unsigned char)0xF2);
	emit_operand(dst, src2, 0);
	}

	pub fn bsfl(&mut self,dst: GPRegister, src: GPRegister) {
	int encode = prefix_and_encode(dst->encoding(), src->encoding());
	emit_int24(0x0F,
	(unsigned char)0xBC,
	0xC0 | encode);
	}

	pub fn bsrl(&mut self,dst: GPRegister, src: GPRegister) {
	int encode = prefix_and_encode(dst->encoding(), src->encoding());
	emit_int24(0x0F,
	(unsigned char)0xBD,
	0xC0 | encode);
	}

	pub fn bswapl(&mut self,Register reg) { // bswap
	int encode = prefix_and_encode(reg->encoding());
	emit_int16(0x0F, (0xC8 | encode));
	}

	pub fn blsil(&mut self,dst: GPRegister, src: GPRegister) {
	assert(VM_Version::supports_bmi1(), "bit manipulation instructions not supported");
	InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(rbx->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, &attributes);
	emit_int16((unsigned char)0xF3, (0xC0 | encode));
	}

	pub fn blsil(&mut self,dst: GPRegister, src: Address) {
	assert(VM_Version::supports_bmi1(), "bit manipulation instructions not supported");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	vex_prefix(src, dst->encoding(), rbx->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, &attributes);
	emit_int8((unsigned char)0xF3);
	emit_operand(rbx, src, 0);
	}

	pub fn blsmskl(&mut self,dst: GPRegister, src: GPRegister) {
	assert(VM_Version::supports_bmi1(), "bit manipulation instructions not supported");
	InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(rdx->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, &attributes);
	emit_int16((unsigned char)0xF3,
	0xC0 | encode);
	}

	pub fn blsmskl(&mut self,dst: GPRegister, src: Address) {
	assert(VM_Version::supports_bmi1(), "bit manipulation instructions not supported");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	vex_prefix(src, dst->encoding(), rdx->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, &attributes);
	emit_int8((unsigned char)0xF3);
	emit_operand(rdx, src, 0);
	}

	pub fn blsrl(&mut self,dst: GPRegister, src: GPRegister) {
	assert(VM_Version::supports_bmi1(), "bit manipulation instructions not supported");
	InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(rcx->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, &attributes);
	emit_int16((unsigned char)0xF3, (0xC0 | encode));
	}

	pub fn blsrl(&mut self,dst: GPRegister, src: Address) {
	assert(VM_Version::supports_bmi1(), "bit manipulation instructions not supported");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	vex_prefix(src, dst->encoding(), rcx->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, &attributes);
	emit_int8((unsigned char)0xF3);
	emit_operand(rcx, src, 0);
	}

	pub fn call(&mut self,Label& L, relocInfo::relocType rtype) {
	// suspect disp32 is always good
	int operand = LP64_ONLY(disp32_operand) NOT_LP64(imm_operand);

	if (L.is_bound()) {
	const int long_size = 5;
	int offs = (int)( target(L) - pc() );
	assert(offs <= 0, "assembler error");
	InstructionMark im(this);
	// 1110 1000 #32-bit disp
	emit_int8((unsigned char)0xE8);
	emit_data(offs - long_size, rtype, operand);
	} else {
	InstructionMark im(this);
	// 1110 1000 #32-bit disp
	L.add_patch_at(code(), locator());

	emit_int8((unsigned char)0xE8);
	emit_data(int(0), rtype, operand);
	}
	}

	pub fn call(&mut self,dst: GPRegister) {
	int encode = prefix_and_encode(dst->encoding());
	emit_int16((unsigned char)0xFF, (0xD0 | encode));
	}


	pub fn call(&mut self,Address adr) {
	InstructionMark im(this);
	prefix(adr);
	emit_int8((unsigned char)0xFF);
	emit_operand(rdx, adr, 0);
	}

	pub fn call_literal(&mut self,address entry, RelocationHolder const& rspec) {
	InstructionMark im(this);
	emit_int8((unsigned char)0xE8);
	intptr_t disp = entry - (pc() + sizeof(i32));
	// Entry is null in case of a scratch emit.
	assert(entry == nullptr || is_simm32(disp), "disp=" INTPTR_FORMAT " must be 32bit offset (call2)", disp);
	// Technically, should use call32_operand, but this format is
	// implied by the fact that we're emitting a call instruction.

	int operand = LP64_ONLY(disp32_operand) NOT_LP64(call32_operand);
	emit_data((int) disp, rspec, operand);
	}

	pub fn cdql(&mut self,) {
	emit_int8((unsigned char)0x99);
	}

	pub fn cld(&mut self,) {
	emit_int8((unsigned char)0xFC);
	}

	pub fn cmovl(&mut self,Condition cc, dst: GPRegister, src: GPRegister) {
	NOT_LP64(guarantee(VM_Version::supports_cmov(), "illegal instruction"));
	int encode = prefix_and_encode(dst->encoding(), src->encoding());
	emit_int24(0x0F,
	0x40 | cc,
	0xC0 | encode);
	}


	pub fn cmovl(&mut self,Condition cc, dst: GPRegister, src: Address) {
	InstructionMark im(this);
	NOT_LP64(guarantee(VM_Version::supports_cmov(), "illegal instruction"));
	prefix(src, dst);
	emit_int16(0x0F, (0x40 | cc));
	emit_operand(dst, src, 0);
	}

	pub fn cmpb(&mut self,dst: Address, imm8: i8) {
	InstructionMark im(this);
	prefix(dst);
	emit_int8((unsigned char)0x80);
	emit_operand(rdi, dst, 1);
	emit_int8(imm8);
	}

	pub fn cmpl(&mut self,dst: Address, imm32: i32) {
	InstructionMark im(this);
	prefix(dst);
	emit_arith_operand(0x81, as_Register(7), dst, imm32);
	}

	pub fn cmpl(&mut self,dst: GPRegister, imm32: i32) {
	prefix(dst);
	emit_arith(0x81, 0xF8, dst, imm32);
	}

	pub fn cmpl(&mut self,dst: GPRegister, src: GPRegister) {
	(void) prefix_and_encode(dst->encoding(), src->encoding());
	emit_arith(0x3B, 0xC0, dst, src);
	}

	pub fn cmpl(&mut self,dst: GPRegister, src: Address) {
	InstructionMark im(this);
	prefix(src, dst);
	emit_int8(0x3B);
	emit_operand(dst, src, 0);
	}

	pub fn cmpl_imm32(&mut self,dst: Address, imm32: i32) {
	InstructionMark im(this);
	prefix(dst);
	emit_arith_operand_imm32(0x81, as_Register(7), dst, imm32);
	}

	pub fn cmpw(&mut self,dst: Address, imm16: i16) {
	InstructionMark im(this);
	emit_int8(0x66);
	prefix(dst);
	emit_int8((unsigned char)0x81);
	emit_operand(rdi, dst, 2);
	emit_int16(imm16);
	}

// The 32-bit cmpxchg compares the value at adr with the contents of rax,
// and stores reg into adr if so; otherwise, the value at adr is loaded into rax,.
// The ZF is set if the compared values were equal, and cleared otherwise.
	pub fn cmpxchgl(&mut self,Register reg, Address adr) { // cmpxchg
	InstructionMark im(this);
	prefix(adr, reg);
	emit_int16(0x0F, (unsigned char)0xB1);
	emit_operand(reg, adr, 0);
	}

	pub fn cmpxchgw(&mut self,Register reg, Address adr) { // cmpxchg
	InstructionMark im(this);
	size_prefix();
	prefix(adr, reg);
	emit_int16(0x0F, (unsigned char)0xB1);
	emit_operand(reg, adr, 0);
	}

// The 8-bit cmpxchg compares the value at adr with the contents of rax,
// and stores reg into adr if so; otherwise, the value at adr is loaded into rax,.
// The ZF is set if the compared values were equal, and cleared otherwise.
	pub fn cmpxchgb(&mut self,Register reg, Address adr) { // cmpxchg
	InstructionMark im(this);
	prefix(adr, reg, true);
	emit_int16(0x0F, (unsigned char)0xB0);
	emit_operand(reg, adr, 0);
	}

	pub fn comisd(&mut self,XMMdst: GPRegister, src: Address) {
	// NOTE: dbx seems to decode this as comiss even though the
	// 0x66 is there. Strangely ucomisd comes out correct
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);;
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
	attributes.set_rex_vex_w_reverted();
	simd_prefix(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int8(0x2F);
	emit_operand(dst, src, 0);
	}

	pub fn comisd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_rex_vex_w_reverted();
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x2F, (0xC0 | encode));
	}

	pub fn comiss(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(dst, xnoreg, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int8(0x2F);
	emit_operand(dst, src, 0);
	}

	pub fn comiss(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16(0x2F, (0xC0 | encode));
	}

	pub fn cpuid(&mut self,) {
	emit_int16(0x0F, (unsigned char)0xA2);
	}

// Opcode / Instruction                      Op /  En  64 - Bit Mode     Compat / Leg Mode Description                  Implemented
// F2 0F 38 F0 / r       CRC32 r32, r / m8   RM        Valid             Valid             Accumulate CRC32 on r / m8.  v
// F2 REX 0F 38 F0 / r   CRC32 r32, r / m8*  RM        Valid             N.E.              Accumulate CRC32 on r / m8.  -
// F2 REX.W 0F 38 F0 / r CRC32 r64, r / m8   RM        Valid             N.E.              Accumulate CRC32 on r / m8.  -
//
// F2 0F 38 F1 / r       CRC32 r32, r / m16  RM        Valid             Valid             Accumulate CRC32 on r / m16. v
//
// F2 0F 38 F1 / r       CRC32 r32, r / m32  RM        Valid             Valid             Accumulate CRC32 on r / m32. v
//
// F2 REX.W 0F 38 F1 / r CRC32 r64, r / m64  RM        Valid             N.E.              Accumulate CRC32 on r / m64. v
	pub fn crc32(&mut self,Register crc, Register v, int8_t sizeInBytes) {
	assert(VM_Version::supports_sse4_2(), "");
	int8_t w = 0x01;
	Prefix p = Prefix_EMPTY;

	emit_int8((unsigned char)0xF2);
	switch (sizeInBytes) {
	case 1:
	w = 0;
	break;
	case 2:
	case 4:
	break;
	LP64_ONLY(case 8:)
	// This instruction is not valid in 32 bits
	// Note:
	// http://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-instruction-set-reference-manual-325383.pdf
	//
	// Page B - 72   Vol. 2C says
	// qwreg2 to qwreg            1111 0010 : 0100 1R0B : 0000 1111 : 0011 1000 : 1111 0000 : 11 qwreg1 qwreg2
	// mem64 to qwreg             1111 0010 : 0100 1R0B : 0000 1111 : 0011 1000 : 1111 0000 : mod qwreg r / m
	//                                                                            F0!!!
	// while 3 - 208 Vol. 2A
	// F2 REX.W 0F 38 F1 / r       CRC32 r64, r / m64             RM         Valid      N.E.Accumulate CRC32 on r / m64.
	//
	// the 0 on a last bit is reserved for a different flavor of this instruction :
	// F2 REX.W 0F 38 F0 / r       CRC32 r64, r / m8              RM         Valid      N.E.Accumulate CRC32 on r / m8.
	p = REX_W;
	break;
	default:
	assert(0, "Unsupported value for a sizeInBytes argument");
	break;
	}
	LP64_ONLY(prefix(crc, v, p);)
	emit_int32(0x0F,
	0x38,
	0xF0 | w,
	0xC0 | ((crc->encoding() & 0x7) << 3) | (v->encoding() & 7));
	}

	pub fn crc32(&mut self,Register crc, Address adr, int8_t sizeInBytes) {
	assert(VM_Version::supports_sse4_2(), "");
	InstructionMark im(this);
	int8_t w = 0x01;
	Prefix p = Prefix_EMPTY;

	emit_int8((int8_t)0xF2);
	switch (sizeInBytes) {
	case 1:
	w = 0;
	break;
	case 2:
	case 4:
	break;
	LP64_ONLY(case 8:)
	// This instruction is not valid in 32 bits
	p = REX_W;
	break;
	default:
	assert(0, "Unsupported value for a sizeInBytes argument");
	break;
	}
	LP64_ONLY(prefix(crc, adr, p);)
	emit_int24(0x0F, 0x38, (0xF0 | w));
	emit_operand(crc, adr, 0);
	}

	pub fn cvtdq2pd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0xE6, (0xC0 | encode));
	}

	pub fn vcvtdq2pd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len <= AVX_256bit ? VM_Version::supports_avx() : VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0xE6, (0xC0 | encode));
	}

	pub fn vcvtps2ph(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, imm8: i8, int vector_len) {
	assert(VM_Version::supports_evex() || VM_Version::supports_f16c(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /*uses_vl */ true);
	int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x1D, (0xC0 | encode), imm8);
	}

	pub fn evcvtps2ph(&mut self,dst: Address, KRegister mask, XMMsrc: GPRegister, imm8: i8, int vector_len) {
	assert(VM_Version::supports_evex(), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /*uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_HVM, /* input_size_in_bits */ EVEX_64bit);
	attributes.reset_is_clear_context();
	attributes.set_embedded_opmask_register_specifier(mask);
	attributes.set_is_evex_instruction();
	vex_prefix(dst, 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int8(0x1D);
	emit_operand(src, dst, 1);
	emit_int8(imm8);
	}

	pub fn vcvtps2ph(&mut self,dst: Address, XMMsrc: GPRegister, imm8: i8, int vector_len) {
	assert(VM_Version::supports_evex() || VM_Version::supports_f16c(), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /*uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_HVM, /* input_size_in_bits */ EVEX_NObit);
	vex_prefix(dst, 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int8(0x1D);
	emit_operand(src, dst, 1);
	emit_int8(imm8);
	}

	pub fn vcvtph2ps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_evex() || VM_Version::supports_f16c(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x13, (0xC0 | encode));
	}

	pub fn vcvtph2ps(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
	assert(VM_Version::supports_evex() || VM_Version::supports_f16c(), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /*uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_HVM, /* input_size_in_bits */ EVEX_NObit);
	vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8(0x13);
	emit_operand(dst, src, 0);
	}

	pub fn cvtdq2ps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16(0x5B, (0xC0 | encode));
	}

	pub fn vcvtdq2ps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len <= AVX_256bit ? VM_Version::supports_avx() : VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16(0x5B, (0xC0 | encode));
	}

	pub fn cvtsd2ss(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_rex_vex_w_reverted();
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int16(0x5A, (0xC0 | encode));
	}

	pub fn cvtsd2ss(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
	attributes.set_rex_vex_w_reverted();
	simd_prefix(dst, dst, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int8(0x5A);
	emit_operand(dst, src, 0);
	}

	pub fn cvtsi2sdl(&mut self,XMMdst: GPRegister, src: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, as_XMMRegister(src->encoding()), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int16(0x2A, (0xC0 | encode));
	}

	pub fn cvtsi2sdl(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(dst, dst, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int8(0x2A);
	emit_operand(dst, src, 0);
	}

	pub fn cvtsi2ssl(&mut self,XMMdst: GPRegister, src: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, as_XMMRegister(src->encoding()), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16(0x2A, (0xC0 | encode));
	}

	pub fn cvtsi2ssl(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(dst, dst, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int8(0x2A);
	emit_operand(dst, src, 0);
	}

	pub fn cvtsi2ssq(&mut self,XMMdst: GPRegister, src: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, as_XMMRegister(src->encoding()), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16(0x2A, (0xC0 | encode));
	}

	pub fn cvtss2sd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16(0x5A, (0xC0 | encode));
	}

	pub fn cvtss2sd(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(dst, dst, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int8(0x5A);
	emit_operand(dst, src, 0);
	}


	pub fn cvttsd2sil(&mut self,dst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(as_XMMRegister(dst->encoding()), xnoreg, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int16(0x2C, (0xC0 | encode));
	}

	pub fn cvtss2sil(&mut self,dst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(as_XMMRegister(dst->encoding()), xnoreg, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16(0x2D, (0xC0 | encode));
	}

	pub fn cvttss2sil(&mut self,dst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(as_XMMRegister(dst->encoding()), xnoreg, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16(0x2C, (0xC0 | encode));
	}

	pub fn cvttpd2dq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	int vector_len = VM_Version::supports_avx512novl() ? AVX_512bit : AVX_128bit;
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0xE6, (0xC0 | encode));
	}

	pub fn pabsb(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_ssse3(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x1C, (0xC0 | encode));
	}

	pub fn pabsw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_ssse3(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x1D, (0xC0 | encode));
	}

	pub fn pabsd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_ssse3(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x1E, (0xC0 | encode));
	}

	pub fn vpabsb(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_128bit ? VM_Version::supports_avx()      :
	vector_len == AVX_256bit ? VM_Version::supports_avx2()     :
	vector_len == AVX_512bit ? VM_Version::supports_avx512bw() : false, "not supported");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x1C, (0xC0 | encode));
	}

	pub fn vpabsw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_128bit ? VM_Version::supports_avx()      :
	vector_len == AVX_256bit ? VM_Version::supports_avx2()     :
	vector_len == AVX_512bit ? VM_Version::supports_avx512bw() : false, "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x1D, (0xC0 | encode));
	}

	pub fn vpabsd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_128bit? VM_Version::supports_avx() :
	vector_len == AVX_256bit? VM_Version::supports_avx2() :
	vector_len == AVX_512bit? VM_Version::supports_evex() : 0, "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x1E, (0xC0 | encode));
	}

	pub fn evpabsq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(UseAVX > 2, "");
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x1F, (0xC0 | encode));
	}

	pub fn vcvtps2pd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len <= AVX_256bit ? VM_Version::supports_avx() : VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16(0x5A, (0xC0 | encode));
	}

	pub fn vcvtpd2ps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len <= AVX_256bit ? VM_Version::supports_avx() : VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	attributes.set_rex_vex_w_reverted();
	emit_int16(0x5A, (0xC0 | encode));
	}

	pub fn vcvttps2dq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len <= AVX_256bit ? VM_Version::supports_avx() : VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16(0x5B, (0xC0 | encode));
	}

	pub fn vcvttpd2dq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len <= AVX_256bit ? VM_Version::supports_avx() : VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0xE6, (0xC0 | encode));
	}

	pub fn vcvtps2dq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len <= AVX_256bit ? VM_Version::supports_avx() : VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x5B, (0xC0 | encode));
	}

	pub fn evcvttps2qq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx512dq(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x7A, (0xC0 | encode));
	}

	pub fn evcvtpd2qq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx512dq(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x7B, (0xC0 | encode));
	}

	pub fn evcvtqq2ps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx512dq(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16(0x5B, (0xC0 | encode));
	}

	pub fn evcvttpd2qq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx512dq(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x7A, (0xC0 | encode));
	}

	pub fn evcvtqq2pd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx512dq(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0xE6, (0xC0 | encode));
	}

	pub fn evpmovwb(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x30, (0xC0 | encode));
	}

	pub fn evpmovdw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(UseAVX > 2, "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x33, (0xC0 | encode));
	}

	pub fn evpmovdb(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(UseAVX > 2, "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x31, (0xC0 | encode));
	}

	pub fn evpmovqd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(UseAVX > 2, "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x35, (0xC0 | encode));
	}

	pub fn evpmovqb(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(UseAVX > 2, "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x32, (0xC0 | encode));
	}

	pub fn evpmovqw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(UseAVX > 2, "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x34, (0xC0 | encode));
	}

	pub fn evpmovsqd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(UseAVX > 2, "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x25, (0xC0 | encode));
	}

	pub fn decl(&mut self,dst: Address) {
	// Don't use it directly. Use MacroAssembler::decrement() instead.
	InstructionMark im(this);
	prefix(dst);
	emit_int8((unsigned char)0xFF);
	emit_operand(rcx, dst, 0);
	}

	pub fn divsd(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
	attributes.set_rex_vex_w_reverted();
	simd_prefix(dst, dst, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int8(0x5E);
	emit_operand(dst, src, 0);
	}

	pub fn divsd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_rex_vex_w_reverted();
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int16(0x5E, (0xC0 | encode));
	}

	pub fn divss(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(dst, dst, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int8(0x5E);
	emit_operand(dst, src, 0);
	}

	pub fn divss(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16(0x5E, (0xC0 | encode));
	}

	pub fn hlt(&mut self,) {
	emit_int8((unsigned char)0xF4);
	}

	pub fn idivl(&mut self,src: GPRegister) {
	int encode = prefix_and_encode(src->encoding());
	emit_int16((unsigned char)0xF7, (0xF8 | encode));
	}

	pub fn divl(&mut self,src: GPRegister) { // Unsigned
	int encode = prefix_and_encode(src->encoding());
	emit_int16((unsigned char)0xF7, (0xF0 | encode));
	}

	pub fn imull(&mut self,src: GPRegister) {
	int encode = prefix_and_encode(src->encoding());
	emit_int16((unsigned char)0xF7, (0xE8 | encode));
	}

	pub fn imull(&mut self,dst: GPRegister, src: GPRegister) {
	int encode = prefix_and_encode(dst->encoding(), src->encoding());
	emit_int24(0x0F,
	(unsigned char)0xAF,
	(0xC0 | encode));
	}

	pub fn imull(&mut self,dst: GPRegister, src: Address, i32 value) {
	InstructionMark im(this);
	prefix(src, dst);
	if (is8bit(value)) {
	emit_int8((unsigned char)0x6B);
	emit_operand(dst, src, 1);
	emit_int8(value);
	} else {
	emit_int8((unsigned char)0x69);
	emit_operand(dst, src, 4);
	emit_int32(value);
	}
	}

	pub fn imull(&mut self,dst: GPRegister, src: GPRegister, int value) {
	int encode = prefix_and_encode(dst->encoding(), src->encoding());
	if (is8bit(value)) {
	emit_int24(0x6B, (0xC0 | encode), value & 0xFF);
	} else {
	emit_int16(0x69, (0xC0 | encode));
	emit_int32(value);
	}
	}

	pub fn imull(&mut self,dst: GPRegister, src: Address) {
	InstructionMark im(this);
	prefix(src, dst);
	emit_int16(0x0F, (unsigned char)0xAF);
	emit_operand(dst, src, 0);
	}


	pub fn incl(&mut self,dst: Address) {
	// Don't use it directly. Use MacroAssembler::increment() instead.
	InstructionMark im(this);
	prefix(dst);
	emit_int8((unsigned char)0xFF);
	emit_operand(rax, dst, 0);
	}

	pub fn jcc(&mut self,Condition cc, Label& L, bool maybe_short) {
	InstructionMark im(this);
	assert((0 <= cc) && (cc < 16), "illegal cc");
	if (L.is_bound()) {
	dst: Address = target(L);
	assert(dst != nullptr, "jcc most probably wrong");

	const int short_size = 2;
	const int long_size = 6;
	intptr_t offs = (intptr_t)dst - (intptr_t)pc();
	if (maybe_short && is8bit(offs - short_size)) {
	// 0111 tttn #8-bit disp
	emit_int16(0x70 | cc, (offs - short_size) & 0xFF);
	} else {
	// 0000 1111 1000 tttn #32-bit disp
	assert(is_simm32(offs - long_size),
	"must be 32bit offset (call4)");
	emit_int16(0x0F, (0x80 | cc));
	emit_int32(offs - long_size);
	}
	} else {
	// Note: could eliminate cond. jumps to this jump if condition
	//       is the same however, seems to be rather unlikely case.
	// Note: use jccb() if label to be bound is very close to get
	//       an 8-bit displacement
	L.add_patch_at(code(), locator());
	emit_int16(0x0F, (0x80 | cc));
	emit_int32(0);
	}
	}

	pub fn jccb_0(&mut self,Condition cc, Label& L, const char* file, int line) {
	if (L.is_bound()) {
	const int short_size = 2;
	address entry = target(L);
	#ifdef ASSERT
	intptr_t dist = (intptr_t)entry - ((intptr_t)pc() + short_size);
	intptr_t delta = short_branch_delta();
	if (delta != 0) {
	dist += (dist < 0 ? (-delta) :delta);
	}
	assert(is8bit(dist), "Dispacement too large for a short jmp at %s:%d", file, line);
	#endif
	intptr_t offs = (intptr_t)entry - (intptr_t)pc();
	// 0111 tttn #8-bit disp
	emit_int16(0x70 | cc, (offs - short_size) & 0xFF);
	} else {
	InstructionMark im(this);
	L.add_patch_at(code(), locator(), file, line);
	emit_int16(0x70 | cc, 0);
	}
	}

	pub fn jmp(&mut self,Address adr) {
	InstructionMark im(this);
	prefix(adr);
	emit_int8((unsigned char)0xFF);
	emit_operand(rsp, adr, 0);
	}

	pub fn jmp(&mut self,Label& L, bool maybe_short) {
	if (L.is_bound()) {
	address entry = target(L);
	assert(entry != nullptr, "jmp most probably wrong");
	InstructionMark im(this);
	const int short_size = 2;
	const int long_size = 5;
	intptr_t offs = entry - pc();
	if (maybe_short && is8bit(offs - short_size)) {
	emit_int16((unsigned char)0xEB, ((offs - short_size) & 0xFF));
	} else {
	emit_int8((unsigned char)0xE9);
	emit_int32(offs - long_size);
	}
	} else {
	// By default, forward jumps are always 32-bit displacements, since
	// we can't yet know where the label will be bound.  If you're sure that
	// the forward jump will not run beyond 256 bytes, use jmpb to
	// force an 8-bit displacement.
	InstructionMark im(this);
	L.add_patch_at(code(), locator());
	emit_int8((unsigned char)0xE9);
	emit_int32(0);
	}
	}

	pub fn jmp(&mut self,Register entry) {
	int encode = prefix_and_encode(entry->encoding());
	emit_int16((unsigned char)0xFF, (0xE0 | encode));
	}

	pub fn jmp_literal(&mut self,address dest, RelocationHolder const& rspec) {
	InstructionMark im(this);
	emit_int8((unsigned char)0xE9);
	assert(dest != nullptr, "must have a target");
	intptr_t disp = dest - (pc() + sizeof(i32));
	assert(is_simm32(disp), "must be 32bit offset (jmp)");
	emit_data(disp, rspec, call32_operand);
	}

	pub fn jmpb_0(&mut self,Label& L, const char* file, int line) {
	if (L.is_bound()) {
	const int short_size = 2;
	address entry = target(L);
	assert(entry != nullptr, "jmp most probably wrong");
	#ifdef ASSERT
	intptr_t dist = (intptr_t)entry - ((intptr_t)pc() + short_size);
	intptr_t delta = short_branch_delta();
	if (delta != 0) {
	dist += (dist < 0 ? (-delta) :delta);
	}
	assert(is8bit(dist), "Dispacement too large for a short jmp at %s:%d", file, line);
	#endif
	intptr_t offs = entry - pc();
	emit_int16((unsigned char)0xEB, (offs - short_size) & 0xFF);
	} else {
	InstructionMark im(this);
	L.add_patch_at(code(), locator(), file, line);
	emit_int16((unsigned char)0xEB, 0);
	}
	}

	pub fn ldmxcsr(&mut self, src: Address) {
	if (UseAVX > 0 ) {
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	vex_prefix(src, 0, 0, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int8((unsigned char)0xAE);
	emit_operand(as_Register(2), src, 0);
	} else {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionMark im(this);
	prefix(src);
	emit_int16(0x0F, (unsigned char)0xAE);
	emit_operand(as_Register(2), src, 0);
	}
	}

	pub fn leal(&mut self,dst: GPRegister, src: Address) {
	InstructionMark im(this);
	prefix(src, dst);
	emit_int8((unsigned char)0x8D);
	emit_operand(dst, src, 0);
	}

	pub fn lfence(&mut self,) {
	emit_int24(0x0F, (unsigned char)0xAE, (unsigned char)0xE8);
	}

	pub fn lock(&mut self,) {
	emit_int8((unsigned char)0xF0);
	}

	pub fn size_prefix(&mut self,) {
	emit_int8(0x66);
	}

	pub fn lzcntl(&mut self,dst: GPRegister, src: GPRegister) {
	assert(VM_Version::supports_lzcnt(), "encoding is treated as BSR");
	emit_int8((unsigned char)0xF3);
	int encode = prefix_and_encode(dst->encoding(), src->encoding());
	emit_int24(0x0F, (unsigned char)0xBD, (0xC0 | encode));
	}

	pub fn lzcntl(&mut self,dst: GPRegister, src: Address) {
	assert(VM_Version::supports_lzcnt(), "encoding is treated as BSR");
	InstructionMark im(this);
	emit_int8((unsigned char)0xF3);
	prefix(src, dst);
	emit_int16(0x0F, (unsigned char)0xBD);
	emit_operand(dst, src, 0);
	}

// Emit mfence instruction
	pub fn mfence(&mut self,) {
	NOT_LP64(assert(VM_Version::supports_sse2(), "unsupported");)
	emit_int24(0x0F, (unsigned char)0xAE, (unsigned char)0xF0);
	}

// Emit sfence instruction
	pub fn sfence(&mut self,) {
	NOT_LP64(assert(VM_Version::supports_sse2(), "unsupported");)
	emit_int24(0x0F, (unsigned char)0xAE, (unsigned char)0xF8);
	}

	pub fn mov(&mut self,dst: GPRegister, src: GPRegister) {
	LP64_ONLY(movq(dst, src)) NOT_LP64(movl(dst, src));
	}

	pub fn movapd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	int vector_len = VM_Version::supports_avx512novl() ? AVX_512bit : AVX_128bit;
	InstructionAttr attributes(vector_len, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_rex_vex_w_reverted();
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x28, (0xC0 | encode));
	}

	pub fn movaps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	int vector_len = VM_Version::supports_avx512novl() ? AVX_512bit : AVX_128bit;
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16(0x28, (0xC0 | encode));
	}

	pub fn movlhps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, src, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16(0x16, (0xC0 | encode));
	}

	pub fn movb(&mut self,dst: GPRegister, src: Address) {
	NOT_LP64(assert(dst->has_byte_register(), "must have byte register"));
	InstructionMark im(this);
	prefix(src, dst, true);
	emit_int8((unsigned char)0x8A);
	emit_operand(dst, src, 0);
	}

	pub fn movddup(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse3(), ""));
	int vector_len = VM_Version::supports_avx512novl() ? AVX_512bit : AVX_128bit;
	InstructionAttr attributes(vector_len, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_rex_vex_w_reverted();
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int16(0x12, 0xC0 | encode);
	}

	pub fn movddup(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse3(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_DUP, /* input_size_in_bits */ EVEX_64bit);
	attributes.set_rex_vex_w_reverted();
	simd_prefix(dst, xnoreg, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int8(0x12);
	emit_operand(dst, src, 0);
	}

	pub fn vmovddup(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
	assert(VM_Version::supports_avx(), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_DUP, /* input_size_in_bits */ EVEX_64bit);
	attributes.set_rex_vex_w_reverted();
	simd_prefix(dst, xnoreg, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int8(0x12);
	emit_operand(dst, src, 0);
	}

	pub fn kmovbl(&mut self,Kdst: GPRegister, Ksrc: GPRegister) {
	assert(VM_Version::supports_avx512dq(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x90, (0xC0 | encode));
	}

	pub fn kmovbl(&mut self,Kdst: GPRegister, src: GPRegister) {
	assert(VM_Version::supports_avx512dq(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x92, (0xC0 | encode));
	}

	pub fn kmovbl(&mut self,dst: GPRegister, Ksrc: GPRegister) {
	assert(VM_Version::supports_avx512dq(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x93, (0xC0 | encode));
	}

	pub fn kmovwl(&mut self,Kdst: GPRegister, src: GPRegister) {
	assert(VM_Version::supports_evex(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x92, (0xC0 | encode));
	}

	pub fn kmovwl(&mut self,dst: GPRegister, Ksrc: GPRegister) {
	assert(VM_Version::supports_evex(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x93, (0xC0 | encode));
	}

	pub fn kmovwl(&mut self,Kdst: GPRegister, src: Address) {
	assert(VM_Version::supports_evex(), "");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	vex_prefix(src, 0, dst->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int8((unsigned char)0x90);
	emit_operand(dst, src, 0);
	}

	pub fn kmovwl(&mut self,dst: Address, Ksrc: GPRegister) {
	assert(VM_Version::supports_evex(), "");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	vex_prefix(dst, 0, src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int8((unsigned char)0x91);
	emit_operand(src, dst, 0);
	}

	pub fn kmovwl(&mut self,Kdst: GPRegister, Ksrc: GPRegister) {
	assert(VM_Version::supports_evex(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x90, (0xC0 | encode));
	}

	pub fn kmovdl(&mut self,Kdst: GPRegister, src: GPRegister) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x92, (0xC0 | encode));
	}

	pub fn kmovdl(&mut self,dst: GPRegister, Ksrc: GPRegister) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x93, (0xC0 | encode));
	}

	pub fn kmovql(&mut self,Kdst: GPRegister, Ksrc: GPRegister) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x90, (0xC0 | encode));
	}

	pub fn kmovql(&mut self,Kdst: GPRegister, src: Address) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	vex_prefix(src, 0, dst->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int8((unsigned char)0x90);
	emit_operand(dst, src, 0);
	}

	pub fn kmovql(&mut self,dst: Address, Ksrc: GPRegister) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	vex_prefix(dst, 0, src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int8((unsigned char)0x91);
	emit_operand(src, dst, 0);
	}

	pub fn kmovql(&mut self,Kdst: GPRegister, src: GPRegister) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x92, (0xC0 | encode));
	}

	pub fn kmovql(&mut self,dst: GPRegister, Ksrc: GPRegister) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x93, (0xC0 | encode));
	}

	pub fn knotwl(&mut self,Kdst: GPRegister, Ksrc: GPRegister) {
	assert(VM_Version::supports_evex(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16(0x44, (0xC0 | encode));
	}

	pub fn knotbl(&mut self,Kdst: GPRegister, Ksrc: GPRegister) {
	assert(VM_Version::supports_evex(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x44, (0xC0 | encode));
	}

	pub fn korbl(&mut self,Kdst: GPRegister, Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512dq(), "");
	InstructionAttr attributes(AVX_256bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x45, (0xC0 | encode));
	}

	pub fn korwl(&mut self,Kdst: GPRegister, Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_evex(), "");
	InstructionAttr attributes(AVX_256bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16(0x45, (0xC0 | encode));
	}

	pub fn kordl(&mut self,Kdst: GPRegister, Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_256bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x45, (0xC0 | encode));
	}

	pub fn korql(&mut self,Kdst: GPRegister, Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_256bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16(0x45, (0xC0 | encode));
	}

	pub fn kxorbl(&mut self,Kdst: GPRegister, Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512dq(), "");
	InstructionAttr attributes(AVX_256bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x47, (0xC0 | encode));
	}

	pub fn kxorwl(&mut self,Kdst: GPRegister, Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_evex(), "");
	InstructionAttr attributes(AVX_256bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16(0x47, (0xC0 | encode));
	}

	pub fn kxordl(&mut self,Kdst: GPRegister, Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_256bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x47, (0xC0 | encode));
	}

	pub fn kxorql(&mut self,Kdst: GPRegister, Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_256bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16(0x47, (0xC0 | encode));
	}

	pub fn kandbl(&mut self,Kdst: GPRegister, Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512dq(), "");
	InstructionAttr attributes(AVX_256bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x41, (0xC0 | encode));
	}

	pub fn kandwl(&mut self,Kdst: GPRegister, Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_evex(), "");
	InstructionAttr attributes(AVX_256bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16(0x41, (0xC0 | encode));
	}

	pub fn kanddl(&mut self,Kdst: GPRegister, Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_256bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x41, (0xC0 | encode));
	}

	pub fn kandql(&mut self,Kdst: GPRegister, Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_256bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16(0x41, (0xC0 | encode));
	}

	pub fn knotdl(&mut self,Kdst: GPRegister, Ksrc: GPRegister) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x44, (0xC0 | encode));
	}

	pub fn knotql(&mut self,Kdst: GPRegister, Ksrc: GPRegister) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16(0x44, (0xC0 | encode));
	}

// This instruction produces ZF or CF flags
	pub fn kortestbl(&mut self,Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512dq(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(src1->encoding(), 0, src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x98, (0xC0 | encode));
	}

// This instruction produces ZF or CF flags
	pub fn kortestwl(&mut self,Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_evex(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(src1->encoding(), 0, src2->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x98, (0xC0 | encode));
	}

// This instruction produces ZF or CF flags
	pub fn kortestdl(&mut self,Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(src1->encoding(), 0, src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x98, (0xC0 | encode));
	}

// This instruction produces ZF or CF flags
	pub fn kortestql(&mut self,Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(src1->encoding(), 0, src2->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x98, (0xC0 | encode));
	}

// This instruction produces ZF or CF flags
	pub fn ktestql(&mut self,Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(src1->encoding(), 0, src2->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x99, (0xC0 | encode));
	}

	pub fn ktestdl(&mut self,Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(src1->encoding(), 0, src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x99, (0xC0 | encode));
	}

	pub fn ktestwl(&mut self,Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512dq(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(src1->encoding(), 0, src2->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x99, (0xC0 | encode));
	}

	pub fn ktestbl(&mut self,Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512dq(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(src1->encoding(), 0, src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x99, (0xC0 | encode));
	}

	pub fn ktestq(&mut self,Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(src1->encoding(), 0, src2->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x99, (0xC0 | encode));
	}

	pub fn ktestd(&mut self,Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(src1->encoding(), 0, src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0x99, (0xC0 | encode));
	}

	pub fn kxnorbl(&mut self,Kdst: GPRegister, Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512dq(), "");
	InstructionAttr attributes(AVX_256bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x46, (0xC0 | encode));
	}

	pub fn kshiftlbl(&mut self,Kdst: GPRegister, Ksrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_avx512dq(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0 , src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int16(0x32, (0xC0 | encode));
	emit_int8(imm8);
	}

	pub fn kshiftlql(&mut self,Kdst: GPRegister, Ksrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0 , src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int16(0x33, (0xC0 | encode));
	emit_int8(imm8);
	}


	pub fn kshiftrbl(&mut self,Kdst: GPRegister, Ksrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_avx512dq(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0 , src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int16(0x30, (0xC0 | encode));
	}

	pub fn kshiftrwl(&mut self,Kdst: GPRegister, Ksrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_evex(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0 , src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int16(0x30, (0xC0 | encode));
	emit_int8(imm8);
	}

	pub fn kshiftrdl(&mut self,Kdst: GPRegister, Ksrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0 , src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int16(0x31, (0xC0 | encode));
	emit_int8(imm8);
	}

	pub fn kshiftrql(&mut self,Kdst: GPRegister, Ksrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0 , src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int16(0x31, (0xC0 | encode));
	emit_int8(imm8);
	}

	pub fn kunpckdql(&mut self,Kdst: GPRegister, Ksrc: GPRegister1, Ksrc: GPRegister2) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(AVX_256bit, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16(0x4B, (0xC0 | encode));
	}

	pub fn movb(&mut self,dst: Address, imm8: i8) {
	InstructionMark im(this);
	prefix(dst);
	emit_int8((unsigned char)0xC6);
	emit_operand(rax, dst, 1);
	emit_int8(imm8);
	}


	pub fn movb(&mut self,dst: Address, src: GPRegister) {
	assert(src->has_byte_register(), "must have byte register");
	InstructionMark im(this);
	prefix(dst, src, true);
	emit_int8((unsigned char)0x88);
	emit_operand(src, dst, 0);
	}

	pub fn movdl(&mut self,XMMdst: GPRegister, src: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, xnoreg, as_XMMRegister(src->encoding()), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x6E, (0xC0 | encode));
	}

	pub fn movdl(&mut self,dst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	// swap src/dst to get correct prefix
	int encode = simd_prefix_and_encode(src, xnoreg, as_XMMRegister(dst->encoding()), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x7E, (0xC0 | encode));
	}

	pub fn movdl(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int8(0x6E);
	emit_operand(dst, src, 0);
	}

	pub fn movdl(&mut self,dst: Address, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(src, xnoreg, dst, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int8(0x7E);
	emit_operand(src, dst, 0);
	}

	pub fn movdqa(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x6F, (0xC0 | encode));
	}

	pub fn movdqa(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	simd_prefix(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int8(0x6F);
	emit_operand(dst, src, 0);
	}

	pub fn movdqu(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	simd_prefix(dst, xnoreg, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int8(0x6F);
	emit_operand(dst, src, 0);
	}

	pub fn movdqu(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16(0x6F, (0xC0 | encode));
	}

	pub fn movdqu(&mut self,dst: Address, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.reset_is_clear_context();
	simd_prefix(src, xnoreg, dst, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int8(0x7F);
	emit_operand(src, dst, 0);
	}

// Move Unaligned 256bit Vector
	pub fn vmovdqu(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(UseAVX > 0, "");
	InstructionAttr attributes(AVX_256bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16(0x6F, (0xC0 | encode));
	}

	pub fn vmovdqu(&mut self,XMMdst: GPRegister, src: Address) {
	assert(UseAVX > 0, "");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_256bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	vex_prefix(src, 0, dst->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int8(0x6F);
	emit_operand(dst, src, 0);
	}

	pub fn vmovdqu(&mut self,dst: Address, XMMsrc: GPRegister) {
	assert(UseAVX > 0, "");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_256bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.reset_is_clear_context();
	// swap src<->dst for encoding
	assert(src != xnoreg, "sanity");
	vex_prefix(dst, 0, src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int8(0x7F);
	emit_operand(src, dst, 0);
	}

	pub fn vpmaskmovd(&mut self,XMMdst: GPRegister, XMMRegister mask, src: Address, int vector_len) {
	assert((VM_Version::supports_avx2() && vector_len == AVX_256bit), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ false, /* uses_vl */ false);
	vex_prefix(src, mask->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8((unsigned char)0x8C);
	emit_operand(dst, src, 0);
	}

	pub fn vpmaskmovq(&mut self,XMMdst: GPRegister, XMMRegister mask, src: Address, int vector_len) {
	assert((VM_Version::supports_avx2() && vector_len == AVX_256bit), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ false, /* uses_vl */ false);
	vex_prefix(src, mask->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8((unsigned char)0x8C);
	emit_operand(dst, src, 0);
	}

	pub fn vmaskmovps(&mut self,XMMdst: GPRegister, src: Address, XMMRegister mask, int vector_len) {
	assert(UseAVX > 0, "requires some form of AVX");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	vex_prefix(src, mask->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8(0x2C);
	emit_operand(dst, src, 0);
	}

	pub fn vmaskmovpd(&mut self,XMMdst: GPRegister, src: Address, XMMRegister mask, int vector_len) {
	assert(UseAVX > 0, "requires some form of AVX");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	vex_prefix(src, mask->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8(0x2D);
	emit_operand(dst, src, 0);
	}

	pub fn vmaskmovps(&mut self,dst: Address, XMMsrc: GPRegister, XMMRegister mask, int vector_len) {
	assert(UseAVX > 0, "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	vex_prefix(dst, mask->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8(0x2E);
	emit_operand(src, dst, 0);
	}

	pub fn vmaskmovpd(&mut self,dst: Address, XMMsrc: GPRegister, XMMRegister mask, int vector_len) {
	assert(UseAVX > 0, "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	vex_prefix(dst, mask->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8(0x2F);
	emit_operand(src, dst, 0);
	}

// Move Unaligned EVEX enabled Vector (programmable : 8,16,32,64)
	pub fn evmovdqub(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
	assert(VM_Version::supports_avx512vlbw(), "");
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_embedded_opmask_register_specifier(mask);
	attributes.set_is_evex_instruction();
	if (merge) {
	attributes.reset_is_clear_context();
	}
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int16(0x6F, (0xC0 | encode));
	}

	pub fn evmovdqub(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	// Unmasked instruction
	evmovdqub(dst, k0, src, /*merge*/ false, vector_len);
	}

	pub fn evmovdqub(&mut self,XMMdst: GPRegister, KRegister mask, src: Address, bool merge, int vector_len) {
	assert(VM_Version::supports_avx512vlbw(), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.set_embedded_opmask_register_specifier(mask);
	attributes.set_is_evex_instruction();
	if (merge) {
	attributes.reset_is_clear_context();
	}
	vex_prefix(src, 0, dst->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int8(0x6F);
	emit_operand(dst, src, 0);
	}

	pub fn evmovdqub(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
	// Unmasked instruction
	evmovdqub(dst, k0, src, /*merge*/ false, vector_len);
	}

	pub fn evmovdqub(&mut self,dst: Address, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
	assert(VM_Version::supports_avx512vlbw(), "");
	assert(src != xnoreg, "sanity");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.set_embedded_opmask_register_specifier(mask);
	attributes.set_is_evex_instruction();
	if (merge) {
	attributes.reset_is_clear_context();
	}
	vex_prefix(dst, 0, src->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int8(0x7F);
	emit_operand(src, dst, 0);
	}

	pub fn evmovdquw(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
	// Unmasked instruction
	evmovdquw(dst, k0, src, /*merge*/ false, vector_len);
	}

	pub fn evmovdquw(&mut self,XMMdst: GPRegister, KRegister mask, src: Address, bool merge, int vector_len) {
	assert(VM_Version::supports_avx512vlbw(), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.set_embedded_opmask_register_specifier(mask);
	attributes.set_is_evex_instruction();
	if (merge) {
	attributes.reset_is_clear_context();
	}
	vex_prefix(src, 0, dst->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int8(0x6F);
	emit_operand(dst, src, 0);
	}

	pub fn evmovdquw(&mut self,dst: Address, XMMsrc: GPRegister, int vector_len) {
	// Unmasked instruction
	evmovdquw(dst, k0, src, /*merge*/ false, vector_len);
	}

	pub fn evmovdquw(&mut self,dst: Address, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
	assert(VM_Version::supports_avx512vlbw(), "");
	assert(src != xnoreg, "sanity");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.set_embedded_opmask_register_specifier(mask);
	attributes.set_is_evex_instruction();
	if (merge) {
	attributes.reset_is_clear_context();
	}
	vex_prefix(dst, 0, src->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int8(0x7F);
	emit_operand(src, dst, 0);
	}

	pub fn evmovdqul(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	// Unmasked instruction
	evmovdqul(dst, k0, src, /*merge*/ false, vector_len);
	}

	pub fn evmovdqul(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
	assert(VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_embedded_opmask_register_specifier(mask);
	attributes.set_is_evex_instruction();
	if (merge) {
	attributes.reset_is_clear_context();
	}
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16(0x6F, (0xC0 | encode));
	}

	pub fn evmovdqul(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
	// Unmasked instruction
	evmovdqul(dst, k0, src, /*merge*/ false, vector_len);
	}

	pub fn evmovdqul(&mut self,XMMdst: GPRegister, KRegister mask, src: Address, bool merge, int vector_len) {
	assert(VM_Version::supports_evex(), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false , /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.set_embedded_opmask_register_specifier(mask);
	attributes.set_is_evex_instruction();
	if (merge) {
	attributes.reset_is_clear_context();
	}
	vex_prefix(src, 0, dst->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int8(0x6F);
	emit_operand(dst, src, 0);
	}

	pub fn evmovdqul(&mut self,dst: Address, XMMsrc: GPRegister, int vector_len) {
	// Unmasked isntruction
	evmovdqul(dst, k0, src, /*merge*/ true, vector_len);
	}

	pub fn evmovdqul(&mut self,dst: Address, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
	assert(VM_Version::supports_evex(), "");
	assert(src != xnoreg, "sanity");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.set_embedded_opmask_register_specifier(mask);
	attributes.set_is_evex_instruction();
	if (merge) {
	attributes.reset_is_clear_context();
	}
	vex_prefix(dst, 0, src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int8(0x7F);
	emit_operand(src, dst, 0);
	}

	pub fn evmovdquq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	// Unmasked instruction
	evmovdquq(dst, k0, src, /*merge*/ false, vector_len);
	}

	pub fn evmovdquq(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
	assert(VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_embedded_opmask_register_specifier(mask);
	attributes.set_is_evex_instruction();
	if (merge) {
	attributes.reset_is_clear_context();
	}
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16(0x6F, (0xC0 | encode));
	}

	pub fn evmovdquq(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
	// Unmasked instruction
	evmovdquq(dst, k0, src, /*merge*/ false, vector_len);
	}

	pub fn evmovdquq(&mut self,XMMdst: GPRegister, KRegister mask, src: Address, bool merge, int vector_len) {
	assert(VM_Version::supports_evex(), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.set_embedded_opmask_register_specifier(mask);
	attributes.set_is_evex_instruction();
	if (merge) {
	attributes.reset_is_clear_context();
	}
	vex_prefix(src, 0, dst->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int8(0x6F);
	emit_operand(dst, src, 0);
	}

	pub fn evmovdquq(&mut self,dst: Address, XMMsrc: GPRegister, int vector_len) {
	// Unmasked instruction
	evmovdquq(dst, k0, src, /*merge*/ true, vector_len);
	}

	pub fn evmovdquq(&mut self,dst: Address, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
	assert(VM_Version::supports_evex(), "");
	assert(src != xnoreg, "sanity");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.set_embedded_opmask_register_specifier(mask);
	if (merge) {
	attributes.reset_is_clear_context();
	}
	attributes.set_is_evex_instruction();
	vex_prefix(dst, 0, src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int8(0x7F);
	emit_operand(src, dst, 0);
	}

// Uses zero extension on 64bit

	pub fn movl(&mut self,dst: GPRegister, imm32: i32) {
	int encode = prefix_and_encode(dst->encoding());
	emit_int8(0xB8 | encode);
	emit_int32(imm32);
	}

	pub fn movl(&mut self,dst: GPRegister, src: GPRegister) {
	int encode = prefix_and_encode(dst->encoding(), src->encoding());
	emit_int16((unsigned char)0x8B, (0xC0 | encode));
	}

	pub fn movl(&mut self,dst: GPRegister, src: Address) {
	InstructionMark im(this);
	prefix(src, dst);
	emit_int8((unsigned char)0x8B);
	emit_operand(dst, src, 0);
	}

	pub fn movl(&mut self,dst: Address, imm32: i32) {
	InstructionMark im(this);
	prefix(dst);
	emit_int8((unsigned char)0xC7);
	emit_operand(rax, dst, 4);
	emit_int32(imm32);
	}

	pub fn movl(&mut self,dst: Address, src: GPRegister) {
	InstructionMark im(this);
	prefix(dst, src);
	emit_int8((unsigned char)0x89);
	emit_operand(src, dst, 0);
	}

// New cpus require to use movsd and movss to avoid partial register stall
// when loading from memory. But for old Opteron use movlpd instead of movsd.
// The selection is done in MacroAssembler::movdbl() and movflt().
	pub fn movlpd(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
	attributes.set_rex_vex_w_reverted();
	simd_prefix(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int8(0x12);
	emit_operand(dst, src, 0);
	}

	pub fn movq(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
	attributes.set_rex_vex_w_reverted();
	simd_prefix(dst, xnoreg, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int8(0x7E);
	emit_operand(dst, src, 0);
	}

	pub fn movq(&mut self,dst: Address, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
	attributes.set_rex_vex_w_reverted();
	simd_prefix(src, xnoreg, dst, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int8((unsigned char)0xD6);
	emit_operand(src, dst, 0);
	}

	pub fn movq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_rex_vex_w_reverted();
	int encode = simd_prefix_and_encode(src, xnoreg, dst, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0xD6, (0xC0 | encode));
	}

	pub fn movq(&mut self,dst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	// swap src/dst to get correct prefix
	int encode = simd_prefix_and_encode(src, xnoreg, as_XMMRegister(dst->encoding()), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x7E, (0xC0 | encode));
	}

	pub fn movq(&mut self,XMMdst: GPRegister, src: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, xnoreg, as_XMMRegister(src->encoding()), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x6E, (0xC0 | encode));
	}

	pub fn movsbl(&mut self,dst: GPRegister, src: Address) { // movsxb
	InstructionMark im(this);
	prefix(src, dst);
	emit_int16(0x0F, (unsigned char)0xBE);
	emit_operand(dst, src, 0);
	}

	pub fn movsbl(&mut self,dst: GPRegister, src: GPRegister) { // movsxb
	NOT_LP64(assert(src->has_byte_register(), "must have byte register"));
	int encode = prefix_and_encode(dst->encoding(), false, src->encoding(), true);
	emit_int24(0x0F, (unsigned char)0xBE, (0xC0 | encode));
	}

	pub fn movsd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_rex_vex_w_reverted();
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int16(0x10, (0xC0 | encode));
	}

	pub fn movsd(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
	attributes.set_rex_vex_w_reverted();
	simd_prefix(dst, xnoreg, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int8(0x10);
	emit_operand(dst, src, 0);
	}

	pub fn movsd(&mut self,dst: Address, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
	attributes.reset_is_clear_context();
	attributes.set_rex_vex_w_reverted();
	simd_prefix(src, xnoreg, dst, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int8(0x11);
	emit_operand(src, dst, 0);
	}

	pub fn movss(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16(0x10, (0xC0 | encode));
	}

	pub fn movss(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(dst, xnoreg, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int8(0x10);
	emit_operand(dst, src, 0);
	}

	pub fn movss(&mut self,dst: Address, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
	attributes.reset_is_clear_context();
	simd_prefix(src, xnoreg, dst, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int8(0x11);
	emit_operand(src, dst, 0);
	}

	pub fn movswl(&mut self,dst: GPRegister, src: Address) { // movsxw
	InstructionMark im(this);
	prefix(src, dst);
	emit_int16(0x0F, (unsigned char)0xBF);
	emit_operand(dst, src, 0);
	}

	pub fn movswl(&mut self,dst: GPRegister, src: GPRegister) { // movsxw
	int encode = prefix_and_encode(dst->encoding(), src->encoding());
	emit_int24(0x0F, (unsigned char)0xBF, (0xC0 | encode));
	}

	pub fn movups(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(dst, xnoreg, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int8(0x10);
	emit_operand(dst, src, 0);
	}

	pub fn vmovups(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
	assert(vector_len == AVX_512bit ? VM_Version::supports_evex() : VM_Version::supports_avx(), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(dst, xnoreg, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int8(0x10);
	emit_operand(dst, src, 0);
	}

	pub fn movups(&mut self,dst: Address, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(src, xnoreg, dst, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int8(0x11);
	emit_operand(src, dst, 0);
	}

	pub fn vmovups(&mut self,dst: Address, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_512bit ? VM_Version::supports_evex() : VM_Version::supports_avx(), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(src, xnoreg, dst, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int8(0x11);
	emit_operand(src, dst, 0);
	}

	pub fn movw(&mut self,dst: Address, imm16: i16) {
	InstructionMark im(this);

	emit_int8(0x66); // switch to 16-bit mode
	prefix(dst);
	emit_int8((unsigned char)0xC7);
	emit_operand(rax, dst, 2);
	emit_int16(imm16);
	}

	pub fn movw(&mut self,dst: GPRegister, src: Address) {
	InstructionMark im(this);
	emit_int8(0x66);
	prefix(src, dst);
	emit_int8((unsigned char)0x8B);
	emit_operand(dst, src, 0);
	}

	pub fn movw(&mut self,dst: Address, src: GPRegister) {
	InstructionMark im(this);
	emit_int8(0x66);
	prefix(dst, src);
	emit_int8((unsigned char)0x89);
	emit_operand(src, dst, 0);
	}

	pub fn movzbl(&mut self,dst: GPRegister, src: Address) { // movzxb
	InstructionMark im(this);
	prefix(src, dst);
	emit_int16(0x0F, (unsigned char)0xB6);
	emit_operand(dst, src, 0);
	}

	pub fn movzbl(&mut self,dst: GPRegister, src: GPRegister) { // movzxb
	NOT_LP64(assert(src->has_byte_register(), "must have byte register"));
	int encode = prefix_and_encode(dst->encoding(), false, src->encoding(), true);
	emit_int24(0x0F, (unsigned char)0xB6, 0xC0 | encode);
	}

	pub fn movzwl(&mut self,dst: GPRegister, src: Address) { // movzxw
	InstructionMark im(this);
	prefix(src, dst);
	emit_int16(0x0F, (unsigned char)0xB7);
	emit_operand(dst, src, 0);
	}

	pub fn movzwl(&mut self,dst: GPRegister, src: GPRegister) { // movzxw
	int encode = prefix_and_encode(dst->encoding(), src->encoding());
	emit_int24(0x0F, (unsigned char)0xB7, 0xC0 | encode);
	}

	pub fn mull(&mut self,src: Address) {
	InstructionMark im(this);
	prefix(src);
	emit_int8((unsigned char)0xF7);
	emit_operand(rsp, src, 0);
	}

	pub fn mull(&mut self,src: GPRegister) {
	int encode = prefix_and_encode(src->encoding());
	emit_int16((unsigned char)0xF7, (0xE0 | encode));
	}

	pub fn mulsd(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
	attributes.set_rex_vex_w_reverted();
	simd_prefix(dst, dst, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int8(0x59);
	emit_operand(dst, src, 0);
	}

	pub fn mulsd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_rex_vex_w_reverted();
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int16(0x59, (0xC0 | encode));
	}

	pub fn mulss(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(dst, dst, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int8(0x59);
	emit_operand(dst, src, 0);
	}

	pub fn mulss(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16(0x59, (0xC0 | encode));
	}

	pub fn negl(&mut self,dst: GPRegister) {
	int encode = prefix_and_encode(dst->encoding());
	emit_int16((unsigned char)0xF7, (0xD8 | encode));
	}

	pub fn negl(&mut self,dst: Address) {
	InstructionMark im(this);
	prefix(dst);
	emit_int8((unsigned char)0xF7);
	emit_operand(as_Register(3), dst, 0);
	}

	pub fn nop(&mut self,int i) {
	#ifdef ASSERT
	assert(i > 0, " ");
	// The fancy nops aren't currently recognized by debuggers making it a
	// pain to disassemble code while debugging. If asserts are on clearly
	// speed is not an issue so simply use the single byte traditional nop
	// to do alignment.

	for (; i > 0 ; i--) emit_int8((unsigned char)0x90);
	return;

	#endif // ASSERT

	if (UseAddressNop && VM_Version::is_intel()) {
	//
	// Using multi-bytes nops "0x0F 0x1F [address]" for Intel
	//  1: 0x90
	//  2: 0x66 0x90
	//  3: 0x66 0x66 0x90 (don't use "0x0F 0x1F 0x00" - need patching safe padding)
	//  4: 0x0F 0x1F 0x40 0x00
	//  5: 0x0F 0x1F 0x44 0x00 0x00
	//  6: 0x66 0x0F 0x1F 0x44 0x00 0x00
	//  7: 0x0F 0x1F 0x80 0x00 0x00 0x00 0x00
	//  8: 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00
	//  9: 0x66 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00
	// 10: 0x66 0x66 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00
	// 11: 0x66 0x66 0x66 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00

	// The rest coding is Intel specific - don't use consecutive address nops

	// 12: 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00 0x66 0x66 0x66 0x90
	// 13: 0x66 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00 0x66 0x66 0x66 0x90
	// 14: 0x66 0x66 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00 0x66 0x66 0x66 0x90
	// 15: 0x66 0x66 0x66 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00 0x66 0x66 0x66 0x90

	while(i >= 15) {
	// For Intel don't generate consecutive address nops (mix with regular nops)
	i -= 15;
	emit_int24(0x66, 0x66, 0x66);
	addr_nop_8();
	emit_int32(0x66, 0x66, 0x66, (unsigned char)0x90);
	}
	switch (i) {
	case 14:
	emit_int8(0x66); // size prefix
	case 13:
	emit_int8(0x66); // size prefix
	case 12:
	addr_nop_8();
	emit_int32(0x66, 0x66, 0x66, (unsigned char)0x90);
	break;
	case 11:
	emit_int8(0x66); // size prefix
	case 10:
	emit_int8(0x66); // size prefix
	case 9:
	emit_int8(0x66); // size prefix
	case 8:
	addr_nop_8();
	break;
	case 7:
	addr_nop_7();
	break;
	case 6:
	emit_int8(0x66); // size prefix
	case 5:
	addr_nop_5();
	break;
	case 4:
	addr_nop_4();
	break;
	case 3:
	// Don't use "0x0F 0x1F 0x00" - need patching safe padding
	emit_int8(0x66); // size prefix
	case 2:
	emit_int8(0x66); // size prefix
	case 1:
	emit_int8((unsigned char)0x90);
	// nop
	break;
	default:
	assert(i == 0, " ");
	}
	return;
	}
	if (UseAddressNop && VM_Version::is_amd_family()) {
	//
	// Using multi-bytes nops "0x0F 0x1F [address]" for AMD.
	//  1: 0x90
	//  2: 0x66 0x90
	//  3: 0x66 0x66 0x90 (don't use "0x0F 0x1F 0x00" - need patching safe padding)
	//  4: 0x0F 0x1F 0x40 0x00
	//  5: 0x0F 0x1F 0x44 0x00 0x00
	//  6: 0x66 0x0F 0x1F 0x44 0x00 0x00
	//  7: 0x0F 0x1F 0x80 0x00 0x00 0x00 0x00
	//  8: 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00
	//  9: 0x66 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00
	// 10: 0x66 0x66 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00
	// 11: 0x66 0x66 0x66 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00

	// The rest coding is AMD specific - use consecutive address nops

	// 12: 0x66 0x0F 0x1F 0x44 0x00 0x00 0x66 0x0F 0x1F 0x44 0x00 0x00
	// 13: 0x0F 0x1F 0x80 0x00 0x00 0x00 0x00 0x66 0x0F 0x1F 0x44 0x00 0x00
	// 14: 0x0F 0x1F 0x80 0x00 0x00 0x00 0x00 0x0F 0x1F 0x80 0x00 0x00 0x00 0x00
	// 15: 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00 0x0F 0x1F 0x80 0x00 0x00 0x00 0x00
	// 16: 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00
	//     Size prefixes (0x66) are added for larger sizes

	while(i >= 22) {
	i -= 11;
	emit_int24(0x66, 0x66, 0x66);
	addr_nop_8();
	}
	// Generate first nop for size between 21-12
	switch (i) {
	case 21:
	i -= 1;
	emit_int8(0x66); // size prefix
	case 20:
	case 19:
	i -= 1;
	emit_int8(0x66); // size prefix
	case 18:
	case 17:
	i -= 1;
	emit_int8(0x66); // size prefix
	case 16:
	case 15:
	i -= 8;
	addr_nop_8();
	break;
	case 14:
	case 13:
	i -= 7;
	addr_nop_7();
	break;
	case 12:
	i -= 6;
	emit_int8(0x66); // size prefix
	addr_nop_5();
	break;
	default:
	assert(i < 12, " ");
	}

	// Generate second nop for size between 11-1
	switch (i) {
	case 11:
	emit_int8(0x66); // size prefix
	case 10:
	emit_int8(0x66); // size prefix
	case 9:
	emit_int8(0x66); // size prefix
	case 8:
	addr_nop_8();
	break;
	case 7:
	addr_nop_7();
	break;
	case 6:
	emit_int8(0x66); // size prefix
	case 5:
	addr_nop_5();
	break;
	case 4:
	addr_nop_4();
	break;
	case 3:
	// Don't use "0x0F 0x1F 0x00" - need patching safe padding
	emit_int8(0x66); // size prefix
	case 2:
	emit_int8(0x66); // size prefix
	case 1:
	emit_int8((unsigned char)0x90);
	// nop
	break;
	default:
	assert(i == 0, " ");
	}
	return;
	}

	if (UseAddressNop && VM_Version::is_zx()) {
	//
	// Using multi-bytes nops "0x0F 0x1F [address]" for ZX
	//  1: 0x90
	//  2: 0x66 0x90
	//  3: 0x66 0x66 0x90 (don't use "0x0F 0x1F 0x00" - need patching safe padding)
	//  4: 0x0F 0x1F 0x40 0x00
	//  5: 0x0F 0x1F 0x44 0x00 0x00
	//  6: 0x66 0x0F 0x1F 0x44 0x00 0x00
	//  7: 0x0F 0x1F 0x80 0x00 0x00 0x00 0x00
	//  8: 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00
	//  9: 0x66 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00
	// 10: 0x66 0x66 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00
	// 11: 0x66 0x66 0x66 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00

	// The rest coding is ZX specific - don't use consecutive address nops

	// 12: 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00 0x66 0x66 0x66 0x90
	// 13: 0x66 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00 0x66 0x66 0x66 0x90
	// 14: 0x66 0x66 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00 0x66 0x66 0x66 0x90
	// 15: 0x66 0x66 0x66 0x0F 0x1F 0x84 0x00 0x00 0x00 0x00 0x00 0x66 0x66 0x66 0x90

	while (i >= 15) {
	// For ZX don't generate consecutive address nops (mix with regular nops)
	i -= 15;
	emit_int24(0x66, 0x66, 0x66);
	addr_nop_8();
	emit_int32(0x66, 0x66, 0x66, (unsigned char)0x90);
	}
	switch (i) {
	case 14:
	emit_int8(0x66); // size prefix
	case 13:
	emit_int8(0x66); // size prefix
	case 12:
	addr_nop_8();
	emit_int32(0x66, 0x66, 0x66, (unsigned char)0x90);
	break;
	case 11:
	emit_int8(0x66); // size prefix
	case 10:
	emit_int8(0x66); // size prefix
	case 9:
	emit_int8(0x66); // size prefix
	case 8:
	addr_nop_8();
	break;
	case 7:
	addr_nop_7();
	break;
	case 6:
	emit_int8(0x66); // size prefix
	case 5:
	addr_nop_5();
	break;
	case 4:
	addr_nop_4();
	break;
	case 3:
	// Don't use "0x0F 0x1F 0x00" - need patching safe padding
	emit_int8(0x66); // size prefix
	case 2:
	emit_int8(0x66); // size prefix
	case 1:
	emit_int8((unsigned char)0x90);
	// nop
	break;
	default:
	assert(i == 0, " ");
	}
	return;
	}

	// Using nops with size prefixes "0x66 0x90".
	// From AMD Optimization Guide:
	//  1: 0x90
	//  2: 0x66 0x90
	//  3: 0x66 0x66 0x90
	//  4: 0x66 0x66 0x66 0x90
	//  5: 0x66 0x66 0x90 0x66 0x90
	//  6: 0x66 0x66 0x90 0x66 0x66 0x90
	//  7: 0x66 0x66 0x66 0x90 0x66 0x66 0x90
	//  8: 0x66 0x66 0x66 0x90 0x66 0x66 0x66 0x90
	//  9: 0x66 0x66 0x90 0x66 0x66 0x90 0x66 0x66 0x90
	// 10: 0x66 0x66 0x66 0x90 0x66 0x66 0x90 0x66 0x66 0x90
	//
	while (i > 12) {
	i -= 4;
	emit_int32(0x66, 0x66, 0x66, (unsigned char)0x90);
	}
	// 1 - 12 nops
	if (i > 8) {
	if (i > 9) {
	i -= 1;
	emit_int8(0x66);
	}
	i -= 3;
	emit_int24(0x66, 0x66, (unsigned char)0x90);
	}
	// 1 - 8 nops
	if (i > 4) {
	if (i > 6) {
	i -= 1;
	emit_int8(0x66);
	}
	i -= 3;
	emit_int24(0x66, 0x66, (unsigned char)0x90);
	}
	switch (i) {
	case 4:
	emit_int8(0x66);
	case 3:
	emit_int8(0x66);
	case 2:
	emit_int8(0x66);
	case 1:
	emit_int8((unsigned char)0x90);
	break;
	default:
	assert(i == 0, " ");
	}
	}

	pub fn notl(&mut self,dst: GPRegister) {
	int encode = prefix_and_encode(dst->encoding());
	emit_int16((unsigned char)0xF7, (0xD0 | encode));
	}

	pub fn orw(&mut self,dst: GPRegister, src: GPRegister) {
	(void)prefix_and_encode(dst->encoding(), src->encoding());
	emit_arith(0x0B, 0xC0, dst, src);
	}

	pub fn orl(&mut self,dst: Address, imm32: i32) {
	InstructionMark im(this);
	prefix(dst);
	emit_arith_operand(0x81, rcx, dst, imm32);
	}

	pub fn orl(&mut self,dst: GPRegister, imm32: i32) {
	prefix(dst);
	emit_arith(0x81, 0xC8, dst, imm32);
	}

	pub fn orl(&mut self,dst: GPRegister, src: Address) {
	InstructionMark im(this);
	prefix(src, dst);
	emit_int8(0x0B);
	emit_operand(dst, src, 0);
	}

	pub fn orl(&mut self,dst: GPRegister, src: GPRegister) {
	(void) prefix_and_encode(dst->encoding(), src->encoding());
	emit_arith(0x0B, 0xC0, dst, src);
	}

	pub fn orl(&mut self,dst: Address, src: GPRegister) {
	InstructionMark im(this);
	prefix(dst, src);
	emit_int8(0x09);
	emit_operand(src, dst, 0);
	}

	pub fn orb(&mut self,dst: Address, imm8: i8) {
	InstructionMark im(this);
	prefix(dst);
	emit_int8((unsigned char)0x80);
	emit_operand(rcx, dst, 1);
	emit_int8(imm8);
	}

	pub fn orb(&mut self,dst: Address, src: GPRegister) {
	InstructionMark im(this);
	prefix(dst, src, true);
	emit_int8(0x08);
	emit_operand(src, dst, 0);
	}

	pub fn packsswb(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x63, (0xC0 | encode));
	}

	pub fn vpacksswb(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(UseAVX > 0, "some form of AVX must be enabled");
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x63, (0xC0 | encode));
	}

	pub fn packssdw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sse2(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x6B, (0xC0 | encode));
	}

	pub fn vpackssdw(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(UseAVX > 0, "some form of AVX must be enabled");
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x6B, (0xC0 | encode));
	}

	pub fn packuswb(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	assert((UseAVX > 0), "SSE mode requires address alignment 16 bytes");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int8(0x67);
	emit_operand(dst, src, 0);
	}

	pub fn packuswb(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x67, (0xC0 | encode));
	}

	pub fn vpackuswb(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(UseAVX > 0, "some form of AVX must be enabled");
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x67, (0xC0 | encode));
	}

	pub fn packusdw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x2B, (0xC0 | encode));
	}

	pub fn vpackusdw(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(UseAVX > 0, "some form of AVX must be enabled");
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x2B, (0xC0 | encode));
	}

	pub fn vpermq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, imm8: i8, int vector_len) {
	assert(VM_Version::supports_avx2(), "");
	assert(vector_len != AVX_128bit, "");
	// VEX.256.66.0F3A.W1 00 /r ib
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x00, (0xC0 | encode), imm8);
	}

	pub fn vpermq(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_256bit ? VM_Version::supports_avx512vl() :
	vector_len == AVX_512bit ? VM_Version::supports_evex()     : false, "not supported");
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x36, (0xC0 | encode));
	}

	pub fn vpermb(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx512_vbmi(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16((unsigned char)0x8D, (0xC0 | encode));
	}

	pub fn vpermb(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
	assert(VM_Version::supports_avx512_vbmi(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8((unsigned char)0x8D);
	emit_operand(dst, src, 0);
	}

	pub fn vpermw(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_128bit ? VM_Version::supports_avx512vlbw() :
	vector_len == AVX_256bit ? VM_Version::supports_avx512vlbw() :
	vector_len == AVX_512bit ? VM_Version::supports_avx512bw()   : false, "not supported");
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16((unsigned char)0x8D, (0xC0 | encode));
	}

	pub fn vpermd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert((vector_len == AVX_256bit && VM_Version::supports_avx2()) ||
	(vector_len == AVX_512bit && VM_Version::supports_evex()), "");
	// VEX.NDS.256.66.0F38.W0 36 /r
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x36, (0xC0 | encode));
	}

	pub fn vpermd(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
	assert((vector_len == AVX_256bit && VM_Version::supports_avx2()) ||
	(vector_len == AVX_512bit && VM_Version::supports_evex()), "");
	// VEX.NDS.256.66.0F38.W0 36 /r
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8(0x36);
	emit_operand(dst, src, 0);
	}

	pub fn vpermps(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert((vector_len == AVX_256bit && VM_Version::supports_avx2()) ||
	(vector_len == AVX_512bit && VM_Version::supports_evex()), "");
	// VEX.NDS.XXX.66.0F38.W0 16 /r
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x16, (0xC0 | encode));
	}

	pub fn vperm2i128(&mut self,XMMdst: GPRegister,  nds: XMMRegister, XMMsrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_avx2(), "");
	InstructionAttr attributes(AVX_256bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x46, (0xC0 | encode), imm8);
	}

	pub fn vperm2f128(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_avx(), "");
	InstructionAttr attributes(AVX_256bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x06, (0xC0 | encode), imm8);
	}

	pub fn vpermilps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, imm8: i8, int vector_len) {
	assert(vector_len <= AVX_256bit ? VM_Version::supports_avx() : VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x04, (0xC0 | encode), imm8);
	}

	pub fn vpermilps(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len <= AVX_256bit ? VM_Version::supports_avx() : VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x0C, (0xC0 | encode));
	}

	pub fn vpermilpd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, imm8: i8, int vector_len) {
	assert(vector_len <= AVX_256bit ? VM_Version::supports_avx() : VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ VM_Version::supports_evex(),/* legacy_mode */ false,/* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_rex_vex_w_reverted();
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x05, (0xC0 | encode), imm8);
	}

	pub fn vpermpd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, imm8: i8, int vector_len) {
	assert(vector_len <= AVX_256bit ? VM_Version::supports_avx2() : VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x01, (0xC0 | encode), imm8);
	}

	pub fn evpermi2q(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x76, (0xC0 | encode));
	}

	pub fn evpermt2b(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx512_vbmi(), "");
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x7D, (0xC0 | encode));
	}

	pub fn evpmultishiftqb(&mut self,XMMdst: GPRegister, XMMRegister ctl, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx512_vbmi(), "");
	InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), ctl->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16((unsigned char)0x83, (unsigned char)(0xC0 | encode));
	}

	pub fn pause(&mut self,) {
	emit_int16((unsigned char)0xF3, (unsigned char)0x90);
	}

	pub fn ud2(&mut self,) {
	emit_int16(0x0F, 0x0B);
	}

	pub fn pcmpestri(&mut self,XMMdst: GPRegister, src: Address, imm8: i8) {
	assert(VM_Version::supports_sse4_2(), "");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	simd_prefix(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int8(0x61);
	emit_operand(dst, src, 1);
	emit_int8(imm8);
	}

	pub fn pcmpestri(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_sse4_2(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x61, (0xC0 | encode), imm8);
	}

// In this context, the dst vector contains the components that are equal, non equal components are zeroed in dst
	pub fn pcmpeqb(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sse2(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x74, (0xC0 | encode));
	}

	pub fn vpcmpCCbwd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int cond_encoding, int vector_len) {
	assert(vector_len == AVX_128bit ? VM_Version::supports_avx() : VM_Version::supports_avx2(), "");
	assert(vector_len <= AVX_256bit, "evex encoding is different - has k register as dest");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(cond_encoding, (0xC0 | encode));
	}

// In this context, the dst vector contains the components that are equal, non equal components are zeroed in dst
	pub fn vpcmpeqb(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_128bit ? VM_Version::supports_avx() : VM_Version::supports_avx2(), "");
	assert(vector_len <= AVX_256bit, "evex encoding is different - has k register as dest");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x74, (0xC0 | encode));
	}

// In this context, kdst is written the mask used to process the equal components
	pub fn evpcmpeqb(&mut self,KRegister kdst, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(kdst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x74, (0xC0 | encode));
	}

	pub fn evpcmpgtb(&mut self,KRegister kdst, nds: XMMRegister, src: Address, int vector_len) {
	assert(VM_Version::supports_avx512vlbw(), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.set_is_evex_instruction();
	int dst_enc = kdst->encoding();
	vex_prefix(src, nds->encoding(), dst_enc, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int8(0x64);
	emit_operand(as_Register(dst_enc), src, 0);
	}

	pub fn evpcmpgtb(&mut self,KRegister kdst, KRegister mask, nds: XMMRegister, src: Address, int vector_len) {
	assert(VM_Version::supports_avx512vlbw(), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.reset_is_clear_context();
	attributes.set_embedded_opmask_register_specifier(mask);
	attributes.set_is_evex_instruction();
	int dst_enc = kdst->encoding();
	vex_prefix(src, nds->encoding(), dst_enc, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int8(0x64);
	emit_operand(as_Register(dst_enc), src, 0);
	}

	pub fn evpcmpuw(&mut self,KRegister kdst, nds: XMMRegister, XMMsrc: GPRegister, ComparisonPredicate vcc, int vector_len) {
	assert(VM_Version::supports_avx512vlbw(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(kdst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x3E, (0xC0 | encode), vcc);
	}

	pub fn evpcmpuw(&mut self,KRegister kdst, nds: XMMRegister, src: Address, ComparisonPredicate vcc, int vector_len) {
	assert(VM_Version::supports_avx512vlbw(), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.set_is_evex_instruction();
	int dst_enc = kdst->encoding();
	vex_prefix(src, nds->encoding(), kdst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int8(0x3E);
	emit_operand(as_Register(dst_enc), src, 1);
	emit_int8(vcc);
	}

	pub fn evpcmpeqb(&mut self,KRegister kdst, nds: XMMRegister, src: Address, int vector_len) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	int dst_enc = kdst->encoding();
	vex_prefix(src, nds->encoding(), dst_enc, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int8(0x74);
	emit_operand(as_Register(dst_enc), src, 0);
	}

	pub fn evpcmpeqb(&mut self,KRegister kdst, KRegister mask, nds: XMMRegister, src: Address, int vector_len) {
	assert(VM_Version::supports_avx512vlbw(), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.reset_is_clear_context();
	attributes.set_embedded_opmask_register_specifier(mask);
	attributes.set_is_evex_instruction();
	vex_prefix(src, nds->encoding(), kdst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int8(0x74);
	emit_operand(as_Register(kdst->encoding()), src, 0);
	}

// In this context, the dst vector contains the components that are equal, non equal components are zeroed in dst
	pub fn pcmpeqw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sse2(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x75, (0xC0 | encode));
	}

// In this context, the dst vector contains the components that are equal, non equal components are zeroed in dst
	pub fn vpcmpeqw(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_128bit ? VM_Version::supports_avx() : VM_Version::supports_avx2(), "");
	assert(vector_len <= AVX_256bit, "evex encoding is different - has k register as dest");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x75, (0xC0 | encode));
	}

// In this context, kdst is written the mask used to process the equal components
	pub fn evpcmpeqw(&mut self,KRegister kdst, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(kdst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x75, (0xC0 | encode));
	}

	pub fn evpcmpeqw(&mut self,KRegister kdst, nds: XMMRegister, src: Address, int vector_len) {
	assert(VM_Version::supports_avx512bw(), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.set_is_evex_instruction();
	int dst_enc = kdst->encoding();
	vex_prefix(src, nds->encoding(), dst_enc, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int8(0x75);
	emit_operand(as_Register(dst_enc), src, 0);
	}

// In this context, the dst vector contains the components that are equal, non equal components are zeroed in dst
	pub fn pcmpeqd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sse2(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x76, (0xC0 | encode));
	}

// In this context, the dst vector contains the components that are equal, non equal components are zeroed in dst
	pub fn vpcmpeqd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_128bit ? VM_Version::supports_avx() : VM_Version::supports_avx2(), "");
	assert(vector_len <= AVX_256bit, "evex encoding is different - has k register as dest");
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x76, (0xC0 | encode));
	}

// In this context, kdst is written the mask used to process the equal components
	pub fn evpcmpeqd(&mut self,KRegister kdst, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	attributes.reset_is_clear_context();
	attributes.set_embedded_opmask_register_specifier(mask);
	int encode = vex_prefix_and_encode(kdst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x76, (0xC0 | encode));
	}

	pub fn evpcmpeqd(&mut self,KRegister kdst, KRegister mask, nds: XMMRegister, src: Address, int vector_len) {
	assert(VM_Version::supports_evex(), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_32bit);
	attributes.set_is_evex_instruction();
	attributes.reset_is_clear_context();
	attributes.set_embedded_opmask_register_specifier(mask);
	int dst_enc = kdst->encoding();
	vex_prefix(src, nds->encoding(), dst_enc, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int8(0x76);
	emit_operand(as_Register(dst_enc), src, 0);
	}

// In this context, the dst vector contains the components that are equal, non equal components are zeroed in dst
	pub fn pcmpeqq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x29, (0xC0 | encode));
	}

	pub fn evpcmpeqq(&mut self,KRegister kdst, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	attributes.reset_is_clear_context();
	attributes.set_embedded_opmask_register_specifier(mask);
	int encode = vex_prefix_and_encode(kdst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x29, (0xC0 | encode));
	}

	pub fn vpcmpCCq(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int cond_encoding, int vector_len) {
	assert(VM_Version::supports_avx(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(cond_encoding, (0xC0 | encode));
	}

// In this context, the dst vector contains the components that are equal, non equal components are zeroed in dst
	pub fn vpcmpeqq(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x29, (0xC0 | encode));
	}

// In this context, kdst is written the mask used to process the equal components
	pub fn evpcmpeqq(&mut self,KRegister kdst, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.reset_is_clear_context();
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(kdst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x29, (0xC0 | encode));
	}

// In this context, kdst is written the mask used to process the equal components
	pub fn evpcmpeqq(&mut self,KRegister kdst, nds: XMMRegister, src: Address, int vector_len) {
	assert(VM_Version::supports_evex(), "");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.reset_is_clear_context();
	attributes.set_is_evex_instruction();
	attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
	int dst_enc = kdst->encoding();
	vex_prefix(src, nds->encoding(), dst_enc, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8(0x29);
	emit_operand(as_Register(dst_enc), src, 0);
	}

	pub fn pcmpgtq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x37, (0xC0 | encode));
	}

	pub fn pmovmskb(&mut self,dst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sse2(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(as_XMMRegister(dst->encoding()), xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0xD7, (0xC0 | encode));
	}

	pub fn vpmovmskb(&mut self,dst: GPRegister, XMMsrc: GPRegister, int vec_enc) {
	assert((VM_Version::supports_avx() && vec_enc == AVX_128bit) ||
	(VM_Version::supports_avx2() && vec_enc  == AVX_256bit), "");
	InstructionAttr attributes(vec_enc, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0xD7, (0xC0 | encode));
	}

	pub fn vmovmskps(&mut self,dst: GPRegister, XMMsrc: GPRegister, int vec_enc) {
	assert(VM_Version::supports_avx(), "");
	InstructionAttr attributes(vec_enc, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16(0x50, (0xC0 | encode));
	}

	pub fn vmovmskpd(&mut self,dst: GPRegister, XMMsrc: GPRegister, int vec_enc) {
	assert(VM_Version::supports_avx(), "");
	InstructionAttr attributes(vec_enc, /* rex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x50, (0xC0 | encode));
	}


	pub fn pextrd(&mut self,dst: GPRegister, XMMsrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(src, xnoreg, as_XMMRegister(dst->encoding()), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x16, (0xC0 | encode), imm8);
	}

	pub fn pextrd(&mut self,dst: Address, XMMsrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(src, xnoreg, dst, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int8(0x16);
	emit_operand(src, dst, 1);
	emit_int8(imm8);
	}

	pub fn pextrq(&mut self,dst: GPRegister, XMMsrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(src, xnoreg, as_XMMRegister(dst->encoding()), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x16, (0xC0 | encode), imm8);
	}

	pub fn pextrq(&mut self,dst: Address, XMMsrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
	simd_prefix(src, xnoreg, dst, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int8(0x16);
	emit_operand(src, dst, 1);
	emit_int8(imm8);
	}

	pub fn pextrw(&mut self,dst: GPRegister, XMMsrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_sse2(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(as_XMMRegister(dst->encoding()), xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int24((unsigned char)0xC5, (0xC0 | encode), imm8);
	}

	pub fn pextrw(&mut self,dst: Address, XMMsrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_16bit);
	simd_prefix(src, xnoreg, dst, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int8(0x15);
	emit_operand(src, dst, 1);
	emit_int8(imm8);
	}

	pub fn pextrb(&mut self,dst: GPRegister, XMMsrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(src, xnoreg, as_XMMRegister(dst->encoding()), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x14, (0xC0 | encode), imm8);
	}

	pub fn pextrb(&mut self,dst: Address, XMMsrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_8bit);
	simd_prefix(src, xnoreg, dst, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int8(0x14);
	emit_operand(src, dst, 1);
	emit_int8(imm8);
	}

	pub fn pinsrd(&mut self,XMMdst: GPRegister, src: GPRegister, imm8: i8) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, as_XMMRegister(src->encoding()), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x22, (0xC0 | encode), imm8);
	}

	pub fn pinsrd(&mut self,XMMdst: GPRegister, src: Address, imm8: i8) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int8(0x22);
	emit_operand(dst, src, 1);
	emit_int8(imm8);
	}

	pub fn vpinsrd(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: GPRegister, imm8: i8) {
	assert(VM_Version::supports_avx(), "");
	InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x22, (0xC0 | encode), imm8);
	}

	pub fn pinsrq(&mut self,XMMdst: GPRegister, src: GPRegister, imm8: i8) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, as_XMMRegister(src->encoding()), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x22, (0xC0 | encode), imm8);
	}

	pub fn pinsrq(&mut self,XMMdst: GPRegister, src: Address, imm8: i8) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
	simd_prefix(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int8(0x22);
	emit_operand(dst, src, 1);
	emit_int8(imm8);
	}

	pub fn vpinsrq(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: GPRegister, imm8: i8) {
	assert(VM_Version::supports_avx(), "");
	InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x22, (0xC0 | encode), imm8);
	}

	pub fn pinsrw(&mut self,XMMdst: GPRegister, src: GPRegister, imm8: i8) {
	assert(VM_Version::supports_sse2(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, as_XMMRegister(src->encoding()), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int24((unsigned char)0xC4, (0xC0 | encode), imm8);
	}

	pub fn pinsrw(&mut self,XMMdst: GPRegister, src: Address, imm8: i8) {
	assert(VM_Version::supports_sse2(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_16bit);
	simd_prefix(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int8((unsigned char)0xC4);
	emit_operand(dst, src, 1);
	emit_int8(imm8);
	}

	pub fn vpinsrw(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: GPRegister, imm8: i8) {
	assert(VM_Version::supports_avx(), "");
	InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int24((unsigned char)0xC4, (0xC0 | encode), imm8);
	}

	pub fn pinsrb(&mut self,XMMdst: GPRegister, src: Address, imm8: i8) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_8bit);
	simd_prefix(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int8(0x20);
	emit_operand(dst, src, 1);
	emit_int8(imm8);
	}

	pub fn pinsrb(&mut self,XMMdst: GPRegister, src: GPRegister, imm8: i8) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, as_XMMRegister(src->encoding()), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x20, (0xC0 | encode), imm8);
	}

	pub fn vpinsrb(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: GPRegister, imm8: i8) {
	assert(VM_Version::supports_avx(), "");
	InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x20, (0xC0 | encode), imm8);
	}

	pub fn insertps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x21, (0xC0 | encode), imm8);
	}

	pub fn vinsertps(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_avx(), "");
	InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x21, (0xC0 | encode), imm8);
	}

	pub fn pmovzxbw(&mut self,XMMdst: GPRegister, src: Address) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_HVM, /* input_size_in_bits */ EVEX_NObit);
	simd_prefix(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8(0x30);
	emit_operand(dst, src, 0);
	}

	pub fn pmovzxbw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x30, (0xC0 | encode));
	}

	pub fn pmovsxbw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x20, (0xC0 | encode));
	}

	pub fn pmovzxdq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x35, (0xC0 | encode));
	}

	pub fn pmovsxbd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x21, (0xC0 | encode));
	}

	pub fn pmovzxbd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x31, (0xC0 | encode));
	}

	pub fn pmovsxbq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x22, (0xC0 | encode));
	}

	pub fn pmovsxwd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x23, (0xC0 | encode));
	}

	pub fn vpmovzxbw(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
	assert(VM_Version::supports_avx(), "");
	InstructionMark im(this);
	assert(dst != xnoreg, "sanity");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_HVM, /* input_size_in_bits */ EVEX_NObit);
	vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8(0x30);
	emit_operand(dst, src, 0);
	}

	pub fn vpmovzxbw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_128bit? VM_Version::supports_avx() :
	vector_len == AVX_256bit? VM_Version::supports_avx2() :
	vector_len == AVX_512bit? VM_Version::supports_avx512bw() : 0, "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x30, (unsigned char) (0xC0 | encode));
	}

	pub fn vpmovsxbw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_128bit? VM_Version::supports_avx() :
	vector_len == AVX_256bit? VM_Version::supports_avx2() :
	vector_len == AVX_512bit? VM_Version::supports_avx512bw() : 0, "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x20, (0xC0 | encode));
	}

	pub fn evpmovzxbw(&mut self,XMMdst: GPRegister, KRegister mask, src: Address, int vector_len) {
	assert(VM_Version::supports_avx512vlbw(), "");
	assert(dst != xnoreg, "sanity");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_HVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.set_embedded_opmask_register_specifier(mask);
	attributes.set_is_evex_instruction();
	vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8(0x30);
	emit_operand(dst, src, 0);
	}

	pub fn evpmovzxbd(&mut self,XMMdst: GPRegister, KRegister mask, src: Address, int vector_len) {
	assert(VM_Version::supports_avx512vl(), "");
	assert(dst != xnoreg, "sanity");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_HVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.set_embedded_opmask_register_specifier(mask);
	attributes.set_is_evex_instruction();
	vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8(0x31);
	emit_operand(dst, src, 0);
	}

	pub fn evpmovzxbd(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
	evpmovzxbd(dst, k0, src, vector_len);
	}

	pub fn evpandd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
	assert(VM_Version::supports_evex(), "");
	// Encoding: EVEX.NDS.XXX.66.0F.W0 DB /r
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	attributes.set_embedded_opmask_register_specifier(mask);
	if (merge) {
	attributes.reset_is_clear_context();
	}
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0xDB, (0xC0 | encode));
	}

	pub fn vpmovzxdq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len > AVX_128bit ? VM_Version::supports_avx2() : VM_Version::supports_avx(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x35, (0xC0 | encode));
	}

	pub fn vpmovzxbd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len > AVX_128bit ? VM_Version::supports_avx2() : VM_Version::supports_avx(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x31, (0xC0 | encode));
	}

	pub fn vpmovzxbq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len > AVX_128bit ? VM_Version::supports_avx2() : VM_Version::supports_avx(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x32, (0xC0 | encode));
	}

	pub fn vpmovsxbd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_128bit ? VM_Version::supports_avx() :
	vector_len == AVX_256bit ? VM_Version::supports_avx2() :
	VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x21, (0xC0 | encode));
	}

	pub fn vpmovsxbq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_128bit ? VM_Version::supports_avx() :
	vector_len == AVX_256bit ? VM_Version::supports_avx2() :
	VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x22, (0xC0 | encode));
	}

	pub fn vpmovsxwd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_128bit ? VM_Version::supports_avx() :
	vector_len == AVX_256bit ? VM_Version::supports_avx2() :
	VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x23, (0xC0 | encode));
	}

	pub fn vpmovsxwq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_128bit ? VM_Version::supports_avx() :
	vector_len == AVX_256bit ? VM_Version::supports_avx2() :
	VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x24, (0xC0 | encode));
	}

	pub fn vpmovsxdq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_128bit ? VM_Version::supports_avx() :
	vector_len == AVX_256bit ? VM_Version::supports_avx2() :
	VM_Version::supports_evex(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x25, (0xC0 | encode));
	}

	pub fn evpmovwb(&mut self,dst: Address, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx512vlbw(), "");
	assert(src != xnoreg, "sanity");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_HVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.set_is_evex_instruction();
	vex_prefix(dst, 0, src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
	emit_int8(0x30);
	emit_operand(src, dst, 0);
	}

	pub fn evpmovwb(&mut self,dst: Address, KRegister mask, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx512vlbw(), "");
	assert(src != xnoreg, "sanity");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_HVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.reset_is_clear_context();
	attributes.set_embedded_opmask_register_specifier(mask);
	attributes.set_is_evex_instruction();
	vex_prefix(dst, 0, src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
	emit_int8(0x30);
	emit_operand(src, dst, 0);
	}

	pub fn evpmovdb(&mut self,dst: Address, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_evex(), "");
	assert(src != xnoreg, "sanity");
	InstructionMark im(this);
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_QVM, /* input_size_in_bits */ EVEX_NObit);
	attributes.set_is_evex_instruction();
	vex_prefix(dst, 0, src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
	emit_int8(0x31);
	emit_operand(src, dst, 0);
	}

	pub fn vpmovzxwd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_128bit? VM_Version::supports_avx() :
	vector_len == AVX_256bit? VM_Version::supports_avx2() :
	vector_len == AVX_512bit? VM_Version::supports_evex() : 0, " ");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x33, (0xC0 | encode));
	}

	pub fn vpmovzxwq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_128bit? VM_Version::supports_avx() :
	vector_len == AVX_256bit? VM_Version::supports_avx2() :
	vector_len == AVX_512bit? VM_Version::supports_evex() : 0, " ");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x34, (0xC0 | encode));
	}

	pub fn pmaddwd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0xF5, (0xC0 | encode));
	}

	pub fn vpmaddwd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_128bit ? VM_Version::supports_avx() :
	(vector_len == AVX_256bit ? VM_Version::supports_avx2() :
	(vector_len == AVX_512bit ? VM_Version::supports_evex() : 0)), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, nds, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16((unsigned char)0xF5, (0xC0 | encode));
	}

	pub fn vpmaddubsw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister1, XMMsrc: GPRegister2, int vector_len) {
	assert(vector_len == AVX_128bit? VM_Version::supports_avx() :
	vector_len == AVX_256bit? VM_Version::supports_avx2() :
	vector_len == AVX_512bit? VM_Version::supports_avx512bw() : 0, "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, src1, src2, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x04, (0xC0 | encode));
	}

	pub fn evpmadd52luq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister1, XMMsrc: GPRegister2, int vector_len) {
	evpmadd52luq(dst, k0, src1, src2, false, vector_len);
	}

	pub fn evpmadd52luq(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister1, XMMsrc: GPRegister2, bool merge, int vector_len) {
	assert(VM_Version::supports_avx512ifma(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	attributes.set_embedded_opmask_register_specifier(mask);
	if (merge) {
	attributes.reset_is_clear_context();
	}

	int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16((unsigned char)0xB4, (0xC0 | encode));
	}

	pub fn evpmadd52huq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister1, XMMsrc: GPRegister2, int vector_len) {
	evpmadd52huq(dst, k0, src1, src2, false, vector_len);
	}

	pub fn evpmadd52huq(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister1, XMMsrc: GPRegister2, bool merge, int vector_len) {
	assert(VM_Version::supports_avx512ifma(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	attributes.set_embedded_opmask_register_specifier(mask);
	if (merge) {
	attributes.reset_is_clear_context();
	}

	int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16((unsigned char)0xB5, (0xC0 | encode));
	}

	pub fn evpdpwssd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_evex(), "");
	assert(VM_Version::supports_avx512_vnni(), "must support vnni");
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x52, (0xC0 | encode));
	}

// generic
	pub fn pop(&mut self,dst: GPRegister) {
	int encode = prefix_and_encode(dst->encoding());
	emit_int8(0x58 | encode);
	}

	pub fn popcntl(&mut self,dst: GPRegister, src: Address) {
	assert(VM_Version::supports_popcnt(), "must support");
	InstructionMark im(this);
	emit_int8((unsigned char)0xF3);
	prefix(src, dst);
	emit_int16(0x0F, (unsigned char)0xB8);
	emit_operand(dst, src, 0);
	}

	pub fn popcntl(&mut self,dst: GPRegister, src: GPRegister) {
	assert(VM_Version::supports_popcnt(), "must support");
	emit_int8((unsigned char)0xF3);
	int encode = prefix_and_encode(dst->encoding(), src->encoding());
	emit_int24(0x0F, (unsigned char)0xB8, (0xC0 | encode));
	}

	pub fn evpopcntb(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
	assert(VM_Version::supports_avx512_bitalg(), "must support avx512bitalg feature");
	assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_embedded_opmask_register_specifier(mask);
	attributes.set_is_evex_instruction();
	if (merge) {
	attributes.reset_is_clear_context();
	}
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x54, (0xC0 | encode));
	}

	pub fn evpopcntw(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
	assert(VM_Version::supports_avx512_bitalg(), "must support avx512bitalg feature");
	assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
	InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	attributes.set_embedded_opmask_register_specifier(mask);
	if (merge) {
	attributes.reset_is_clear_context();
	}
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x54, (0xC0 | encode));
	}

	pub fn evpopcntd(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
	assert(VM_Version::supports_avx512_vpopcntdq(), "must support vpopcntdq feature");
	assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	attributes.set_embedded_opmask_register_specifier(mask);
	if (merge) {
	attributes.reset_is_clear_context();
	}
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x55, (0xC0 | encode));
	}

	pub fn evpopcntq(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
	assert(VM_Version::supports_avx512_vpopcntdq(), "must support vpopcntdq feature");
	assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
	InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	attributes.set_embedded_opmask_register_specifier(mask);
	if (merge) {
	attributes.reset_is_clear_context();
	}
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x55, (0xC0 | encode));
	}

	pub fn popf(&mut self,) {
	emit_int8((unsigned char)0x9D);
	}

	#ifndef _LP64 // no 32bit push/pop on amd64
	pub fn popl(&mut self,dst: Address) {
	// NOTE: this will adjust stack by 8byte on 64bits
	InstructionMark im(this);
	prefix(dst);
	emit_int8((unsigned char)0x8F);
	emit_operand(rax, dst, 0);
	}
	#endif

	pub fn prefetchnta(&mut self,src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse(), "must support"));
	InstructionMark im(this);
	prefix(src);
	emit_int16(0x0F, 0x18);
	emit_operand(rax, src, 0); // 0, src
	}

	pub fn prefetchr(&mut self,src: Address) {
	assert(VM_Version::supports_3dnow_prefetch(), "must support");
	InstructionMark im(this);
	prefix(src);
	emit_int16(0x0F, 0x0D);
	emit_operand(rax, src, 0); // 0, src
	}

	pub fn prefetcht0(&mut self,src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse(), "must support"));
	InstructionMark im(this);
	prefix(src);
	emit_int16(0x0F, 0x18);
	emit_operand(rcx, src, 0); // 1, src
	}

	pub fn prefetcht1(&mut self,src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse(), "must support"));
	InstructionMark im(this);
	prefix(src);
	emit_int16(0x0F, 0x18);
	emit_operand(rdx, src, 0); // 2, src
	}

	pub fn prefetcht2(&mut self,src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse(), "must support"));
	InstructionMark im(this);
	prefix(src);
	emit_int16(0x0F, 0x18);
	emit_operand(rbx, src, 0); // 3, src
	}

	pub fn prefetchw(&mut self,src: Address) {
	assert(VM_Version::supports_3dnow_prefetch(), "must support");
	InstructionMark im(this);
	prefix(src);
	emit_int16(0x0F, 0x0D);
	emit_operand(rcx, src, 0); // 1, src
	}

	pub fn prefix(&mut self,Prefix p) {
	emit_int8(p);
	}

	pub fn pshufb(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_ssse3(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x00, (0xC0 | encode));
	}

	pub fn evpshufb(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
	assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	attributes.set_embedded_opmask_register_specifier(mask);
	if (merge) {
	attributes.reset_is_clear_context();
	}
	int encode = simd_prefix_and_encode(dst, nds, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x00, (0xC0 | encode));
	}

	pub fn vpshufb(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_128bit? VM_Version::supports_avx() :
	vector_len == AVX_256bit? VM_Version::supports_avx2() :
	vector_len == AVX_512bit? VM_Version::supports_avx512bw() : 0, "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, nds, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x00, (0xC0 | encode));
	}

	pub fn pshufb(&mut self,XMMdst: GPRegister, src: Address) {
	assert(VM_Version::supports_ssse3(), "");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	simd_prefix(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8(0x00);
	emit_operand(dst, src, 0);
	}

	pub fn pshufd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int mode) {
	assert(isByte(mode), "invalid value");
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	int vector_len = VM_Version::supports_avx512novl() ? AVX_512bit : AVX_128bit;
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int24(0x70, (0xC0 | encode), mode & 0xFF);
	}

	pub fn vpshufd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int mode, int vector_len) {
	assert(vector_len == AVX_128bit? VM_Version::supports_avx() :
	(vector_len == AVX_256bit? VM_Version::supports_avx2() :
	(vector_len == AVX_512bit? VM_Version::supports_evex() : 0)), "");
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int24(0x70, (0xC0 | encode), mode & 0xFF);
	}

	pub fn pshufd(&mut self,XMMdst: GPRegister, src: Address, int mode) {
	assert(isByte(mode), "invalid value");
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	assert((UseAVX > 0), "SSE mode requires address alignment 16 bytes");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int8(0x70);
	emit_operand(dst, src, 1);
	emit_int8(mode & 0xFF);
	}

	pub fn pshufhw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int mode) {
	assert(isByte(mode), "invalid value");
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int24(0x70, (0xC0 | encode), mode & 0xFF);
	}

	pub fn vpshufhw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int mode, int vector_len) {
	assert(vector_len == AVX_128bit ? VM_Version::supports_avx() :
	(vector_len == AVX_256bit ? VM_Version::supports_avx2() :
	(vector_len == AVX_512bit ? VM_Version::supports_avx512bw() : false)), "");
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int24(0x70, (0xC0 | encode), mode & 0xFF);
	}

	pub fn pshuflw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int mode) {
	assert(isByte(mode), "invalid value");
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int24(0x70, (0xC0 | encode), mode & 0xFF);
	}

	pub fn pshuflw(&mut self,XMMdst: GPRegister, src: Address, int mode) {
	assert(isByte(mode), "invalid value");
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	assert((UseAVX > 0), "SSE mode requires address alignment 16 bytes");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	simd_prefix(dst, xnoreg, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int8(0x70);
	emit_operand(dst, src, 1);
	emit_int8(mode & 0xFF);
	}

	pub fn vpshuflw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int mode, int vector_len) {
	assert(vector_len == AVX_128bit ? VM_Version::supports_avx() :
	(vector_len == AVX_256bit ? VM_Version::supports_avx2() :
	(vector_len == AVX_512bit ? VM_Version::supports_avx512bw() : false)), "");
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int24(0x70, (0xC0 | encode), mode & 0xFF);
	}

	pub fn evshufi64x2(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, imm8: i8, int vector_len) {
	assert(VM_Version::supports_evex(), "requires EVEX support");
	assert(vector_len == Assembler::AVX_256bit || vector_len == Assembler::AVX_512bit, "");
	InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x43, (0xC0 | encode), imm8 & 0xFF);
	}

	pub fn shufpd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, imm8: i8) {
	assert(isByte(imm8), "invalid value");
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int24((unsigned char)0xC6, (0xC0 | encode), imm8 & 0xFF);
	}

	pub fn vshufpd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, imm8: i8, int vector_len) {
	InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_rex_vex_w_reverted();
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int24((unsigned char)0xC6, (0xC0 | encode), imm8 & 0xFF);
	}

	pub fn shufps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, imm8: i8) {
	assert(isByte(imm8), "invalid value");
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int24((unsigned char)0xC6, (0xC0 | encode), imm8 & 0xFF);
	}

	pub fn vshufps(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, imm8: i8, int vector_len) {
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int24((unsigned char)0xC6, (0xC0 | encode), imm8 & 0xFF);
	}

	pub fn psrldq(&mut self,XMMdst: GPRegister, int shift) {
	// Shift left 128 bit value in dst XMMRegister by shift number of bytes.
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(xmm3, dst, dst, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int24(0x73, (0xC0 | encode), shift);
	}

	pub fn vpsrldq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int shift, int vector_len) {
	assert(vector_len == AVX_128bit ? VM_Version::supports_avx() :
	vector_len == AVX_256bit ? VM_Version::supports_avx2() :
	vector_len == AVX_512bit ? VM_Version::supports_avx512bw() : 0, "");
	InstructionAttr attributes(vector_len, /*vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(xmm3->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int24(0x73, (0xC0 | encode), shift & 0xFF);
	}

	pub fn pslldq(&mut self,XMMdst: GPRegister, int shift) {
	// Shift left 128 bit value in dst XMMRegister by shift number of bytes.
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	// XMM7 is for /7 encoding: 66 0F 73 /7 ib
	int encode = simd_prefix_and_encode(xmm7, dst, dst, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int24(0x73, (0xC0 | encode), shift);
	}

	pub fn vpslldq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int shift, int vector_len) {
	assert(vector_len == AVX_128bit ? VM_Version::supports_avx() :
	vector_len == AVX_256bit ? VM_Version::supports_avx2() :
	vector_len == AVX_512bit ? VM_Version::supports_avx512bw() : 0, "");
	InstructionAttr attributes(vector_len, /*vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = vex_prefix_and_encode(xmm7->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int24(0x73, (0xC0 | encode), shift & 0xFF);
	}

	pub fn ptest(&mut self,XMMdst: GPRegister, src: Address) {
	assert(VM_Version::supports_sse4_1(), "");
	assert((UseAVX > 0), "SSE mode requires address alignment 16 bytes");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	simd_prefix(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8(0x17);
	emit_operand(dst, src, 0);
	}

	pub fn ptest(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sse4_1() || VM_Version::supports_avx(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8(0x17);
	emit_int8((0xC0 | encode));
	}

	pub fn vptest(&mut self,XMMdst: GPRegister, src: Address) {
	assert(VM_Version::supports_avx(), "");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_256bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	assert(dst != xnoreg, "sanity");
	// swap src<->dst for encoding
	vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int8(0x17);
	emit_operand(dst, src, 0);
	}

	pub fn vptest(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_avx(), "");
	InstructionAttr attributes(AVX_256bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x17, (0xC0 | encode));
	}

	pub fn vptest(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x17, (0xC0 | encode));
	}

	pub fn vtestps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
	assert(VM_Version::supports_avx(), "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x0E, (0xC0 | encode));
	}

	pub fn evptestmb(&mut self,Kdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_512bit ? VM_Version::supports_avx512bw() : VM_Version::supports_avx512vlbw(), "");
	// Encoding: EVEX.NDS.XXX.66.0F38.W0 DB /r
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x26, (0xC0 | encode));
	}

	pub fn evptestmd(&mut self,Kdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_512bit ? VM_Version::supports_evex() : VM_Version::supports_avx512vl(), "");
	// Encoding: EVEX.NDS.XXX.66.0F38.W0 DB /r
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x27, (0xC0 | encode));
	}

	pub fn evptestnmd(&mut self,Kdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
	assert(vector_len == AVX_512bit ? VM_Version::supports_evex() : VM_Version::supports_avx512vl(), "");
	// Encoding: EVEX.NDS.XXX.F3.0F38.W0 DB /r
	InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
	emit_int16(0x27, (0xC0 | encode));
	}

	pub fn punpcklbw(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	assert((UseAVX > 0), "SSE mode requires address alignment 16 bytes");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_vlbw, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
	simd_prefix(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int8(0x60);
	emit_operand(dst, src, 0);
	}

	pub fn punpcklbw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_vlbw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x60, (0xC0 | encode));
	}

	pub fn punpckldq(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	assert((UseAVX > 0), "SSE mode requires address alignment 16 bytes");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int8(0x62);
	emit_operand(dst, src, 0);
	}

	pub fn punpckldq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x62, (0xC0 | encode));
	}

	pub fn punpcklqdq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_rex_vex_w_reverted();
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x6C, (0xC0 | encode));
	}

	pub fn evpunpcklqdq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister1, XMMsrc: GPRegister2, int vector_len) {
	evpunpcklqdq(dst, k0, src1, src2, false, vector_len);
	}

	pub fn evpunpcklqdq(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister1, XMMsrc: GPRegister2, bool merge, int vector_len) {
	assert(VM_Version::supports_evex(), "requires AVX512F");
	assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires AVX512VL");
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	attributes.set_embedded_opmask_register_specifier(mask);
	if (merge) {
	attributes.reset_is_clear_context();
	}

	int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x6C, (0xC0 | encode));
	}

	pub fn evpunpckhqdq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister1, XMMsrc: GPRegister2, int vector_len) {
	evpunpckhqdq(dst, k0, src1, src2, false, vector_len);
	}

	pub fn evpunpckhqdq(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister1, XMMsrc: GPRegister2, bool merge, int vector_len) {
	assert(VM_Version::supports_evex(), "requires AVX512F");
	assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires AVX512VL");
	InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	attributes.set_embedded_opmask_register_specifier(mask);
	if (merge) {
	attributes.reset_is_clear_context();
	}

	int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x6D, (0xC0 | encode));
	}

	pub fn push(&mut self,imm32: i32) {
	// in 64bits we push 64bits onto the stack but only
	// take a 32bit immediate
	emit_int8(0x68);
	emit_int32(imm32);
	}

	pub fn push(&mut self,src: GPRegister) {
	int encode = prefix_and_encode(src->encoding());
	emit_int8(0x50 | encode);
	}

	pub fn pushf(&mut self,) {
	emit_int8((unsigned char)0x9C);
	}

	#ifndef _LP64 // no 32bit push/pop on amd64
	pub fn pushl(&mut self,src: Address) {
	// Note this will push 64bit on 64bit
	InstructionMark im(this);
	prefix(src);
	emit_int8((unsigned char)0xFF);
	emit_operand(rsi, src, 0);
	}
	#endif

	pub fn rcll(&mut self,dst: GPRegister, imm8: i8) {
	assert(isShiftCount(imm8), "illegal shift count");
	int encode = prefix_and_encode(dst->encoding());
	if (imm8 == 1) {
	emit_int16((unsigned char)0xD1, (0xD0 | encode));
	} else {
	emit_int24((unsigned char)0xC1, (0xD0 | encode), imm8);
	}
	}

	pub fn rcpps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16(0x53, (0xC0 | encode));
	}

	pub fn rcpss(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16(0x53, (0xC0 | encode));
	}

	pub fn rdtsc(&mut self,) {
	emit_int16(0x0F, 0x31);
	}

// copies data from [esi] to [edi] using rcx pointer sized words
// generic
	pub fn rep_mov(&mut self,) {
	// REP
	// MOVSQ
	LP64_ONLY(emit_int24((unsigned char)0xF3, REX_W, (unsigned char)0xA5);)
	NOT_LP64( emit_int16((unsigned char)0xF3,        (unsigned char)0xA5);)
	}

// sets rcx bytes with rax, value at [edi]
	pub fn rep_stosb(&mut self,) {
	// REP
	// STOSB
	LP64_ONLY(emit_int24((unsigned char)0xF3, REX_W, (unsigned char)0xAA);)
	NOT_LP64( emit_int16((unsigned char)0xF3,        (unsigned char)0xAA);)
	}

// sets rcx pointer sized words with rax, value at [edi]
// generic
	pub fn rep_stos(&mut self,) {
	// REP
	// LP64:STOSQ, LP32:STOSD
	LP64_ONLY(emit_int24((unsigned char)0xF3, REX_W, (unsigned char)0xAB);)
	NOT_LP64( emit_int16((unsigned char)0xF3,        (unsigned char)0xAB);)
	}

// scans rcx pointer sized words at [edi] for occurrence of rax,
// generic
	pub fn repne_scan(&mut self,) { // repne_scan
	// SCASQ
	LP64_ONLY(emit_int24((unsigned char)0xF2, REX_W, (unsigned char)0xAF);)
	NOT_LP64( emit_int16((unsigned char)0xF2,        (unsigned char)0xAF);)
	}

	#ifdef _LP64
// scans rcx 4 byte words at [edi] for occurrence of rax,
// generic
	pub fn repne_scanl(&mut self,) { // repne_scan
	// SCASL
	emit_int16((unsigned char)0xF2, (unsigned char)0xAF);
	}
	#endif

	pub fn ret(&mut self,imm16: i16) {
	if (imm16 == 0) {
	emit_int8((unsigned char)0xC3);
	} else {
	emit_int8((unsigned char)0xC2);
	emit_int16(imm16);
	}
	}

	pub fn roll(&mut self,dst: GPRegister, imm8: i8) {
	assert(isShiftCount(imm8), "illegal shift count");
	int encode = prefix_and_encode(dst->encoding());
	if (imm8 == 1) {
	emit_int16((unsigned char)0xD1, (0xC0 | encode));
	} else {
	emit_int24((unsigned char)0xC1, (0xc0 | encode), imm8);
	}
	}

	pub fn roll(&mut self,dst: GPRegister) {
	int encode = prefix_and_encode(dst->encoding());
	emit_int16((unsigned char)0xD3, (0xC0 | encode));
	}

	pub fn rorl(&mut self,dst: GPRegister, imm8: i8) {
	assert(isShiftCount(imm8), "illegal shift count");
	int encode = prefix_and_encode(dst->encoding());
	if (imm8 == 1) {
	emit_int16((unsigned char)0xD1, (0xC8 | encode));
	} else {
	emit_int24((unsigned char)0xC1, (0xc8 | encode), imm8);
	}
	}

	pub fn rorl(&mut self,dst: GPRegister) {
	int encode = prefix_and_encode(dst->encoding());
	emit_int16((unsigned char)0xD3, (0xC8 | encode));
	}

	#ifdef _LP64
	pub fn rorq(&mut self,dst: GPRegister) {
	int encode = prefixq_and_encode(dst->encoding());
	emit_int16((unsigned char)0xD3, (0xC8 | encode));
	}

	pub fn rorq(&mut self,dst: GPRegister, imm8: i8) {
	assert(isShiftCount(imm8 >> 1), "illegal shift count");
	int encode = prefixq_and_encode(dst->encoding());
	if (imm8 == 1) {
	emit_int16((unsigned char)0xD1, (0xC8 | encode));
	} else {
	emit_int24((unsigned char)0xC1, (0xc8 | encode), imm8);
	}
	}

	pub fn rolq(&mut self,dst: GPRegister) {
	int encode = prefixq_and_encode(dst->encoding());
	emit_int16((unsigned char)0xD3, (0xC0 | encode));
	}

	pub fn rolq(&mut self,dst: GPRegister, imm8: i8) {
	assert(isShiftCount(imm8 >> 1), "illegal shift count");
	int encode = prefixq_and_encode(dst->encoding());
	if (imm8 == 1) {
	emit_int16((unsigned char)0xD1, (0xC0 | encode));
	} else {
	emit_int24((unsigned char)0xC1, (0xc0 | encode), imm8);
	}
	}
	#endif

	pub fn sahf(&mut self,) {
	#ifdef _LP64
	// Not supported in 64bit mode
	ShouldNotReachHere();
	#endif
	emit_int8((unsigned char)0x9E);
	}

	pub fn sall(&mut self,dst: Address, imm8: i8) {
	InstructionMark im(this);
	assert(isShiftCount(imm8), "illegal shift count");
	prefix(dst);
	if (imm8 == 1) {
	emit_int8((unsigned char)0xD1);
	emit_operand(as_Register(4), dst, 0);
	}
	else {
	emit_int8((unsigned char)0xC1);
	emit_operand(as_Register(4), dst, 1);
	emit_int8(imm8);
	}
	}

	pub fn sall(&mut self,dst: Address) {
	InstructionMark im(this);
	prefix(dst);
	emit_int8((unsigned char)0xD3);
	emit_operand(as_Register(4), dst, 0);
	}

	pub fn sall(&mut self,dst: GPRegister, imm8: i8) {
	assert(isShiftCount(imm8), "illegal shift count");
	int encode = prefix_and_encode(dst->encoding());
	if (imm8 == 1) {
	emit_int16((unsigned char)0xD1, (0xE0 | encode));
	} else {
	emit_int24((unsigned char)0xC1, (0xE0 | encode), imm8);
	}
	}

	pub fn sall(&mut self,dst: GPRegister) {
	int encode = prefix_and_encode(dst->encoding());
	emit_int16((unsigned char)0xD3, (0xE0 | encode));
	}

	pub fn sarl(&mut self,dst: Address, imm8: i8) {
	assert(isShiftCount(imm8), "illegal shift count");
	InstructionMark im(this);
	prefix(dst);
	if (imm8 == 1) {
	emit_int8((unsigned char)0xD1);
	emit_operand(as_Register(7), dst, 0);
	}
	else {
	emit_int8((unsigned char)0xC1);
	emit_operand(as_Register(7), dst, 1);
	emit_int8(imm8);
	}
	}

	pub fn sarl(&mut self,dst: Address) {
	InstructionMark im(this);
	prefix(dst);
	emit_int8((unsigned char)0xD3);
	emit_operand(as_Register(7), dst, 0);
	}

	pub fn sarl(&mut self,dst: GPRegister, imm8: i8) {
	int encode = prefix_and_encode(dst->encoding());
	assert(isShiftCount(imm8), "illegal shift count");
	if (imm8 == 1) {
	emit_int16((unsigned char)0xD1, (0xF8 | encode));
	} else {
	emit_int24((unsigned char)0xC1, (0xF8 | encode), imm8);
	}
	}

	pub fn sarl(&mut self,dst: GPRegister) {
	int encode = prefix_and_encode(dst->encoding());
	emit_int16((unsigned char)0xD3, (0xF8 | encode));
	}

	pub fn sbbl(&mut self,dst: Address, imm32: i32) {
	InstructionMark im(this);
	prefix(dst);
	emit_arith_operand(0x81, rbx, dst, imm32);
	}

	pub fn sbbl(&mut self,dst: GPRegister, imm32: i32) {
	prefix(dst);
	emit_arith(0x81, 0xD8, dst, imm32);
	}


	pub fn sbbl(&mut self,dst: GPRegister, src: Address) {
	InstructionMark im(this);
	prefix(src, dst);
	emit_int8(0x1B);
	emit_operand(dst, src, 0);
	}

	pub fn sbbl(&mut self,dst: GPRegister, src: GPRegister) {
	(void) prefix_and_encode(dst->encoding(), src->encoding());
	emit_arith(0x1B, 0xC0, dst, src);
	}

	pub fn setb(&mut self,Condition cc, dst: GPRegister) {
	assert(0 <= cc && cc < 16, "illegal cc");
	int encode = prefix_and_encode(dst->encoding(), true);
	emit_int24(0x0F, (unsigned char)0x90 | cc, (0xC0 | encode));
	}

	pub fn palignr(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_ssse3(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x0F, (0xC0 | encode), imm8);
	}

	pub fn vpalignr(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, imm8: i8, int vector_len) {
	assert(vector_len == AVX_128bit? VM_Version::supports_avx() :
	vector_len == AVX_256bit? VM_Version::supports_avx2() :
	0, "");
	InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
	int encode = simd_prefix_and_encode(dst, nds, src, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x0F, (0xC0 | encode), imm8);
	}

	pub fn evalignq(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, uint8_t imm8) {
	assert(VM_Version::supports_evex(), "");
	InstructionAttr attributes(AVX_512bit, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
	attributes.set_is_evex_instruction();
	int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x3, (0xC0 | encode), imm8);
	}

	pub fn pblendw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x0E, (0xC0 | encode), imm8);
	}

	pub fn sha1rnds4(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, imm8: i8) {
	assert(VM_Version::supports_sha(), "");
	int encode = rex_prefix_and_encode(dst->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_3A, /* rex_w */ false);
	emit_int24((unsigned char)0xCC, (0xC0 | encode), (unsigned char)imm8);
	}

	pub fn sha1nexte(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sha(), "");
	int encode = rex_prefix_and_encode(dst->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, /* rex_w */ false);
	emit_int16((unsigned char)0xC8, (0xC0 | encode));
	}

	pub fn sha1msg1(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sha(), "");
	int encode = rex_prefix_and_encode(dst->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, /* rex_w */ false);
	emit_int16((unsigned char)0xC9, (0xC0 | encode));
	}

	pub fn sha1msg2(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sha(), "");
	int encode = rex_prefix_and_encode(dst->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, /* rex_w */ false);
	emit_int16((unsigned char)0xCA, (0xC0 | encode));
	}

// xmm0 is implicit additional source to this instruction.
	pub fn sha256rnds2(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sha(), "");
	int encode = rex_prefix_and_encode(dst->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, /* rex_w */ false);
	emit_int16((unsigned char)0xCB, (0xC0 | encode));
	}

	pub fn sha256msg1(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sha(), "");
	int encode = rex_prefix_and_encode(dst->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, /* rex_w */ false);
	emit_int16((unsigned char)0xCC, (0xC0 | encode));
	}

	pub fn sha256msg2(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	assert(VM_Version::supports_sha(), "");
	int encode = rex_prefix_and_encode(dst->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, /* rex_w */ false);
	emit_int16((unsigned char)0xCD, (0xC0 | encode));
	}


	pub fn shll(&mut self,dst: GPRegister, imm8: i8) {
	assert(isShiftCount(imm8), "illegal shift count");
	int encode = prefix_and_encode(dst->encoding());
	if (imm8 == 1 ) {
	emit_int16((unsigned char)0xD1, (0xE0 | encode));
	} else {
	emit_int24((unsigned char)0xC1, (0xE0 | encode), imm8);
	}
	}

	pub fn shll(&mut self,dst: GPRegister) {
	int encode = prefix_and_encode(dst->encoding());
	emit_int16((unsigned char)0xD3, (0xE0 | encode));
	}

	pub fn shrl(&mut self,dst: GPRegister, imm8: i8) {
	assert(isShiftCount(imm8), "illegal shift count");
	int encode = prefix_and_encode(dst->encoding());
	if (imm8 == 1) {
	emit_int16((unsigned char)0xD1, (0xE8 | encode));
	}
	else {
	emit_int24((unsigned char)0xC1, (0xE8 | encode), imm8);
	}
	}

	pub fn shrl(&mut self,dst: GPRegister) {
	int encode = prefix_and_encode(dst->encoding());
	emit_int16((unsigned char)0xD3, (0xE8 | encode));
	}

	pub fn shrl(&mut self,dst: Address) {
	InstructionMark im(this);
	prefix(dst);
	emit_int8((unsigned char)0xD3);
	emit_operand(as_Register(5), dst, 0);
	}

	pub fn shrl(&mut self,dst: Address, imm8: i8) {
	InstructionMark im(this);
	assert(isShiftCount(imm8), "illegal shift count");
	prefix(dst);
	if (imm8 == 1) {
	emit_int8((unsigned char)0xD1);
	emit_operand(as_Register(5), dst, 0);
	}
	else {
	emit_int8((unsigned char)0xC1);
	emit_operand(as_Register(5), dst, 1);
	emit_int8(imm8);
	}
	}


	pub fn shldl(&mut self,dst: GPRegister, src: GPRegister) {
	int encode = prefix_and_encode(src->encoding(), dst->encoding());
	emit_int24(0x0F, (unsigned char)0xA5, (0xC0 | encode));
	}

	pub fn shldl(&mut self,dst: GPRegister, src: GPRegister, int8_t imm8) {
	int encode = prefix_and_encode(src->encoding(), dst->encoding());
	emit_int32(0x0F, (unsigned char)0xA4, (0xC0 | encode), imm8);
	}

	pub fn shrdl(&mut self,dst: GPRegister, src: GPRegister) {
	int encode = prefix_and_encode(src->encoding(), dst->encoding());
	emit_int24(0x0F, (unsigned char)0xAD, (0xC0 | encode));
	}

	pub fn shrdl(&mut self,dst: GPRegister, src: GPRegister, int8_t imm8) {
	int encode = prefix_and_encode(src->encoding(), dst->encoding());
	emit_int32(0x0F, (unsigned char)0xAC, (0xC0 | encode), imm8);
	}

	#ifdef _LP64
	pub fn shldq(&mut self,dst: GPRegister, src: GPRegister, int8_t imm8) {
	int encode = prefixq_and_encode(src->encoding(), dst->encoding());
	emit_int32(0x0F, (unsigned char)0xA4, (0xC0 | encode), imm8);
	}

	pub fn shrdq(&mut self,dst: GPRegister, src: GPRegister, int8_t imm8) {
	int encode = prefixq_and_encode(src->encoding(), dst->encoding());
	emit_int32(0x0F, (unsigned char)0xAC, (0xC0 | encode), imm8);
	}
	#endif

// copies a single word from [esi] to [edi]
	pub fn smovl(&mut self,) {
	emit_int8((unsigned char)0xA5);
	}

	pub fn roundsd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, rmode: i32) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int24(0x0B, (0xC0 | encode), (unsigned char)rmode);
	}

	pub fn roundsd(&mut self,XMMdst: GPRegister, src: Address, rmode: i32) {
	assert(VM_Version::supports_sse4_1(), "");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	simd_prefix(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
	emit_int8(0x0B);
	emit_operand(dst, src, 1);
	emit_int8((unsigned char)rmode);
	}

	pub fn sqrtsd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_rex_vex_w_reverted();
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int16(0x51, (0xC0 | encode));
	}

	pub fn sqrtsd(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
	attributes.set_rex_vex_w_reverted();
	simd_prefix(dst, dst, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int8(0x51);
	emit_operand(dst, src, 0);
	}

	pub fn sqrtss(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16(0x51, (0xC0 | encode));
	}

	pub fn std(&mut self,) {
	emit_int8((unsigned char)0xFD);
	}

	pub fn sqrtss(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(dst, dst, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int8(0x51);
	emit_operand(dst, src, 0);
	}

	pub fn stmxcsr(&mut self, dst: Address) {
	if (UseAVX > 0 ) {
	assert(VM_Version::supports_avx(), "");
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
	vex_prefix(dst, 0, 0, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int8((unsigned char)0xAE);
	emit_operand(as_Register(3), dst, 0);
	} else {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionMark im(this);
	prefix(dst);
	emit_int16(0x0F, (unsigned char)0xAE);
	emit_operand(as_Register(3), dst, 0);
	}
	}

	pub fn subl(&mut self,dst: Address, imm32: i32) {
	InstructionMark im(this);
	prefix(dst);
	emit_arith_operand(0x81, rbp, dst, imm32);
	}

	pub fn subl(&mut self,dst: Address, src: GPRegister) {
	InstructionMark im(this);
	prefix(dst, src);
	emit_int8(0x29);
	emit_operand(src, dst, 0);
	}

	pub fn subl(&mut self,dst: GPRegister, imm32: i32) {
	prefix(dst);
	emit_arith(0x81, 0xE8, dst, imm32);
	}

// Force generation of a 4 byte immediate value even if it fits into 8bit
	pub fn subl_imm32(&mut self,dst: GPRegister, imm32: i32) {
	prefix(dst);
	emit_arith_imm32(0x81, 0xE8, dst, imm32);
	}

	pub fn subl(&mut self,dst: GPRegister, src: Address) {
	InstructionMark im(this);
	prefix(src, dst);
	emit_int8(0x2B);
	emit_operand(dst, src, 0);
	}

	pub fn subl(&mut self,dst: GPRegister, src: GPRegister) {
	(void) prefix_and_encode(dst->encoding(), src->encoding());
	emit_arith(0x2B, 0xC0, dst, src);
	}

	pub fn subsd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_rex_vex_w_reverted();
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int16(0x5C, (0xC0 | encode));
	}

	pub fn subsd(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
	attributes.set_rex_vex_w_reverted();
	simd_prefix(dst, dst, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
	emit_int8(0x5C);
	emit_operand(dst, src, 0);
	}

	pub fn subss(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true , /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int16(0x5C, (0xC0 | encode));
	}

	pub fn subss(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(dst, dst, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
	emit_int8(0x5C);
	emit_operand(dst, src, 0);
	}

	pub fn testb(&mut self,dst: GPRegister, imm8: i8) {
	NOT_LP64(assert(dst->has_byte_register(), "must have byte register"));
	if (dst == rax) {
	emit_int8((unsigned char)0xA8);
	emit_int8(imm8);
	} else {
	(void) prefix_and_encode(dst->encoding(), true);
	emit_arith_b(0xF6, 0xC0, dst, imm8);
	}
	}

	pub fn testb(&mut self,dst: Address, imm8: i8) {
	InstructionMark im(this);
	prefix(dst);
	emit_int8((unsigned char)0xF6);
	emit_operand(rax, dst, 1);
	emit_int8(imm8);
	}

	pub fn testl(&mut self,dst: Address, imm32: i32) {
	InstructionMark im(this);
	prefix(dst);
	emit_int8((unsigned char)0xF7);
	emit_operand(as_Register(0), dst, 4);
	emit_int32(imm32);
	}

	pub fn testl(&mut self,dst: GPRegister, imm32: i32) {
	// not using emit_arith because test
	// doesn't support sign-extension of
	// 8bit operands
	if (dst == rax) {
	emit_int8((unsigned char)0xA9);
	emit_int32(imm32);
	} else {
	int encode = dst->encoding();
	encode = prefix_and_encode(encode);
	emit_int16((unsigned char)0xF7, (0xC0 | encode));
	emit_int32(imm32);
	}
	}

	pub fn testl(&mut self,dst: GPRegister, src: GPRegister) {
	(void) prefix_and_encode(dst->encoding(), src->encoding());
	emit_arith(0x85, 0xC0, dst, src);
	}

	pub fn testl(&mut self,dst: GPRegister, src: Address) {
	InstructionMark im(this);
	prefix(src, dst);
	emit_int8((unsigned char)0x85);
	emit_operand(dst, src, 0);
	}

	pub fn tzcntl(&mut self,dst: GPRegister, src: GPRegister) {
	assert(VM_Version::supports_bmi1(), "tzcnt instruction not supported");
	emit_int8((unsigned char)0xF3);
	int encode = prefix_and_encode(dst->encoding(), src->encoding());
	emit_int24(0x0F,
	(unsigned char)0xBC,
	0xC0 | encode);
	}

	pub fn tzcntl(&mut self,dst: GPRegister, src: Address) {
	assert(VM_Version::supports_bmi1(), "tzcnt instruction not supported");
	InstructionMark im(this);
	emit_int8((unsigned char)0xF3);
	prefix(src, dst);
	emit_int16(0x0F, (unsigned char)0xBC);
	emit_operand(dst, src, 0);
	}

	pub fn tzcntq(&mut self,dst: GPRegister, src: GPRegister) {
	assert(VM_Version::supports_bmi1(), "tzcnt instruction not supported");
	emit_int8((unsigned char)0xF3);
	int encode = prefixq_and_encode(dst->encoding(), src->encoding());
	emit_int24(0x0F, (unsigned char)0xBC, (0xC0 | encode));
	}

	pub fn tzcntq(&mut self,dst: GPRegister, src: Address) {
	assert(VM_Version::supports_bmi1(), "tzcnt instruction not supported");
	InstructionMark im(this);
	emit_int8((unsigned char)0xF3);
	prefixq(src, dst);
	emit_int16(0x0F, (unsigned char)0xBC);
	emit_operand(dst, src, 0);
	}

	pub fn ucomisd(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
	attributes.set_rex_vex_w_reverted();
	simd_prefix(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int8(0x2E);
	emit_operand(dst, src, 0);
	}

	pub fn ucomisd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse2(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_rex_vex_w_reverted();
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
	emit_int16(0x2E, (0xC0 | encode));
	}

	pub fn ucomiss(&mut self,XMMdst: GPRegister, src: Address) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionMark im(this);
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
	simd_prefix(dst, xnoreg, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int8(0x2E);
	emit_operand(dst, src, 0);
	}

	pub fn ucomiss(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
	NOT_LP64(assert(VM_Version::supports_sse(), ""));
	InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
	int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
	emit_int16(0x2E, (0xC0 | encode));
	}

	pub fn xabort(&mut self,int8_t imm8) {
	emit_int24((unsigned char)0xC6, (unsigned char)0xF8, (imm8 & 0xFF));
	}

	pub fn xaddb(&mut self,dst: Address, src: GPRegister) {
	InstructionMark im(this);
	prefix(dst, src, true);
	emit_int16(0x0F, (unsigned char)0xC0);
	emit_operand(src, dst, 0);
	}

	pub fn xaddw(&mut self,dst: Address, src: GPRegister) {
	InstructionMark im(this);
	emit_int8(0x66);
	prefix(dst, src);
	emit_int16(0x0F, (unsigned char)0xC1);
	emit_operand(src, dst, 0);
	}

	pub fn xaddl(&mut self,dst: Address, src: GPRegister) {
	InstructionMark im(this);
	prefix(dst, src);
	emit_int16(0x0F, (unsigned char)0xC1);
	emit_operand(src, dst, 0);
	}

	pub fn xbegin(&mut self,Label& abort, relocInfo::relocType rtype) {
	InstructionMark im(this);
	relocate(rtype);
	if (abort.is_bound()) {
	address entry = target(abort);
	assert(entry != nullptr, "abort entry null");
	intptr_t offset = entry - pc();
	emit_int16((unsigned char)0xC7, (unsigned char)0xF8);
	emit_int32(offset - 6); // 2 opcode + 4 address
	} else {
	abort.add_patch_at(code(), locator());
	emit_int16((unsigned char)0xC7, (unsigned char)0xF8);
	emit_int32(0);
	}
	}

	pub fn xchgb(&mut self,dst: GPRegister, src: Address) { // xchg
	InstructionMark im(this);
	prefix(src, dst, true);
	emit_int8((unsigned char)0x86);
	emit_operand(dst, src, 0);
	}

	pub fn xchgw(&mut self,dst: GPRegister, src: Address) { // xchg
	InstructionMark im(this);
	emit_int8(0x66);
	prefix(src, dst);
	emit_int8((unsigned char)0x87);
	emit_operand(dst, src, 0);
	}

	pub fn xchgl(&mut self,dst: GPRegister, src: Address) { // xchg
	InstructionMark im(this);
	prefix(src, dst);
	emit_int8((unsigned char)0x87);
	emit_operand(dst, src, 0);
	}

	pub fn xchgl(&mut self,dst: GPRegister, src: GPRegister) {
	int encode = prefix_and_encode(dst->encoding(), src->encoding());
	emit_int16((unsigned char)0x87, (0xC0 | encode));
	}

	pub fn xend(&mut self,) {
	emit_int24(0x0F, 0x01, (unsigned char)0xD5);
	}

	pub fn xgetbv(&mut self,) {
	emit_int24(0x0F, 0x01, (unsigned char)0xD0);
	}

	pub fn xorl(&mut self,dst: Address, imm32: i32) {
	InstructionMark im(this);
	prefix(dst);
	emit_arith_operand(0x81, as_Register(6), dst, imm32);
	}

	pub fn xorl(&mut self,dst: GPRegister, imm32: i32) {
	prefix(dst);
	emit_arith(0x81, 0xF0, dst, imm32);
	}

	pub fn xorl(&mut self,dst: GPRegister, src: Address) {
	InstructionMark im(this);
	prefix(src, dst);
	emit_int8(0x33);
	emit_operand(dst, src, 0);
	}

	pub fn xorl(&mut self,dst: GPRegister, src: GPRegister) {
	(void) prefix_and_encode(dst->encoding(), src->encoding());
	emit_arith(0x33, 0xC0, dst, src);
	}

	pub fn xorl(&mut self,dst: Address, src: GPRegister) {
	InstructionMark im(this);
	prefix(dst, src);
	emit_int8(0x31);
	emit_operand(src, dst, 0);
	}

	pub fn xorb(&mut self,dst: GPRegister, src: Address) {
	InstructionMark im(this);
	prefix(src, dst);
	emit_int8(0x32);
	emit_operand(dst, src, 0);
	}

	pub fn xorb(&mut self,dst: Address, src: GPRegister) {
	InstructionMark im(this);
	prefix(dst, src, true);
	emit_int8(0x30);
	emit_operand(src, dst, 0);
	}

	pub fn xorw(&mut self,dst: GPRegister, src: GPRegister) {
	(void)prefix_and_encode(dst->encoding(), src->encoding());
	emit_arith(0x33, 0xC0, dst, src);
	}
}

// AVX 3-operands scalar float-point arithmetic instructions
impl Assembler {
	pub fn vaddsd(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
		attributes.set_rex_vex_w_reverted();
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
		emit_int8(0x58);
		emit_operand(dst, src, 0);
	}

	pub fn vaddsd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
		emit_int16(0x58, (0xC0 | encode));
	}

	pub fn vaddss(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
		emit_int8(0x58);
		emit_operand(dst, src, 0);
	}

	pub fn vaddss(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
		emit_int16(0x58, (0xC0 | encode));
	}

	pub fn vdivsd(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
		attributes.set_rex_vex_w_reverted();
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
		emit_int8(0x5E);
		emit_operand(dst, src, 0);
	}

	pub fn vdivsd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5E, (0xC0 | encode));
	}

	pub fn vdivss(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
		emit_int8(0x5E);
		emit_operand(dst, src, 0);
	}

	pub fn vdivss(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5E, (0xC0 | encode));
	}

	pub fn vfmadd231sd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister1, XMMsrc: GPRegister2) {
		assert(VM_Version::supports_fma(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xB9, (0xC0 | encode));
	}

	pub fn vfmadd231ss(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister1, XMMsrc: GPRegister2) {
		assert(VM_Version::supports_fma(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xB9, (0xC0 | encode));
	}

	pub fn vmulsd(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
		attributes.set_rex_vex_w_reverted();
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
		emit_int8(0x59);
		emit_operand(dst, src, 0);
	}

	pub fn vmulsd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
		emit_int16(0x59, (0xC0 | encode));
	}

	pub fn vmulss(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
		emit_int8(0x59);
		emit_operand(dst, src, 0);
	}

	pub fn vmulss(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
		emit_int16(0x59, (0xC0 | encode));
	}

	pub fn vsubsd(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
		attributes.set_rex_vex_w_reverted();
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
		emit_int8(0x5C);
		emit_operand(dst, src, 0);
	}

	pub fn vsubsd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5C, (0xC0 | encode));
	}

	pub fn vsubss(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
		emit_int8(0x5C);
		emit_operand(dst, src, 0);
	}

	pub fn vsubss(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5C, (0xC0 | encode));
	}
}

//====================VECTOR ARITHMETIC=====================================
impl Assembler {
// Float-point vector arithmetic

	pub fn addpd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x58, (0xC0 | encode));
	}

	pub fn addpd(&mut self,XMMdst: GPRegister, src: Address) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		simd_prefix(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8(0x58);
		emit_operand(dst, src, 0);
	}


	pub fn addps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x58, (0xC0 | encode));
	}

	pub fn vaddpd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x58, (0xC0 | encode));
	}

	pub fn vaddps(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x58, (0xC0 | encode));
	}

	pub fn vaddpd(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		attributes.set_rex_vex_w_reverted();
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8(0x58);
		emit_operand(dst, src, 0);
	}

	pub fn vaddps(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int8(0x58);
		emit_operand(dst, src, 0);
	}

	pub fn subpd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5C, (0xC0 | encode));
	}

	pub fn subps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5C, (0xC0 | encode));
	}

	pub fn vsubpd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5C, (0xC0 | encode));
	}

	pub fn vsubps(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5C, (0xC0 | encode));
	}

	pub fn vsubpd(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		attributes.set_rex_vex_w_reverted();
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8(0x5C);
		emit_operand(dst, src, 0);
	}

	pub fn vsubps(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int8(0x5C);
		emit_operand(dst, src, 0);
	}

	pub fn mulpd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x59, (0xC0 | encode));
	}

	pub fn mulpd(&mut self,XMMdst: GPRegister, src: Address) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		attributes.set_rex_vex_w_reverted();
		simd_prefix(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8(0x59);
		emit_operand(dst, src, 0);
	}

	pub fn mulps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x59, (0xC0 | encode));
	}

	pub fn vmulpd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x59, (0xC0 | encode));
	}

	pub fn vmulps(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x59, (0xC0 | encode));
	}

	pub fn vmulpd(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		attributes.set_rex_vex_w_reverted();
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8(0x59);
		emit_operand(dst, src, 0);
	}

	pub fn vmulps(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int8(0x59);
		emit_operand(dst, src, 0);
	}

	pub fn vfmadd231pd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister1, XMMsrc: GPRegister2, int vector_len) {
		assert(VM_Version::supports_fma(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xB8, (0xC0 | encode));
	}

	pub fn vfmadd231ps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister1, XMMsrc: GPRegister2, int vector_len) {
		assert(VM_Version::supports_fma(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xB8, (0xC0 | encode));
	}

	pub fn vfmadd231pd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister1, src: Address2, int vector_len) {
		assert(VM_Version::supports_fma(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		vex_prefix(src2, src1->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xB8);
		emit_operand(dst, src2, 0);
	}

	pub fn vfmadd231ps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister1, src: Address2, int vector_len) {
		assert(VM_Version::supports_fma(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src2, src1->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xB8);
		emit_operand(dst, src2, 0);
	}

	pub fn divpd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5E, (0xC0 | encode));
	}

	pub fn divps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5E, (0xC0 | encode));
	}

	pub fn vdivpd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5E, (0xC0 | encode));
	}

	pub fn vdivps(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5E, (0xC0 | encode));
	}

	pub fn vdivpd(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		attributes.set_rex_vex_w_reverted();
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8(0x5E);
		emit_operand(dst, src, 0);
	}

	pub fn vdivps(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int8(0x5E);
		emit_operand(dst, src, 0);
	}

	pub fn vroundpd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, rmode: i32, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int24(0x09, (0xC0 | encode), (rmode));
	}

	pub fn vroundpd(&mut self,XMMdst: GPRegister, src: Address, rmode: i32,  int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int8(0x09);
		emit_operand(dst, src, 1);
		emit_int8((rmode));
	}

	pub fn vrndscalepd(&mut self,XMMdst: GPRegister,  XMMsrc: GPRegister,  rmode: i32, int vector_len) {
		assert(VM_Version::supports_evex(), "requires EVEX support");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int24(0x09, (0xC0 | encode), (rmode));
	}

	pub fn vrndscalepd(&mut self,XMMdst: GPRegister, src: Address, rmode: i32, int vector_len) {
		assert(VM_Version::supports_evex(), "requires EVEX support");
		assert(dst != xnoreg, "sanity");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int8(0x09);
		emit_operand(dst, src, 1);
		emit_int8((rmode));
	}

	pub fn vsqrtpd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x51, (0xC0 | encode));
	}

	pub fn vsqrtpd(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		attributes.set_rex_vex_w_reverted();
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8(0x51);
		emit_operand(dst, src, 0);
	}

	pub fn vsqrtps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x51, (0xC0 | encode));
	}

	pub fn vsqrtps(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int8(0x51);
		emit_operand(dst, src, 0);
	}

	pub fn andpd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ !_legacy_mode_dq, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x54, (0xC0 | encode));
	}

	pub fn andps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x54, (0xC0 | encode));
	}

	pub fn andps(&mut self,XMMdst: GPRegister, src: Address) {
		NOT_LP64(assert(VM_Version::supports_sse(), ""));
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_32bit);
		simd_prefix(dst, dst, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int8(0x54);
		emit_operand(dst, src, 0);
	}

	pub fn andpd(&mut self,XMMdst: GPRegister, src: Address) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* rex_w */ !_legacy_mode_dq, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		attributes.set_rex_vex_w_reverted();
		simd_prefix(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8(0x54);
		emit_operand(dst, src, 0);
	}

	pub fn vandpd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ !_legacy_mode_dq, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x54, (0xC0 | encode));
	}

	pub fn vandps(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x54, (0xC0 | encode));
	}

	pub fn vandpd(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ !_legacy_mode_dq, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		attributes.set_rex_vex_w_reverted();
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8(0x54);
		emit_operand(dst, src, 0);
	}

	pub fn vandps(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int8(0x54);
		emit_operand(dst, src, 0);
	}

	pub fn unpckhpd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8(0x15);
		emit_int8((0xC0 | encode));
	}

	pub fn unpcklpd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x14, (0xC0 | encode));
	}

	pub fn xorpd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ !_legacy_mode_dq, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x57, (0xC0 | encode));
	}

	pub fn xorps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x57, (0xC0 | encode));
	}

	pub fn xorpd(&mut self,XMMdst: GPRegister, src: Address) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* rex_w */ !_legacy_mode_dq, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		attributes.set_rex_vex_w_reverted();
		simd_prefix(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8(0x57);
		emit_operand(dst, src, 0);
	}

	pub fn xorps(&mut self,XMMdst: GPRegister, src: Address) {
		NOT_LP64(assert(VM_Version::supports_sse(), ""));
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_32bit);
		simd_prefix(dst, dst, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int8(0x57);
		emit_operand(dst, src, 0);
	}

	pub fn vxorpd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ !_legacy_mode_dq, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x57, (0xC0 | encode));
	}

	pub fn vxorps(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x57, (0xC0 | encode));
	}

	pub fn vxorpd(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ !_legacy_mode_dq, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		attributes.set_rex_vex_w_reverted();
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8(0x57);
		emit_operand(dst, src, 0);
	}

	pub fn vxorps(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int8(0x57);
		emit_operand(dst, src, 0);
	}

	// Integer vector arithmetic
	pub fn vphaddw(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx() && (vector_len == 0) ||
				   VM_Version::supports_avx2(), "256 bit integer vectors requires AVX2");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x01, (0xC0 | encode));
	}

	pub fn vphaddd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx() && (vector_len == 0) ||
				   VM_Version::supports_avx2(), "256 bit integer vectors requires AVX2");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x02, (0xC0 | encode));
	}

	pub fn paddb(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xFC, (0xC0 | encode));
	}

	pub fn paddw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xFD, (0xC0 | encode));
	}

	pub fn paddd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xFE, (0xC0 | encode));
	}

	pub fn paddd(&mut self,XMMdst: GPRegister, src: Address) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		simd_prefix(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xFE);
		emit_operand(dst, src, 0);
	}

	pub fn paddq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xD4, (0xC0 | encode));
	}

	pub fn phaddw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_sse3(), "");
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x01, (0xC0 | encode));
	}

	pub fn phaddd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_sse3(), "");
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x02, (0xC0 | encode));
	}

	pub fn vpaddb(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xFC, (0xC0 | encode));
	}

	pub fn vpaddw(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xFD, (0xC0 | encode));
	}

	pub fn vpaddd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xFE, (0xC0 | encode));
	}

	pub fn vpaddq(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xD4, (0xC0 | encode));
	}

	pub fn vpaddb(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xFC);
		emit_operand(dst, src, 0);
	}

	pub fn vpaddw(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xFD);
		emit_operand(dst, src, 0);
	}

	pub fn vpaddd(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xFE);
		emit_operand(dst, src, 0);
	}

	pub fn vpaddq(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		attributes.set_rex_vex_w_reverted();
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xD4);
		emit_operand(dst, src, 0);
	}

	pub fn psubb(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xF8, (0xC0 | encode));
	}

	pub fn psubw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xF9, (0xC0 | encode));
	}

	pub fn psubd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xFA, (0xC0 | encode));
	}

	pub fn psubq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xFB);
		emit_int8((0xC0 | encode));
	}

	pub fn vpsubusb(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xD8, (0xC0 | encode));
	}

	pub fn vpsubb(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xF8, (0xC0 | encode));
	}

	pub fn vpsubw(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xF9, (0xC0 | encode));
	}

	pub fn vpsubd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xFA, (0xC0 | encode));
	}

	pub fn vpsubq(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xFB, (0xC0 | encode));
	}

	pub fn vpsubb(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xF8);
		emit_operand(dst, src, 0);
	}

	pub fn vpsubw(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xF9);
		emit_operand(dst, src, 0);
	}

	pub fn vpsubd(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xFA);
		emit_operand(dst, src, 0);
	}

	pub fn vpsubq(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		attributes.set_rex_vex_w_reverted();
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xFB);
		emit_operand(dst, src, 0);
	}

	pub fn pmullw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xD5, (0xC0 | encode));
	}

	pub fn pmulld(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_sse4_1(), "");
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x40, (0xC0 | encode));
	}

	pub fn pmuludq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_sse2(), "");
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xF4, (0xC0 | encode));
	}

	pub fn vpmulhuw(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert((vector_len == AVX_128bit && VM_Version::supports_avx()) ||
				   (vector_len == AVX_256bit && VM_Version::supports_avx2()) ||
				   (vector_len == AVX_512bit && VM_Version::supports_avx512bw()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xE4, (0xC0 | encode));
	}

	pub fn vpmullw(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xD5, (0xC0 | encode));
	}

	pub fn vpmulld(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x40, (0xC0 | encode));
	}

	pub fn evpmullq(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 2, "requires some form of EVEX");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x40, (0xC0 | encode));
	}

	pub fn vpmuludq(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xF4, (0xC0 | encode));
	}

	pub fn vpmullw(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xD5);
		emit_operand(dst, src, 0);
	}

	pub fn vpmulld(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x40);
		emit_operand(dst, src, 0);
	}

	pub fn evpmullq(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(UseAVX > 2, "requires some form of EVEX");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ _legacy_mode_dq, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		attributes.set_is_evex_instruction();
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x40);
		emit_operand(dst, src, 0);
	}

	// Min, max
	pub fn pminsb(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_sse4_1(), "");
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x38, (0xC0 | encode));
	}

	pub fn vpminsb(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(vector_len == AVX_128bit ? VM_Version::supports_avx() :
			(vector_len == AVX_256bit ? VM_Version::supports_avx2() : VM_Version::supports_avx512bw()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x38, (0xC0 | encode));
	}

	pub fn pminsw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_sse2(), "");
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xEA, (0xC0 | encode));
	}

	pub fn vpminsw(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(vector_len == AVX_128bit ? VM_Version::supports_avx() :
			(vector_len == AVX_256bit ? VM_Version::supports_avx2() : VM_Version::supports_avx512bw()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xEA, (0xC0 | encode));
	}

	pub fn pminsd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_sse4_1(), "");
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x39, (0xC0 | encode));
	}

	pub fn vpminsd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(vector_len == AVX_128bit ? VM_Version::supports_avx() :
			(vector_len == AVX_256bit ? VM_Version::supports_avx2() : VM_Version::supports_evex()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x39, (0xC0 | encode));
	}

	pub fn vpminsq(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 2, "requires AVX512F");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x39, (0xC0 | encode));
	}

	pub fn minps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5D, (0xC0 | encode));
	}
	pub fn vminps(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(vector_len >= AVX_512bit ? VM_Version::supports_evex() : VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5D, (0xC0 | encode));
	}

	pub fn minpd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5D, (0xC0 | encode));
	}
	pub fn vminpd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(vector_len >= AVX_512bit ? VM_Version::supports_evex() : VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5D, (0xC0 | encode));
	}

	pub fn pmaxsb(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_sse4_1(), "");
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x3C, (0xC0 | encode));
	}

	pub fn vpmaxsb(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(vector_len == AVX_128bit ? VM_Version::supports_avx() :
			(vector_len == AVX_256bit ? VM_Version::supports_avx2() : VM_Version::supports_avx512bw()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x3C, (0xC0 | encode));
	}

	pub fn pmaxsw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_sse2(), "");
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xEE, (0xC0 | encode));
	}

	pub fn vpmaxsw(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(vector_len == AVX_128bit ? VM_Version::supports_avx() :
			(vector_len == AVX_256bit ? VM_Version::supports_avx2() : VM_Version::supports_avx512bw()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xEE, (0xC0 | encode));
	}

	pub fn pmaxsd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_sse4_1(), "");
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x3D, (0xC0 | encode));
	}

	pub fn vpmaxsd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(vector_len == AVX_128bit ? VM_Version::supports_avx() :
			(vector_len == AVX_256bit ? VM_Version::supports_avx2() : VM_Version::supports_evex()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x3D, (0xC0 | encode));
	}

	pub fn vpmaxsq(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 2, "requires AVX512F");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x3D, (0xC0 | encode));
	}

	pub fn maxps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5F, (0xC0 | encode));
	}

	pub fn vmaxps(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(vector_len >= AVX_512bit ? VM_Version::supports_evex() : VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5F, (0xC0 | encode));
	}

	pub fn maxpd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5F, (0xC0 | encode));
	}

	pub fn vmaxpd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(vector_len >= AVX_512bit ? VM_Version::supports_evex() : VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5F, (0xC0 | encode));
	}

	// Shift packed integers left by specified number of bits.
	pub fn psllw(&mut self,XMMdst: GPRegister, int shift) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		// XMM6 is for /6 encoding: 66 0F 71 /6 ib
		int encode = simd_prefix_and_encode(xmm6, dst, dst, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x71, (0xC0 | encode), shift & 0xFF);
	}

	pub fn pslld(&mut self,XMMdst: GPRegister, int shift) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		// XMM6 is for /6 encoding: 66 0F 72 /6 ib
		int encode = simd_prefix_and_encode(xmm6, dst, dst, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x72, (0xC0 | encode), shift & 0xFF);
	}

	pub fn psllq(&mut self,XMMdst: GPRegister, int shift) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		// XMM6 is for /6 encoding: 66 0F 73 /6 ib
		int encode = simd_prefix_and_encode(xmm6, dst, dst, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x73, (0xC0 | encode), shift & 0xFF);
	}

	pub fn psllw(&mut self,XMMdst: GPRegister, XMMRegister shift) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, shift, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xF1, (0xC0 | encode));
	}

	pub fn pslld(&mut self,XMMdst: GPRegister, XMMRegister shift) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, shift, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xF2, (0xC0 | encode));
	}

	pub fn psllq(&mut self,XMMdst: GPRegister, XMMRegister shift) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = simd_prefix_and_encode(dst, dst, shift, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xF3, (0xC0 | encode));
	}

	pub fn vpsllw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int shift, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		// XMM6 is for /6 encoding: 66 0F 71 /6 ib
		int encode = vex_prefix_and_encode(xmm6->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x71, (0xC0 | encode), shift & 0xFF);
	}

	pub fn vpslld(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int shift, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		// XMM6 is for /6 encoding: 66 0F 72 /6 ib
		int encode = vex_prefix_and_encode(xmm6->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x72, (0xC0 | encode), shift & 0xFF);
	}

	pub fn vpsllq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int shift, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		// XMM6 is for /6 encoding: 66 0F 73 /6 ib
		int encode = vex_prefix_and_encode(xmm6->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x73, (0xC0 | encode), shift & 0xFF);
	}

	pub fn vpsllw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xF1, (0xC0 | encode));
	}

	pub fn vpslld(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xF2, (0xC0 | encode));
	}

	pub fn vpsllq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xF3, (0xC0 | encode));
	}

	// Shift packed integers logically right by specified number of bits.
	pub fn psrlw(&mut self,XMMdst: GPRegister, int shift) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		// XMM2 is for /2 encoding: 66 0F 71 /2 ib
		int encode = simd_prefix_and_encode(xmm2, dst, dst, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x71, (0xC0 | encode), shift & 0xFF);
	}

	pub fn psrld(&mut self,XMMdst: GPRegister, int shift) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		// XMM2 is for /2 encoding: 66 0F 72 /2 ib
		int encode = simd_prefix_and_encode(xmm2, dst, dst, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x72, (0xC0 | encode), shift & 0xFF);
	}

	pub fn psrlq(&mut self,XMMdst: GPRegister, int shift) {
		// Do not confuse it with psrldq SSE2 instruction which
		// shifts 128 bit value in xmm register by number of bytes.
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		// XMM2 is for /2 encoding: 66 0F 73 /2 ib
		int encode = simd_prefix_and_encode(xmm2, dst, dst, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x73, (0xC0 | encode), shift & 0xFF);
	}

	pub fn psrlw(&mut self,XMMdst: GPRegister, XMMRegister shift) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, shift, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xD1, (0xC0 | encode));
	}

	pub fn psrld(&mut self,XMMdst: GPRegister, XMMRegister shift) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, shift, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xD2, (0xC0 | encode));
	}

	pub fn psrlq(&mut self,XMMdst: GPRegister, XMMRegister shift) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = simd_prefix_and_encode(dst, dst, shift, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xD3, (0xC0 | encode));
	}

	pub fn vpsrlw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int shift, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		// XMM2 is for /2 encoding: 66 0F 71 /2 ib
		int encode = vex_prefix_and_encode(xmm2->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x71, (0xC0 | encode), shift & 0xFF);
	}

	pub fn vpsrld(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int shift, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		// XMM2 is for /2 encoding: 66 0F 72 /2 ib
		int encode = vex_prefix_and_encode(xmm2->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x72, (0xC0 | encode), shift & 0xFF);
	}

	pub fn vpsrlq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int shift, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		// XMM2 is for /2 encoding: 66 0F 73 /2 ib
		int encode = vex_prefix_and_encode(xmm2->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x73, (0xC0 | encode), shift & 0xFF);
	}

	pub fn vpsrlw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xD1, (0xC0 | encode));
	}

	pub fn vpsrld(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xD2, (0xC0 | encode));
	}

	pub fn vpsrlq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xD3, (0xC0 | encode));
	}

	pub fn evpsrlvw(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx512bw(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x10, (0xC0 | encode));
	}

	pub fn evpsllvw(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx512bw(), "");
		InstructionAttr attributes(vector_len, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x12, (0xC0 | encode));
	}

	// Shift packed integers arithmetically right by specified number of bits.
	pub fn psraw(&mut self,XMMdst: GPRegister, int shift) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		// XMM4 is for /4 encoding: 66 0F 71 /4 ib
		int encode = simd_prefix_and_encode(xmm4, dst, dst, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x71, (0xC0 | encode), shift & 0xFF);
	}

	pub fn psrad(&mut self,XMMdst: GPRegister, int shift) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		// XMM4 is for /4 encoding: 66 0F 72 /4 ib
		int encode = simd_prefix_and_encode(xmm4, dst, dst, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8(0x72);
		emit_int8((0xC0 | encode));
		emit_int8(shift & 0xFF);
	}

	pub fn psraw(&mut self,XMMdst: GPRegister, XMMRegister shift) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, shift, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xE1, (0xC0 | encode));
	}

	pub fn psrad(&mut self,XMMdst: GPRegister, XMMRegister shift) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, shift, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xE2, (0xC0 | encode));
	}

	pub fn vpsraw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int shift, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		// XMM4 is for /4 encoding: 66 0F 71 /4 ib
		int encode = vex_prefix_and_encode(xmm4->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x71, (0xC0 | encode), shift & 0xFF);
	}

	pub fn vpsrad(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int shift, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		// XMM4 is for /4 encoding: 66 0F 71 /4 ib
		int encode = vex_prefix_and_encode(xmm4->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x72, (0xC0 | encode), shift & 0xFF);
	}

	pub fn vpsraw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xE1, (0xC0 | encode));
	}

	pub fn vpsrad(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xE2, (0xC0 | encode));
	}

	pub fn evpsraq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int shift, int vector_len) {
		assert(UseAVX > 2, "requires AVX512");
		assert ((VM_Version::supports_avx512vl() || vector_len == 2), "requires AVX512vl");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(xmm4->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24((unsigned char)0x72, (0xC0 | encode), shift & 0xFF);
	}

	pub fn evpsraq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(UseAVX > 2, "requires AVX512");
		assert ((VM_Version::supports_avx512vl() || vector_len == 2), "requires AVX512vl");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xE2, (0xC0 | encode));
	}

	// logical operations packed integers
	pub fn pand(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xDB, (0xC0 | encode));
	}

	pub fn vpand(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xDB, (0xC0 | encode));
	}

	pub fn vpand(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xDB);
		emit_operand(dst, src, 0);
	}

	pub fn evpandq(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		evpandq(dst, k0, nds, src, false, vector_len);
	}

	pub fn evpandq(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		evpandq(dst, k0, nds, src, false, vector_len);
	}

	//Variable Shift packed integers logically left.
	pub fn vpsllvd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(UseAVX > 1, "requires AVX2");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x47, (0xC0 | encode));
	}

	pub fn vpsllvq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(UseAVX > 1, "requires AVX2");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x47, (0xC0 | encode));
	}

	//Variable Shift packed integers logically right.
	pub fn vpsrlvd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(UseAVX > 1, "requires AVX2");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x45, (0xC0 | encode));
	}

	pub fn vpsrlvq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(UseAVX > 1, "requires AVX2");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x45, (0xC0 | encode));
	}

	//Variable right Shift arithmetic packed integers .
	pub fn vpsravd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(UseAVX > 1, "requires AVX2");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x46, (0xC0 | encode));
	}

	pub fn evpsravw(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx512bw(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x11, (0xC0 | encode));
	}

	pub fn evpsravq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(UseAVX > 2, "requires AVX512");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires AVX512VL");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x46, (0xC0 | encode));
	}

	pub fn vpshldvd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(VM_Version::supports_avx512_vbmi2(), "requires vbmi2");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x71, (0xC0 | encode));
	}

	pub fn vpshrdvd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(VM_Version::supports_avx512_vbmi2(), "requires vbmi2");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x73, (0xC0 | encode));
	}

	pub fn pandn(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xDF, (0xC0 | encode));
	}

	pub fn vpandn(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xDF, (0xC0 | encode));
	}

	pub fn por(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xEB, (0xC0 | encode));
	}

	pub fn vpor(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xEB, (0xC0 | encode));
	}

	pub fn vpor(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xEB);
		emit_operand(dst, src, 0);
	}

	pub fn evporq(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		evporq(dst, k0, nds, src, false, vector_len);
	}

	pub fn evporq(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		evporq(dst, k0, nds, src, false, vector_len);
	}

	pub fn evpord(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		// Encoding: EVEX.NDS.XXX.66.0F.W0 EB /r
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xEB, (0xC0 | encode));
	}

	pub fn evpord(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		// Encoding: EVEX.NDS.XXX.66.0F.W0 EB /r
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_NObit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xEB);
		emit_operand(dst, src, 0);
	}

	pub fn pxor(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xEF, (0xC0 | encode));
	}

	pub fn vpxor(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		assert(vector_len == AVX_128bit ? VM_Version::supports_avx() :
			vector_len == AVX_256bit ? VM_Version::supports_avx2() :
				   vector_len == AVX_512bit ? VM_Version::supports_evex() : 0, "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xEF, (0xC0 | encode));
	}

	pub fn vpxor(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		assert(vector_len == AVX_128bit ? VM_Version::supports_avx() :
			vector_len == AVX_256bit ? VM_Version::supports_avx2() :
				   vector_len == AVX_512bit ? VM_Version::supports_evex() : 0, "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xEF);
		emit_operand(dst, src, 0);
	}

	pub fn vpxorq(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 2, "requires some form of EVEX");
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xEF, (0xC0 | encode));
	}

	pub fn evpxord(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		// Encoding: EVEX.NDS.XXX.66.0F.W0 EF /r
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xEF, (0xC0 | encode));
	}

	pub fn evpxord(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xEF);
		emit_operand(dst, src, 0);
	}

	pub fn evpxorq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		// Encoding: EVEX.NDS.XXX.66.0F.W1 EF /r
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xEF, (0xC0 | encode));
	}

	pub fn evpxorq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xEF);
		emit_operand(dst, src, 0);
	}

	pub fn evpandd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xDB);
		emit_operand(dst, src, 0);
	}

	pub fn evpandq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "requires AVX512F");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires AVX512VL");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xDB, (0xC0 | encode));
	}

	pub fn evpandq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "requires AVX512F");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires AVX512VL");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xDB);
		emit_operand(dst, src, 0);
	}

	pub fn evporq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "requires AVX512F");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires AVX512VL");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xEB, (0xC0 | encode));
	}

	pub fn evporq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "requires AVX512F");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires AVX512VL");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xEB);
		emit_operand(dst, src, 0);
	}

	pub fn evpxorq(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_evex(), "requires EVEX support");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xEF, (0xC0 | encode));
	}

	pub fn evpxorq(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_evex(), "requires EVEX support");
		assert(dst != xnoreg, "sanity");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xEF);
		emit_operand(dst, src, 0);
	}

	pub fn evprold(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int shift, int vector_len) {
		assert(VM_Version::supports_evex(), "requires EVEX support");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires VL support");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(xmm1->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x72, (0xC0 | encode), shift & 0xFF);
	}

	pub fn evprolq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int shift, int vector_len) {
		assert(VM_Version::supports_evex(), "requires EVEX support");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires VL support");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(xmm1->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x72, (0xC0 | encode), shift & 0xFF);
	}

	pub fn evprord(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int shift, int vector_len) {
		assert(VM_Version::supports_evex(), "requires EVEX support");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires VL support");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(xmm0->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x72, (0xC0 | encode), shift & 0xFF);
	}

	pub fn evprorq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int shift, int vector_len) {
		assert(VM_Version::supports_evex(), "requires EVEX support");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires VL support");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(xmm0->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x72, (0xC0 | encode), shift & 0xFF);
	}

	pub fn evprolvd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(VM_Version::supports_evex(), "requires EVEX support");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires VL support");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x15, (unsigned char)(0xC0 | encode));
	}

	pub fn evprolvq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(VM_Version::supports_evex(), "requires EVEX support");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires VL support");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x15, (unsigned char)(0xC0 | encode));
	}

	pub fn evprorvd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(VM_Version::supports_evex(), "requires EVEX support");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires VL support");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x14, (unsigned char)(0xC0 | encode));
	}

	pub fn evprorvq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, XMMRegister shift, int vector_len) {
		assert(VM_Version::supports_evex(), "requires EVEX support");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires VL support");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), src->encoding(), shift->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x14, (unsigned char)(0xC0 | encode));
	}

	pub fn evplzcntd(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512cd(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x44, (0xC0 | encode));
	}

	pub fn evplzcntq(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512cd(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x44, (0xC0 | encode));
	}

	pub fn vpternlogd(&mut self,XMMdst: GPRegister, imm8: i8, XMMsrc: GPRegister2, XMMsrc: GPRegister3, int vector_len) {
		assert(VM_Version::supports_evex(), "requires EVEX support");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires VL support");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), src2->encoding(), src3->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int8(0x25);
		emit_int8((unsigned char)(0xC0 | encode));
		emit_int8(imm8);
	}

	pub fn vpternlogd(&mut self,XMMdst: GPRegister, imm8: i8, XMMsrc: GPRegister2, src: Address3, int vector_len) {
		assert(VM_Version::supports_evex(), "requires EVEX support");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires VL support");
		assert(dst != xnoreg, "sanity");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		vex_prefix(src3, src2->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int8(0x25);
		emit_operand(dst, src3, 1);
		emit_int8(imm8);
	}

	pub fn vpternlogq(&mut self,XMMdst: GPRegister, imm8: i8, XMMsrc: GPRegister2, XMMsrc: GPRegister3, int vector_len) {
		assert(VM_Version::supports_evex(), "requires AVX512F");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires AVX512VL");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), src2->encoding(), src3->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int8(0x25);
		emit_int8((unsigned char)(0xC0 | encode));
		emit_int8(imm8);
	}

	pub fn vpternlogq(&mut self,XMMdst: GPRegister, imm8: i8, XMMsrc: GPRegister2, src: Address3, int vector_len) {
		assert(VM_Version::supports_evex(), "requires EVEX support");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires VL support");
		assert(dst != xnoreg, "sanity");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		vex_prefix(src3, src2->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int8(0x25);
		emit_operand(dst, src3, 1);
		emit_int8(imm8);
	}

	pub fn evexpandps(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0x88, (0xC0 | encode));
	}

	pub fn evexpandpd(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0x88, (0xC0 | encode));
	}

	pub fn evpexpandb(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512_vbmi2(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x62, (0xC0 | encode));
	}

	pub fn evpexpandw(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512_vbmi2(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x62, (0xC0 | encode));
	}

	pub fn evpexpandd(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0x89, (0xC0 | encode));
	}

	pub fn evpexpandq(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0x89, (0xC0 | encode));
	}

// vinserti forms

	pub fn vinserti128(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_avx2(), "");
		assert(imm8 <= 0x01, "imm8: %u", imm8);
		InstructionAttr attributes(AVX_256bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		// last byte:
		// 0x00 - insert into lower 128 bits
		// 0x01 - insert into upper 128 bits
		emit_int24(0x38, (0xC0 | encode), imm8 & 0x01);
	}

	pub fn vinserti128(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, uint8_t imm8) {
		assert(VM_Version::supports_avx2(), "");
		assert(dst != xnoreg, "sanity");
		assert(imm8 <= 0x01, "imm8: %u", imm8);
		InstructionMark im(this);
		InstructionAttr attributes(AVX_256bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T4, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int8(0x38);
		emit_operand(dst, src, 1);
		// 0x00 - insert into lower 128 bits
		// 0x01 - insert into upper 128 bits
		emit_int8(imm8 & 0x01);
	}

	pub fn vinserti32x4(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_evex(), "");
		assert(imm8 <= 0x03, "imm8: %u", imm8);
		InstructionAttr attributes(AVX_512bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		// imm8:
		// 0x00 - insert into q0 128 bits (0..127)
		// 0x01 - insert into q1 128 bits (128..255)
		// 0x02 - insert into q2 128 bits (256..383)
		// 0x03 - insert into q3 128 bits (384..511)
		emit_int24(0x38, (0xC0 | encode), imm8 & 0x03);
	}

	pub fn vinserti32x4(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, uint8_t imm8) {
		assert(VM_Version::supports_evex(), "");
		assert(dst != xnoreg, "sanity");
		assert(imm8 <= 0x03, "imm8: %u", imm8);
		InstructionMark im(this);
		InstructionAttr attributes(AVX_512bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T4, /* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int8(0x18);
		emit_operand(dst, src, 1);
		// 0x00 - insert into q0 128 bits (0..127)
		// 0x01 - insert into q1 128 bits (128..255)
		// 0x02 - insert into q2 128 bits (256..383)
		// 0x03 - insert into q3 128 bits (384..511)
		emit_int8(imm8 & 0x03);
	}

	pub fn vinserti64x4(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_evex(), "");
		assert(imm8 <= 0x01, "imm8: %u", imm8);
		InstructionAttr attributes(AVX_512bit, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		//imm8:
		// 0x00 - insert into lower 256 bits
		// 0x01 - insert into upper 256 bits
		emit_int24(0x3A, (0xC0 | encode), imm8 & 0x01);
	}


// vinsertf forms

	pub fn vinsertf128(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_avx(), "");
		assert(imm8 <= 0x01, "imm8: %u", imm8);
		InstructionAttr attributes(AVX_256bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		// imm8:
		// 0x00 - insert into lower 128 bits
		// 0x01 - insert into upper 128 bits
		emit_int24(0x18, (0xC0 | encode), imm8 & 0x01);
	}

	pub fn vinsertf128(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, uint8_t imm8) {
		assert(VM_Version::supports_avx(), "");
		assert(dst != xnoreg, "sanity");
		assert(imm8 <= 0x01, "imm8: %u", imm8);
		InstructionMark im(this);
		InstructionAttr attributes(AVX_256bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T4, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int8(0x18);
		emit_operand(dst, src, 1);
		// 0x00 - insert into lower 128 bits
		// 0x01 - insert into upper 128 bits
		emit_int8(imm8 & 0x01);
	}

	pub fn vinsertf32x4(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_evex(), "");
		assert(imm8 <= 0x03, "imm8: %u", imm8);
		InstructionAttr attributes(AVX_512bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		// imm8:
		// 0x00 - insert into q0 128 bits (0..127)
		// 0x01 - insert into q1 128 bits (128..255)
		// 0x02 - insert into q0 128 bits (256..383)
		// 0x03 - insert into q1 128 bits (384..512)
		emit_int24(0x18, (0xC0 | encode), imm8 & 0x03);
	}

	pub fn vinsertf32x4(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, uint8_t imm8) {
		assert(VM_Version::supports_evex(), "");
		assert(dst != xnoreg, "sanity");
		assert(imm8 <= 0x03, "imm8: %u", imm8);
		InstructionMark im(this);
		InstructionAttr attributes(AVX_512bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T4, /* input_size_in_bits */ EVEX_32bit);
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int8(0x18);
		emit_operand(dst, src, 1);
		// 0x00 - insert into q0 128 bits (0..127)
		// 0x01 - insert into q1 128 bits (128..255)
		// 0x02 - insert into q0 128 bits (256..383)
		// 0x03 - insert into q1 128 bits (384..512)
		emit_int8(imm8 & 0x03);
	}

	pub fn vinsertf64x4(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_evex(), "");
		assert(imm8 <= 0x01, "imm8: %u", imm8);
		InstructionAttr attributes(AVX_512bit, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		// imm8:
		// 0x00 - insert into lower 256 bits
		// 0x01 - insert into upper 256 bits
		emit_int24(0x1A, (0xC0 | encode), imm8 & 0x01);
	}

	pub fn vinsertf64x4(&mut self,XMMdst: GPRegister, nds: XMMRegister, src: Address, uint8_t imm8) {
		assert(VM_Version::supports_evex(), "");
		assert(dst != xnoreg, "sanity");
		assert(imm8 <= 0x01, "imm8: %u", imm8);
		InstructionMark im(this);
		InstructionAttr attributes(AVX_512bit, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T4, /* input_size_in_bits */ EVEX_64bit);
		attributes.set_is_evex_instruction();
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int8(0x1A);
		emit_operand(dst, src, 1);
		// 0x00 - insert into lower 256 bits
		// 0x01 - insert into upper 256 bits
		emit_int8(imm8 & 0x01);
	}


// vextracti forms

	pub fn vextracti128(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_avx2(), "");
		assert(imm8 <= 0x01, "imm8: %u", imm8);
		InstructionAttr attributes(AVX_256bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		// imm8:
		// 0x00 - extract from lower 128 bits
		// 0x01 - extract from upper 128 bits
		emit_int24(0x39, (0xC0 | encode), imm8 & 0x01);
	}

	pub fn vextracti128(&mut self,dst: Address, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_avx2(), "");
		assert(src != xnoreg, "sanity");
		assert(imm8 <= 0x01, "imm8: %u", imm8);
		InstructionMark im(this);
		InstructionAttr attributes(AVX_256bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T4, /* input_size_in_bits */ EVEX_32bit);
		attributes.reset_is_clear_context();
		vex_prefix(dst, 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int8(0x39);
		emit_operand(src, dst, 1);
		// 0x00 - extract from lower 128 bits
		// 0x01 - extract from upper 128 bits
		emit_int8(imm8 & 0x01);
	}

	pub fn vextracti32x4(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_evex(), "");
		assert(imm8 <= 0x03, "imm8: %u", imm8);
		InstructionAttr attributes(AVX_512bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		// imm8:
		// 0x00 - extract from bits 127:0
		// 0x01 - extract from bits 255:128
		// 0x02 - extract from bits 383:256
		// 0x03 - extract from bits 511:384
		emit_int24(0x39, (0xC0 | encode), imm8 & 0x03);
	}

	pub fn vextracti32x4(&mut self,dst: Address, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_evex(), "");
		assert(src != xnoreg, "sanity");
		assert(imm8 <= 0x03, "imm8: %u", imm8);
		InstructionMark im(this);
		InstructionAttr attributes(AVX_512bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T4, /* input_size_in_bits */ EVEX_32bit);
		attributes.reset_is_clear_context();
		attributes.set_is_evex_instruction();
		vex_prefix(dst, 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int8(0x39);
		emit_operand(src, dst, 1);
		// 0x00 - extract from bits 127:0
		// 0x01 - extract from bits 255:128
		// 0x02 - extract from bits 383:256
		// 0x03 - extract from bits 511:384
		emit_int8(imm8 & 0x03);
	}

	pub fn vextracti64x2(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_avx512dq(), "");
		assert(imm8 <= 0x03, "imm8: %u", imm8);
		InstructionAttr attributes(AVX_512bit, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		// imm8:
		// 0x00 - extract from bits 127:0
		// 0x01 - extract from bits 255:128
		// 0x02 - extract from bits 383:256
		// 0x03 - extract from bits 511:384
		emit_int24(0x39, (0xC0 | encode), imm8 & 0x03);
	}

	pub fn vextracti64x4(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_evex(), "");
		assert(imm8 <= 0x01, "imm8: %u", imm8);
		InstructionAttr attributes(AVX_512bit, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		// imm8:
		// 0x00 - extract from lower 256 bits
		// 0x01 - extract from upper 256 bits
		emit_int24(0x3B, (0xC0 | encode), imm8 & 0x01);
	}

	pub fn vextracti64x4(&mut self,dst: Address, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_evex(), "");
		assert(src != xnoreg, "sanity");
		assert(imm8 <= 0x01, "imm8: %u", imm8);
		InstructionMark im(this);
		InstructionAttr attributes(AVX_512bit, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T4, /* input_size_in_bits */ EVEX_64bit);
		attributes.reset_is_clear_context();
		attributes.set_is_evex_instruction();
		vex_prefix(dst, 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int8(0x38);
		emit_operand(src, dst, 1);
		// 0x00 - extract from lower 256 bits
		// 0x01 - extract from upper 256 bits
		emit_int8(imm8 & 0x01);
	}
// vextractf forms

	pub fn vextractf128(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_avx(), "");
		assert(imm8 <= 0x01, "imm8: %u", imm8);
		InstructionAttr attributes(AVX_256bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		// imm8:
		// 0x00 - extract from lower 128 bits
		// 0x01 - extract from upper 128 bits
		emit_int24(0x19, (0xC0 | encode), imm8 & 0x01);
	}

	pub fn vextractf128(&mut self,dst: Address, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_avx(), "");
		assert(src != xnoreg, "sanity");
		assert(imm8 <= 0x01, "imm8: %u", imm8);
		InstructionMark im(this);
		InstructionAttr attributes(AVX_256bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T4, /* input_size_in_bits */ EVEX_32bit);
		attributes.reset_is_clear_context();
		vex_prefix(dst, 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int8(0x19);
		emit_operand(src, dst, 1);
		// 0x00 - extract from lower 128 bits
		// 0x01 - extract from upper 128 bits
		emit_int8(imm8 & 0x01);
	}

	pub fn vextractf32x4(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_evex(), "");
		assert(imm8 <= 0x03, "imm8: %u", imm8);
		InstructionAttr attributes(AVX_512bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		// imm8:
		// 0x00 - extract from bits 127:0
		// 0x01 - extract from bits 255:128
		// 0x02 - extract from bits 383:256
		// 0x03 - extract from bits 511:384
		emit_int24(0x19, (0xC0 | encode), imm8 & 0x03);
	}

	pub fn vextractf32x4(&mut self,dst: Address, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_evex(), "");
		assert(src != xnoreg, "sanity");
		assert(imm8 <= 0x03, "imm8: %u", imm8);
		InstructionMark im(this);
		InstructionAttr attributes(AVX_512bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T4, /* input_size_in_bits */ EVEX_32bit);
		attributes.reset_is_clear_context();
		attributes.set_is_evex_instruction();
		vex_prefix(dst, 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int8(0x19);
		emit_operand(src, dst, 1);
		// 0x00 - extract from bits 127:0
		// 0x01 - extract from bits 255:128
		// 0x02 - extract from bits 383:256
		// 0x03 - extract from bits 511:384
		emit_int8(imm8 & 0x03);
	}

	pub fn vextractf64x2(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_avx512dq(), "");
		assert(imm8 <= 0x03, "imm8: %u", imm8);
		InstructionAttr attributes(AVX_512bit, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		// imm8:
		// 0x00 - extract from bits 127:0
		// 0x01 - extract from bits 255:128
		// 0x02 - extract from bits 383:256
		// 0x03 - extract from bits 511:384
		emit_int24(0x19, (0xC0 | encode), imm8 & 0x03);
	}

	pub fn vextractf64x4(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_evex(), "");
		assert(imm8 <= 0x01, "imm8: %u", imm8);
		InstructionAttr attributes(AVX_512bit, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		// imm8:
		// 0x00 - extract from lower 256 bits
		// 0x01 - extract from upper 256 bits
		emit_int24(0x1B, (0xC0 | encode), imm8 & 0x01);
	}

	pub fn vextractf64x4(&mut self,dst: Address, XMMsrc: GPRegister, uint8_t imm8) {
		assert(VM_Version::supports_evex(), "");
		assert(src != xnoreg, "sanity");
		assert(imm8 <= 0x01, "imm8: %u", imm8);
		InstructionMark im(this);
		InstructionAttr attributes(AVX_512bit, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T4,/* input_size_in_bits */  EVEX_64bit);
		attributes.reset_is_clear_context();
		attributes.set_is_evex_instruction();
		vex_prefix(dst, 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int8(0x1B);
		emit_operand(src, dst, 1);
		// 0x00 - extract from lower 256 bits
		// 0x01 - extract from upper 256 bits
		emit_int8(imm8 & 0x01);
	}

	// duplicate 1-byte integer data from src into programmed locations in dest : requires AVX512BW and AVX512VL
	pub fn vpbroadcastb(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx2(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x78, (0xC0 | encode));
	}

	pub fn vpbroadcastb(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx2(), "");
		assert(dst != xnoreg, "sanity");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_8bit);
		// swap src<->dst for encoding
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x78);
		emit_operand(dst, src, 0);
	}

	// duplicate 2-byte integer data from src into programmed locations in dest : requires AVX512BW and AVX512VL
	pub fn vpbroadcastw(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx2(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x79, (0xC0 | encode));
	}

	pub fn vpbroadcastw(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx2(), "");
		assert(dst != xnoreg, "sanity");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_16bit);
		// swap src<->dst for encoding
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x79);
		emit_operand(dst, src, 0);
	}

	pub fn vpsadbw(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xF6, (0xC0 | encode));
	}

	pub fn vpunpckhwd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x69, (0xC0 | encode));
	}

	pub fn vpunpcklwd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x61, (0xC0 | encode));
	}

	pub fn vpunpckhdq(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x6A, (0xC0 | encode));
	}

	pub fn vpunpckldq(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX > 0, "requires some form of AVX");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x62, (0xC0 | encode));
	}

	// xmm/mem sourced byte/word/dword/qword replicate
	pub fn evpaddb(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xFC, (0xC0 | encode));
	}

	pub fn evpaddb(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xFC);
		emit_operand(dst, src, 0);
	}

	pub fn evpaddw(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xFD, (0xC0 | encode));
	}

	pub fn evpaddw(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xFD);
		emit_operand(dst, src, 0);
	}

	pub fn evpaddd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xFE, (0xC0 | encode));
	}

	pub fn evpaddd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xFE);
		emit_operand(dst, src, 0);
	}

	pub fn evpaddq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xD4, (0xC0 | encode));
	}

	pub fn evpaddq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xD4);
		emit_operand(dst, src, 0);
	}

	pub fn evaddps(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x58, (0xC0 | encode));
	}

	pub fn evaddps(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int8(0x58);
		emit_operand(dst, src, 0);
	}

	pub fn evaddpd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x58, (0xC0 | encode));
	}

	pub fn evaddpd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8(0x58);
		emit_operand(dst, src, 0);
	}

	pub fn evpsubb(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xF8, (0xC0 | encode));
	}

	pub fn evpsubb(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xF8);
		emit_operand(dst, src, 0);
	}

	pub fn evpsubw(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xF9, (0xC0 | encode));
	}

	pub fn evpsubw(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xF9);
		emit_operand(dst, src, 0);
	}

	pub fn evpsubd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xFA, (0xC0 | encode));
	}

	pub fn evpsubd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xFA);
		emit_operand(dst, src, 0);
	}

	pub fn evpsubq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xFB, (0xC0 | encode));
	}

	pub fn evpsubq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xFB);
		emit_operand(dst, src, 0);
	}

	pub fn evsubps(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5C, (0xC0 | encode));
	}

	pub fn evsubps(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int8(0x5C);
		emit_operand(dst, src, 0);
	}

	pub fn evsubpd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5C, (0xC0 | encode));
	}

	pub fn evsubpd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8(0x5C);
		emit_operand(dst, src, 0);
	}

	pub fn evpmullw(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xD5, (0xC0 | encode));
	}

	pub fn evpmullw(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xD5);
		emit_operand(dst, src, 0);
	}

	pub fn evpmulld(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x40, (0xC0 | encode));
	}

	pub fn evpmulld(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x40);
		emit_operand(dst, src, 0);
	}

	pub fn evpmullq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512dq() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x40, (0xC0 | encode));
	}

	pub fn evpmullq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_avx512dq() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x40);
		emit_operand(dst, src, 0);
	}

	pub fn evmulps(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x59, (0xC0 | encode));
	}

	pub fn evmulps(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int8(0x59);
		emit_operand(dst, src, 0);
	}

	pub fn evmulpd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x59, (0xC0 | encode));
	}

	pub fn evmulpd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8(0x59);
		emit_operand(dst, src, 0);
	}

	pub fn evsqrtps(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len,/* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x51, (0xC0 | encode));
	}

	pub fn evsqrtps(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int8(0x51);
		emit_operand(dst, src, 0);
	}

	pub fn evsqrtpd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len,/* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x51, (0xC0 | encode));
	}

	pub fn evsqrtpd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8(0x51);
		emit_operand(dst, src, 0);
	}


	pub fn evdivps(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len,/* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5E, (0xC0 | encode));
	}

	pub fn evdivps(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int8(0x5E);
		emit_operand(dst, src, 0);
	}

	pub fn evdivpd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len,/* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5E, (0xC0 | encode));
	}

	pub fn evdivpd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8(0x5E);
		emit_operand(dst, src, 0);
	}

	pub fn evpabsb(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x1C, (0xC0 | encode));
	}


	pub fn evpabsb(&mut self,XMMdst: GPRegister, KRegister mask, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x1C);
		emit_operand(dst, src, 0);
	}

	pub fn evpabsw(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x1D, (0xC0 | encode));
	}


	pub fn evpabsw(&mut self,XMMdst: GPRegister, KRegister mask, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x1D);
		emit_operand(dst, src, 0);
	}

	pub fn evpabsd(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x1E, (0xC0 | encode));
	}


	pub fn evpabsd(&mut self,XMMdst: GPRegister, KRegister mask, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x1E);
		emit_operand(dst, src, 0);
	}

	pub fn evpabsq(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x1F, (0xC0 | encode));
	}


	pub fn evpabsq(&mut self,XMMdst: GPRegister, KRegister mask, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x1F);
		emit_operand(dst, src, 0);
	}

	pub fn evpfma213ps(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xA8, (0xC0 | encode));
	}

	pub fn evpfma213ps(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xA8);
		emit_operand(dst, src, 0);
	}

	pub fn evpfma213pd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xA8, (0xC0 | encode));
	}

	pub fn evpfma213pd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		InstructionMark im(this);
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true,/* legacy_mode */ false, /* no_mask_reg */ false,/* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV,/* input_size_in_bits */ EVEX_32bit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xA8);
		emit_operand(dst, src, 0);
	}

	pub fn evpermb(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512_vbmi() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* rex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0x8D, (0xC0 | encode));
	}

	pub fn evpermb(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512_vbmi() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0x8D);
		emit_operand(dst, src, 0);
	}

	pub fn evpermw(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0x8D, (0xC0 | encode));
	}

	pub fn evpermw(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0x8D);
		emit_operand(dst, src, 0);
	}

	pub fn evpermd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex() && vector_len > AVX_128bit, "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x36, (0xC0 | encode));
	}

	pub fn evpermd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		assert(VM_Version::supports_evex() && vector_len > AVX_128bit, "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x36);
		emit_operand(dst, src, 0);
	}

	pub fn evpermq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex() && vector_len > AVX_128bit, "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x36, (0xC0 | encode));
	}

	pub fn evpermq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		assert(VM_Version::supports_evex() && vector_len > AVX_128bit, "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x36);
		emit_operand(dst, src, 0);
	}

	pub fn evpsllw(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, int shift, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(xmm6->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x71, (0xC0 | encode), shift & 0xFF);
	}

	pub fn evpslld(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, int shift, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(xmm6->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x72, (0xC0 | encode), shift & 0xFF);
	}

	pub fn evpsllq(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, int shift, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(xmm6->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x73, (0xC0 | encode), shift & 0xFF);
	}

	pub fn evpsrlw(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, int shift, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(xmm2->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x71, (0xC0 | encode), shift & 0xFF);
	}

	pub fn evpsrld(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, int shift, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(xmm2->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x72, (0xC0 | encode), shift & 0xFF);
	}

	pub fn evpsrlq(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, int shift, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(xmm2->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x73, (0xC0 | encode), shift & 0xFF);
	}

	pub fn evpsraw(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, int shift, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(xmm4->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x71, (0xC0 | encode), shift & 0xFF);
	}

	pub fn evpsrad(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, int shift, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(xmm4->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x72, (0xC0 | encode), shift & 0xFF);
	}

	pub fn evpsraq(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, int shift, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(xmm4->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x72, (0xC0 | encode), shift & 0xFF);
	}

	pub fn evpsllw(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xF1, (0xC0 | encode));
	}

	pub fn evpslld(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xF2, (0xC0 | encode));
	}

	pub fn evpsllq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xF3, (0xC0 | encode));
	}

	pub fn evpsrlw(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xD1, (0xC0 | encode));
	}

	pub fn evpsrld(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xD2, (0xC0 | encode));
	}

	pub fn evpsrlq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xD3, (0xC0 | encode));
	}

	pub fn evpsraw(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xE1, (0xC0 | encode));
	}

	pub fn evpsrad(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xE2, (0xC0 | encode));
	}

	pub fn evpsraq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xE2, (0xC0 | encode));
	}

	pub fn evpsllvw(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x12, (0xC0 | encode));
	}

	pub fn evpsllvd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x47, (0xC0 | encode));
	}

	pub fn evpsllvq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x47, (0xC0 | encode));
	}

	pub fn evpsrlvw(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x10, (0xC0 | encode));
	}

	pub fn evpsrlvd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x45, (0xC0 | encode));
	}

	pub fn evpsrlvq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x45, (0xC0 | encode));
	}

	pub fn evpsravw(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x11, (0xC0 | encode));
	}

	pub fn evpsravd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x46, (0xC0 | encode));
	}

	pub fn evpsravq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x46, (0xC0 | encode));
	}

	pub fn evpminsb(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x38, (0xC0 | encode));
	}

	pub fn evpminsb(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x38);
		emit_operand(dst, src, 0);
	}

	pub fn evpminsw(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xEA, (0xC0 | encode));
	}

	pub fn evpminsw(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xEA);
		emit_operand(dst, src, 0);
	}

	pub fn evpminsd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x39, (0xC0 | encode));
	}

	pub fn evpminsd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x39);
		emit_operand(dst, src, 0);
	}

	pub fn evpminsq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x39, (0xC0 | encode));
	}

	pub fn evpminsq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x39);
		emit_operand(dst, src, 0);
	}


	pub fn evpmaxsb(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x3C, (0xC0 | encode));
	}

	pub fn evpmaxsb(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x3C);
		emit_operand(dst, src, 0);
	}

	pub fn evpmaxsw(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16((unsigned char)0xEE, (0xC0 | encode));
	}

	pub fn evpmaxsw(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512bw() && (vector_len == AVX_512bit || VM_Version::supports_avx512vl()), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int8((unsigned char)0xEE);
		emit_operand(dst, src, 0);
	}

	pub fn evpmaxsd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x3D, (0xC0 | encode));
	}

	pub fn evpmaxsd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x3D);
		emit_operand(dst, src, 0);
	}

	pub fn evpmaxsq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x3D, (0xC0 | encode));
	}

	pub fn evpmaxsq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, src: Address, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src, nds->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x3D);
		emit_operand(dst, src, 0);
	}

	pub fn evpternlogd(&mut self,XMMdst: GPRegister, imm8: i8, KRegister mask, XMMsrc: GPRegister2, XMMsrc: GPRegister3, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "requires EVEX support");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires VL support");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), src2->encoding(), src3->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int24(0x25, (unsigned char)(0xC0 | encode), imm8);
	}

	pub fn evpternlogd(&mut self,XMMdst: GPRegister, imm8: i8, KRegister mask, XMMsrc: GPRegister2, src: Address3, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "requires EVEX support");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires VL support");
		assert(dst != xnoreg, "sanity");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src3, src2->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int8(0x25);
		emit_operand(dst, src3, 1);
		emit_int8(imm8);
	}

	pub fn evpternlogq(&mut self,XMMdst: GPRegister, imm8: i8, KRegister mask, XMMsrc: GPRegister2, XMMsrc: GPRegister3, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "requires EVEX support");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires VL support");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), src2->encoding(), src3->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int24(0x25, (unsigned char)(0xC0 | encode), imm8);
	}

	pub fn evpternlogq(&mut self,XMMdst: GPRegister, imm8: i8, KRegister mask, XMMsrc: GPRegister2, src: Address3, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "requires EVEX support");
		assert(vector_len == Assembler::AVX_512bit || VM_Version::supports_avx512vl(), "requires VL support");
		assert(dst != xnoreg, "sanity");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_64bit);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		vex_prefix(src3, src2->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int8(0x25);
		emit_operand(dst, src3, 1);
		emit_int8(imm8);
	}

	pub fn gf2p8affineqb(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, imm8: i8) {
		assert(VM_Version::supports_gfni(), "");
		assert(VM_Version::supports_sse(), "");
		InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int24((unsigned char)0xCE, (unsigned char)(0xC0 | encode), imm8);
	}

	pub fn vgf2p8affineqb(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister2, XMMsrc: GPRegister3, imm8: i8, int vector_len) {
		assert(VM_Version::supports_gfni(), "requires GFNI support");
		assert(VM_Version::supports_sse(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src2->encoding(), src3->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int24((unsigned char)0xCE, (unsigned char)(0xC0 | encode), imm8);
	}

	// duplicate 4-byte integer data from src into programmed locations in dest : requires AVX512VL
	pub fn vpbroadcastd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
		assert(UseAVX >= 2, "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x58, (0xC0 | encode));
	}

	pub fn vpbroadcastd(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx2(), "");
		assert(dst != xnoreg, "sanity");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
		// swap src<->dst for encoding
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x58);
		emit_operand(dst, src, 0);
	}

	// duplicate 8-byte integer data from src into programmed locations in dest : requires AVX512VL
	pub fn vpbroadcastq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx2(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x59, (0xC0 | encode));
	}

	pub fn vpbroadcastq(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx2(), "");
		assert(dst != xnoreg, "sanity");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
		// swap src<->dst for encoding
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x59);
		emit_operand(dst, src, 0);
	}

	pub fn evbroadcasti32x4(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
		assert(vector_len != Assembler::AVX_128bit, "");
		assert(VM_Version::supports_evex(), "");
		assert(dst != xnoreg, "sanity");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		attributes.set_address_attributes(/* tuple_type */ EVEX_T4, /* input_size_in_bits */ EVEX_32bit);
		// swap src<->dst for encoding
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x5A);
		emit_operand(dst, src, 0);
	}

	pub fn evbroadcasti64x2(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
		assert(vector_len != Assembler::AVX_128bit, "");
		assert(VM_Version::supports_avx512dq(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x5A, (0xC0 | encode));
	}

	pub fn evbroadcasti64x2(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
		assert(vector_len != Assembler::AVX_128bit, "");
		assert(VM_Version::supports_avx512dq(), "");
		assert(dst != xnoreg, "sanity");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		attributes.set_address_attributes(/* tuple_type */ EVEX_T2, /* input_size_in_bits */ EVEX_64bit);
		// swap src<->dst for encoding
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x5A);
		emit_operand(dst, src, 0);
	}

// scalar single/double precision replicate

	// duplicate single precision data from src into programmed locations in dest : requires AVX512VL
	pub fn vbroadcastss(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx2(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x18, (0xC0 | encode));
	}

	pub fn vbroadcastss(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		assert(dst != xnoreg, "sanity");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
		// swap src<->dst for encoding
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x18);
		emit_operand(dst, src, 0);
	}

	// duplicate double precision data from src into programmed locations in dest : requires AVX512VL
	pub fn vbroadcastsd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx2(), "");
		assert(vector_len == AVX_256bit || vector_len == AVX_512bit, "");
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x19, (0xC0 | encode));
	}

	pub fn vbroadcastsd(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		assert(vector_len == AVX_256bit || vector_len == AVX_512bit, "");
		assert(dst != xnoreg, "sanity");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
		attributes.set_rex_vex_w_reverted();
		// swap src<->dst for encoding
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x19);
		emit_operand(dst, src, 0);
	}

	pub fn vbroadcastf128(&mut self,XMMdst: GPRegister, src: Address, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		assert(vector_len == AVX_256bit, "");
		assert(dst != xnoreg, "sanity");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T4, /* input_size_in_bits */ EVEX_32bit);
		// swap src<->dst for encoding
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8(0x1A);
		emit_operand(dst, src, 0);
	}

// gpr source broadcast forms

	// duplicate 1-byte integer data from src into programmed locations in dest : requires AVX512BW and AVX512VL
	pub fn evpbroadcastb(&mut self,XMMdst: GPRegister, src: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx512bw(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x7A, (0xC0 | encode));
	}

	// duplicate 2-byte integer data from src into programmed locations in dest : requires AVX512BW and AVX512VL
	pub fn evpbroadcastw(&mut self,XMMdst: GPRegister, src: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx512bw(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x7B, (0xC0 | encode));
	}

	// duplicate 4-byte integer data from src into programmed locations in dest : requires AVX512VL
	pub fn evpbroadcastd(&mut self,XMMdst: GPRegister, src: GPRegister, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x7C, (0xC0 | encode));
	}

	// duplicate 8-byte integer data from src into programmed locations in dest : requires AVX512VL
	pub fn evpbroadcastq(&mut self,XMMdst: GPRegister, src: GPRegister, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x7C, (0xC0 | encode));
	}

	pub fn vpgatherdd(&mut self,XMMdst: GPRegister, src: Address, XMMRegister mask, int vector_len) {
		assert(VM_Version::supports_avx2(), "");
		assert(vector_len == Assembler::AVX_128bit || vector_len == Assembler::AVX_256bit, "");
		assert(dst != xnoreg, "sanity");
		assert(src.isxmmindex(),"expected to be xmm index");
		assert(dst != src.xmmindex(), "instruction will #UD if dst and index are the same");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		vex_prefix(src, mask->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0x90);
		emit_operand(dst, src, 0);
	}

	pub fn vpgatherdq(&mut self,XMMdst: GPRegister, src: Address, XMMRegister mask, int vector_len) {
		assert(VM_Version::supports_avx2(), "");
		assert(vector_len == Assembler::AVX_128bit || vector_len == Assembler::AVX_256bit, "");
		assert(dst != xnoreg, "sanity");
		assert(src.isxmmindex(),"expected to be xmm index");
		assert(dst != src.xmmindex(), "instruction will #UD if dst and index are the same");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		vex_prefix(src, mask->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0x90);
		emit_operand(dst, src, 0);
	}

	pub fn vgatherdpd(&mut self,XMMdst: GPRegister, src: Address, XMMRegister mask, int vector_len) {
		assert(VM_Version::supports_avx2(), "");
		assert(vector_len == Assembler::AVX_128bit || vector_len == Assembler::AVX_256bit, "");
		assert(dst != xnoreg, "sanity");
		assert(src.isxmmindex(),"expected to be xmm index");
		assert(dst != src.xmmindex(), "instruction will #UD if dst and index are the same");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		vex_prefix(src, mask->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0x92);
		emit_operand(dst, src, 0);
	}

	pub fn vgatherdps(&mut self,XMMdst: GPRegister, src: Address, XMMRegister mask, int vector_len) {
		assert(VM_Version::supports_avx2(), "");
		assert(vector_len == Assembler::AVX_128bit || vector_len == Assembler::AVX_256bit, "");
		assert(dst != xnoreg, "sanity");
		assert(src.isxmmindex(),"expected to be xmm index");
		assert(dst != src.xmmindex(), "instruction will #UD if dst and index are the same");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ false, /* uses_vl */ true);
		vex_prefix(src, mask->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0x92);
		emit_operand(dst, src, 0);
	}
	pub fn evpgatherdd(&mut self,XMMdst: GPRegister, KRegister mask, src: Address, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(dst != xnoreg, "sanity");
		assert(src.isxmmindex(),"expected to be xmm index");
		assert(dst != src.xmmindex(), "instruction will #UD if dst and index are the same");
		assert(mask != k0, "instruction will #UD if mask is in k0");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
		attributes.reset_is_clear_context();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.set_is_evex_instruction();
		// swap src<->dst for encoding
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0x90);
		emit_operand(dst, src, 0);
	}

	pub fn evpgatherdq(&mut self,XMMdst: GPRegister, KRegister mask, src: Address, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(dst != xnoreg, "sanity");
		assert(src.isxmmindex(),"expected to be xmm index");
		assert(dst != src.xmmindex(), "instruction will #UD if dst and index are the same");
		assert(mask != k0, "instruction will #UD if mask is in k0");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
		attributes.reset_is_clear_context();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.set_is_evex_instruction();
		// swap src<->dst for encoding
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0x90);
		emit_operand(dst, src, 0);
	}

	pub fn evgatherdpd(&mut self,XMMdst: GPRegister, KRegister mask, src: Address, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(dst != xnoreg, "sanity");
		assert(src.isxmmindex(),"expected to be xmm index");
		assert(dst != src.xmmindex(), "instruction will #UD if dst and index are the same");
		assert(mask != k0, "instruction will #UD if mask is in k0");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
		attributes.reset_is_clear_context();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.set_is_evex_instruction();
		// swap src<->dst for encoding
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0x92);
		emit_operand(dst, src, 0);
	}

	pub fn evgatherdps(&mut self,XMMdst: GPRegister, KRegister mask, src: Address, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(dst != xnoreg, "sanity");
		assert(src.isxmmindex(),"expected to be xmm index");
		assert(dst != src.xmmindex(), "instruction will #UD if dst and index are the same");
		assert(mask != k0, "instruction will #UD if mask is in k0");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
		attributes.reset_is_clear_context();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.set_is_evex_instruction();
		// swap src<->dst for encoding
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0x92);
		emit_operand(dst, src, 0);
	}

	pub fn evpscatterdd(&mut self,dst: Address, KRegister mask, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(mask != k0, "instruction will #UD if mask is in k0");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
		attributes.reset_is_clear_context();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.set_is_evex_instruction();
		vex_prefix(dst, 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xA0);
		emit_operand(src, dst, 0);
	}

	pub fn evpscatterdq(&mut self,dst: Address, KRegister mask, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(mask != k0, "instruction will #UD if mask is in k0");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
		attributes.reset_is_clear_context();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.set_is_evex_instruction();
		vex_prefix(dst, 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xA0);
		emit_operand(src, dst, 0);
	}

	pub fn evscatterdps(&mut self,dst: Address, KRegister mask, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(mask != k0, "instruction will #UD if mask is in k0");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
		attributes.reset_is_clear_context();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.set_is_evex_instruction();
		vex_prefix(dst, 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xA2);
		emit_operand(src, dst, 0);
	}

	pub fn evscatterdpd(&mut self,dst: Address, KRegister mask, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(mask != k0, "instruction will #UD if mask is in k0");
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_32bit);
		attributes.reset_is_clear_context();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.set_is_evex_instruction();
		vex_prefix(dst, 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xA2);
		emit_operand(src, dst, 0);
	}
	// Carry-Less Multiplication Quadword
	pub fn pclmulqdq(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister, int mask) {
		assert(VM_Version::supports_clmul(), "");
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, dst, src, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int24(0x44, (0xC0 | encode), (unsigned char)mask);
	}

	// Carry-Less Multiplication Quadword
	pub fn vpclmulqdq(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int mask) {
		assert(VM_Version::supports_avx() && VM_Version::supports_clmul(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int24(0x44, (0xC0 | encode), (unsigned char)mask);
	}

	pub fn evpclmulqdq(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int mask, int vector_len) {
		assert(VM_Version::supports_avx512_vpclmulqdq(), "Requires vector carryless multiplication support");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int24(0x44, (0xC0 | encode), (unsigned char)mask);
	}

	pub fn vzeroupper_uncached(&mut self,) {
		if (VM_Version::supports_vzeroupper()) {
			InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
			(void)vex_prefix_and_encode(0, 0, 0, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
			emit_int8(0x77);
		}
	}

	pub fn vfpclassss(&mut self,KRegister kdst, XMMsrc: GPRegister, uint8_t imm8) {
		// Encoding: EVEX.LIG.66.0F3A.W0 67 /r ib
		assert(VM_Version::supports_evex(), "");
		assert(VM_Version::supports_avx512dq(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ false);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(kdst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int24((unsigned char)0x67, (unsigned char)(0xC0 | encode), imm8);
	}

	pub fn vfpclasssd(&mut self,KRegister kdst, XMMsrc: GPRegister, uint8_t imm8) {
		// Encoding: EVEX.LIG.66.0F3A.W1 67 /r ib
		assert(VM_Version::supports_evex(), "");
		assert(VM_Version::supports_avx512dq(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ false);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(kdst->encoding(), 0, src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int24((unsigned char)0x67, (unsigned char)(0xC0 | encode), imm8);
	}

	pub fn fld_x(&mut self,Address adr) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xDB);
		emit_operand32(rbp, adr, 0);
	}

	pub fn fstp_x(&mut self,Address adr) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xDB);
		emit_operand32(rdi, adr, 0);
	}

	pub fn emit_operand32(&mut self,Register reg, Address adr, int post_addr_length) {
		assert(reg->encoding() < 8, "no extended registers");
		assert(!adr.base_needs_rex() && !adr.index_needs_rex(), "no extended registers");
		emit_operand(reg, adr._base, adr._index, adr._scale, adr._disp, adr._rspec, post_addr_length);
	}

	#ifndef _LP64
// 32bit only pieces of the assembler

	pub fn emms(&mut self,) {
		NOT_LP64(assert(VM_Version::supports_mmx(), ""));
		emit_int16(0x0F, 0x77);
	}

	pub fn vzeroupper(&mut self,) {
		vzeroupper_uncached();
	}

	pub fn cmp_literal32(&mut self,src: GPRegister1, imm32: i32, RelocationHolder const& rspec) {
	// NO PREFIX AS NEVER 64BIT
	InstructionMark im(this);
	emit_int16((unsigned char)0x81, (0xF8 | src1->encoding()));
	emit_data(imm32, rspec, 0);
	}

	pub fn cmp_literal32(&mut self,src: Address1, imm32: i32, RelocationHolder const& rspec) {
	// NO PREFIX AS NEVER 64BIT (not even 32bit versions of 64bit regs
	InstructionMark im(this);
	emit_int8((unsigned char)0x81);
	emit_operand(rdi, src1, 4);
	emit_data(imm32, rspec, 0);
	}

	// The 64-bit (32bit platform) cmpxchg compares the value at adr with the contents of rdx:rax,
// and stores rcx:rbx into adr if so; otherwise, the value at adr is loaded
// into rdx:rax.  The ZF is set if the compared values were equal, and cleared otherwise.
	pub fn cmpxchg8(&mut self,Address adr) {
		InstructionMark im(this);
		emit_int16(0x0F, (unsigned char)0xC7);
		emit_operand(rcx, adr, 0);
	}

	pub fn decl(&mut self,dst: GPRegister) {
		// Don't use it directly. Use MacroAssembler::decrementl() instead.
		emit_int8(0x48 | dst->encoding());
	}

// 64bit doesn't use the x87

	pub fn emit_farith(&mut self,int b1, int b2, int i) {
		assert(isByte(b1) && isByte(b2), "wrong opcode");
		assert(0 <= i &&  i < 8, "illegal stack offset");
		emit_int16(b1, b2 + i);
	}

	pub fn fabs(&mut self,) {
		emit_int16((unsigned char)0xD9, (unsigned char)0xE1);
	}

	pub fn fadd(&mut self,int i) {
		emit_farith(0xD8, 0xC0, i);
	}

	pub fn fadd_d(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xDC);
		emit_operand32(rax, src, 0);
	}

	pub fn fadd_s(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xD8);
		emit_operand32(rax, src, 0);
	}

	pub fn fadda(&mut self,int i) {
		emit_farith(0xDC, 0xC0, i);
	}

	pub fn faddp(&mut self,int i) {
		emit_farith(0xDE, 0xC0, i);
	}

	pub fn fchs(&mut self,) {
		emit_int16((unsigned char)0xD9, (unsigned char)0xE0);
	}

	pub fn fcom(&mut self,int i) {
		emit_farith(0xD8, 0xD0, i);
	}

	pub fn fcomp(&mut self,int i) {
		emit_farith(0xD8, 0xD8, i);
	}

	pub fn fcomp_d(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xDC);
		emit_operand32(rbx, src, 0);
	}

	pub fn fcomp_s(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xD8);
		emit_operand32(rbx, src, 0);
	}

	pub fn fcompp(&mut self,) {
		emit_int16((unsigned char)0xDE, (unsigned char)0xD9);
	}

	pub fn fcos(&mut self,) {
		emit_int16((unsigned char)0xD9, (unsigned char)0xFF);
	}

	pub fn fdecstp(&mut self,) {
		emit_int16((unsigned char)0xD9, (unsigned char)0xF6);
	}

	pub fn fdiv(&mut self,int i) {
		emit_farith(0xD8, 0xF0, i);
	}

	pub fn fdiv_d(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xDC);
		emit_operand32(rsi, src, 0);
	}

	pub fn fdiv_s(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xD8);
		emit_operand32(rsi, src, 0);
	}

	pub fn fdiva(&mut self,int i) {
		emit_farith(0xDC, 0xF8, i);
	}

// Note: The Intel manual (Pentium Processor User's Manual, Vol.3, 1994)
//       is erroneous for some of the floating-point instructions below.

	pub fn fdivp(&mut self,int i) {
		emit_farith(0xDE, 0xF8, i);                    // ST(0) <- ST(0) / ST(1) and pop (Intel manual wrong)
	}

	pub fn fdivr(&mut self,int i) {
		emit_farith(0xD8, 0xF8, i);
	}

	pub fn fdivr_d(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xDC);
		emit_operand32(rdi, src, 0);
	}

	pub fn fdivr_s(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xD8);
		emit_operand32(rdi, src, 0);
	}

	pub fn fdivra(&mut self,int i) {
		emit_farith(0xDC, 0xF0, i);
	}

	pub fn fdivrp(&mut self,int i) {
		emit_farith(0xDE, 0xF0, i);                    // ST(0) <- ST(1) / ST(0) and pop (Intel manual wrong)
	}

	pub fn ffree(&mut self,int i) {
		emit_farith(0xDD, 0xC0, i);
	}

	pub fn fild_d(&mut self,Address adr) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xDF);
		emit_operand32(rbp, adr, 0);
	}

	pub fn fild_s(&mut self,Address adr) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xDB);
		emit_operand32(rax, adr, 0);
	}

	pub fn fincstp(&mut self,) {
		emit_int16((unsigned char)0xD9, (unsigned char)0xF7);
	}

	pub fn finit(&mut self,) {
		emit_int24((unsigned char)0x9B, (unsigned char)0xDB, (unsigned char)0xE3);
	}

	pub fn fist_s(&mut self,Address adr) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xDB);
		emit_operand32(rdx, adr, 0);
	}

	pub fn fistp_d(&mut self,Address adr) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xDF);
		emit_operand32(rdi, adr, 0);
	}

	pub fn fistp_s(&mut self,Address adr) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xDB);
		emit_operand32(rbx, adr, 0);
	}

	pub fn fld1(&mut self,) {
		emit_int16((unsigned char)0xD9, (unsigned char)0xE8);
	}

	pub fn fld_d(&mut self,Address adr) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xDD);
		emit_operand32(rax, adr, 0);
	}

	pub fn fld_s(&mut self,Address adr) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xD9);
		emit_operand32(rax, adr, 0);
	}


	pub fn fld_s(&mut self,int index) {
		emit_farith(0xD9, 0xC0, index);
	}

	pub fn fldcw(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xD9);
		emit_operand32(rbp, src, 0);
	}

	pub fn fldenv(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xD9);
		emit_operand32(rsp, src, 0);
	}

	pub fn fldlg2(&mut self,) {
		emit_int16((unsigned char)0xD9, (unsigned char)0xEC);
	}

	pub fn fldln2(&mut self,) {
		emit_int16((unsigned char)0xD9, (unsigned char)0xED);
	}

	pub fn fldz(&mut self,) {
		emit_int16((unsigned char)0xD9, (unsigned char)0xEE);
	}

	pub fn flog(&mut self,) {
		fldln2();
		fxch();
		fyl2x();
	}

	pub fn flog10(&mut self,) {
		fldlg2();
		fxch();
		fyl2x();
	}

	pub fn fmul(&mut self,int i) {
		emit_farith(0xD8, 0xC8, i);
	}

	pub fn fmul_d(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xDC);
		emit_operand32(rcx, src, 0);
	}

	pub fn fmul_s(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xD8);
		emit_operand32(rcx, src, 0);
	}

	pub fn fmula(&mut self,int i) {
		emit_farith(0xDC, 0xC8, i);
	}

	pub fn fmulp(&mut self,int i) {
		emit_farith(0xDE, 0xC8, i);
	}

	pub fn fnsave(&mut self,dst: Address) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xDD);
		emit_operand32(rsi, dst, 0);
	}

	pub fn fnstcw(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int16((unsigned char)0x9B, (unsigned char)0xD9);
		emit_operand32(rdi, src, 0);
	}

	pub fn fnstsw_ax(&mut self,) {
		emit_int16((unsigned char)0xDF, (unsigned char)0xE0);
	}

	pub fn fprem(&mut self,) {
		emit_int16((unsigned char)0xD9, (unsigned char)0xF8);
	}

	pub fn fprem1(&mut self,) {
		emit_int16((unsigned char)0xD9, (unsigned char)0xF5);
	}

	pub fn frstor(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xDD);
		emit_operand32(rsp, src, 0);
	}

	pub fn fsin(&mut self,) {
		emit_int16((unsigned char)0xD9, (unsigned char)0xFE);
	}

	pub fn fsqrt(&mut self,) {
		emit_int16((unsigned char)0xD9, (unsigned char)0xFA);
	}

	pub fn fst_d(&mut self,Address adr) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xDD);
		emit_operand32(rdx, adr, 0);
	}

	pub fn fst_s(&mut self,Address adr) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xD9);
		emit_operand32(rdx, adr, 0);
	}

	pub fn fstp_d(&mut self,Address adr) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xDD);
		emit_operand32(rbx, adr, 0);
	}

	pub fn fstp_d(&mut self,int index) {
		emit_farith(0xDD, 0xD8, index);
	}

	pub fn fstp_s(&mut self,Address adr) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xD9);
		emit_operand32(rbx, adr, 0);
	}

	pub fn fsub(&mut self,int i) {
		emit_farith(0xD8, 0xE0, i);
	}

	pub fn fsub_d(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xDC);
		emit_operand32(rsp, src, 0);
	}

	pub fn fsub_s(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xD8);
		emit_operand32(rsp, src, 0);
	}

	pub fn fsuba(&mut self,int i) {
		emit_farith(0xDC, 0xE8, i);
	}

	pub fn fsubp(&mut self,int i) {
		emit_farith(0xDE, 0xE8, i);                    // ST(0) <- ST(0) - ST(1) and pop (Intel manual wrong)
	}

	pub fn fsubr(&mut self,int i) {
		emit_farith(0xD8, 0xE8, i);
	}

	pub fn fsubr_d(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xDC);
		emit_operand32(rbp, src, 0);
	}

	pub fn fsubr_s(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int8((unsigned char)0xD8);
		emit_operand32(rbp, src, 0);
	}

	pub fn fsubra(&mut self,int i) {
		emit_farith(0xDC, 0xE0, i);
	}

	pub fn fsubrp(&mut self,int i) {
		emit_farith(0xDE, 0xE0, i);                    // ST(0) <- ST(1) - ST(0) and pop (Intel manual wrong)
	}

	pub fn ftan(&mut self,) {
		emit_int32((unsigned char)0xD9, (unsigned char)0xF2, (unsigned char)0xDD, (unsigned char)0xD8);
	}

	pub fn ftst(&mut self,) {
		emit_int16((unsigned char)0xD9, (unsigned char)0xE4);
	}

	pub fn fucomi(&mut self,int i) {
		// make sure the instruction is supported (introduced for P6, together with cmov)
		guarantee(VM_Version::supports_cmov(), "illegal instruction");
		emit_farith(0xDB, 0xE8, i);
	}

	pub fn fucomip(&mut self,int i) {
		// make sure the instruction is supported (introduced for P6, together with cmov)
		guarantee(VM_Version::supports_cmov(), "illegal instruction");
		emit_farith(0xDF, 0xE8, i);
	}

	pub fn fwait(&mut self,) {
		emit_int8((unsigned char)0x9B);
	}

	pub fn fxch(&mut self,int i) {
		emit_farith(0xD9, 0xC8, i);
	}

	pub fn fyl2x(&mut self,) {
		emit_int16((unsigned char)0xD9, (unsigned char)0xF1);
	}

	pub fn frndint(&mut self,) {
		emit_int16((unsigned char)0xD9, (unsigned char)0xFC);
	}

	pub fn f2xm1(&mut self,) {
		emit_int16((unsigned char)0xD9, (unsigned char)0xF0);
	}

	pub fn fldl2e(&mut self,) {
		emit_int16((unsigned char)0xD9, (unsigned char)0xEA);
	}
	#endif // !_LP64

	// SSE SIMD prefix byte values corresponding to VexSimdPrefix encoding.
	static int simd_pre[4] = { 0, 0x66, 0xF3, 0xF2 };
	// SSE opcode second byte values (first is 0x0F) corresponding to VexOpcode encoding.
	static int simd_opc[4] = { 0,    0, 0x38, 0x3A };

	// Generate SSE legacy REX prefix and SIMD opcode based on VEX encoding.
	pub fn rex_prefix(&mut self,Address adr, XMMRegister xreg, VexSimdPrefix pre, VexOpcode opc, bool rex_w) {
		if (pre > 0) {
			emit_int8(simd_pre[pre]);
		}
		if (rex_w) {
			prefixq(adr, xreg);
		} else {
			prefix(adr, xreg);
		}
		if (opc > 0) {
			emit_int8(0x0F);
			int opc2 = simd_opc[opc];
			if (opc2 > 0) {
				emit_int8(opc2);
			}
		}
	}

	int Assembler::rex_prefix_and_encode(int dst_enc, int src_enc, VexSimdPrefix pre, VexOpcode opc, bool rex_w) {
	if (pre > 0) {
	emit_int8(simd_pre[pre]);
	}
	int encode = (rex_w) ? prefixq_and_encode(dst_enc, src_enc) : prefix_and_encode(dst_enc, src_enc);
	if (opc > 0) {
	emit_int8(0x0F);
	int opc2 = simd_opc[opc];
	if (opc2 > 0) {
	emit_int8(opc2);
	}
	}
	return encode;
	}


	pub fn vex_prefix(&mut self,bool vex_r, bool vex_b, bool vex_x, int nds_enc, VexSimdPrefix pre, VexOpcode opc) {
		int vector_len = _attributes->get_vector_len();
		bool vex_w = _attributes->is_rex_vex_w();
		if (vex_b || vex_x || vex_w || (opc == VEX_OPCODE_0F_38) || (opc == VEX_OPCODE_0F_3A)) {
			int byte1 = (vex_r ? VEX_R : 0) | (vex_x ? VEX_X : 0) | (vex_b ? VEX_B : 0);
			byte1 = (~byte1) & 0xE0;
			byte1 |= opc;

			int byte2 = ((~nds_enc) & 0xf) << 3;
			byte2 |= (vex_w ? VEX_W : 0) | ((vector_len > 0) ? 4 : 0) | pre;

			emit_int24((unsigned char)VEX_3bytes, byte1, byte2);
		} else {
			int byte1 = vex_r ? VEX_R : 0;
			byte1 = (~byte1) & 0x80;
			byte1 |= ((~nds_enc) & 0xf) << 3;
			byte1 |= ((vector_len > 0 ) ? 4 : 0) | pre;
			emit_int16((unsigned char)VEX_2bytes, byte1);
		}
	}

	// This is a 4 byte encoding
	pub fn evex_prefix(&mut self,bool vex_r, bool vex_b, bool vex_x, bool evex_r, bool evex_v, int nds_enc, VexSimdPrefix pre, VexOpcode opc){
		// EVEX 0x62 prefix
		// byte1 = EVEX_4bytes;

		bool vex_w = _attributes->is_rex_vex_w();
		int evex_encoding = (vex_w ? VEX_W : 0);
		// EVEX.b is not currently used for broadcast of single element or data rounding modes
		_attributes->set_evex_encoding(evex_encoding);

		// P0: byte 2, initialized to RXBR`00mm
		// instead of not'd
		int byte2 = (vex_r ? VEX_R : 0) | (vex_x ? VEX_X : 0) | (vex_b ? VEX_B : 0) | (evex_r ? EVEX_Rb : 0);
		byte2 = (~byte2) & 0xF0;
		// confine opc opcode extensions in mm bits to lower two bits
		// of form {0F, 0F_38, 0F_3A}
		byte2 |= opc;

		// P1: byte 3 as Wvvvv1pp
		int byte3 = ((~nds_enc) & 0xf) << 3;
		// p[10] is always 1
		byte3 |= EVEX_F;
		byte3 |= (vex_w & 1) << 7;
		// confine pre opcode extensions in pp bits to lower two bits
		// of form {66, F3, F2}
		byte3 |= pre;

		// P2: byte 4 as zL'Lbv'aaa
		// kregs are implemented in the low 3 bits as aaa
		int byte4 = (_attributes->is_no_reg_mask()) ?
		0 :
			_attributes->get_embedded_opmask_register_specifier();
		// EVEX.v` for extending EVEX.vvvv or VIDX
		byte4 |= (evex_v ? 0: EVEX_V);
		// third EXEC.b for broadcast actions
		byte4 |= (_attributes->is_extended_context() ? EVEX_Rb : 0);
		// fourth EVEX.L'L for vector length : 0 is 128, 1 is 256, 2 is 512, currently we do not support 1024
		byte4 |= ((_attributes->get_vector_len())& 0x3) << 5;
		// last is EVEX.z for zero/merge actions
		if (_attributes->is_no_reg_mask() == false &&
			_attributes->get_embedded_opmask_register_specifier() != 0) {
			byte4 |= (_attributes->is_clear_context() ? EVEX_Z : 0);
		}

		emit_int32(EVEX_4bytes, byte2, byte3, byte4);
	}

	pub fn vex_prefix(&mut self,Address adr, int nds_enc, int xreg_enc, VexSimdPrefix pre, VexOpcode opc, InstructionAttr *attributes) {
		bool vex_r = (xreg_enc & 8) == 8;
		bool vex_b = adr.base_needs_rex();
		bool vex_x;
		if (adr.isxmmindex()) {
			vex_x = adr.xmmindex_needs_rex();
		} else {
			vex_x = adr.index_needs_rex();
		}
		set_attributes(attributes);

		// For EVEX instruction (which is not marked as pure EVEX instruction) check and see if this instruction
		// is allowed in legacy mode and has resources which will fit in it.
		// Pure EVEX instructions will have is_evex_instruction set in their definition.
		if (!attributes->is_legacy_mode()) {
			if (UseAVX > 2 && !attributes->is_evex_instruction() && !is_managed()) {
				if ((attributes->get_vector_len() != AVX_512bit) && (nds_enc < 16) && (xreg_enc < 16)) {
					attributes->set_is_legacy_mode();
				}
			}
		}

		if (UseAVX > 2) {
			assert(((!attributes->uses_vl()) ||
				(attributes->get_vector_len() == AVX_512bit) ||
				(!_legacy_mode_vl) ||
					(attributes->is_legacy_mode())),"XMM register should be 0-15");
			assert(((nds_enc < 16 && xreg_enc < 16) || (!attributes->is_legacy_mode())),"XMM register should be 0-15");
		}

		clear_managed();
		if (UseAVX > 2 && !attributes->is_legacy_mode())
		{
			bool evex_r = (xreg_enc >= 16);
			bool evex_v;
			// EVEX.V' is set to true when VSIB is used as we may need to use higher order XMM registers (16-31)
			if (adr.isxmmindex())  {
				evex_v = ((adr._xmmindex->encoding() > 15) ? true : false);
			} else {
				evex_v = (nds_enc >= 16);
			}
			attributes->set_is_evex_instruction();
			evex_prefix(vex_r, vex_b, vex_x, evex_r, evex_v, nds_enc, pre, opc);
		} else {
			if (UseAVX > 2 && attributes->is_rex_vex_w_reverted()) {
				attributes->set_rex_vex_w(false);
			}
			vex_prefix(vex_r, vex_b, vex_x, nds_enc, pre, opc);
		}
	}

	int Assembler::vex_prefix_and_encode(int dst_enc, int nds_enc, int src_enc, VexSimdPrefix pre, VexOpcode opc, InstructionAttr *attributes) {
	bool vex_r = (dst_enc & 8) == 8;
	bool vex_b = (src_enc & 8) == 8;
	bool vex_x = false;
	set_attributes(attributes);

	// For EVEX instruction (which is not marked as pure EVEX instruction) check and see if this instruction
	// is allowed in legacy mode and has resources which will fit in it.
	// Pure EVEX instructions will have is_evex_instruction set in their definition.
	if (!attributes->is_legacy_mode()) {
	if (UseAVX > 2 && !attributes->is_evex_instruction() && !is_managed()) {
	if ((!attributes->uses_vl() || (attributes->get_vector_len() != AVX_512bit)) &&
	(dst_enc < 16) && (nds_enc < 16) && (src_enc < 16)) {
	attributes->set_is_legacy_mode();
	}
	}
	}

	if (UseAVX > 2) {
	// All the scalar fp instructions (with uses_vl as false) can have legacy_mode as false
	// Instruction with uses_vl true are vector instructions
	// All the vector instructions with AVX_512bit length can have legacy_mode as false
	// All the vector instructions with < AVX_512bit length can have legacy_mode as false if AVX512vl() is supported
	// Rest all should have legacy_mode set as true
	assert(((!attributes->uses_vl()) ||
	(attributes->get_vector_len() == AVX_512bit) ||
	(!_legacy_mode_vl) ||
	(attributes->is_legacy_mode())),"XMM register should be 0-15");
	// Instruction with legacy_mode true should have dst, nds and src < 15
	assert(((dst_enc < 16 && nds_enc < 16 && src_enc < 16) || (!attributes->is_legacy_mode())),"XMM register should be 0-15");
	}

	clear_managed();
	if (UseAVX > 2 && !attributes->is_legacy_mode())
	{
	bool evex_r = (dst_enc >= 16);
	bool evex_v = (nds_enc >= 16);
	// can use vex_x as bank extender on rm encoding
	vex_x = (src_enc >= 16);
	attributes->set_is_evex_instruction();
	evex_prefix(vex_r, vex_b, vex_x, evex_r, evex_v, nds_enc, pre, opc);
	} else {
	if (UseAVX > 2 && attributes->is_rex_vex_w_reverted()) {
	attributes->set_rex_vex_w(false);
	}
	vex_prefix(vex_r, vex_b, vex_x, nds_enc, pre, opc);
	}

	// return modrm byte components for operands
	return (((dst_enc & 7) << 3) | (src_enc & 7));
	}


	pub fn simd_prefix(&mut self,XMMRegister xreg, nds: XMMRegister, Address adr, VexSimdPrefix pre,
					   VexOpcode opc, InstructionAttr *attributes) {
		if (UseAVX > 0) {
			int xreg_enc = xreg->encoding();
			int nds_enc = nds->is_valid() ? nds->encoding() : 0;
			vex_prefix(adr, nds_enc, xreg_enc, pre, opc, attributes);
		} else {
			assert((nds == xreg) || (nds == xnoreg), "wrong sse encoding");
			rex_prefix(adr, xreg, pre, opc, attributes->is_rex_vex_w());
		}
	}

	int Assembler::simd_prefix_and_encode(XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, VexSimdPrefix pre,
	VexOpcode opc, InstructionAttr *attributes) {
	int dst_enc = dst->encoding();
	int src_enc = src->encoding();
	if (UseAVX > 0) {
	int nds_enc = nds->is_valid() ? nds->encoding() : 0;
	return vex_prefix_and_encode(dst_enc, nds_enc, src_enc, pre, opc, attributes);
	} else {
	assert((nds == dst) || (nds == src) || (nds == xnoreg), "wrong sse encoding");
	return rex_prefix_and_encode(dst_enc, src_enc, pre, opc, attributes->is_rex_vex_w());
	}
	}

	pub fn vmaxss(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5F, (0xC0 | encode));
	}

	pub fn vmaxsd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5F, (0xC0 | encode));
	}

	pub fn vminss(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5D, (0xC0 | encode));
	}

	pub fn vminsd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ VM_Version::supports_evex(), /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		attributes.set_rex_vex_w_reverted();
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
		emit_int16(0x5D, (0xC0 | encode));
	}

	pub fn vcmppd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int cop, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		assert(vector_len <= AVX_256bit, "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = simd_prefix_and_encode(dst, nds, src, VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24((unsigned char)0xC2, (0xC0 | encode), (0xF & cop));
	}

	pub fn blendvpb(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister1, XMMsrc: GPRegister2, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		assert(vector_len <= AVX_256bit, "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src1->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		int src2_enc = src2->encoding();
		emit_int24(0x4C, (0xC0 | encode), (0xF0 & src2_enc << 4));
	}

	pub fn vblendvpd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister1, XMMsrc: GPRegister2, int vector_len) {
		assert(UseAVX > 0 && (vector_len == AVX_128bit || vector_len == AVX_256bit), "");
		assert(vector_len <= AVX_256bit, "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src1->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		int src2_enc = src2->encoding();
		emit_int24(0x4B, (0xC0 | encode), (0xF0 & src2_enc << 4));
	}

	pub fn vpblendd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, imm8: i8, int vector_len) {
		assert(VM_Version::supports_avx2(), "");
		assert(vector_len <= AVX_256bit, "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int24(0x02, (0xC0 | encode), (unsigned char)imm8);
	}

	pub fn vcmpps(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int comparison, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		assert(vector_len <= AVX_256bit, "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int24((unsigned char)0xC2, (0xC0 | encode), (unsigned char)comparison);
	}

	pub fn evcmpps(&mut self,KRegister kdst, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister,
				   ComparisonPredicateFP comparison, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		// Encoding: EVEX.NDS.XXX.0F.W0 C2 /r ib
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.reset_is_clear_context();
		int encode = vex_prefix_and_encode(kdst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int24((unsigned char)0xC2, (0xC0 | encode), comparison);
	}

	pub fn evcmppd(&mut self,KRegister kdst, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister,
				   ComparisonPredicateFP comparison, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		// Encoding: EVEX.NDS.XXX.66.0F.W1 C2 /r ib
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.reset_is_clear_context();
		int encode = vex_prefix_and_encode(kdst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24((unsigned char)0xC2, (0xC0 | encode), comparison);
	}

	pub fn blendvps(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_sse4_1(), "");
		assert(UseAVX <= 0, "sse encoding is inconsistent with avx encoding");
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x14, (0xC0 | encode));
	}

	pub fn blendvpd(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_sse4_1(), "");
		assert(UseAVX <= 0, "sse encoding is inconsistent with avx encoding");
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x15, (0xC0 | encode));
	}

	pub fn pblendvb(&mut self,XMMdst: GPRegister, XMMsrc: GPRegister) {
		assert(VM_Version::supports_sse4_1(), "");
		assert(UseAVX <= 0, "sse encoding is inconsistent with avx encoding");
		InstructionAttr attributes(AVX_128bit, /* rex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = simd_prefix_and_encode(dst, xnoreg, src, VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x10, (0xC0 | encode));
	}

	pub fn vblendvps(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister1, XMMsrc: GPRegister2, int vector_len) {
		assert(UseAVX > 0 && (vector_len == AVX_128bit || vector_len == AVX_256bit), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src1->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		int src2_enc = src2->encoding();
		emit_int24(0x4A, (0xC0 | encode), (0xF0 & src2_enc << 4));
	}

	pub fn vblendps(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, imm8: i8, int vector_len) {
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		emit_int24(0x0C, (0xC0 | encode), imm8);
	}

	pub fn vpcmpgtb(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(vector_len == AVX_128bit ? VM_Version::supports_avx() : VM_Version::supports_avx2(), "");
		assert(vector_len <= AVX_256bit, "evex encoding is different - has k register as dest");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x64, (0xC0 | encode));
	}

	pub fn vpcmpgtw(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(vector_len == AVX_128bit ? VM_Version::supports_avx() : VM_Version::supports_avx2(), "");
		assert(vector_len <= AVX_256bit, "evex encoding is different - has k register as dest");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x65, (0xC0 | encode));
	}

	pub fn vpcmpgtd(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(vector_len == AVX_128bit ? VM_Version::supports_avx() : VM_Version::supports_avx2(), "");
		assert(vector_len <= AVX_256bit, "evex encoding is different - has k register as dest");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x66, (0xC0 | encode));
	}

	pub fn vpcmpgtq(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, int vector_len) {
		assert(vector_len == AVX_128bit ? VM_Version::supports_avx() : VM_Version::supports_avx2(), "");
		assert(vector_len <= AVX_256bit, "evex encoding is different - has k register as dest");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x37, (0xC0 | encode));
	}

	pub fn evpcmpd(&mut self,KRegister kdst, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister,
				   int comparison, bool is_signed, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(comparison >= Assembler::eq && comparison <= Assembler::_true, "");
		// Encoding: EVEX.NDS.XXX.66.0F3A.W0 1F /r ib
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.reset_is_clear_context();
		int encode = vex_prefix_and_encode(kdst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		int opcode = is_signed ? 0x1F : 0x1E;
		emit_int24(opcode, (0xC0 | encode), comparison);
	}

	pub fn evpcmpd(&mut self,KRegister kdst, KRegister mask, nds: XMMRegister, src: Address,
				   int comparison, bool is_signed, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(comparison >= Assembler::eq && comparison <= Assembler::_true, "");
		// Encoding: EVEX.NDS.XXX.66.0F3A.W0 1F /r ib
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_NObit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.reset_is_clear_context();
		int dst_enc = kdst->encoding();
		vex_prefix(src, nds->encoding(), dst_enc, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		int opcode = is_signed ? 0x1F : 0x1E;
		emit_int8((unsigned char)opcode);
		emit_operand(as_Register(dst_enc), src, 1);
		emit_int8((unsigned char)comparison);
	}

	pub fn evpcmpq(&mut self,KRegister kdst, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister,
				   int comparison, bool is_signed, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(comparison >= Assembler::eq && comparison <= Assembler::_true, "");
		// Encoding: EVEX.NDS.XXX.66.0F3A.W1 1F /r ib
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.reset_is_clear_context();
		int encode = vex_prefix_and_encode(kdst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		int opcode = is_signed ? 0x1F : 0x1E;
		emit_int24(opcode, (0xC0 | encode), comparison);
	}

	pub fn evpcmpq(&mut self,KRegister kdst, KRegister mask, nds: XMMRegister, src: Address,
				   int comparison, bool is_signed, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(comparison >= Assembler::eq && comparison <= Assembler::_true, "");
		// Encoding: EVEX.NDS.XXX.66.0F3A.W1 1F /r ib
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FV, /* input_size_in_bits */ EVEX_NObit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.reset_is_clear_context();
		int dst_enc = kdst->encoding();
		vex_prefix(src, nds->encoding(), dst_enc, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		int opcode = is_signed ? 0x1F : 0x1E;
		emit_int8((unsigned char)opcode);
		emit_operand(as_Register(dst_enc), src, 1);
		emit_int8((unsigned char)comparison);
	}

	pub fn evpcmpb(&mut self,KRegister kdst, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister,
				   int comparison, bool is_signed, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(VM_Version::supports_avx512bw(), "");
		assert(comparison >= Assembler::eq && comparison <= Assembler::_true, "");
		// Encoding: EVEX.NDS.XXX.66.0F3A.W0 3F /r ib
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.reset_is_clear_context();
		int encode = vex_prefix_and_encode(kdst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		int opcode = is_signed ? 0x3F : 0x3E;
		emit_int24(opcode, (0xC0 | encode), comparison);
	}

	pub fn evpcmpb(&mut self,KRegister kdst, KRegister mask, nds: XMMRegister, src: Address,
				   int comparison, bool is_signed, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(VM_Version::supports_avx512bw(), "");
		assert(comparison >= Assembler::eq && comparison <= Assembler::_true, "");
		// Encoding: EVEX.NDS.XXX.66.0F3A.W0 3F /r ib
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.reset_is_clear_context();
		int dst_enc = kdst->encoding();
		vex_prefix(src, nds->encoding(), dst_enc, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		int opcode = is_signed ? 0x3F : 0x3E;
		emit_int8((unsigned char)opcode);
		emit_operand(as_Register(dst_enc), src, 1);
		emit_int8((unsigned char)comparison);
	}

	pub fn evpcmpw(&mut self,KRegister kdst, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister,
				   int comparison, bool is_signed, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(VM_Version::supports_avx512bw(), "");
		assert(comparison >= Assembler::eq && comparison <= Assembler::_true, "");
		// Encoding: EVEX.NDS.XXX.66.0F3A.W1 3F /r ib
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.reset_is_clear_context();
		int encode = vex_prefix_and_encode(kdst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		int opcode = is_signed ? 0x3F : 0x3E;
		emit_int24(opcode, (0xC0 | encode), comparison);
	}

	pub fn evpcmpw(&mut self,KRegister kdst, KRegister mask, nds: XMMRegister, src: Address,
				   int comparison, bool is_signed, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(VM_Version::supports_avx512bw(), "");
		assert(comparison >= Assembler::eq && comparison <= Assembler::_true, "");
		// Encoding: EVEX.NDS.XXX.66.0F3A.W1 3F /r ib
		InstructionMark im(this);
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_address_attributes(/* tuple_type */ EVEX_FVM, /* input_size_in_bits */ EVEX_NObit);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.reset_is_clear_context();
		int dst_enc = kdst->encoding();
		vex_prefix(src, nds->encoding(), dst_enc, VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		int opcode = is_signed ? 0x3F : 0x3E;
		emit_int8((unsigned char)opcode);
		emit_operand(as_Register(dst_enc), src, 1);
		emit_int8((unsigned char)comparison);
	}

	pub fn evprord(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, int shift, bool merge, int vector_len) {
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(xmm0->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x72, (0xC0 | encode), shift & 0xFF);
	}

	pub fn evprorq(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, int shift, bool merge, int vector_len) {
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(xmm0->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x72, (0xC0 | encode), shift & 0xFF);
	}

	pub fn evprorvd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x14, (0xC0 | encode));
	}

	pub fn evprorvq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x14, (0xC0 | encode));
	}

	pub fn evprold(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, int shift, bool merge, int vector_len) {
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(xmm1->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x72, (0xC0 | encode), shift & 0xFF);
	}

	pub fn evprolq(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, int shift, bool merge, int vector_len) {
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(xmm1->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int24(0x72, (0xC0 | encode), shift & 0xFF);
	}

	pub fn evprolvd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x15, (0xC0 | encode));
	}

	pub fn evprolvq(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x15, (0xC0 | encode));
	}

	pub fn vpblendvb(&mut self,XMMdst: GPRegister, nds: XMMRegister, XMMsrc: GPRegister, XMMRegister mask, int vector_len) {
		assert(VM_Version::supports_avx(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_3A, &attributes);
		int mask_enc = mask->encoding();
		emit_int24(0x4C, (0xC0 | encode), 0xF0 & mask_enc << 4);
	}

	pub fn evblendmpd(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		// Encoding: EVEX.NDS.XXX.66.0F38.W1 65 /r
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x65, (0xC0 | encode));
	}

	pub fn evblendmps(&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		// Encoding: EVEX.NDS.XXX.66.0F38.W0 65 /r
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x65, (0xC0 | encode));
	}

	pub fn evpblendmb (&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(VM_Version::supports_avx512bw(), "");
		// Encoding: EVEX.NDS.512.66.0F38.W0 66 /r
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x66, (0xC0 | encode));
	}

	pub fn evpblendmw (&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(VM_Version::supports_avx512bw(), "");
		// Encoding: EVEX.NDS.512.66.0F38.W1 66 /r
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ _legacy_mode_bw, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x66, (0xC0 | encode));
	}

	pub fn evpblendmd (&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		//Encoding: EVEX.NDS.512.66.0F38.W0 64 /r
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x64, (0xC0 | encode));
	}

	pub fn evpblendmq (&mut self,XMMdst: GPRegister, KRegister mask, nds: XMMRegister, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		//Encoding: EVEX.NDS.512.66.0F38.W1 64 /r
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		attributes.set_embedded_opmask_register_specifier(mask);
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(dst->encoding(), nds->encoding(), src->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x64, (0xC0 | encode));
	}

	pub fn bzhiq(&mut self,dst: GPRegister, src: GPRegister1, src: GPRegister2) {
		assert(VM_Version::supports_bmi2(), "bit manipulation instructions not supported");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), src2->encoding(), src1->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xF5, (0xC0 | encode));
	}

	pub fn pextl(&mut self,dst: GPRegister, src: GPRegister1, src: GPRegister2) {
		assert(VM_Version::supports_bmi2(), "bit manipulation instructions not supported");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xF5, (0xC0 | encode));
	}

	pub fn pdepl(&mut self,dst: GPRegister, src: GPRegister1, src: GPRegister2) {
		assert(VM_Version::supports_bmi2(), "bit manipulation instructions not supported");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xF5, (0xC0 | encode));
	}

	pub fn pextq(&mut self,dst: GPRegister, src: GPRegister1, src: GPRegister2) {
		assert(VM_Version::supports_bmi2(), "bit manipulation instructions not supported");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xF5, (0xC0 | encode));
	}

	pub fn pdepq(&mut self,dst: GPRegister, src: GPRegister1, src: GPRegister2) {
		assert(VM_Version::supports_bmi2(), "bit manipulation instructions not supported");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xF5, (0xC0 | encode));
	}

	pub fn pextl(&mut self,dst: GPRegister, src: GPRegister1, src: Address2) {
		assert(VM_Version::supports_bmi2(), "bit manipulation instructions not supported");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		vex_prefix(src2, src1->encoding(), dst->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xF5);
		emit_operand(dst, src2, 0);
	}

	pub fn pdepl(&mut self,dst: GPRegister, src: GPRegister1, src: Address2) {
		assert(VM_Version::supports_bmi2(), "bit manipulation instructions not supported");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		vex_prefix(src2, src1->encoding(), dst->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xF5);
		emit_operand(dst, src2, 0);
	}

	pub fn pextq(&mut self,dst: GPRegister, src: GPRegister1, src: Address2) {
		assert(VM_Version::supports_bmi2(), "bit manipulation instructions not supported");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		vex_prefix(src2, src1->encoding(), dst->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xF5);
		emit_operand(dst, src2, 0);
	}

	pub fn pdepq(&mut self,dst: GPRegister, src: GPRegister1, src: Address2) {
		assert(VM_Version::supports_bmi2(), "bit manipulation instructions not supported");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		vex_prefix(src2, src1->encoding(), dst->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xF5);
		emit_operand(dst, src2, 0);
	}

	pub fn sarxl(&mut self,dst: GPRegister, src: GPRegister1, src: GPRegister2) {
		assert(VM_Version::supports_bmi2(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src2->encoding(), src1->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xF7, (0xC0 | encode));
	}

	pub fn sarxl(&mut self,dst: GPRegister, src: Address1, src: GPRegister2) {
		assert(VM_Version::supports_bmi2(), "");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		vex_prefix(src1, src2->encoding(), dst->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xF7);
		emit_operand(dst, src1, 0);
	}

	pub fn sarxq(&mut self,dst: GPRegister, src: GPRegister1, src: GPRegister2) {
		assert(VM_Version::supports_bmi2(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src2->encoding(), src1->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xF7, (0xC0 | encode));
	}

	pub fn sarxq(&mut self,dst: GPRegister, src: Address1, src: GPRegister2) {
		assert(VM_Version::supports_bmi2(), "");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		vex_prefix(src1, src2->encoding(), dst->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xF7);
		emit_operand(dst, src1, 0);
	}

	pub fn shlxl(&mut self,dst: GPRegister, src: GPRegister1, src: GPRegister2) {
		assert(VM_Version::supports_bmi2(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src2->encoding(), src1->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xF7, (0xC0 | encode));
	}

	pub fn shlxl(&mut self,dst: GPRegister, src: Address1, src: GPRegister2) {
		assert(VM_Version::supports_bmi2(), "");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		vex_prefix(src1, src2->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xF7);
		emit_operand(dst, src1, 0);
	}

	pub fn shlxq(&mut self,dst: GPRegister, src: GPRegister1, src: GPRegister2) {
		assert(VM_Version::supports_bmi2(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src2->encoding(), src1->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xF7, (0xC0 | encode));
	}

	pub fn shlxq(&mut self,dst: GPRegister, src: Address1, src: GPRegister2) {
		assert(VM_Version::supports_bmi2(), "");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		vex_prefix(src1, src2->encoding(), dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xF7);
		emit_operand(dst, src1, 0);
	}

	pub fn shrxl(&mut self,dst: GPRegister, src: GPRegister1, src: GPRegister2) {
		assert(VM_Version::supports_bmi2(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src2->encoding(), src1->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xF7, (0xC0 | encode));
	}

	pub fn shrxl(&mut self,dst: GPRegister, src: Address1, src: GPRegister2) {
		assert(VM_Version::supports_bmi2(), "");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		vex_prefix(src1, src2->encoding(), dst->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xF7);
		emit_operand(dst, src1, 0);
	}

	pub fn shrxq(&mut self,dst: GPRegister, src: GPRegister1, src: GPRegister2) {
		assert(VM_Version::supports_bmi2(), "");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		int encode = vex_prefix_and_encode(dst->encoding(), src2->encoding(), src1->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xF7, (0xC0 | encode));
	}

	pub fn shrxq(&mut self,dst: GPRegister, src: Address1, src: GPRegister2) {
		assert(VM_Version::supports_bmi2(), "");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ true);
		vex_prefix(src1, src2->encoding(), dst->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xF7);
		emit_operand(dst, src1, 0);
	}

	pub fn evpmovq2m(&mut self,Kdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx512vldq(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x39, (0xC0 | encode));
	}

	pub fn evpmovd2m(&mut self,Kdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx512vldq(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x39, (0xC0 | encode));
	}

	pub fn evpmovw2m(&mut self,Kdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx512vlbw(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x29, (0xC0 | encode));
	}

	pub fn evpmovb2m(&mut self,Kdst: GPRegister, XMMsrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx512vlbw(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x29, (0xC0 | encode));
	}

	pub fn evpmovm2q(&mut self,XMMdst: GPRegister, Ksrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx512vldq(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x38, (0xC0 | encode));
	}

	pub fn evpmovm2d(&mut self,XMMdst: GPRegister, Ksrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx512vldq(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x38, (0xC0 | encode));
	}

	pub fn evpmovm2w(&mut self,XMMdst: GPRegister, Ksrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx512vlbw(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x28, (0xC0 | encode));
	}

	pub fn evpmovm2b(&mut self,XMMdst: GPRegister, Ksrc: GPRegister, int vector_len) {
		assert(VM_Version::supports_avx512vlbw(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ true);
		attributes.set_is_evex_instruction();
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F3, VEX_OPCODE_0F_38, &attributes);
		emit_int16(0x28, (0xC0 | encode));
	}

	pub fn evpcompressb(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512_vbmi2(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.set_is_evex_instruction();
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0x63, (0xC0 | encode));
	}

	pub fn evpcompressw(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_avx512_vbmi2(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.set_is_evex_instruction();
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0x63, (0xC0 | encode));
	}

	pub fn evpcompressd(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.set_is_evex_instruction();
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0x8B, (0xC0 | encode));
	}

	pub fn evpcompressq(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.set_is_evex_instruction();
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0x8B, (0xC0 | encode));
	}

	pub fn evcompressps(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ false, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.set_is_evex_instruction();
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0x8A, (0xC0 | encode));
	}

	pub fn evcompresspd(&mut self,XMMdst: GPRegister, KRegister mask, XMMsrc: GPRegister, bool merge, int vector_len) {
		assert(VM_Version::supports_evex(), "");
		assert(vector_len == AVX_512bit || VM_Version::supports_avx512vl(), "");
		InstructionAttr attributes(vector_len, /* vex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ false, /* uses_vl */ true);
		attributes.set_embedded_opmask_register_specifier(mask);
		attributes.set_is_evex_instruction();
		if (merge) {
			attributes.reset_is_clear_context();
		}
		int encode = vex_prefix_and_encode(src->encoding(), 0, dst->encoding(), VEX_SIMD_66, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0x8A, (0xC0 | encode));
	}

	#ifndef _LP64

	pub fn incl(&mut self,dst: GPRegister) {
		// Don't use it directly. Use MacroAssembler::incrementl() instead.
		emit_int8(0x40 | dst->encoding());
	}

	pub fn lea(&mut self,dst: GPRegister, src: Address) {
		leal(dst, src);
	}

	pub fn mov_literal32(&mut self,dst: Address, imm32: i32, RelocationHolder const& rspec) {
	InstructionMark im(this);
	emit_int8((unsigned char)0xC7);
	emit_operand(rax, dst, 4);
	emit_data((int)imm32, rspec, 0);
	}

	pub fn mov_literal32(&mut self,dst: GPRegister, imm32: i32, RelocationHolder const& rspec) {
	InstructionMark im(this);
	int encode = prefix_and_encode(dst->encoding());
	emit_int8((0xB8 | encode));
	emit_data((int)imm32, rspec, 0);
	}

	pub fn popa(&mut self,) { // 32bit
		emit_int8(0x61);
	}

	pub fn push_literal32(&mut self,imm32: i32, RelocationHolder const& rspec) {
	InstructionMark im(this);
	emit_int8(0x68);
	emit_data(imm32, rspec, 0);
	}

	pub fn pusha(&mut self,) { // 32bit
		emit_int8(0x60);
	}

	#else // LP64

// 64bit only pieces of the assembler

// This should only be used by 64bit instructions that can use rip-relative
// it cannot be used by instructions that want an immediate value.

	// Determine whether an address is always reachable in rip-relative addressing mode
// when accessed from the code cache.
	static bool is_always_reachable(address target, relocInfo::relocType reloc_type) {
	switch (reloc_type) {
	// This should be rip-relative and easily reachable.
	case relocInfo::internal_word_type: {
	return true;
	}
	// This should be rip-relative within the code cache and easily
	// reachable until we get huge code caches. (At which point
	// IC code is going to have issues).
	case relocInfo::virtual_call_type:
	case relocInfo::opt_virtual_call_type:
	case relocInfo::static_call_type:
	case relocInfo::static_stub_type: {
	return true;
	}
	case relocInfo::runtime_call_type:
	case relocInfo::external_word_type:
	case relocInfo::poll_return_type: // these are really external_word but need special
	case relocInfo::poll_type: {      // relocs to identify them
	return CodeCache::contains(target);
	}
	default: {
	return false;
	}
	}
	}

	// Determine whether an address is reachable in rip-relative addressing mode from the code cache.
	static bool is_reachable(address target, relocInfo::relocType reloc_type) {
	if (is_always_reachable(target, reloc_type)) {
	return true;
	}
	switch (reloc_type) {
	// None will force a 64bit literal to the code stream. Likely a placeholder
	// for something that will be patched later and we need to certain it will
	// always be reachable.
	case relocInfo::none: {
	return false;
	}
	case relocInfo::runtime_call_type:
	case relocInfo::external_word_type:
	case relocInfo::poll_return_type: // these are really external_word but need special
	case relocInfo::poll_type: {      // relocs to identify them
	assert(!CodeCache::contains(target), "always reachable");
	if (ForceUnreachable) {
	return false; // stress the correction code
	}
	// For external_word_type/runtime_call_type if it is reachable from where we
	// are now (possibly a temp buffer) and where we might end up
	// anywhere in the code cache then we are always reachable.
	// This would have to change if we ever save/restore shared code to be more pessimistic.
	// Code buffer has to be allocated in the code cache, so check against
	// code cache boundaries cover that case.
	//
	// In rip-relative addressing mode, an effective address is formed by adding displacement
	// to the 64-bit RIP of the next instruction which is not known yet. Considering target address
	// is guaranteed to be outside of the code cache, checking against code cache boundaries is enough
	// to account for that.
	return Assembler::is_simm32(target - CodeCache::low_bound()) &&
	Assembler::is_simm32(target - CodeCache::high_bound());
	}
	default: {
	return false;
	}
	}
	}

	bool Assembler::reachable(AddressLiteral adr) {
	assert(CodeCache::contains(pc()), "required");
	if (adr.is_lval()) {
	return false;
	}
	return is_reachable(adr.target(), adr.reloc());
	}

	bool Assembler::always_reachable(AddressLiteral adr) {
	assert(CodeCache::contains(pc()), "required");
	if (adr.is_lval()) {
	return false;
	}
	return is_always_reachable(adr.target(), adr.reloc());
	}

	pub fn emit_data64(&mut self,jlong data,
					   relocInfo::relocType rtype,
					   int format) {
		if (rtype == relocInfo::none) {
			emit_int64(data);
		} else {
			emit_data64(data, Relocation::spec_simple(rtype), format);
		}
	}

	pub fn emit_data64(&mut self,jlong data,
					   RelocationHolder const& rspec,
	int format) {
	assert(imm_operand == 0, "default format must be immediate in this file");
	assert(imm_operand == format, "must be immediate");
	assert(inst_mark() != nullptr, "must be inside InstructionMark");
	// Do not use AbstractAssembler::relocate, which is not intended for
	// embedded words.  Instead, relocate to the enclosing instruction.
	code_section()->relocate(inst_mark(), rspec, format);
	#ifdef ASSERT
	check_relocation(rspec, format);
	#endif
	emit_int64(data);
	}

	pub fn prefix(&mut self,Register reg) {
		if (reg->encoding() >= 8) {
			prefix(REX_B);
		}
	}

	pub fn prefix(&mut self,dst: GPRegister, src: GPRegister, Prefix p) {
		if (src->encoding() >= 8) {
			p = (Prefix)(p | REX_B);
		}
		if (dst->encoding() >= 8) {
			p = (Prefix)(p | REX_R);
		}
		if (p != Prefix_EMPTY) {
			// do not generate an empty prefix
			prefix(p);
		}
	}

	pub fn prefix(&mut self,dst: GPRegister, Address adr, Prefix p) {
		if (adr.base_needs_rex()) {
			if (adr.index_needs_rex()) {
				assert(false, "prefix(dst: GPRegister, Address adr, Prefix p) does not support handling of an X");
			} else {
				p = (Prefix)(p | REX_B);
			}
		} else {
			if (adr.index_needs_rex()) {
				assert(false, "prefix(dst: GPRegister, Address adr, Prefix p) does not support handling of an X");
			}
		}
		if (dst->encoding() >= 8) {
			p = (Prefix)(p | REX_R);
		}
		if (p != Prefix_EMPTY) {
			// do not generate an empty prefix
			prefix(p);
		}
	}

	pub fn prefix(&mut self,Address adr) {
		if (adr.base_needs_rex()) {
			if (adr.index_needs_rex()) {
				prefix(REX_XB);
			} else {
				prefix(REX_B);
			}
		} else {
			if (adr.index_needs_rex()) {
				prefix(REX_X);
			}
		}
	}

	pub fn prefix(&mut self,Address adr, Register reg, bool byteinst) {
		if (reg->encoding() < 8) {
			if (adr.base_needs_rex()) {
				if (adr.index_needs_rex()) {
					prefix(REX_XB);
				} else {
					prefix(REX_B);
				}
			} else {
				if (adr.index_needs_rex()) {
					prefix(REX_X);
				} else if (byteinst && reg->encoding() >= 4) {
					prefix(REX);
				}
			}
		} else {
			if (adr.base_needs_rex()) {
				if (adr.index_needs_rex()) {
					prefix(REX_RXB);
				} else {
					prefix(REX_RB);
				}
			} else {
				if (adr.index_needs_rex()) {
					prefix(REX_RX);
				} else {
					prefix(REX_R);
				}
			}
		}
	}

	pub fn prefix(&mut self,Address adr, XMMRegister reg) {
		if (reg->encoding() < 8) {
			if (adr.base_needs_rex()) {
				if (adr.index_needs_rex()) {
					prefix(REX_XB);
				} else {
					prefix(REX_B);
				}
			} else {
				if (adr.index_needs_rex()) {
					prefix(REX_X);
				}
			}
		} else {
			if (adr.base_needs_rex()) {
				if (adr.index_needs_rex()) {
					prefix(REX_RXB);
				} else {
					prefix(REX_RB);
				}
			} else {
				if (adr.index_needs_rex()) {
					prefix(REX_RX);
				} else {
					prefix(REX_R);
				}
			}
		}
	}

	int Assembler::prefix_and_encode(int reg_enc, bool byteinst) {
	if (reg_enc >= 8) {
	prefix(REX_B);
	reg_enc -= 8;
	} else if (byteinst && reg_enc >= 4) {
	prefix(REX);
	}
	return reg_enc;
	}

	int Assembler::prefix_and_encode(int dst_enc, bool dst_is_byte, int src_enc, bool src_is_byte) {
	if (dst_enc < 8) {
	if (src_enc >= 8) {
	prefix(REX_B);
	src_enc -= 8;
	} else if ((src_is_byte && src_enc >= 4) || (dst_is_byte && dst_enc >= 4)) {
	prefix(REX);
	}
	} else {
	if (src_enc < 8) {
	prefix(REX_R);
	} else {
	prefix(REX_RB);
	src_enc -= 8;
	}
	dst_enc -= 8;
	}
	return dst_enc << 3 | src_enc;
	}

	int8_t Assembler::get_prefixq(Address adr) {
	int8_t prfx = get_prefixq(adr, rax);
	assert(REX_W <= prfx && prfx <= REX_WXB, "must be");
	return prfx;
	}

	int8_t Assembler::get_prefixq(Address adr, src: GPRegister) {
	int8_t prfx = (int8_t)(REX_W +
	((int)adr.base_needs_rex()) +
	((int)adr.index_needs_rex() << 1) +
	((int)(src->encoding() >= 8) << 2));
	#ifdef ASSERT
	if (src->encoding() < 8) {
	if (adr.base_needs_rex()) {
	if (adr.index_needs_rex()) {
	assert(prfx == REX_WXB, "must be");
	} else {
	assert(prfx == REX_WB, "must be");
	}
	} else {
	if (adr.index_needs_rex()) {
	assert(prfx == REX_WX, "must be");
	} else {
	assert(prfx == REX_W, "must be");
	}
	}
	} else {
	if (adr.base_needs_rex()) {
	if (adr.index_needs_rex()) {
	assert(prfx == REX_WRXB, "must be");
	} else {
	assert(prfx == REX_WRB, "must be");
	}
	} else {
	if (adr.index_needs_rex()) {
	assert(prfx == REX_WRX, "must be");
	} else {
	assert(prfx == REX_WR, "must be");
	}
	}
	}
	#endif
	return prfx;
	}

	pub fn prefixq(&mut self,Address adr) {
		emit_int8(get_prefixq(adr));
	}

	pub fn prefixq(&mut self,Address adr, src: GPRegister) {
		emit_int8(get_prefixq(adr, src));
	}

	pub fn prefixq(&mut self,Address adr, XMMsrc: GPRegister) {
		if (src->encoding() < 8) {
			if (adr.base_needs_rex()) {
				if (adr.index_needs_rex()) {
					prefix(REX_WXB);
				} else {
					prefix(REX_WB);
				}
			} else {
				if (adr.index_needs_rex()) {
					prefix(REX_WX);
				} else {
					prefix(REX_W);
				}
			}
		} else {
			if (adr.base_needs_rex()) {
				if (adr.index_needs_rex()) {
					prefix(REX_WRXB);
				} else {
					prefix(REX_WRB);
				}
			} else {
				if (adr.index_needs_rex()) {
					prefix(REX_WRX);
				} else {
					prefix(REX_WR);
				}
			}
		}
	}

	int Assembler::prefixq_and_encode(int reg_enc) {
	if (reg_enc < 8) {
	prefix(REX_W);
	} else {
	prefix(REX_WB);
	reg_enc -= 8;
	}
	return reg_enc;
	}

	int Assembler::prefixq_and_encode(int dst_enc, int src_enc) {
	if (dst_enc < 8) {
	if (src_enc < 8) {
	prefix(REX_W);
	} else {
	prefix(REX_WB);
	src_enc -= 8;
	}
	} else {
	if (src_enc < 8) {
	prefix(REX_WR);
	} else {
	prefix(REX_WRB);
	src_enc -= 8;
	}
	dst_enc -= 8;
	}
	return dst_enc << 3 | src_enc;
	}

	pub fn adcq(&mut self,dst: GPRegister, imm32: i32) {
		(void) prefixq_and_encode(dst->encoding());
		emit_arith(0x81, 0xD0, dst, imm32);
	}

	pub fn adcq(&mut self,dst: GPRegister, src: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(src, dst), 0x13);
		emit_operand(dst, src, 0);
	}

	pub fn adcq(&mut self,dst: GPRegister, src: GPRegister) {
		(void) prefixq_and_encode(dst->encoding(), src->encoding());
		emit_arith(0x13, 0xC0, dst, src);
	}

	pub fn addq(&mut self,dst: Address, imm32: i32) {
		InstructionMark im(this);
		prefixq(dst);
		emit_arith_operand(0x81, rax, dst, imm32);
	}

	pub fn addq(&mut self,dst: Address, src: GPRegister) {
		InstructionMark im(this);
		emit_int16(get_prefixq(dst, src), 0x01);
		emit_operand(src, dst, 0);
	}

	pub fn addq(&mut self,dst: GPRegister, imm32: i32) {
		(void) prefixq_and_encode(dst->encoding());
		emit_arith(0x81, 0xC0, dst, imm32);
	}

	pub fn addq(&mut self,dst: GPRegister, src: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(src, dst), 0x03);
		emit_operand(dst, src, 0);
	}

	pub fn addq(&mut self,dst: GPRegister, src: GPRegister) {
		(void) prefixq_and_encode(dst->encoding(), src->encoding());
		emit_arith(0x03, 0xC0, dst, src);
	}

	pub fn adcxq(&mut self,dst: GPRegister, src: GPRegister) {
		//assert(VM_Version::supports_adx(), "adx instructions not supported");
		emit_int8(0x66);
		int encode = prefixq_and_encode(dst->encoding(), src->encoding());
		emit_int32(0x0F,
				   0x38,
				   (unsigned char)0xF6,
		(0xC0 | encode));
	}

	pub fn adoxq(&mut self,dst: GPRegister, src: GPRegister) {
		//assert(VM_Version::supports_adx(), "adx instructions not supported");
		emit_int8((unsigned char)0xF3);
		int encode = prefixq_and_encode(dst->encoding(), src->encoding());
		emit_int32(0x0F,
				   0x38,
				   (unsigned char)0xF6,
		(0xC0 | encode));
	}

	pub fn andq(&mut self,dst: Address, imm32: i32) {
		InstructionMark im(this);
		prefixq(dst);
		emit_arith_operand(0x81, as_Register(4), dst, imm32);
	}

	pub fn andq(&mut self,dst: GPRegister, imm32: i32) {
		(void) prefixq_and_encode(dst->encoding());
		emit_arith(0x81, 0xE0, dst, imm32);
	}

	pub fn andq(&mut self,dst: GPRegister, src: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(src, dst), 0x23);
		emit_operand(dst, src, 0);
	}

	pub fn andq(&mut self,dst: GPRegister, src: GPRegister) {
		(void) prefixq_and_encode(dst->encoding(), src->encoding());
		emit_arith(0x23, 0xC0, dst, src);
	}

	pub fn andq(&mut self,dst: Address, src: GPRegister) {
		InstructionMark im(this);
		emit_int16(get_prefixq(dst, src), 0x21);
		emit_operand(src, dst, 0);
	}

	pub fn andnq(&mut self,dst: GPRegister, src: GPRegister1, src: GPRegister2) {
		assert(VM_Version::supports_bmi1(), "bit manipulation instructions not supported");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), src1->encoding(), src2->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xF2, (0xC0 | encode));
	}

	pub fn andnq(&mut self,dst: GPRegister, src: GPRegister1, src: Address2) {
		assert(VM_Version::supports_bmi1(), "bit manipulation instructions not supported");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		vex_prefix(src2, src1->encoding(), dst->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xF2);
		emit_operand(dst, src2, 0);
	}

	pub fn bsfq(&mut self,dst: GPRegister, src: GPRegister) {
		int encode = prefixq_and_encode(dst->encoding(), src->encoding());
		emit_int24(0x0F, (unsigned char)0xBC, (0xC0 | encode));
	}

	pub fn bsrq(&mut self,dst: GPRegister, src: GPRegister) {
		int encode = prefixq_and_encode(dst->encoding(), src->encoding());
		emit_int24(0x0F, (unsigned char)0xBD, (0xC0 | encode));
	}

	pub fn bswapq(&mut self,Register reg) {
		int encode = prefixq_and_encode(reg->encoding());
		emit_int16(0x0F, (0xC8 | encode));
	}

	pub fn blsiq(&mut self,dst: GPRegister, src: GPRegister) {
		assert(VM_Version::supports_bmi1(), "bit manipulation instructions not supported");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(rbx->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xF3, (0xC0 | encode));
	}

	pub fn blsiq(&mut self,dst: GPRegister, src: Address) {
		assert(VM_Version::supports_bmi1(), "bit manipulation instructions not supported");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		vex_prefix(src, dst->encoding(), rbx->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xF3);
		emit_operand(rbx, src, 0);
	}

	pub fn blsmskq(&mut self,dst: GPRegister, src: GPRegister) {
		assert(VM_Version::supports_bmi1(), "bit manipulation instructions not supported");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(rdx->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xF3, (0xC0 | encode));
	}

	pub fn blsmskq(&mut self,dst: GPRegister, src: Address) {
		assert(VM_Version::supports_bmi1(), "bit manipulation instructions not supported");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		vex_prefix(src, dst->encoding(), rdx->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xF3);
		emit_operand(rdx, src, 0);
	}

	pub fn blsrq(&mut self,dst: GPRegister, src: GPRegister) {
		assert(VM_Version::supports_bmi1(), "bit manipulation instructions not supported");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(rcx->encoding(), dst->encoding(), src->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xF3, (0xC0 | encode));
	}

	pub fn blsrq(&mut self,dst: GPRegister, src: Address) {
		assert(VM_Version::supports_bmi1(), "bit manipulation instructions not supported");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		vex_prefix(src, dst->encoding(), rcx->encoding(), VEX_SIMD_NONE, VEX_OPCODE_0F_38, &attributes);
		emit_int8((unsigned char)0xF3);
		emit_operand(rcx, src, 0);
	}

	pub fn cdqq(&mut self,) {
		emit_int16(REX_W, (unsigned char)0x99);
	}

	pub fn clflush(&mut self,Address adr) {
		assert(VM_Version::supports_clflush(), "should do");
		prefix(adr);
		emit_int16(0x0F, (unsigned char)0xAE);
		emit_operand(rdi, adr, 0);
	}

	pub fn clflushopt(&mut self,Address adr) {
		assert(VM_Version::supports_clflushopt(), "should do!");
		// adr should be base reg only with no index or offset
		assert(adr.index() == noreg, "index should be noreg");
		assert(adr.scale() == Address::no_scale, "scale should be no_scale");
		assert(adr.disp() == 0, "displacement should be 0");
		// instruction prefix is 0x66
		emit_int8(0x66);
		prefix(adr);
		// opcode family is 0x0F 0xAE
		emit_int16(0x0F, (unsigned char)0xAE);
		// extended opcode byte is 7 == rdi
		emit_operand(rdi, adr, 0);
	}

	pub fn clwb(&mut self,Address adr) {
		assert(VM_Version::supports_clwb(), "should do!");
		// adr should be base reg only with no index or offset
		assert(adr.index() == noreg, "index should be noreg");
		assert(adr.scale() == Address::no_scale, "scale should be no_scale");
		assert(adr.disp() == 0, "displacement should be 0");
		// instruction prefix is 0x66
		emit_int8(0x66);
		prefix(adr);
		// opcode family is 0x0f 0xAE
		emit_int16(0x0F, (unsigned char)0xAE);
		// extended opcode byte is 6 == rsi
		emit_operand(rsi, adr, 0);
	}

	pub fn cmovq(&mut self,Condition cc, dst: GPRegister, src: GPRegister) {
		int encode = prefixq_and_encode(dst->encoding(), src->encoding());
		emit_int24(0x0F, (0x40 | cc), (0xC0 | encode));
	}

	pub fn cmovq(&mut self,Condition cc, dst: GPRegister, src: Address) {
		InstructionMark im(this);
		emit_int24(get_prefixq(src, dst), 0x0F, (0x40 | cc));
		emit_operand(dst, src, 0);
	}

	pub fn cmpq(&mut self,dst: Address, imm32: i32) {
		InstructionMark im(this);
		prefixq(dst);
		emit_arith_operand(0x81, as_Register(7), dst, imm32);
	}

	pub fn cmpq(&mut self,dst: GPRegister, imm32: i32) {
		(void) prefixq_and_encode(dst->encoding());
		emit_arith(0x81, 0xF8, dst, imm32);
	}

	pub fn cmpq(&mut self,dst: Address, src: GPRegister) {
		InstructionMark im(this);
		emit_int16(get_prefixq(dst, src), 0x39);
		emit_operand(src, dst, 0);
	}

	pub fn cmpq(&mut self,dst: GPRegister, src: GPRegister) {
		(void) prefixq_and_encode(dst->encoding(), src->encoding());
		emit_arith(0x3B, 0xC0, dst, src);
	}

	pub fn cmpq(&mut self,dst: GPRegister, src: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(src, dst), 0x3B);
		emit_operand(dst, src, 0);
	}

	pub fn cmpxchgq(&mut self,Register reg, Address adr) {
		InstructionMark im(this);
		emit_int24(get_prefixq(adr, reg), 0x0F, (unsigned char)0xB1);
		emit_operand(reg, adr, 0);
	}

	pub fn cvtsi2sdq(&mut self,XMMdst: GPRegister, src: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = simd_prefix_and_encode(dst, dst, as_XMMRegister(src->encoding()), VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
		emit_int16(0x2A, (0xC0 | encode));
	}

	pub fn cvtsi2sdq(&mut self,XMMdst: GPRegister, src: Address) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
		simd_prefix(dst, dst, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
		emit_int8(0x2A);
		emit_operand(dst, src, 0);
	}

	pub fn cvtsi2ssq(&mut self,XMMdst: GPRegister, src: Address) {
		NOT_LP64(assert(VM_Version::supports_sse(), ""));
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		attributes.set_address_attributes(/* tuple_type */ EVEX_T1S, /* input_size_in_bits */ EVEX_64bit);
		simd_prefix(dst, dst, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
		emit_int8(0x2A);
		emit_operand(dst, src, 0);
	}

	pub fn cvttsd2siq(&mut self,dst: GPRegister, src: Address) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		// F2 REX.W 0F 2C /r
		// CVTTSD2SI r64, xmm1/m64
		InstructionMark im(this);
		emit_int32((unsigned char)0xF2, REX_W, 0x0F, 0x2C);
		emit_operand(dst, src, 0);
	}

	pub fn cvttsd2siq(&mut self,dst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = simd_prefix_and_encode(as_XMMRegister(dst->encoding()), xnoreg, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
		emit_int16(0x2C, (0xC0 | encode));
	}

	pub fn cvtsd2siq(&mut self,dst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = simd_prefix_and_encode(as_XMMRegister(dst->encoding()), xnoreg, src, VEX_SIMD_F2, VEX_OPCODE_0F, &attributes);
		emit_int16(0x2D, (0xC0 | encode));
	}

	pub fn cvttss2siq(&mut self,dst: GPRegister, XMMsrc: GPRegister) {
		NOT_LP64(assert(VM_Version::supports_sse(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = simd_prefix_and_encode(as_XMMRegister(dst->encoding()), xnoreg, src, VEX_SIMD_F3, VEX_OPCODE_0F, &attributes);
		emit_int16(0x2C, (0xC0 | encode));
	}

	pub fn decl(&mut self,dst: GPRegister) {
		// Don't use it directly. Use MacroAssembler::decrementl() instead.
		// Use two-byte form (one-byte form is a REX prefix in 64-bit mode)
		int encode = prefix_and_encode(dst->encoding());
		emit_int16((unsigned char)0xFF, (0xC8 | encode));
	}

	pub fn decq(&mut self,dst: GPRegister) {
		// Don't use it directly. Use MacroAssembler::decrementq() instead.
		// Use two-byte form (one-byte from is a REX prefix in 64-bit mode)
		int encode = prefixq_and_encode(dst->encoding());
		emit_int16((unsigned char)0xFF, 0xC8 | encode);
	}

	pub fn decq(&mut self,dst: Address) {
		// Don't use it directly. Use MacroAssembler::decrementq() instead.
		InstructionMark im(this);
		emit_int16(get_prefixq(dst), (unsigned char)0xFF);
		emit_operand(rcx, dst, 0);
	}

	pub fn fxrstor(&mut self,src: Address) {
		emit_int24(get_prefixq(src), 0x0F, (unsigned char)0xAE);
		emit_operand(as_Register(1), src, 0);
	}

	pub fn xrstor(&mut self,src: Address) {
		emit_int24(get_prefixq(src), 0x0F, (unsigned char)0xAE);
		emit_operand(as_Register(5), src, 0);
	}

	pub fn fxsave(&mut self,dst: Address) {
		emit_int24(get_prefixq(dst), 0x0F, (unsigned char)0xAE);
		emit_operand(as_Register(0), dst, 0);
	}

	pub fn xsave(&mut self,dst: Address) {
		emit_int24(get_prefixq(dst), 0x0F, (unsigned char)0xAE);
		emit_operand(as_Register(4), dst, 0);
	}

	pub fn idivq(&mut self,src: GPRegister) {
		int encode = prefixq_and_encode(src->encoding());
		emit_int16((unsigned char)0xF7, (0xF8 | encode));
	}

	pub fn divq(&mut self,src: GPRegister) {
		int encode = prefixq_and_encode(src->encoding());
		emit_int16((unsigned char)0xF7, (0xF0 | encode));
	}

	pub fn imulq(&mut self,dst: GPRegister, src: GPRegister) {
		int encode = prefixq_and_encode(dst->encoding(), src->encoding());
		emit_int24(0x0F, (unsigned char)0xAF, (0xC0 | encode));
	}

	pub fn imulq(&mut self,src: GPRegister) {
		int encode = prefixq_and_encode(src->encoding());
		emit_int16((unsigned char)0xF7, (0xE8 | encode));
	}

	pub fn imulq(&mut self,dst: GPRegister, src: Address, i32 value) {
		InstructionMark im(this);
		prefixq(src, dst);
		if (is8bit(value)) {
			emit_int8((unsigned char)0x6B);
			emit_operand(dst, src, 1);
			emit_int8(value);
		} else {
			emit_int8((unsigned char)0x69);
			emit_operand(dst, src, 4);
			emit_int32(value);
		}
	}

	pub fn imulq(&mut self,dst: GPRegister, src: GPRegister, int value) {
		int encode = prefixq_and_encode(dst->encoding(), src->encoding());
		if (is8bit(value)) {
			emit_int24(0x6B, (0xC0 | encode), (value & 0xFF));
		} else {
			emit_int16(0x69, (0xC0 | encode));
			emit_int32(value);
		}
	}

	pub fn imulq(&mut self,dst: GPRegister, src: Address) {
		InstructionMark im(this);
		emit_int24(get_prefixq(src, dst), 0x0F, (unsigned char)0xAF);
		emit_operand(dst, src, 0);
	}

	pub fn incl(&mut self,dst: GPRegister) {
		// Don't use it directly. Use MacroAssembler::incrementl() instead.
		// Use two-byte form (one-byte from is a REX prefix in 64-bit mode)
		int encode = prefix_and_encode(dst->encoding());
		emit_int16((unsigned char)0xFF, (0xC0 | encode));
	}

	pub fn incq(&mut self,dst: GPRegister) {
		// Don't use it directly. Use MacroAssembler::incrementq() instead.
		// Use two-byte form (one-byte from is a REX prefix in 64-bit mode)
		int encode = prefixq_and_encode(dst->encoding());
		emit_int16((unsigned char)0xFF, (0xC0 | encode));
	}

	pub fn incq(&mut self,dst: Address) {
		// Don't use it directly. Use MacroAssembler::incrementq() instead.
		InstructionMark im(this);
		emit_int16(get_prefixq(dst), (unsigned char)0xFF);
		emit_operand(rax, dst, 0);
	}

	pub fn lea(&mut self,dst: GPRegister, src: Address) {
		leaq(dst, src);
	}

	pub fn leaq(&mut self,dst: GPRegister, src: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(src, dst), (unsigned char)0x8D);
		emit_operand(dst, src, 0);
	}

	pub fn mov64(&mut self,dst: GPRegister, imm64: i64) {
		InstructionMark im(this);
		int encode = prefixq_and_encode(dst->encoding());
		emit_int8(0xB8 | encode);
		emit_int64(imm64);
	}

	pub fn mov64(&mut self,dst: GPRegister, imm64: i64, relocInfo::relocType rtype, int format) {
		InstructionMark im(this);
		int encode = prefixq_and_encode(dst->encoding());
		emit_int8(0xB8 | encode);
		emit_data64(imm64, rtype, format);
	}

	pub fn mov_literal64(&mut self,dst: GPRegister, imm64: isize, RelocationHolder const& rspec) {
	InstructionMark im(this);
	int encode = prefixq_and_encode(dst->encoding());
	emit_int8(0xB8 | encode);
	emit_data64(imm64, rspec);
	}

	pub fn mov_narrow_oop(&mut self,dst: GPRegister, imm32: i32, RelocationHolder const& rspec) {
	InstructionMark im(this);
	int encode = prefix_and_encode(dst->encoding());
	emit_int8(0xB8 | encode);
	emit_data((int)imm32, rspec, narrow_oop_operand);
	}

	pub fn mov_narrow_oop(&mut self,dst: Address, imm32: i32,  RelocationHolder const& rspec) {
	InstructionMark im(this);
	prefix(dst);
	emit_int8((unsigned char)0xC7);
	emit_operand(rax, dst, 4);
	emit_data((int)imm32, rspec, narrow_oop_operand);
	}

	pub fn cmp_narrow_oop(&mut self,src: GPRegister1, imm32: i32, RelocationHolder const& rspec) {
	InstructionMark im(this);
	int encode = prefix_and_encode(src1->encoding());
	emit_int16((unsigned char)0x81, (0xF8 | encode));
	emit_data((int)imm32, rspec, narrow_oop_operand);
	}

	pub fn cmp_narrow_oop(&mut self,src: Address1, imm32: i32, RelocationHolder const& rspec) {
	InstructionMark im(this);
	prefix(src1);
	emit_int8((unsigned char)0x81);
	emit_operand(rax, src1, 4);
	emit_data((int)imm32, rspec, narrow_oop_operand);
	}

	pub fn lzcntq(&mut self,dst: GPRegister, src: GPRegister) {
		assert(VM_Version::supports_lzcnt(), "encoding is treated as BSR");
		emit_int8((unsigned char)0xF3);
		int encode = prefixq_and_encode(dst->encoding(), src->encoding());
		emit_int24(0x0F, (unsigned char)0xBD, (0xC0 | encode));
	}

	pub fn lzcntq(&mut self,dst: GPRegister, src: Address) {
		assert(VM_Version::supports_lzcnt(), "encoding is treated as BSR");
		InstructionMark im(this);
		emit_int8((unsigned char)0xF3);
		prefixq(src, dst);
		emit_int16(0x0F, (unsigned char)0xBD);
		emit_operand(dst, src, 0);
	}

	pub fn movdq(&mut self,XMMdst: GPRegister, src: GPRegister) {
		// table D-1 says MMX/SSE2
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = simd_prefix_and_encode(dst, xnoreg, as_XMMRegister(src->encoding()), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x6E, (0xC0 | encode));
	}

	pub fn movdq(&mut self,dst: GPRegister, XMMsrc: GPRegister) {
		// table D-1 says MMX/SSE2
		NOT_LP64(assert(VM_Version::supports_sse2(), ""));
		InstructionAttr attributes(AVX_128bit, /* rex_w */ true, /* legacy_mode */ false, /* no_mask_reg */ true, /* uses_vl */ false);
		// swap src/dst to get correct prefix
		int encode = simd_prefix_and_encode(src, xnoreg, as_XMMRegister(dst->encoding()), VEX_SIMD_66, VEX_OPCODE_0F, &attributes);
		emit_int16(0x7E,
				   (0xC0 | encode));
	}

	pub fn movq(&mut self,dst: GPRegister, src: GPRegister) {
		int encode = prefixq_and_encode(dst->encoding(), src->encoding());
		emit_int16((unsigned char)0x8B,
		(0xC0 | encode));
	}

	pub fn movq(&mut self,dst: GPRegister, src: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(src, dst), (unsigned char)0x8B);
		emit_operand(dst, src, 0);
	}

	pub fn movq(&mut self,dst: Address, src: GPRegister) {
		InstructionMark im(this);
		emit_int16(get_prefixq(dst, src), (unsigned char)0x89);
		emit_operand(src, dst, 0);
	}

	pub fn movq(&mut self,dst: Address, imm32: i32) {
		InstructionMark im(this);
		emit_int16(get_prefixq(dst), (unsigned char)0xC7);
		emit_operand(as_Register(0), dst, 4);
		emit_int32(imm32);
	}

	pub fn movq(&mut self,dst: GPRegister, imm32: i32) {
		int encode = prefixq_and_encode(dst->encoding());
		emit_int16((unsigned char)0xC7, (0xC0 | encode));
		emit_int32(imm32);
	}

	pub fn movsbq(&mut self,dst: GPRegister, src: Address) {
		InstructionMark im(this);
		emit_int24(get_prefixq(src, dst),
				   0x0F,
				   (unsigned char)0xBE);
		emit_operand(dst, src, 0);
	}

	pub fn movsbq(&mut self,dst: GPRegister, src: GPRegister) {
		int encode = prefixq_and_encode(dst->encoding(), src->encoding());
		emit_int24(0x0F, (unsigned char)0xBE, (0xC0 | encode));
	}

	pub fn movslq(&mut self,dst: GPRegister, imm32: i32) {
		// dbx shows movslq(rcx, 3) as movq     $0x0000000049000000,(%rbx)
		// and movslq(r8, 3); as movl     $0x0000000048000000,(%rbx)
		// as a result we shouldn't use until tested at runtime...
		ShouldNotReachHere();
		InstructionMark im(this);
		int encode = prefixq_and_encode(dst->encoding());
		emit_int8(0xC7 | encode);
		emit_int32(imm32);
	}

	pub fn movslq(&mut self,dst: Address, imm32: i32) {
		assert(is_simm32(imm32), "lost bits");
		InstructionMark im(this);
		emit_int16(get_prefixq(dst), (unsigned char)0xC7);
		emit_operand(rax, dst, 4);
		emit_int32(imm32);
	}

	pub fn movslq(&mut self,dst: GPRegister, src: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(src, dst), 0x63);
		emit_operand(dst, src, 0);
	}

	pub fn movslq(&mut self,dst: GPRegister, src: GPRegister) {
		int encode = prefixq_and_encode(dst->encoding(), src->encoding());
		emit_int16(0x63, (0xC0 | encode));
	}

	pub fn movswq(&mut self,dst: GPRegister, src: Address) {
		InstructionMark im(this);
		emit_int24(get_prefixq(src, dst),
				   0x0F,
				   (unsigned char)0xBF);
		emit_operand(dst, src, 0);
	}

	pub fn movswq(&mut self,dst: GPRegister, src: GPRegister) {
		int encode = prefixq_and_encode(dst->encoding(), src->encoding());
		emit_int24(0x0F, (unsigned char)0xBF, (0xC0 | encode));
	}

	pub fn movzbq(&mut self,dst: GPRegister, src: Address) {
		InstructionMark im(this);
		emit_int24(get_prefixq(src, dst),
				   0x0F,
				   (unsigned char)0xB6);
		emit_operand(dst, src, 0);
	}

	pub fn movzbq(&mut self,dst: GPRegister, src: GPRegister) {
		int encode = prefixq_and_encode(dst->encoding(), src->encoding());
		emit_int24(0x0F, (unsigned char)0xB6, (0xC0 | encode));
	}

	pub fn movzwq(&mut self,dst: GPRegister, src: Address) {
		InstructionMark im(this);
		emit_int24(get_prefixq(src, dst),
				   0x0F,
				   (unsigned char)0xB7);
		emit_operand(dst, src, 0);
	}

	pub fn movzwq(&mut self,dst: GPRegister, src: GPRegister) {
		int encode = prefixq_and_encode(dst->encoding(), src->encoding());
		emit_int24(0x0F, (unsigned char)0xB7, (0xC0 | encode));
	}

	pub fn mulq(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(src), (unsigned char)0xF7);
		emit_operand(rsp, src, 0);
	}

	pub fn mulq(&mut self,src: GPRegister) {
		int encode = prefixq_and_encode(src->encoding());
		emit_int16((unsigned char)0xF7, (0xE0 | encode));
	}

	pub fn mulxq(&mut self,dst: GPRegister1, dst: GPRegister2, src: GPRegister) {
		assert(VM_Version::supports_bmi2(), "bit manipulation instructions not supported");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst1->encoding(), dst2->encoding(), src->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F_38, &attributes);
		emit_int16((unsigned char)0xF6, (0xC0 | encode));
	}

	pub fn negq(&mut self,dst: GPRegister) {
		int encode = prefixq_and_encode(dst->encoding());
		emit_int16((unsigned char)0xF7, (0xD8 | encode));
	}

	pub fn negq(&mut self,dst: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(dst), (unsigned char)0xF7);
		emit_operand(as_Register(3), dst, 0);
	}

	pub fn notq(&mut self,dst: GPRegister) {
		int encode = prefixq_and_encode(dst->encoding());
		emit_int16((unsigned char)0xF7, (0xD0 | encode));
	}

	pub fn btsq(&mut self,dst: Address, imm8: i8) {
		assert(isByte(imm8), "not a byte");
		InstructionMark im(this);
		emit_int24(get_prefixq(dst),
				   0x0F,
				   (unsigned char)0xBA);
		emit_operand(rbp /* 5 */, dst, 1);
		emit_int8(imm8);
	}

	pub fn btrq(&mut self,dst: Address, imm8: i8) {
		assert(isByte(imm8), "not a byte");
		InstructionMark im(this);
		emit_int24(get_prefixq(dst),
				   0x0F,
				   (unsigned char)0xBA);
		emit_operand(rsi /* 6 */, dst, 1);
		emit_int8(imm8);
	}

	pub fn orq(&mut self,dst: Address, imm32: i32) {
		InstructionMark im(this);
		prefixq(dst);
		emit_arith_operand(0x81, as_Register(1), dst, imm32);
	}

	pub fn orq(&mut self,dst: Address, src: GPRegister) {
		InstructionMark im(this);
		emit_int16(get_prefixq(dst, src), (unsigned char)0x09);
		emit_operand(src, dst, 0);
	}

	pub fn orq(&mut self,dst: GPRegister, imm32: i32) {
		(void) prefixq_and_encode(dst->encoding());
		emit_arith(0x81, 0xC8, dst, imm32);
	}

	pub fn orq_imm32(&mut self,dst: GPRegister, imm32: i32) {
		(void) prefixq_and_encode(dst->encoding());
		emit_arith_imm32(0x81, 0xC8, dst, imm32);
	}

	pub fn orq(&mut self,dst: GPRegister, src: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(src, dst), 0x0B);
		emit_operand(dst, src, 0);
	}

	pub fn orq(&mut self,dst: GPRegister, src: GPRegister) {
		(void) prefixq_and_encode(dst->encoding(), src->encoding());
		emit_arith(0x0B, 0xC0, dst, src);
	}

	pub fn popcntq(&mut self,dst: GPRegister, src: Address) {
		assert(VM_Version::supports_popcnt(), "must support");
		InstructionMark im(this);
		emit_int32((unsigned char)0xF3,
		get_prefixq(src, dst),
		0x0F,
		(unsigned char)0xB8);
		emit_operand(dst, src, 0);
	}

	pub fn popcntq(&mut self,dst: GPRegister, src: GPRegister) {
		assert(VM_Version::supports_popcnt(), "must support");
		emit_int8((unsigned char)0xF3);
		int encode = prefixq_and_encode(dst->encoding(), src->encoding());
		emit_int24(0x0F, (unsigned char)0xB8, (0xC0 | encode));
	}

	pub fn popq(&mut self,dst: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(dst), (unsigned char)0x8F);
		emit_operand(rax, dst, 0);
	}

	pub fn popq(&mut self,dst: GPRegister) {
		emit_int8((unsigned char)0x58 | dst->encoding());
	}

// Precomputable: popa, pusha, vzeroupper

	// The result of these routines are invariant from one invocation to another
// invocation for the duration of a run. Caching the result on bootstrap
// and copying it out on subsequent invocations can thus be beneficial
	static bool     precomputed = false;

	static u_char* popa_code  = nullptr;
	static int     popa_len   = 0;

	static u_char* pusha_code = nullptr;
	static int     pusha_len  = 0;

	static u_char* vzup_code  = nullptr;
	static int     vzup_len   = 0;

	pub fn precompute_instructions(&mut self,) {
		assert(!Universe::is_fully_initialized(), "must still be single threaded");
		guarantee(!precomputed, "only once");
		precomputed = true;
		ResourceMark rm;

		// Make a temporary buffer big enough for the routines we're capturing
		int size = 256;
		char* tmp_code = NEW_RESOURCE_ARRAY(char, size);
		CodeBuffer buffer((address)tmp_code, size);
		MacroAssembler masm(&buffer);

		address begin_popa  = masm.code_section()->end();
		masm.popa_uncached();
		address end_popa    = masm.code_section()->end();
		masm.pusha_uncached();
		address end_pusha   = masm.code_section()->end();
		masm.vzeroupper_uncached();
		address end_vzup    = masm.code_section()->end();

		// Save the instructions to permanent buffers.
		popa_len = (int)(end_popa - begin_popa);
		popa_code = NEW_C_HEAP_ARRAY(u_char, popa_len, mtInternal);
		memcpy(popa_code, begin_popa, popa_len);

		pusha_len = (int)(end_pusha - end_popa);
		pusha_code = NEW_C_HEAP_ARRAY(u_char, pusha_len, mtInternal);
		memcpy(pusha_code, end_popa, pusha_len);

		vzup_len = (int)(end_vzup - end_pusha);
		if (vzup_len > 0) {
			vzup_code = NEW_C_HEAP_ARRAY(u_char, vzup_len, mtInternal);
			memcpy(vzup_code, end_pusha, vzup_len);
		} else {
			vzup_code = pusha_code; // dummy
		}

		assert(masm.code()->total_oop_size() == 0 &&
			masm.code()->total_metadata_size() == 0 &&
				   masm.code()->total_relocation_size() == 0,
			   "pre-computed code can't reference oops, metadata or contain relocations");
	}

	static void emit_copy(CodeSection* code_section, u_char* src, int src_len) {
	assert(src != nullptr, "code to copy must have been pre-computed");
	assert(code_section->limit() - code_section->end() > src_len, "code buffer not large enough");
	address end = code_section->end();
	memcpy(end, src, src_len);
	code_section->set_end(end + src_len);
	}

	pub fn popa(&mut self,) { // 64bit
		emit_copy(code_section(), popa_code, popa_len);
	}

	pub fn popa_uncached(&mut self,) { // 64bit
		movq(r15, Address(rsp, 0));
		movq(r14, Address(rsp, wordSize));
		movq(r13, Address(rsp, 2 * wordSize));
		movq(r12, Address(rsp, 3 * wordSize));
		movq(r11, Address(rsp, 4 * wordSize));
		movq(r10, Address(rsp, 5 * wordSize));
		movq(r9,  Address(rsp, 6 * wordSize));
		movq(r8,  Address(rsp, 7 * wordSize));
		movq(rdi, Address(rsp, 8 * wordSize));
		movq(rsi, Address(rsp, 9 * wordSize));
		movq(rbp, Address(rsp, 10 * wordSize));
		// Skip rsp as it is restored automatically to the value
		// before the corresponding pusha when popa is done.
		movq(rbx, Address(rsp, 12 * wordSize));
		movq(rdx, Address(rsp, 13 * wordSize));
		movq(rcx, Address(rsp, 14 * wordSize));
		movq(rax, Address(rsp, 15 * wordSize));

		addq(rsp, 16 * wordSize);
	}

	// Does not actually store the value of rsp on the stack.
// The slot for rsp just contains an arbitrary value.
	pub fn pusha(&mut self,) { // 64bit
		emit_copy(code_section(), pusha_code, pusha_len);
	}

	// Does not actually store the value of rsp on the stack.
// The slot for rsp just contains an arbitrary value.
	pub fn pusha_uncached(&mut self,) { // 64bit
		subq(rsp, 16 * wordSize);

		movq(Address(rsp, 15 * wordSize), rax);
		movq(Address(rsp, 14 * wordSize), rcx);
		movq(Address(rsp, 13 * wordSize), rdx);
		movq(Address(rsp, 12 * wordSize), rbx);
		// Skip rsp as the value is normally not used. There are a few places where
		// the original value of rsp needs to be known but that can be computed
		// from the value of rsp immediately after pusha (rsp + 16 * wordSize).
		movq(Address(rsp, 10 * wordSize), rbp);
		movq(Address(rsp, 9 * wordSize), rsi);
		movq(Address(rsp, 8 * wordSize), rdi);
		movq(Address(rsp, 7 * wordSize), r8);
		movq(Address(rsp, 6 * wordSize), r9);
		movq(Address(rsp, 5 * wordSize), r10);
		movq(Address(rsp, 4 * wordSize), r11);
		movq(Address(rsp, 3 * wordSize), r12);
		movq(Address(rsp, 2 * wordSize), r13);
		movq(Address(rsp, wordSize), r14);
		movq(Address(rsp, 0), r15);
	}

	pub fn vzeroupper(&mut self,) {
		emit_copy(code_section(), vzup_code, vzup_len);
	}

	pub fn vzeroall(&mut self,) {
		assert(VM_Version::supports_avx(), "requires AVX");
		InstructionAttr attributes(AVX_256bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		(void)vex_prefix_and_encode(0, 0, 0, VEX_SIMD_NONE, VEX_OPCODE_0F, &attributes);
		emit_int8(0x77);
	}

	pub fn pushq(&mut self,src: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(src), (unsigned char)0xFF);
		emit_operand(rsi, src, 0);
	}

	pub fn rclq(&mut self,dst: GPRegister, imm8: i8) {
		assert(isShiftCount(imm8 >> 1), "illegal shift count");
		int encode = prefixq_and_encode(dst->encoding());
		if (imm8 == 1) {
			emit_int16((unsigned char)0xD1, (0xD0 | encode));
		} else {
			emit_int24((unsigned char)0xC1, (0xD0 | encode), imm8);
		}
	}

	pub fn rcrq(&mut self,dst: GPRegister, imm8: i8) {
		assert(isShiftCount(imm8 >> 1), "illegal shift count");
		int encode = prefixq_and_encode(dst->encoding());
		if (imm8 == 1) {
			emit_int16((unsigned char)0xD1, (0xD8 | encode));
		} else {
			emit_int24((unsigned char)0xC1, (0xD8 | encode), imm8);
		}
	}

	pub fn rorxl(&mut self,dst: GPRegister, src: GPRegister, imm8: i8) {
		assert(VM_Version::supports_bmi2(), "bit manipulation instructions not supported");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F_3A, &attributes);
		emit_int24((unsigned char)0xF0, (0xC0 | encode), imm8);
	}

	pub fn rorxl(&mut self,dst: GPRegister, src: Address, imm8: i8) {
		assert(VM_Version::supports_bmi2(), "bit manipulation instructions not supported");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ false, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F_3A, &attributes);
		emit_int8((unsigned char)0xF0);
		emit_operand(dst, src, 1);
		emit_int8(imm8);
	}

	pub fn rorxq(&mut self,dst: GPRegister, src: GPRegister, imm8: i8) {
		assert(VM_Version::supports_bmi2(), "bit manipulation instructions not supported");
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		int encode = vex_prefix_and_encode(dst->encoding(), 0, src->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F_3A, &attributes);
		emit_int24((unsigned char)0xF0, (0xC0 | encode), imm8);
	}

	pub fn rorxq(&mut self,dst: GPRegister, src: Address, imm8: i8) {
		assert(VM_Version::supports_bmi2(), "bit manipulation instructions not supported");
		InstructionMark im(this);
		InstructionAttr attributes(AVX_128bit, /* vex_w */ true, /* legacy_mode */ true, /* no_mask_reg */ true, /* uses_vl */ false);
		vex_prefix(src, 0, dst->encoding(), VEX_SIMD_F2, VEX_OPCODE_0F_3A, &attributes);
		emit_int8((unsigned char)0xF0);
		emit_operand(dst, src, 1);
		emit_int8(imm8);
	}

	#ifdef _LP64
	pub fn salq(&mut self,dst: Address, imm8: i8) {
		InstructionMark im(this);
		assert(isShiftCount(imm8 >> 1), "illegal shift count");
		if (imm8 == 1) {
			emit_int16(get_prefixq(dst), (unsigned char)0xD1);
			emit_operand(as_Register(4), dst, 0);
		}
		else {
			emit_int16(get_prefixq(dst), (unsigned char)0xC1);
			emit_operand(as_Register(4), dst, 1);
			emit_int8(imm8);
		}
	}

	pub fn salq(&mut self,dst: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(dst), (unsigned char)0xD3);
		emit_operand(as_Register(4), dst, 0);
	}

	pub fn salq(&mut self,dst: GPRegister, imm8: i8) {
		assert(isShiftCount(imm8 >> 1), "illegal shift count");
		int encode = prefixq_and_encode(dst->encoding());
		if (imm8 == 1) {
			emit_int16((unsigned char)0xD1, (0xE0 | encode));
		} else {
			emit_int24((unsigned char)0xC1, (0xE0 | encode), imm8);
		}
	}

	pub fn salq(&mut self,dst: GPRegister) {
		int encode = prefixq_and_encode(dst->encoding());
		emit_int16((unsigned char)0xD3, (0xE0 | encode));
	}

	pub fn sarq(&mut self,dst: Address, imm8: i8) {
		InstructionMark im(this);
		assert(isShiftCount(imm8 >> 1), "illegal shift count");
		if (imm8 == 1) {
			emit_int16(get_prefixq(dst), (unsigned char)0xD1);
			emit_operand(as_Register(7), dst, 0);
		}
		else {
			emit_int16(get_prefixq(dst), (unsigned char)0xC1);
			emit_operand(as_Register(7), dst, 1);
			emit_int8(imm8);
		}
	}

	pub fn sarq(&mut self,dst: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(dst), (unsigned char)0xD3);
		emit_operand(as_Register(7), dst, 0);
	}

	pub fn sarq(&mut self,dst: GPRegister, imm8: i8) {
		assert(isShiftCount(imm8 >> 1), "illegal shift count");
		int encode = prefixq_and_encode(dst->encoding());
		if (imm8 == 1) {
			emit_int16((unsigned char)0xD1, (0xF8 | encode));
		} else {
			emit_int24((unsigned char)0xC1, (0xF8 | encode), imm8);
		}
	}

	pub fn sarq(&mut self,dst: GPRegister) {
		int encode = prefixq_and_encode(dst->encoding());
		emit_int16((unsigned char)0xD3, (0xF8 | encode));
	}
	#endif

	pub fn sbbq(&mut self,dst: Address, imm32: i32) {
		InstructionMark im(this);
		prefixq(dst);
		emit_arith_operand(0x81, rbx, dst, imm32);
	}

	pub fn sbbq(&mut self,dst: GPRegister, imm32: i32) {
		(void) prefixq_and_encode(dst->encoding());
		emit_arith(0x81, 0xD8, dst, imm32);
	}

	pub fn sbbq(&mut self,dst: GPRegister, src: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(src, dst), 0x1B);
		emit_operand(dst, src, 0);
	}

	pub fn sbbq(&mut self,dst: GPRegister, src: GPRegister) {
		(void) prefixq_and_encode(dst->encoding(), src->encoding());
		emit_arith(0x1B, 0xC0, dst, src);
	}

	pub fn shlq(&mut self,dst: GPRegister, imm8: i8) {
		assert(isShiftCount(imm8 >> 1), "illegal shift count");
		int encode = prefixq_and_encode(dst->encoding());
		if (imm8 == 1) {
			emit_int16((unsigned char)0xD1, (0xE0 | encode));
		} else {
			emit_int24((unsigned char)0xC1, (0xE0 | encode), imm8);
		}
	}

	pub fn shlq(&mut self,dst: GPRegister) {
		int encode = prefixq_and_encode(dst->encoding());
		emit_int16((unsigned char)0xD3, (0xE0 | encode));
	}

	pub fn shrq(&mut self,dst: GPRegister, imm8: i8) {
		assert(isShiftCount(imm8 >> 1), "illegal shift count");
		int encode = prefixq_and_encode(dst->encoding());
		if (imm8 == 1) {
			emit_int16((unsigned char)0xD1, (0xE8 | encode));
		}
		else {
			emit_int24((unsigned char)0xC1, (0xE8 | encode), imm8);
		}
	}

	pub fn shrq(&mut self,dst: GPRegister) {
		int encode = prefixq_and_encode(dst->encoding());
		emit_int16((unsigned char)0xD3, 0xE8 | encode);
	}

	pub fn shrq(&mut self,dst: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(dst), (unsigned char)0xD3);
		emit_operand(as_Register(5), dst, 0);
	}

	pub fn shrq(&mut self,dst: Address, imm8: i8) {
		InstructionMark im(this);
		assert(isShiftCount(imm8 >> 1), "illegal shift count");
		if (imm8 == 1) {
			emit_int16(get_prefixq(dst), (unsigned char)0xD1);
			emit_operand(as_Register(5), dst, 0);
		}
		else {
			emit_int16(get_prefixq(dst), (unsigned char)0xC1);
			emit_operand(as_Register(5), dst, 1);
			emit_int8(imm8);
		}
	}

	pub fn subq(&mut self,dst: Address, imm32: i32) {
		InstructionMark im(this);
		prefixq(dst);
		emit_arith_operand(0x81, rbp, dst, imm32);
	}

	pub fn subq(&mut self,dst: Address, src: GPRegister) {
		InstructionMark im(this);
		emit_int16(get_prefixq(dst, src), 0x29);
		emit_operand(src, dst, 0);
	}

	pub fn subq(&mut self,dst: GPRegister, imm32: i32) {
		(void) prefixq_and_encode(dst->encoding());
		emit_arith(0x81, 0xE8, dst, imm32);
	}

	// Force generation of a 4 byte immediate value even if it fits into 8bit
	pub fn subq_imm32(&mut self,dst: GPRegister, imm32: i32) {
		(void) prefixq_and_encode(dst->encoding());
		emit_arith_imm32(0x81, 0xE8, dst, imm32);
	}

	pub fn subq(&mut self,dst: GPRegister, src: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(src, dst), 0x2B);
		emit_operand(dst, src, 0);
	}

	pub fn subq(&mut self,dst: GPRegister, src: GPRegister) {
		(void) prefixq_and_encode(dst->encoding(), src->encoding());
		emit_arith(0x2B, 0xC0, dst, src);
	}

	pub fn testq(&mut self,dst: Address, imm32: i32) {
		InstructionMark im(this);
		emit_int16(get_prefixq(dst), (unsigned char)0xF7);
		emit_operand(as_Register(0), dst, 4);
		emit_int32(imm32);
	}

	pub fn testq(&mut self,dst: GPRegister, imm32: i32) {
		// not using emit_arith because test
		// doesn't support sign-extension of
		// 8bit operands
		if (dst == rax) {
			prefix(REX_W);
			emit_int8((unsigned char)0xA9);
			emit_int32(imm32);
		} else {
			int encode = dst->encoding();
			encode = prefixq_and_encode(encode);
			emit_int16((unsigned char)0xF7, (0xC0 | encode));
			emit_int32(imm32);
		}
	}

	pub fn testq(&mut self,dst: GPRegister, src: GPRegister) {
		(void) prefixq_and_encode(dst->encoding(), src->encoding());
		emit_arith(0x85, 0xC0, dst, src);
	}

	pub fn testq(&mut self,dst: GPRegister, src: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(src, dst), (unsigned char)0x85);
		emit_operand(dst, src, 0);
	}

	pub fn xaddq(&mut self,dst: Address, src: GPRegister) {
		InstructionMark im(this);
		emit_int24(get_prefixq(dst, src), 0x0F, (unsigned char)0xC1);
		emit_operand(src, dst, 0);
	}

	pub fn xchgq(&mut self,dst: GPRegister, src: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(src, dst), (unsigned char)0x87);
		emit_operand(dst, src, 0);
	}

	pub fn xchgq(&mut self,dst: GPRegister, src: GPRegister) {
		int encode = prefixq_and_encode(dst->encoding(), src->encoding());
		emit_int16((unsigned char)0x87, (0xc0 | encode));
	}

	pub fn xorq(&mut self,dst: GPRegister, src: GPRegister) {
		(void) prefixq_and_encode(dst->encoding(), src->encoding());
		emit_arith(0x33, 0xC0, dst, src);
	}

	pub fn xorq(&mut self,dst: GPRegister, src: Address) {
		InstructionMark im(this);
		emit_int16(get_prefixq(src, dst), 0x33);
		emit_operand(dst, src, 0);
	}

	pub fn xorq(&mut self,dst: GPRegister, imm32: i32) {
		(void) prefixq_and_encode(dst->encoding());
		emit_arith(0x81, 0xF0, dst, imm32);
	}

	pub fn xorq(&mut self,dst: Address, imm32: i32) {
		InstructionMark im(this);
		prefixq(dst);
		emit_arith_operand(0x81, as_Register(6), dst, imm32);
	}

	pub fn xorq(&mut self,dst: Address, src: GPRegister) {
		InstructionMark im(this);
		emit_int16(get_prefixq(dst, src), 0x31);
		emit_operand(src, dst, 0);
	}

	#endif // !LP64

	void InstructionAttr::set_address_attributes(int tuple_type, int input_size_in_bits) {
	if (VM_Version::supports_evex()) {
	_tuple_type = tuple_type;
	_input_size_in_bits = input_size_in_bits;
	}
	}

}

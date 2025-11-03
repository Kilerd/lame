;
;
; 	assembler routines to detect CPU-features
;
;	MMX / 3DNow! / SSE / SSE2
;
;	for the LAME project
;	Frank Klemm, Robert Hegemann 2000-10-12
;

%include "nasm.h"

	globaldef	has_MMX_nasm
	globaldef	has_3DNow_nasm
	globaldef	has_SSE_nasm
	globaldef	has_SSE2_nasm
	globaldef	has_AVX_nasm
	globaldef	has_AVX2_nasm
	globaldef	has_FMA_nasm
	globaldef	has_AVX512F_nasm

        segment_code

testCPUID:
	pushfd	                        
	pop	eax
	mov	ecx,eax
	xor	eax,0x200000
	push	eax
	popfd
	pushfd
	pop	eax
	cmp	eax,ecx
	ret

;---------------------------------------
;	int  has_MMX_nasm (void)
;---------------------------------------

has_MMX_nasm:
        pushad
	call	testCPUID
	jz	return0		; no CPUID command, so no MMX

	mov	eax,0x1
	CPUID
	test	edx,0x800000
	jz	return0		; no MMX support
	jmp	return1		; MMX support
        
;---------------------------------------
;	int  has_SSE_nasm (void)
;---------------------------------------

has_SSE_nasm:
        pushad
	call	testCPUID
	jz	return0		; no CPUID command, so no SSE
        
	mov	eax,0x1
	CPUID
	test	edx,0x02000000
	jz	return0		; no SSE support
	jmp	return1		; SSE support
        
;---------------------------------------
;	int  has_SSE2_nasm (void)
;---------------------------------------

has_SSE2_nasm:
        pushad
	call	testCPUID
	jz	return0		; no CPUID command, so no SSE2
        
	mov	eax,0x1
	CPUID
	test	edx,0x04000000
	jz	return0		; no SSE2 support
	jmp	return1		; SSE2 support
        
;---------------------------------------
;	int  has_3DNow_nasm (void)
;---------------------------------------

has_3DNow_nasm:
        pushad
	call	testCPUID
	jz	return0		; no CPUID command, so no 3DNow!

	mov	eax,0x80000000
	CPUID
	cmp	eax,0x80000000
	jbe	return0		; no extended MSR(1), so no 3DNow!

	mov	eax,0x80000001
	CPUID
	test	edx,0x80000000
	jz	return0		; no 3DNow! support
				; 3DNow! support
return1:
	popad
	xor	eax,eax
	inc	eax
	ret

return0:
	popad
	xor	eax,eax
	ret

;---------------------------------------
;	int  has_AVX_nasm (void)
;---------------------------------------

has_AVX_nasm:
        pushad
	call	testCPUID
	jz	return0		; no CPUID command, so no AVX

	mov	eax,0x1
	CPUID
	test	ecx,0x10000000	; test bit 28 (AVX)
	jz	return0		; no AVX support
	jmp	return1		; AVX support

;---------------------------------------
;	int  has_FMA_nasm (void)
;---------------------------------------

has_FMA_nasm:
        pushad
	call	testCPUID
	jz	return0		; no CPUID command, so no FMA

	mov	eax,0x1
	CPUID
	test	ecx,0x1000	; test bit 12 (FMA)
	jz	return0		; no FMA support
	jmp	return1		; FMA support

;---------------------------------------
;	int  has_AVX2_nasm (void)
;---------------------------------------

has_AVX2_nasm:
        pushad
	call	testCPUID
	jz	return0		; no CPUID command, so no AVX2

	; Check if leaf 7 is supported
	mov	eax,0x0
	CPUID
	cmp	eax,0x7
	jb	return0		; CPUID leaf 7 not supported

	mov	eax,0x7
	xor	ecx,ecx		; sub-leaf 0
	CPUID
	test	ebx,0x20	; test bit 5 (AVX2)
	jz	return0		; no AVX2 support
	jmp	return1		; AVX2 support

;---------------------------------------
;	int  has_AVX512F_nasm (void)
;---------------------------------------

has_AVX512F_nasm:
        pushad
	call	testCPUID
	jz	return0		; no CPUID command, so no AVX-512F

	; Check if leaf 7 is supported
	mov	eax,0x0
	CPUID
	cmp	eax,0x7
	jb	return0		; CPUID leaf 7 not supported

	mov	eax,0x7
	xor	ecx,ecx		; sub-leaf 0
	CPUID
	test	ebx,0x10000	; test bit 16 (AVX512F)
	jz	return0		; no AVX-512F support
	jmp	return1		; AVX-512F support

        end

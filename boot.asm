[BITS 16]
[ORG 0x7C00]

start:
    jmp 0x0000:start_real
disk_error:
    mov si, disk_error_string
    mov ah, 0x0E ; Screen print?

disk_error_print_loop:
    mov al, [si]
    cmp al, 0
    je infinite_loop

    int 0x10
    inc si

    jmp disk_error_print_loop

start_real:
    mov [boot_drive], dl

read_bootloader:
    mov ax, 0x1000
    mov es, ax
    mov ah, 2 ; Read sector code
    mov al, 2 ; Number of sectors to read
    mov ch, 0 ; cylinder number, which cylinder on disl
    mov cl, 2 ; Sectpr number, which sector to start reading from
    mov dh, 0 ; Head number, 0 for top side of disk
    mov dl, [boot_drive] ; which drive to read from
    mov bx, 0 ; ES:BX = 0x1000:0x0000 = 0x10000. Loads data in memory here

    int 0x13
    jc disk_error ; jump if carry flag is set

    ; Enable A20
    in al, 0x92
    or al, 2
    out 0x92, al

    ; Prepare CPU for mode switching
    mov ax, 0
    mov ds, ax ; data segment must be set to 0
    cli ; disable interrupts

    lgdt [gdt_descriptor]

    ; Switch to protected mode
    mov eax, cr0
    or eax, 0x1
    mov cr0, eax

    jmp 0x08:protected_mode_start ; 0x08 means offset now, jumps to gdt entry 1

[BITS 32]
protected_mode_start:
    ; Update all segment registers to 0x10
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Set up stack pointer
    mov esp, 0x90000

    mov byte [0xB8000], 'B'
    mov byte [0xB8001], 0x0F

    ; Copy kernel from 0x10000 to 0x100000 (previous 16bit had not access to memory so high up)
    mov esi, 0x10000 ; Source index (where to read from)
    mov edi, 0x100000 ; Destination index (where to write to)
    mov ecx, 512 * 2 ; Size: sectors * 512 bytes (count)
    rep movsb ; Copy byte by byte (movsb: move string byte) (rep: repeat)

    ; Jump to kernel
    jmp 0x100000 

    jmp infinite_loop

infinite_loop:
    jmp $

; Data section
boot_drive: db 0
disk_error_string: db "Disk Error", 0

gdt_start:
gdt_null:
    dq 0x0 ; 8 0 bytes
gdt_code:
    dw 0xFFFF ; Size of memory to use
    dw 0x0000 ; Start of memory we use
    db 0x00 ; Next byte for start of memory
    db 0x9A ; Access byte
    db 0xCF ; flags + limit bytes
    db 0x00 ; Final byte for start of memory
gdt_data:
    dw 0xFFFF ; Size of memory to use
    dw 0x0000 ; Start of memory we use
    db 0x00 ; Next byte for start of memory
    db 0x92 ; Access byte with permissions set for data
    db 0xCF ; flags + limit bytes
    db 0x00 ; Final byte for start of memory
gdt_end:
gdt_descriptor:
    dw gdt_end - gdt_start - 1 ; size of gdt
    dd gdt_start ; start address of gdt

times 510-($-$$) db 0
dw 0xAA55
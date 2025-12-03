# Explication détaillée des fichiers du Kernel

## Table des matières
1. [linker.ld - Script de liaison](#linkerld---script-de-liaison)
2. [boot.asm - Bootstrap en assembleur](#bootasm---bootstrap-en-assembleur)
3. [i386-unknown-none.json - Spécification de cible](#i386-unknown-nonejson---spécification-de-cible)
4. [lib.rs - Kernel Rust](#librs---kernel-rust)
5. [grub.cfg - Configuration du bootloader](#grubcfg---configuration-du-bootloader)

---

## linker.ld - Script de liaison

### Rôle
Le linker script définit **comment organiser les sections du programme en mémoire**. Il indique au linker (`ld`) où placer le code, les données et les différentes sections du kernel.

### Structure détaillée

```ld
ENTRY(_start)
```
**Point d'entrée du programme** : Indique que la première fonction à exécuter est `_start` (définie dans `boot.asm`). C'est l'adresse où le bootloader (GRUB) va sauter après avoir chargé le kernel.

---

```ld
SECTIONS {
    . = 1M;
```
**Adresse de départ : 1 Mégabyte (0x100000)**
- Le symbole `.` représente l'adresse mémoire courante
- `1M` = 1048576 bytes = 0x100000 en hexadécimal
- Cette adresse est standard pour les kernels Multiboot :
  - `0x00000 - 0x9FFFF` : Mémoire basse utilisée par le BIOS
  - `0xA0000 - 0xFFFFF` : Mémoire vidéo et ROM BIOS
  - `0x100000+` : Zone libre pour le kernel

---

```ld
.boot : {
    *(.multiboot)
}
```
**Section `.boot` - En-tête Multiboot**
- Contient l'en-tête Multiboot de `boot.asm`
- **DOIT être en premier** dans le binaire final
- GRUB scanne les premiers 8 KB du kernel pour trouver cette signature
- Sans cet en-tête au début, GRUB refusera de charger le kernel

---

```ld
.text : {
    *(.text)
}
```
**Section `.text` - Code exécutable**
- Contient tout le code machine du kernel
- Inclut `_start` de `boot.asm` et le code compilé de Rust
- `*(.text)` = "tous les fichiers objets, leurs sections .text"
- Permissions typiques : Read + Execute (RX)

---

```ld
.rodata : {
    *(.rodata)
}
```
**Section `.rodata` - Données en lecture seule**
- Contient les constantes et chaînes littérales
- Exemple : `const VGA_MEMORY: usize = 0xB8000;`
- Permissions typiques : Read-Only (R)
- Séparer `.rodata` de `.text` améliore la sécurité

---

```ld
.data : {
    *(.data)
}
```
**Section `.data` - Données initialisées**
- Variables globales/statiques avec valeur initiale
- Exemple : `static mut COUNTER: u32 = 0;`
- Permissions : Read + Write (RW)
- Chargée depuis le binaire (valeurs copiées)

---

```ld
.bss : {
    *(.bss)
}
```
**Section `.bss` - Données non-initialisées**
- Variables globales sans valeur initiale (zéro par défaut)
- Exemple : `static mut BUFFER: [u8; 1024];`
- **N'occupe PAS d'espace dans le binaire final** (juste une taille)
- Le loader initialise cette zone à zéro au démarrage
- Économise de l'espace disque pour les gros buffers

---

### Layout mémoire résultant

```
0x100000  ┌─────────────────┐
          │  .multiboot     │  <- En-tête GRUB (magic, flags)
          ├─────────────────┤
          │  .text          │  <- Code: _start, kernel_main
          ├─────────────────┤
          │  .rodata        │  <- Constantes (VGA_MEMORY, etc.)
          ├─────────────────┤
          │  .data          │  <- Variables initialisées
          ├─────────────────┤
          │  .bss           │  <- Variables non-init (stack)
          └─────────────────┘
```

---

## boot.asm - Bootstrap en assembleur

### Rôle
**Premier code exécuté au boot**. Prépare l'environnement minimal pour exécuter du code Rust (configuration de la pile, passage de paramètres Multiboot).

### Structure détaillée

```asm
bits 32
```
**Mode 32-bit** : Indique à NASM de générer du code pour le mode protégé 32-bit (x86).

---

### Constantes Multiboot

```asm
MBALIGN   equ 1 << 0     ; Bit 0 : Aligner modules sur pages (4KB)
MEMINFO   equ 1 << 1     ; Bit 1 : Demander la carte mémoire
FLAGS     equ MBALIGN | MEMINFO
```
**Flags de fonctionnalités** :
- `MBALIGN` : Les modules doivent être alignés sur 4 KB
- `MEMINFO` : GRUB doit fournir une carte de la mémoire disponible

```asm
MAGIC     equ 0x1BADB002
```
**Magic number** : Signature Multiboot obligatoire. GRUB cherche cette valeur spécifique.

```asm
CHECKSUM  equ -(MAGIC + FLAGS)
```
**Checksum de validation** : `MAGIC + FLAGS + CHECKSUM` doit être égal à zéro (en 32-bit).

---

### Section Multiboot

```asm
section .multiboot
align 4
    dd MAGIC      ; 0x1BADB002
    dd FLAGS      ; 0x00000003
    dd CHECKSUM   ; Complément à zéro
```
**En-tête Multiboot** (12 bytes) :
- `dd` = Define Doubleword (4 bytes)
- `align 4` = Aligné sur 4 bytes
- Cette structure est placée au tout début du binaire par le linker script

**Format attendu par GRUB** :
```
Offset  | Contenu
--------|------------------
0x00    | Magic (0x1BADB002)
0x04    | Flags
0x08    | Checksum
```

---

### Section Stack (BSS)

```asm
section .bss
align 16
stack_bottom:
    resb 16384    ; Réserve 16 KB (16 * 1024 bytes)
stack_top:
```
**Pile d'exécution** :
- `resb` = Reserve Bytes (n'initialise pas)
- Taille : 16 KB (suffisant pour les appels de fonctions)
- `align 16` : Alignement requis pour les performances x86
- La pile **grandit vers le bas** : `esp` démarre à `stack_top`

**Pourquoi nécessaire ?**
- Rust/C utilisent la pile pour :
  - Variables locales
  - Adresses de retour de fonctions
  - Passage de paramètres
- Sans pile configurée → Crash immédiat

---

### Section Code

```asm
section .text
global _start
extern kernel_main
```
**Déclarations** :
- `global _start` : Exporte le symbole (visible par le linker)
- `extern kernel_main` : Importe une fonction définie ailleurs (Rust)

---

```asm
_start:
    mov esp, stack_top
```
**Configuration du Stack Pointer** :
- `esp` = Extended Stack Pointer (registre pointant vers le sommet de la pile)
- Pointe maintenant vers `stack_top` (fin de la zone réservée)

---

```asm
    push ebx    ; Multiboot info structure
    push eax    ; Multiboot magic number
```
**Passage de paramètres Multiboot** :
- À l'entrée, GRUB place dans les registres :
  - `eax` = Magic number (0x2BADB002 si boot Multiboot réussi)
  - `ebx` = Adresse physique de la structure `multiboot_info`
- `push` empile ces valeurs (convention d'appel cdecl)
- Ordre inverse car la pile grandit vers le bas

---

```asm
    call kernel_main
```
**Appel de la fonction Rust** :
- `call` empile l'adresse de retour et saute à `kernel_main`
- Équivalent C : `kernel_main(eax, ebx);`

---

```asm
    cli
.hang:
    hlt
    jmp .hang
```
**Boucle infinie finale** :
- `cli` = Clear Interrupts (désactive les interruptions)
- `hlt` = Halt (met le CPU en veille jusqu'à interruption)
- Si une interruption survient (IRQ matériel), on revient à `hlt`
- **Cas d'usage** : Si `kernel_main` retourne (ne devrait jamais arriver)

---

## i386-unknown-none.json - Spécification de cible

### Rôle
**Définit une cible de compilation personnalisée pour Rust**. Comme i386 bare-metal n'est pas une cible intégrée, ce fichier décrit précisément l'architecture, l'ABI et les fonctionnalités supportées.

### Champs détaillés

```json
"llvm-target": "i386-unknown-none",
```
**Triple de cible LLVM** :
- `i386` : Architecture (x86 32-bit, Intel 80386)
- `unknown` : Vendor (pas de fabricant spécifique)
- `none` : OS (bare-metal, pas de système d'exploitation)

---

```json
"data-layout": "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-i128:128-f64:32:64-f80:32-n8:16:32-S128",
```
**Layout de données LLVM** (détermine comment organiser les types en mémoire) :
- `e` : Little-endian (byte de poids faible en premier)
- `m:e` : Mangling ELF
- `p:32:32` : Pointeurs = 32 bits, alignement 32 bits
- `p270:32:32` : Pointeurs addrspace 270 (code)
- `p271:32:32` : Pointeurs addrspace 271 (globals)
- `p272:64:64` : Pointeurs addrspace 272 (constant)
- `i128:128` : Entiers 128-bit alignés sur 128 bits
- `f64:32:64` : Doubles alignés sur 32 bits (ABI) mais préférence 64
- `f80:32` : Long double (x87) aligné sur 32 bits
- `n8:16:32` : Tailles natives d'entiers (8, 16, 32 bits)
- `S128` : Stack alignée sur 128 bits

**Pourquoi crucial ?** Un mauvais layout = comportement incorrect ou crashes.

---

```json
"arch": "x86",
"target-endian": "little",
"target-pointer-width": 32,
"target-c-int-width": 32,
```
**Caractéristiques de base** :
- `arch` : Famille d'architecture (x86)
- `target-endian` : Ordre des bytes (little = LSB first)
- `target-pointer-width` : Taille des pointeurs (32 bits)
- `target-c-int-width` : Taille du type `int` en C (32 bits)

---

```json
"os": "none",
"executables": true,
```
**Environnement d'exécution** :
- `os: none` : Pas d'OS sous-jacent (bare-metal)
- `executables: true` : Peut produire des binaires exécutables

---

```json
"linker-flavor": "ld.lld",
"linker": "rust-lld",
```
**Configuration du linker** :
- `linker-flavor` : Type de linker (LLD, version LLVM de ld)
- `linker` : Commande (rust-lld = wrapper Rust autour de lld)
- **Pourquoi LLD ?** Plus moderne que GNU ld, meilleure intégration LLVM

---

```json
"panic-strategy": "abort",
```
**Gestion des panics** :
- `abort` : Panic = arrêt immédiat (pas d'unwinding de la stack)
- Alternative : `unwind` (dérouler la pile, appeler destructeurs)
- **Pourquoi abort ?** Unwinding nécessite runtime complexe (incompatible bare-metal)

---

```json
"disable-redzone": true,
```
**Red zone x86-64** :
- Zone de 128 bytes **sous** le stack pointer (RSP-128 à RSP)
- Utilisée par le compilateur pour variables temporaires sans ajuster RSP
- **Problème en kernel** : Les interruptions peuvent écraser cette zone !
- **Solution** : Désactiver complètement

---

```json
"features": "-mmx,-sse",
```
**Extensions CPU désactivées** :
- `-mmx` : Pas d'instructions MMX (registres MM0-MM7)
- `-sse` : Pas d'instructions SSE (registres XMM0-XMM15)

**Pourquoi ?**
- MMX/SSE nécessitent initialiser le coprocesseur (FPU)
- Complexifie la gestion du contexte lors des interruptions
- Rust utilisera des émulations software si besoin

---

```json
"cpu": "i386"
```
**CPU minimum supporté** :
- i386 = Intel 80386 (1985)
- Pas d'instructions modernes (SSE2, AVX, etc.)
- Compatible avec tous les x86 depuis 30+ ans

---

## lib.rs - Kernel Rust

### Attributs globaux

```rust
#![no_std]
```
**Pas de bibliothèque standard** :
- Désactive `std` (dépend de l'OS : fichiers, threads, heap)
- Seul `core` est disponible (types primitifs, traits, macros)
- Nécessaire en bare-metal (pas d'OS sous-jacent)

---

```rust
#![no_main]
```
**Pas de point d'entrée standard** :
- Normalement Rust appelle `fn main()`
- Ici le point d'entrée est `kernel_main` (appelé depuis l'ASM)
- Désactive le runtime Rust standard

---

### Constantes VGA

```rust
const VGA_MEMORY: usize = 0xB8000;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
```
**Mode texte VGA** :
- `0xB8000` : Adresse physique du buffer vidéo (mappée par le hardware)
- 80 colonnes × 25 lignes = 2000 caractères
- Chaque caractère = 2 bytes (ASCII + attributs)

**Format d'un caractère VGA** :
```
Byte 0: Code ASCII du caractère (ex: 'A' = 0x41)
Byte 1: Attributs
  [7]     : Blink (clignotement)
  [6-4]   : Background color (3 bits)
  [3-0]   : Foreground color (4 bits)
```

---

### Point d'entrée Rust

```rust
#[no_mangle]
pub extern "C" fn kernel_main(_magic: u32, _addr: u32) {
```
**Déclaration de la fonction** :
- `#[no_mangle]` : Garde le nom exact "kernel_main" dans le symbole
  - Sans ça : Rust manglerait en `_ZN6kernel12kernel_main17h...`
  - Nécessaire pour que l'ASM trouve le symbole
- `extern "C"` : Convention d'appel C (paramètres sur la pile)
- `_magic` : Magic number Multiboot (0x2BADB002)
- `_addr` : Pointeur vers `multiboot_info_t` (carte mémoire, modules, etc.)

**Convention d'appel cdecl (x86)** :
1. Paramètres empilés de droite à gauche
2. `eax` contient la valeur de retour
3. L'appelant nettoie la pile

---

### Écriture VGA

```rust
let vga = VGA_MEMORY as *mut u16;
```
**Cast vers pointeur raw** :
- `VGA_MEMORY` (adresse `usize`) → `*mut u16` (pointeur mutable vers u16)
- Chaque caractère VGA = 16 bits = `u16`
- Utilisation de pointeurs raw car on accède directement au hardware

---

```rust
let msg = b"Hello from Rust kernel!";
```
**Chaîne de bytes** :
- `b"..."` = byte string literal (`&[u8]`)
- Équivalent à `&[72, 101, 108, 108, 111, ...]` (codes ASCII)

---

```rust
unsafe {
    // Clear screen
    for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
        *vga.add(i) = 0x0F00 | b' ' as u16;
    }
```
**Effacer l'écran** :
- Boucle sur 2000 cellules (80×25)
- `vga.add(i)` : Arithmétique de pointeurs (avance de `i * sizeof(u16)` bytes)
- `0x0F00` : Attributs (blanc sur noir)
  - `0x0` : Background noir
  - `0xF` : Foreground blanc brillant
- `b' '` : Caractère espace (ASCII 0x20)
- `|` : OR binaire pour combiner attributs + caractère

**Résultat** : Chaque cellule contient `0x0F20` (espace blanc sur noir)

---

```rust
    // Print message
    for (i, &byte) in msg.iter().enumerate() {
        *vga.add(i) = 0x0F00 | byte as u16;
    }
}
```
**Afficher le message** :
- `.iter().enumerate()` : Itère avec index
- `&byte` : Pattern matching pour déréférencer
- Écrit chaque caractère avec les mêmes attributs

---

```rust
loop {
    unsafe {
        core::arch::asm!("hlt");
    }
}
```
**Boucle infinie** :
- `core::arch::asm!` : Inline assembly (nécessite `unsafe`)
- `hlt` : Instruction CPU qui met en veille jusqu'à interruption
- Économise de l'énergie (CPU idle)

**Pourquoi une boucle ?**
- Si une interruption survient (ex: timer), le CPU sort de `hlt`
- On re-execute `hlt` immédiatement

---

### Panic Handler

```rust
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
```
**Gestionnaire de panic obligatoire** :
- En `no_std`, il faut définir le comportement en cas de panic
- `-> !` : Type "never" (jamais de retour)
- `PanicInfo` contient message et location du panic

**Comportement actuel** : Halt le CPU (pas d'affichage d'erreur)

**Amélioration possible** :
```rust
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Afficher message en rouge
    let vga = 0xB8000 as *mut u16;
    let msg = b"KERNEL PANIC!";
    unsafe {
        for (i, &byte) in msg.iter().enumerate() {
            *vga.add(i) = 0x4F00 | byte as u16; // Rouge vif
        }
    }
    loop { unsafe { core::arch::asm!("hlt"); } }
}
```

---

## grub.cfg - Configuration du bootloader

### Structure

```
set timeout=0
```
**Timeout du menu GRUB** :
- `0` secondes = Pas d'attente, boot immédiat
- Valeur typique : `5` (5 secondes pour choisir)

---

```
set default=0
```
**Entrée par défaut** :
- Index `0` = Première entrée de menu
- Si timeout expire, cette entrée est choisie

---

```
menuentry "My Custom Kernel" {
```
**Définition d'une entrée de menu** :
- Texte affiché : "My Custom Kernel"
- Peut avoir plusieurs `menuentry` pour différents kernels

---

```
    multiboot /boot/kernel.bin
```
**Commande Multiboot** :
- Indique à GRUB de charger `kernel.bin` selon le protocole Multiboot
- Chemin relatif à la racine de l'ISO (dans `isodir/boot/`)
- GRUB va :
  1. Charger le fichier en mémoire à 1 MB
  2. Vérifier l'en-tête Multiboot
  3. Préparer la structure `multiboot_info`
  4. Passer en mode protégé 32-bit
  5. Sauter à `_start`

---

```
    boot
}
```
**Commande de démarrage** :
- Lance effectivement le kernel
- Transfère le contrôle du CPU

---

### Fonctionnalités Multiboot offertes par GRUB

Quand GRUB charge un kernel Multiboot, il fournit :

1. **Carte mémoire** (`multiboot_info.mmap`)
   - Régions de RAM disponibles
   - Zones réservées (ACPI, hardware)

2. **Ligne de commande** (`multiboot_info.cmdline`)
   - Arguments passés au kernel
   - Exemple : `multiboot /boot/kernel.bin root=/dev/sda1`

3. **Modules** (`multiboot_info.mods`)
   - Ramdisks, drivers à charger
   - Exemple : `module /boot/initrd.img`

4. **Informations boot** :
   - Nom du bootloader
   - Framebuffer graphique
   - Symboles de débogage

---

### Processus complet de boot

```
┌─────────────────────────────────────┐
│ 1. PC Power-On                      │
│    - BIOS/UEFI initialise hardware  │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│ 2. BIOS lit le MBR (secteur 0)     │
│    - Trouve GRUB Stage 1            │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│ 3. GRUB Stage 1.5 / Stage 2         │
│    - Charge filesystem drivers      │
│    - Lit grub.cfg                   │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│ 4. GRUB affiche menu                │
│    - Timeout 0s → sélection auto    │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│ 5. GRUB charge kernel.bin           │
│    - Copie en RAM à 0x100000        │
│    - Vérifie header Multiboot       │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│ 6. GRUB prépare environnement       │
│    - eax = 0x2BADB002               │
│    - ebx = adresse multiboot_info   │
│    - CPU en mode protégé 32-bit     │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│ 7. GRUB saute à _start              │
│    (adresse définie dans linker.ld) │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│ 8. boot.asm : _start                │
│    - mov esp, stack_top             │
│    - push ebx / push eax            │
│    - call kernel_main               │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│ 9. lib.rs : kernel_main()           │
│    - Efface écran VGA               │
│    - Affiche "Hello from Rust..."   │
│    - Boucle hlt infinie             │
└─────────────────────────────────────┘
```

---

## Résumé des interactions entre fichiers

```
┌─────────────┐
│  grub.cfg   │ ──┐
└─────────────┘   │
                  │  GRUB lit la config
                  │  et charge le kernel
┌─────────────┐   │
│ kernel.bin  │ ◄─┘
└──────┬──────┘
       │ Créé par ld avec linker.ld
       │
       ├──────────────────────┬──────────────────┐
       │                      │                  │
┌──────▼──────┐      ┌────────▼───────┐   ┌─────▼──────┐
│   boot.o    │      │  libkernel.a   │   │ linker.ld  │
│ (boot.asm)  │      │   (lib.rs)     │   │            │
└─────────────┘      └────────────────┘   └────────────┘
       │                      │
       │                      │ Compilé avec cible →
       │                      │
       │             ┌────────▼───────────────────┐
       │             │ i386-unknown-none.json     │
       │             │ rust-toolchain.toml        │
       │             └────────────────────────────┘
       │
       └──► Point d'entrée : _start
                 │
                 └──► Appelle kernel_main (Rust)
```

---

## Pour aller plus loin

### Prochaines étapes typiques :

1. **GDT (Global Descriptor Table)** : Configurer la segmentation x86
2. **IDT (Interrupt Descriptor Table)** : Gérer interruptions et exceptions
3. **Keyboard driver** : Lire les entrées clavier (port 0x60)
4. **Heap allocator** : Implémenter `malloc`/`free` pour utiliser `alloc`
5. **Pagination** : Activer la MMU (Memory Management Unit)
6. **Multitasking** : Scheduler et changement de contexte
7. **Filesystem** : Lire des fichiers (FAT32, ext2, etc.)

### Ressources recommandées :

- **OSDev Wiki** : https://wiki.osdev.org/
- **Writing an OS in Rust** : https://os.phil-opp.com/
- **Intel Software Developer Manual** : Documentation CPU complète
- **Multiboot Specification** : https://www.gnu.org/software/grub/manual/multiboot/

---

**Votre kernel est maintenant prêt à booter ! 🚀**

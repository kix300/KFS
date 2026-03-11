# KFS - Kernel From Scratch

Un projet de système d'exploitation minimaliste développé en Rust et assembleur x86.

## Description

KFS est un noyau de système d'exploitation bare-metal conçu pour l'architecture x86 (32-bit). Le projet implémente les fonctionnalités de base d'un OS, incluant la gestion du VGA buffer, du clavier et de la souris.

## Fonctionnalités

- **Boot multiboot** : Compatible avec GRUB
- **Affichage VGA** : Buffer texte avec support de couleurs
- **Gestion du clavier** : Entrées clavier PS/2
- **Support de la souris** : Détection des mouvements, clics et molette
- **GDT (Global Descriptor Table)** : Configuration des segments mémoire
- **Mode tests** : Suite de tests intégrée

## Prérequis

- Rust (nightly)
- QEMU (pour l'émulation)
- NASM (assembleur)
- GNU Make
- GRUB (pour le bootloader)

## Compilation et exécution

### Lancer le kernel

```bash
make run
```

### Lancer les tests

```bash
make test
```

### Nettoyer le projet

```bash
make clean
```

## Structure du projet

```
KFS/
├── src/
│   ├── boot.asm          # Point d'entrée et header multiboot
│   ├── gdt.asm           # Configuration de la GDT
│   ├── linker.ld         # Script de linkage
│   └── kernel/
│       ├── src/
│       │   ├── kernel.rs       # Point d'entrée Rust
│       │   ├── vga_buffer/     # Gestion de l'affichage
│       │   ├── device/         # Drivers (clavier, souris)
│       │   ├── x86/            # Code spécifique x86
│       │   ├── panic/          # Gestion des panics
│       │   └── tests/          # Tests du kernel
│       └── Cargo.toml
└── Makefile
```

## Architecture

- **Langage** : Rust (no_std) + Assembleur x86
- **Target** : i386-unknown-none (bare-metal)
- **Bootloader** : GRUB2
- **Émulateur** : QEMU

## Notes techniques

Le projet utilise :
- `#![no_std]` : Pas de bibliothèque standard
- `#![no_main]` : Point d'entrée personnalisé
- Interruptions désactivées (polling pour clavier/souris)
- Stack de 16 KB

## Mode test

Le kernel peut être compilé avec le flag `kfs_test` pour exécuter une suite de tests au démarrage :

```bash
make test
```

## Auteur

Created by kix

---

*Projet éducatif de développement d'OS from scratch*

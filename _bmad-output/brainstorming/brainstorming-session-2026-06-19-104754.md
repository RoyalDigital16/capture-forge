---
stepsCompleted: [1]
inputDocuments: []
session_topic: 'Recorder Core — innovations UX, fonctionnalités et edge cases pour un enregistreur d''écran Rust/WASM nouvelle génération'
session_goals: '1. Identifier les fonctionnalités différenciatrices face à l''original et aux alternatives (Loom, OBS, etc.) 2. Brainstormer des solutions aux edge cases (crash recovery, multi-source, mémoire) 3. Imaginer l''expérience utilisateur idéale 4. Explorer le modèle de contribution communautaire'
selected_approach: ''
techniques_used: []
ideas_generated: []
context_file: ''
---

# Brainstorming Session Results

**Facilitator:** {{user_name}}
**Date:** {{date}}

# Brainstorming — CaptureForge

## Contexte

**CaptureForge** (Alyssa X) : enregistreur d'écran Chrome open-source (GPLv3), 18.3k stars.
Rebrandé en Demokraft AI avec cloud payant. Code source : React 18 + Webpack.

**Objectif** : fork communautaire **CaptureForge** en Rust via **Oxichrome**.

---

## Architecture en 3 sous-produits

```
┌─────────────────────────────────────────────────────────┐
│  CaptureForge                                     │
│                                                          │
│  ┌─────────────────────┐  ┌─────────┐  ┌──────────────┐ │
│  │ Recorder Core (P0)  │  │ Editor  │  │ AI/Enrich.   │ │
│  │                     │  │ (P1)    │  │ (P2, option) │ │
│  │ Screen/Tab record   │  │ Player  │  │ sherpa-onnx  │ │
│  │ Micro               │  │ Trim    │  │ aisdk        │ │
│  │ Pause/Resume/Stop   │  │ Mute    │  │ DOM capture  │ │
│  │ WEBM export         │  │ Crop    │  │ Smart search │ │
│  │ OPFS storage        │  │ Export  │  │              │ │
│  │ Crash recovery      │  │         │  │              │ │
│  └─────────────────────┘  └─────────┘  └──────────────┘ │
│                                                          │
│  Chaque niveau est indépendant et livrable séparément.   │
│  Un bug dans l'IA ne bloque jamais le recording.         │
└─────────────────────────────────────────────────────────┘
```

---

## Analyse technique

### Original CaptureForge (React/JS) vs CaptureForge (Rust/Oxichrome)

| Aspect | Original | Community |
|--------|----------|-----------|
| Framework | React 18 + Radix UI | Leptos 0.7 |
| Build | Webpack 5 (17 entry points) | cargo oxichrome build |
| Canvas | Fabric.js | web-sys Canvas 2D |
| IA Vision | MediaPipe Tasks-Vision | JS interop (même lib) |
| IA Audio | ❌ | sherpa-onnx (optionnel) |
| IA LLM | ❌ | aisdk (optionnel) |
| Vidéo | FFmpeg WASM, WebCodecs | FFmpeg WASM (JS shim) |
| Stockage | localforage + OPFS | opfs crate + indexed_db_futures |
| Export | WEBM, MP4, GIF | WEBM (P0), MP4/GIF (P1) |
| Audio | wavesurfer.js | Web Audio API |
| Télémétrie | Sentry | Aucune |

### Stratégie Rust / JS

**Rust-first avec shims JS minimaux.**

| Technologie | Approche |
|------------|----------|
| Core (recording, storage, state, UI) | Rust only |
| MediaRecorder | web-sys (Rust natif) |
| FFmpeg WASM | JS shim (`js/ffmpeg.js`) — pas d'alternative Rust satisfaisante |
| MediaPipe | JS shim (`js/mediapipe.js`) |
| tabCapture, offscreen | JS shim (`js/chrome_shim.js`) — API Chrome non dans web-sys |
| OPFS | opfs crate (Rust natif) |
| sherpa-onnx | Crate Rust natif |

---

## Risques séparés par sous-produit

### Recorder Core

| Risque | Probabilité | Impact | Mitigation |
|--------|------------|--------|------------|
| MediaRecorder échoue sur certains GPU | Faible | Moyen | fallback VP8, message clair |
| OPFS non disponible | Très faible | Faible | fallback IndexedDB |
| Oxichrome v0.2 immature | Moyen | Haut | Fork du repo si nécessaire |
| APIs Chrome (tabCapture) pas en web-sys | Faible | Faible | JS shim de 20 lignes |

### Editor

| Risque | Probabilité | Impact | Mitigation |
|--------|------------|--------|------------|
| FFmpeg WASM lent en browser | Moyen | Moyen | Web Worker séparé |
| Canvas annotations lent en WASM | Faible | Faible | web-sys Canvas direct |

### AI/Enrichment

| Risque | Probabilité | Impact | Mitigation |
|--------|------------|--------|------------|
| sherpa-onnx WASM > modèle 20MB | Haut | Faible | Download progressif, feature gate |
| Taille ONNX + WASM > mémoire | Moyen | Faible | Zipformer tiny uniquement |

---

## Questions ouvertes

1. Licence : GPLv3 (comme original) ou MIT (comme Oxichrome) ?
2. Publication Chrome Web Store ou GitHub uniquement ?
3. Modèle par défaut sherpa-onnx : Zipformer EN (20MB) ou Moonshine tiny (20MB) ?
4. Quelle priorité pour Firefox : avant ou après la V1.0 ?

---

## Idées futures

- Mode CLI Rust (batch processing)
- Plugin system (API Rust pour extensions)
- Streaming direct (RTMP/S3)
- Background removal Rust pur (rullama)
- Intégration PM : export tutoriel vers Notion/Jira

---

## Session Overview

**Topic:** Recorder Core — innovations UX, fonctionnalités et edge cases pour un enregistreur d'écran Rust/WASM nouvelle génération

**Goals:**
1. Identifier les fonctionnalités différenciatrices face à l'original et aux alternatives (Loom, OBS, etc.)
2. Brainstormer des solutions aux edge cases (crash recovery, multi-source, mémoire)
3. Imaginer l'expérience utilisateur idéale
4. Explorer le modèle de contribution communautaire

### Context Guidance

Session basée sur l'analyse technique préliminaire couvrant l'architecture Rust/Oxichrome, la comparaison avec l'original CaptureForge (React/JS), les risques par sous-produit, et la stratégie Rust-first avec shims JS minimaux.

### Session Setup

Configuration validée par l'utilisateur le 2026-06-19. En attente de sélection de l'approche de brainstorming.

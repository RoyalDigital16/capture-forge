---
title: "PRFAQ Distillate: Capture Forge"
type: llm-distillate
source: "prfaq-capture-forge.md"
created: "2026-06-19"
purpose: "Token-efficient context for downstream PRD creation"
---

## Concept Core

- **Headline**: "Capture Forge : une capture qui devient une preuve"
- **Subheadline**: Extension open-source pour transformer une capture écran en artefact réutilisable, sans compte, sans watermark, sans cloud imposé
- **Vision phrase**: "Capture Forge transforme une session écran en source de vérité réutilisable, au lieu de produire seulement une vidéo de plus"
- **Emotional angle**: Soulagement de ne plus refaire trois fois le même travail pour trois publics différents
- **Persona**: Ingénieur·e produit / QA / support technique — quelqu'un qui documente bugs, flux, configurations quotidiennement
- **Concept type**: Open-source product (MIT license, community-driven, no company backing)

## Rejected Framings

- "Un meilleur recorder" → trop peu différenciant, ne capte pas la vision
- "Source de vérité" → trop abstrait pour un scroll-stop, remplacé par "preuve"
- "Workflow" → remplacé par "flux" (plus humain, moins corporate)
- Les métaphores "matière première" et "répertoire de sessions" → trop aspirantes pour V0.1, remplacées par des formulations ancrées dans l'utilité immédiate
- La promesse AudienceLens retirée du Press Release et du How It Works (projetait trop loin) → gardée en filigrane dans la différenciation architecturale

## Press Release - Key Decisions

- **Headline**: retenue "une capture qui devient une preuve" — le mot "preuve" fait tilt immédiat chez QA et support
- **Problem paragraph**: pure douleur, sans mention des causes structurelles, sans chiffre non sourcé
- **Differentiation**: pas un comparatif produit, mais une "raison d'existence" — une capture utile doit rester locale, réutilisable et sous contrôle
- **How It Works**: scope V0.1 strict, aucune mention des AudienceLenses, pas de détail technique (OPFS/chunks/SW) — juste l'expérience utilisateur fluide
- **Founder quote**: "Le vrai coût d'un bug, ce n'est pas de le reproduire — c'est de le faire comprendre à quelqu'un qui ne l'a pas vu"
- **User quote**: scope V0.1 — "Je peux enregistrer une fois, récupérer un fichier propre, et l'envoyer tout de suite"
- **Getting Started**: zéro promesse future, uniquement l'installation et l'export WebM natif

## Customer FAQ - Key Insights

- **Q8 (vision vs réalité)** : question la plus dangereuse. Réponse retenue : reconnaître que V0.1 ressemble à un recorder, mais la différence est dans la structure. "V0.1 n'est pas encore la destination finale, mais c'est déjà le bon fondement."
- **Q6 (Screenity)** : Screenity est un monolithe React/JS ; Capture Forge est un moteur Rust/WASM modulaire. La réponse ne dénigre pas, elle reconnaît que Screenity est bon et que Capture Forge est conçu pour une évolution différente.
- **Q2 (data loss / crash)** : le protocole deux phases + heartbeat + triple vérification. Message clé : perte maximale d'un chunk (~10s), rapport d'intégrité natif.
- **Q3 (MP4)** : WebM en V0.1 (format natif MediaRecorder), MP4 en V0.5 (FFmpeg WASM). Honnête sur le gap.
- **Q5 (permissions)** : stockage local + capture + micro seulement. Code Rust/WASM vérifiable.
- **Q7 (workflow d'équipe)** : outil individuel en V0.1. Pas d'engagement sur le collaboratif. Honnêteté radicale.
- **Chiffres non sourcés volontairement retirés** : "15-30 min" de gain n'apparaît pas dans le PRFAQ final.

## Internal FAQ - Key Insights

- **Q7 (continuité)** : réponse à trois niveaux — données exportables/portables (pas de lock-in), licence MIT (fork possible), architecture modulaire (reprise viable). La phrase clé : "Ce n'est pas une garantie de continuité humaine. C'est une garantie que le produit ne se transforme pas en impasse technique."
- **Q3 (moteur d'adoption)** : l'adoption vient de l'intégration dans un geste de travail répétitif, pas du marketing. Trois accélérateurs : effet de réseau professionnel, GitHub/open-source (confiance), fiabilité (moins de captures perdues = habitude).
- **Q2 (Oxichrome)** : risque accepté, stratégie = contrib upstream + fork borné si nécessaire. Le code métier n'en dépend pas.
- **Q1 (risque technique)** : pipeline d'écriture OPFS sous contrainte mémoire/temps réel est le plus risqué. Stratégie conservative : en cas de doute, marquer le chunk suspect plutôt que supposer valide.
- **Q5 (calendrier)** : V0.1 Q3 2026 tendu mais réaliste. Editor et Firefox sont indépendants et ne bloquent pas la V0.1.
- **Q6 (copie)** : ce qui est difficile à copier = la combinaison architecture modulaire + philosophie résiliente + cohérence produit. Meilleure protection = vitesse d'exécution et clarté de vision.

## The Verdict - Key Findings

### Forged
- Thèse produit originale (aucun concurrent n'exploite "capture → actif réutilisable")
- Persona précis et vécu (pas théorique)
- Architecture résiliente comme différenciateur concret
- Honest framing : le gap V0.1 vs vision est reconnu, pas ignoré

### Needs more heat
- Le saut V0.1 → AudienceLens : la communication doit être chirurgicale
- WebM vs MP4 : défendable mais exclut une partie du marché
- Oxichrome : dépendance jeune, CI/build reproductible à investir tôt

### Cracks
- Pas de stratégie de continuité au-delà du MIT — acceptable V0.1, problématique si passage à l'échelle
- "Session structurée réutilisable" pas encore exploitable par l'utilisateur V0.1 (livre un WebM comme tout le monde)
- Aucun "why now" développé — pourquoi ce moment est le bon ?

### Open question
- "Est-ce que la V0.1 est suffisamment utile par elle-même pour créer l'adoption qui justifiera les versions suivantes ?" — le marché répondra.

## Technical Context & Constraints

- Stack : Rust + Oxichrome + web-sys + Leptos + OPFS
- Target : Chrome 120+ (référence), Firefox (P1)
- Architecture : 3 sous-produits indépendants (Recorder Core P0 / Editor P1 / AI P2), feature-gated
- Résilience : protocole `.partial → .bin`, heartbeat offscreen, triple vérification, IntegrityReport
- Build : `wasm-pack build --target web` + `cargo oxichrome build`
- Format V0.1 : WebM (VP8 + Opus), export MP4 en V0.5 via FFmpeg WASM
- Feature flags Cargo : `recorder`, `storage`, `export` (default) ; `overlay`, `editor`, `camera`, `stt`, `llm`, `dom` (optionnels)
- JS shims minimaux : `chrome_shim.js` (tabCapture), `ffmpeg.js` (P1), `mediapipe.js` (P1)
- Risques techniques clés : fiabilité OPFS sous contrainte, maturité Oxichrome v0.2, SW lifecycle MV3

## Scope Signals

### In (V0.1)
- Screen/tab capture + micro (GetDisplayMedia / tabCapture)
- Pause / resume / stop / cancel
- Export WebM (VP8 + Opus)
- Stockage OPFS + IndexedDB fallback
- Chunk status lifecycle (`.partial → .written → .committed → .verified`)
- Triple verification (manifest vs files, size, sequence)
- IntegrityReport natif après recovery
- Heartbeat offscreen → SW
- Popup (mode selector, mic toggle)
- Countdown + timer + preview

### Out (explicit)
- Region selection, webcam PiP (P1)
- Editor (trim, mute, crop) (P1)
- Annotations canvas (P1)
- Export MP4 / GIF (P1)
- Firefox (P1)
- STT/IA locale (P2)
- AudienceLenses (P2)
- Bibliothèque de sessions partagée (non engagé)

## Coaching Notes

- Le persona (ingé produit/QA/support) est vécu par le fondateur, pas théorique — ça transparaît dans la précision des réponses FAQ
- Le concept a été challengé sur trois axes : scope V0.1 vs vision, différenciation réelle, et soutenabilité
- Le trio de questions le plus dur : Q8 Customer FAQ (vision vs réalité), Q7 Internal FAQ (continuité), Q3 Internal FAQ (moteur d'adoption)
- La réponse à Q7 Internal FAQ a dû être retravaillée deux fois : le "fork MIT" seul ne suffit pas — la continuité doit couvrir usage, maintenance ET gouvernance
- L'absence de "why now" est la faiblesse la moins discutée du PRFAQ — à investiguer dans le PRD
- Format de sortie par défaut : markdown (ce fichier)

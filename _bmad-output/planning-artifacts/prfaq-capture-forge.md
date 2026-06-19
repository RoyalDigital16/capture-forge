---
title: "PRFAQ: Capture Forge"
status: "complete"
created: "2026-06-19"
updated: "2026-06-19"
stage: "5"
inputs:
  - "docs/product-brief.md"
  - "docs/prd.md"
  - "docs/architect.md"
  - "docs/ux-designer.md"
  - "docs/audience-lens-architecture.md"
  - "docs/sprint-stories-resilient-storage.md"
  - "_bmad-output/planning-artifacts/briefs/brief-capture-forge-2026-06-19/brief.md"
  - "_bmad-output/planning-artifacts/brainstorming-session-2026-06-19-104754.md"
---

# Capture Forge : une capture qui devient une preuve

## L'extension open-source pour transformer une capture écran en artefact réutilisable, sans compte, sans watermark, et sans cloud imposé

**Paris, France — 19 juin 2026** — Chaque fois qu'une équipe veut documenter un bug, un flux ou une configuration, elle doit encore assembler la preuve. Enregistrer, vérifier, annoter, couper, exporter, partager — puis souvent réexpliquer par écrit ce que la vidéo seule ne montre pas. La capture n'est jamais le livrable final ; c'est une matière première qui demande encore trop de transformations avant d'être utile.

Le marché des extensions d'enregistrement ne manque pas d'options. Loom a popularisé le partage rapide, mais le cloud reste central. OBS offre une puissance de capture inégalée, mais reste un outil de studio. Screenity a prouvé qu'un recorder local et open-source pouvait exister dans le navigateur, sans pour autant aller jusqu'à une architecture pensée pour évoluer.

Capture Forge ne cherche pas à être un meilleur Loom ni un OBS plus simple. Il part d'une autre hypothèse : une capture utile doit rester locale, réutilisable et sous contrôle de l'utilisateur. Son moteur Rust/WASM permet d'activer uniquement les capacités nécessaires à chaque usage, sans alourdir le cœur du produit.

Pas de compte. Pas de télémétrie. Pas de limite cachée. La confiance n'est pas un argument marketing ; c'est une contrainte d'architecture.

> "Le vrai coût d'un bug, ce n'est pas de le reproduire — c'est de le faire comprendre à quelqu'un qui ne l'a pas vu. Capture Forge transforme la preuve en transmission."
> — Herold, Fondateur, Capture Forge

### Comment ça fonctionne

L'utilisateur installe Capture Forge depuis le Chrome Web Store ou GitHub. Pas de compte, pas de formulaire. Un clic sur l'icône, il choisit écran entier ou onglet, avec ou sans micro, puis lance l'enregistrement avec un compte à rebours 3-2-1.

Pendant la capture, tout reste local et résilient. L'utilisateur peut mettre en pause, reprendre ou annuler sans perdre l'enregistrement déjà réalisé. Rien ne dépend d'un upload ni d'un cloud obligatoire.

Quand il arrête, il arrive sur un écran de prévisualisation. Il peut télécharger le WebM brut et repartir immédiatement avec un fichier exploitable.

Une capture. Un fichier. Zéro détour.

> "Je peux enregistrer une fois, récupérer un fichier propre, et l'envoyer tout de suite sans passer par un cloud ni refaire le travail ailleurs. Enfin un outil qui me laisse repartir avec quelque chose d'exploitable, pas juste une capture brute."
> — Alex, Ingénieur produit, éditeur SaaS

### Premiers pas

Capture Forge est disponible dès aujourd'hui sur le Chrome Web Store et sur GitHub (nightly, alpha, contributions). Extension Manifest V3, zéro compte, zéro télémétrie.

Installez l'extension, ouvrez le popup, choisissez votre mode de capture (écran entier ou onglet), activez le micro si nécessaire, et lancez l'enregistrement. L'export WebM natif est disponible immédiatement après l'arrêt.

---

## Customer FAQ

### Q : Votre vision est séduisante — "une capture, plusieurs publications". Mais en V0.1, vous livrez juste un recorder WebM, pas différent des autres. Est-ce que la vision ne masque pas le fait que le produit réel n'est pas encore différenciant ?

A : Cette question est juste. En surface, Capture Forge V0.1 ressemble à un recorder WebM. Mais la différence n'est pas dans le geste, elle est dans la structure. Chaque capture est écrite localement comme un artefact fiable, avec récupération en cas d'interruption et sans dépendance à un cloud.

Ce choix compte tout de suite pour l'utilisateur : il enregistre, il prévisualise, il exporte, et il repart avec un fichier qu'il contrôle entièrement. La vision "une capture, plusieurs usages" n'est pas un vernis ajouté après coup ; elle commence dès le premier chunk, parce que le produit traite la session comme quelque chose qu'on peut conserver, pas seulement consommer.

Autrement dit : V0.1 n'est pas encore la destination finale, mais c'est déjà le bon fondement.

### Q : Pourquoi vous plutôt que Screenity ? C'est gratuit, open-source, local, sans compte, sans watermark — exactement ce que vous décrivez.

A : Screenity est un excellent outil, et il a prouvé qu'un recorder local et privé pouvait exister dans le navigateur. La différence est dans l'ambition architecturale. Screenity est construit comme un monolithe React/JS — performant pour son périmètre, mais difficile à faire évoluer au-delà de la capture et des annotations. 

Capture Forge est bâti sur un moteur Rust/WASM modulaire. Concrètement, cela signifie : le code non utilisé n'est pas chargé (feature flags), la résilience est intégrée au protocole d'écriture (pas ajoutée après coup), et l'extension peut s'enrichir de nouvelles capacités (éditeur, IA, lenses) sans réécrire le cœur. Si Screenity répond à votre besoin aujourd'hui, continuez à l'utiliser. Si vous cherchez un outil qui peut évoluer avec vous sans changer de fondation, Capture Forge est conçu pour ça.

### Q : Je perds mes enregistrements si mon navigateur plante ?

A : C'est le problème que nous avons le plus travaillé. L'architecture de Capture Forge est conçue pour la résilience : chaque chunk est écrit avec un protocole deux phases (`.partial → .bin`), le service worker envoie un heartbeat via un offscreen document, et au redémarrage, une vérification triple (manifeste vs fichiers, taille, séquence) produit un rapport d'intégrité. Concrètement : si le service worker est tué, vous récupérez tout ce qui a été écrit jusqu'à la dernière seconde complète. La perte maximale est d'un chunk (~10s de vidéo), et vous êtes informé précisément de ce qui a été perdu.

### Q : J'ai besoin de MP4, pas de WebM. Jira ne lit pas le WebM, Slack le lit mal. Pourquoi devrais-je me contenter d'un format que mon équipe n'utilise pas ?

A : La V0.1 exporte en WebM (VP8 + Opus) — le format natif du MediaRecorder, parfaitement lisible par Chrome, Firefox, Edge, VLC, et la plupart des lecteurs modernes. L'export MP4 arrive en V0.5 (via FFmpeg WASM), avec l'éditeur vidéo. Si le MP4 est un blocage immédiat, Capture Forge n'est pas encore pour vous — mais le WebM est déjà un excellent format pour les revues techniques et les bugs reports.

### Q : Pourquoi ne pas utiliser Loom qui est gratuit et déjà installé par toute mon équipe ?

A : Loom est excellent pour la communication rapide, mais ce n'est pas le même produit. Loom impose un compte, une limite de 5 minutes en gratuit, un watermark, et stocke vos vidéos dans son cloud. Vous ne contrôlez pas vos données, vous ne pouvez pas les réutiliser hors de la plateforme, et la vidéo est un livrable final, pas un objet que vous pouvez retravailler ou dériver. Capture Forge garde tout en local, ne limite rien, et produit une session que vous pouvez transformer pour différents usages.

### Q : Combien de permissions l'extension demande ? Je dois passer par mon IT pour approuver ça.

A : Capture Forge demande l'accès au stockage local (OPFS), à la capture d'écran/onglet (getDisplayMedia / tabCapture), et au micro. C'est tout. Pas d'accès au réseau, pas de lecture de l'historique de navigation, pas de modification du contenu des pages. Les permissions sont déclarées dans le manifeste V3 et visibles avant l'installation. Le code source est en Rust/WASM, vérifiable par toute personne qui compile l'extension depuis les sources.

### Q : C'est open-source. Qui maintient ? Qui décide de la roadmap ?

A : Capture Forge est distribué sous licence MIT (cœur original). La roadmap est publique, guidée par les principes produits (local-first, pas de compte, pas d'échec silencieux). Les contributions sont les bienvenues via GitHub. À ce stade, le projet est porté par son fondateur et une communauté early-stage, sans entreprise derrière. C'est un risque à connaître — mais c'est aussi ce qui garantit l'absence de dérive payante ou de lock-in propriétaire.

### Q : Est-ce que Capture Forge gère le workflow d'équipe ? Je suis dans une équipe de 12, si je suis le seul à voir mes enregistrements, ça ne m'avance pas.

A : En V0.1, Capture Forge est un outil individuel — la session et l'export sont locaux. Il n'y a pas de bibliothèque partagée, pas de workspace d'équipe, pas de publication centralisée. L'utilisateur exporte son WebM et le partage comme il le fait aujourd'hui (pièce jointe, Slack, Jira, Drive). Le workflow d'équipe (bibliothèque de sessions, publications partagées, espaces collaboratifs) est une évolution naturelle pour les versions futures, mais ce n'est pas un engagement pris aujourd'hui.

---

## Internal FAQ

### Q : Est-ce qu'on construit un produit dont des équipes dépendront, sachant qu'il n'a aucune garantie de continuité au-delà de l'engagement actuel du fondateur ?

A : Oui, il existe un risque de ralentissement ou d'abandon — comme pour beaucoup de projets open-source portés par une vision individuelle. Mais ce risque est réduit par design, pas seulement par intention.

D'abord, les utilisateurs ne dépendent pas d'un cloud propriétaire pour récupérer leur travail : leurs enregistrements restent exportables, lisibles et portables. Ensuite, la licence MIT facilite la reprise si la communauté ou un nouveau mainteneur souhaite continuer. Enfin, l'architecture modulaire et la séparation claire des responsabilités rendent le projet compréhensible et maintenable sans repartir de zéro.

Ce n'est pas une garantie de continuité humaine. C'est une garantie que le produit ne se transforme pas en impasse technique ou en boîte noire si l'équipe initiale ralentit.

### Q : Comment passe-t-on d'un recorder WebM compétent à un outil de référence pour les équipes produit ? Quel est le vrai moteur d'adoption ?

A : Le vrai moteur d'adoption n'est pas la distribution seule ; c'est l'intégration dans un geste de travail répétitif. Capture Forge devient utile quand documenter un bug, montrer un flux ou partager une preuve cesse d'être une opération spéciale et devient un réflexe.

Ensuite, trois accélérateurs prennent le relais. D'abord, l'usage se propage naturellement dans les équipes techniques quand un utilisateur interne devient le point de référence. Ensuite, GitHub et l'open-source créent de la confiance et attirent les bons contributeurs. Enfin, la fiabilité du produit — moins de captures perdues, moins de reprises, moins de frictions — transforme l'outil en habitude.

La croissance n'est donc pas un sujet séparé du produit ; elle découle de la fréquence du problème résolu et de la simplicité de la solution.

### Q : Oxichrome v0.2 est le framework central, et c'est une dépendance jeune. Que se passe-t-il s'il stagne ou prend une direction incompatible ?

A : C'est un risque identifié et accepté. La stratégie est double : contribuer en amont quand possible (PR upstream, partage d'expérience), et forker si nécessaire. La dépendance à Oxichrome est concentrée dans les proc macros et le runtime d'extension — pas dans le code métier. Si le fork devient nécessaire, il est maintenable parce que le périmètre à couvrir est borné (initialisation, cycle de vie, routage de messages).

Ce risque est compensé par le gain architectural : tout le métier (recording, storage, state, UI) est en Rust natif avec web-sys direct, pas en JS interop fragile. Sans Oxichrome, chaque module devrait réimplémenter la même plomberie d'extension. La dépendance est un investissement, pas une dette cachée.

### Q : Quel est le risque technique principal ? Le protocole de résilience tient-il en conditions réelles ?

A : Le risque technique le plus concret est la fiabilité du pipeline d'écriture OPFS sous contrainte de mémoire et de temps réel. Le protocole deux phases (`.partial → .bin`) et la triple vérification sont conçus pour des défaillances franches (crash SW, perte réseau). Les scénarios plus subtils — saturation mémoire progressive, corruption partielle de fichier, contention d'accès à OPFS — sont moins testés et pourraient révéler des failles en conditions réelles.

La mitigation est un plan de tests progressif : d'abord des tests unitaires sur la machine à états, puis des tests d'intégration avec simulation de crash, puis une beta fermée avec des utilisateurs réels avant la release publique. Le protocole est conçu pour être *conservative* : en cas de doute, on marque le chunk comme suspect et on le signale dans le rapport d'intégrité, plutôt que de supposer qu'il est valide.

### Q : Comment finance-t-on la maintenance d'un projet open-source MIT sans revenus ?

A : La V0.1 ne nécessite pas de financement externe — le projet est porté par son fondateur, avec des coûts proches de zéro (hébergement GitHub, Chrome Web Store, nom de domaine). La question devient pertinente si le projet dépasse le stade de l'outil individuel et commence à être utilisé par des équipes.

À ce stade, les voies naturelles sont : les contributions open-source (code, documentation, tests), le sponsoring GitHub (individuel ou entreprise), et à plus long terme des services optionnels qui n'enferment pas le cœur du produit (hébergement de bibliothèques de sessions, publication de lenses, instance de partage d'équipe). Mais rien de tout ça n'est engagé aujourd'hui — la priorité est de livrer un outil qui justifie son existence par sa qualité.

### Q : La roadmap annonce V0.1 Q3 2026, Editor Q4, Firefox début 2027. Est-ce que ce calendrier tient si le projet avance à temps partiel ?

A : Le calendrier est tendu mais réaliste pour le périmètre actuel. La V0.1 Recorder Core est le sous-produit le mieux cadré (architecture définie, dépendances identifiées, protocole d'écriture spécifié). L'Editor (trim, mute, crop, player) et Firefox sont des efforts supplémentaires significatifs mais indépendants — ils ne bloquent pas la V0.1.

Si le rythme ralentit, l'ordre de priorité ne change pas : V0.1 d'abord, solide. Editor ensuite, si l'adoption le justifie. Firefox quand l'architecture est stabilisée et que la demande est confirmée. Le calendrier est une intention, pas un engagement.

### Q : Qu'est-ce qui nous protège d'être copiés par Screenity, Loom ou OBS ?

A : Rien ne nous protège d'être copiés sur des features individuelles. Screenity peut ajouter un protocole de résilience. Loom peut permettre l'export sans compte. OBS peut sortir une interface navigateur simplifiée.

Ce qui est difficile à copier, c'est la *combinaison* : une architecture Rust/WASM modulaire qui permet d'ajouter des capacités (éditeur, IA, lenses) sans réécrire le cœur, une philosophie locale et résiliente intégrée dès le premier chunk, et une vision produit qui traite chaque capture comme un actif réutilisable et non comme une vidéo à consommer. Copier l'interface est facile. Copier une architecture et une cohérence produit prend des années.

La meilleure protection contre la copie n'est pas juridique — c'est la vitesse d'exécution et la clarté de la vision.

### Q : Comment éviter de diluer l'énergie entre la V0.1 concrète et la vision V2.0+ ?

A : La réponse est dans l'architecture : chaque sous-produit (Recorder Core, Editor, AI) est indépendant et feature-gated. La V0.1 peut être livrée, utilisée et appréciée sans qu'aucune ligne de code des versions ultérieures n'existe. La vision n'est pas un prérequis à l'utilité présente — c'est une conséquence de la qualité de la fondation.

Concrètement, la règle est : on ne développe pas une feature en avance. Le chunk status lifecycle et la triple vérification sont en V0.1 parce qu'ils sont nécessaires à la résilience du recorder, pas parce qu'ils préparent les Audience Lenses. Si la V2.0+ n'arrive jamais, la V0.1 reste un excellent recorder local. La vision est une direction, pas un plan de charge.

---

## The Verdict

### Forgé dans l'acier

- **Thèse produit originale** : traiter une capture comme un actif réutilisable plutôt qu'une vidéo à consommer — c'est le seul angle que personne n'exploite dans le marché actuel. Le PRFAQ tient debout parce que la vision n'est pas un vernis : elle est encodée dans l'architecture dès la V0.1 (chunk structuré, protocole résilient, feature flags).
- **Persona solide** : l'ingénieur·e produit/QA/support technique est un utilisateur réel avec un problème réel et un rituel de travail à haute fréquence. La question Q3 (moteur d'adoption) a révélé que la croissance n'est pas un problème séparé — elle découle de la répétition du geste résolu.
- **Architecture résiliente comme différenciateur** : le protocole deux phases + triple vérification est un avantage concret, pas un argument marketing. Dans un marché où la perte de capture est un problème sous-estimé, c'est un vrai critère de choix pour des utilisateurs techniques.
- **Honest framing** : la réponse à Q8 (vision vs V0.1 réelle) est crédible parce qu'elle reconnaît le gap sans l'ignorer ni le sur-vendre. "V0.1 n'est pas encore la destination finale, mais c'est déjà le bon fondement" — c'est la phrase qui porte tout le PRFAQ.
- **Différenciation par la cohérence** : les trois niveaux du PRFAQ (Press Release, Customer FAQ, Internal FAQ) racontent la même histoire. Il n'y a pas de décalage entre la promesse externe et la réalité interne.

### Demande plus de chaleur

- **Le saut V0.1 → AudienceLens reste la plus grande tension non résolue.** La réponse Q8 tient pour un client averti, mais pour un utilisateur qui découvre le produit sans contexte, le risque de "c'est juste un recorder WebM" est réel. La communication et le positionnement doivent être chirurgicalement clairs pour qu'un early adopter voie le fondement, pas seulement le manque.
- **WebM vs MP4** : la décision est défendable techniquement, mais elle exclut une partie du marché (Jira, équipes non techniques, Windows natif). Le PRFAQ en parle honnêtement, mais ce sera un obstacle récurrent dans chaque conversation d'adoption tant que le MP4 n'est pas livré.
- **La dépendance Oxichrome** : le risque est gérable (périmètre borné, fork possible), mais il ajoute de la complexité au build pipeline et à la reproductibilité des builds. Un investissement tôt dans la CI (tests cross-machine, build reproducible) réduirait ce risque significativement.

### Fissures dans les fondations

- **Pas de stratégie de continuité au-delà du MIT.** L'Internal FAQ Q7 a révélé que "fork et licence" ne suffisent pas si le projet dépasse le stade de l'outil individuel. C'est acceptable pour une V0.1, mais dès que des équipes construisent des workflows autour de Capture Forge, l'absence de structure de gouvernance (même minimale : core maintainers, processus de décision, critères de compatibilité) deviendra un problème.
- **Le moteur d'adoption repose sur un "et après" pas encore construit.** Le geste répétitif (documenter un bug, partager une preuve) est réel, mais Capture Forge V0.1 livre un fichier WebM — le même format que tout autre recorder. La valeur de "session structurée réutilisable" n'est pas encore exploitable par l'utilisateur. Le risque est que l'early adopter technique teste, valide le fondement, mais reparte avec une expérience indifférenciée.
- **Aucune réponse à "pourquoi maintenant ?"** Le PRFAQ décrit bien le problème et la solution, mais ne donne pas de raison impérieuse pour laquelle *ce moment* est le bon. Le Manifest V3 ? La lassitude des clouds ? La maturité de Rust/WASM dans le navigateur ? Chacune de ces pistes est une amorce, mais aucune n'est développée comme un "why now" stratégique.

### Résumé

Capture Forge a une thèse produit forte, une architecture qui la soutient, et un PRFAQ honnête. Les fissures sont réelles mais pas mortelles pour la V0.1 — elles deviennent critiques si le projet passe à l'échelle sans y répondre.

La question ouverte qui reste : **"est-ce que la V0.1 est suffisamment utile par elle-même pour créer l'adoption qui justifiera les versions suivantes ?"** — c'est la seule question que ni le PRFAQ ni l'architecture ne peuvent prédire. Le marché répondra.

<!-- coaching-notes-stage-1 -->
<!--
Concept type: Open-source product (community-driven, MIT license)
Initial assumptions challenged:
  - L'utilisateur n'a pas commencé par "je veux faire une extension Chrome" mais par le problème humain (transformer un événement en preuve).
  - Le positionnement "pas de compte" a été rapidement dépassé par la vision "capture → source de vérité réutilisable".
  - Le vrai concurrent mental n'est pas OBS ou Loom, c'est la chaîne de friction que l'utilisateur subit au quotidien.
Why this direction over alternatives:
  - Rust/WASM n'est pas un choix esthétique : c'est une réponse aux contraintes MV3 (SW lifecycle, mémoire prévisible, feature gating).
  - Le protocole de résilience (chunk status, triple verification) est le fondement qui rend possible la confiance dans la session comme source réutilisable.
Key subagent findings:
  - Les docs existantes confirment et renforcent la vision : la résilience, les feature flags, les Audience Lenses sont des piliers déjà documentés.
  - Le marché est effectivement saturé de "recorders" mais pas de "moteurs de publication multi-public".
User context:
  - Herold est un fondateur technique avec une vision produit claire, capable de faire le saut du "quoi" au "pourquoi".
  - Son persona principal (ingénieur·e produit/QA/support) est vécu, pas théorique.
-->

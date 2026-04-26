# Open-Source-Lizenzen

Eine Open-Source-Lizenz ist ein rechtliches Dokument, das festlegt, was du mit der Software tun darfst und was nicht. Die Lizenz gilt auch, wenn du kein Geld für die Software bezahlt hast.

Ohne Lizenz gilt standardmäßig das Urheberrecht: du darfst nichts damit machen.

**Analogie:** Eine Lizenz ist wie ein Mietvertrag. Sie regelt, was erlaubt ist: ob du die Wohnung untervermieten, umbauen oder gewerblich nutzen darfst. 

---

## Zwei große Kategorien

Open-Source-Lizenzen lassen sich in zwei Grundtypen einteilen:

| Typ           | Merkmal                                                              | Beispiele            |
|---------------|----------------------------------------------------------------------|----------------------|
| **Permissiv** | Wenige Einschränkungen, auch proprietäre Nutzung erlaubt             | [MIT](../../wiki/glossar.md#mit-lizenz), [Apache 2.0](../../wiki/glossar.md#apache-20-apache-license-20), [BSD](../../wiki/glossar.md#bsd-berkeley-software-distribution) |
| **[Copyleft](../../wiki/glossar.md#copyleft)**  | Abgeleitete Werke müssen unter gleicher Lizenz veröffentlicht werden | [GPL](../../wiki/glossar.md#gpl-gnu-general-public-license), [LGPL](../../wiki/glossar.md#lgpl-gnu-lesser-general-public-license), [AGPL](../../wiki/glossar.md#agpl-gnu-affero-general-public-license)      |

---

## Die wichtigsten Lizenzen im Detail

### MIT-Lizenz

Die **beliebteste** und **einfachste** Open-Source-Lizenz. Extrem permissiv.

**Du darfst:**
- die Software frei nutzen, kopieren, verändern und verbreiten
- sie in proprietären Projekten und kommerziellen Produkten einsetzen
- den Quellcode deiner Änderungen behalten (nicht veröffentlichen)

**Du musst:**
- den ursprünglichen Copyright-Hinweis und die Lizenz beilegen

**Bekannte Projekte:** React, Vue.js, Rails, jQuery, .NET Core

```
MIT License
Copyright (c) 2024 Max Mustermann

Permission is hereby granted, free of charge, to any person obtaining a copy...
```

### Apache-2.0-Lizenz

Ähnlich permissiv wie MIT, aber mit explizitem **Patent-Grant**: Der Lizenzgeber gewährt dir auch eine Lizenz für alle Patente, die in der Software stecken. Damit kann z.B. verhindert werden, dass der Lizenzgeber den Code nachträglich patentiert und dann die Nutzer der Software wegen einem Patentbruch verklagt.

**Zusätzlich gegenüber MIT:**
- Änderungen müssen als solche gekennzeichnet werden
- Expliziter Patentschutz für Nutzer

**Bekannte Projekte:** Android, Kubernetes, Elasticsearch (alt), Apache HTTP Server

### GNU General Public License (GPL)

Die bekannteste **Copyleft**-Lizenz, entwickelt von Richard Stallman für das GNU-Projekt.

**Du darfst:**
- die Software frei nutzen, ändern und verbreiten

**Du musst:**
- Änderungen und abgeleitete Werke **ebenfalls unter der GPL** veröffentlichen
- Den Quellcode mitliefern oder zugänglich machen

**Bekannte Projekte:** Linux-Kernel, WordPress, Git, GCC

**Wichtig:** Wenn du GPL-Software in deinem Projekt nutzt und es weitergibst, muss dein gesamtes Projekt ebenfalls unter der GPL stehen. Das nennt man den **"Copyleft-Effekt"** oder umgangssprachlich "Viralität".

---

### GNU Lesser General Public License (LGPL)

Eine abgeschwächte Version der GPL. Gedacht für **Bibliotheken**, die auch in proprietären Projekten verwendbar sein sollen.

**Der Unterschied zur GPL:**
- Du kannst LGPL-Bibliotheken in proprietärer Software *verlinken/einbinden*,
  ohne dein Projekt unter LGPL stellen zu müssen
- Änderungen an der Bibliothek selbst müssen aber weiterhin unter LGPL stehen

**Bekannte Projekte:** GTK, Qt (teilweise), glibc

---

### GNU Affero General Public License (AGPL)

Die **strengste** der GPL-Varianten. Schließt eine wichtige Lücke der normalen GPL:

**Das Problem mit GPL und [SaaS](../../wiki/glossar.md#saas-software-as-a-service):**
Du könntest GPL-Software auf einem Server betreiben und anderen als Dienst anbieten, ohne den Quellcode zu veröffentlichen - denn du *verbreitest* die Software nicht, du nutzt sie nur intern.

**Die AGPL-Lösung:**
Wer AGPL-Software über ein Netzwerk anbietet (z.B. als Web-App), muss den Quellcode ebenfalls veröffentlichen.

**Bekannte Projekte:** [Nextcloud](../../wiki/glossar.md#nextcloud), Gitea, [Mastodon](../../wiki/glossar.md#mastodon), MongoDB (ab 2018)

**Aus diesem Grund** wechseln Unternehmen manchmal von MIT/Apache auf AGPL, wenn sie verhindern wollen, dass Cloud-Anbieter ihre Software ohne Beitrag nutzen.

---

### BSD-Lizenzen (2-Clause, 3-Clause)

Sehr ähnlich der MIT-Lizenz, ebenfalls permissiv. Historisch älter.

- **2-Clause BSD**: Nahezu identisch mit MIT
- **3-Clause BSD**: Zusätzlich: Der Name der Originalautorin darf nicht für Werbung verwendet werden

**Bekannte Projekte:** FreeBSD, OpenBSD, Django (bis 2008)

---

### Creative Commons (CC) - kein Code, aber Inhalte

Creative Commons ist keine Software-Lizenz, aber wichtig für Dokumentation, Bilder,
Texte und Daten.

| Lizenz   | Bedeutung                                     |
|----------|-----------------------------------------------|
| CC0      | Public Domain - keinerlei Einschränkungen     |
| CC BY    | Namensnennung erforderlich                    |
| CC BY-SA | Namensnennung + gleiche Lizenz (wie Copyleft) |
| CC BY-NC | Namensnennung + keine kommerzielle Nutzung    |
| CC BY-ND | Namensnennung + keine Änderungen              |

---

## Lizenz-Vergleich auf einen Blick

| Lizenz     | Typ                | Kommerzielle Nutzung | Änderungen veröffentlichen? | Patent-Schutz |
|------------|--------------------|----------------------|-----------------------------|---------------|
| MIT        | Permissiv          | Ja                   | Nein                        | Nein          |
| Apache 2.0 | Permissiv          | Ja                   | Nein                        | Ja            |
| BSD (2/3)  | Permissiv          | Ja                   | Nein                        | Nein          |
| LGPL       | Schwaches Copyleft | Ja (als Bibliothek)  | Nur Bibliothek              | Nein          |
| GPL v2/v3  | Copyleft           | Ja                   | Ja                          | GPL v3: Ja    |
| AGPL       | Starkes Copyleft   | Ja                   | Ja (auch bei SaaS)          | Ja            |

---

## Lizenzen kombinieren - Kompatibilität

Nicht alle Lizenzen lassen sich in einem Projekt kombinieren. Wichtige Faustregeln:

- **MIT + Apache 2.0** → problemlos kombinierbar
- **MIT + GPL** → das Ergebnis muss unter GPL stehen
- **GPL + AGPL** → im gleichen Projekt schwierig
- **GPL v2 + GPL v3** → nicht automatisch kompatibel

**Tipp:** Für eigene Projekte empfiehlt sich MIT (maximale Freiheit) oder AGPL (wenn du Cloud-Nutzung ohne Rückfluss verhindern willst).

Es gibt auch Projekte, die gleichzeitig unter MIT und Apache 2.0 stehen. Das hat den vorteil, dass nachträglich niemand, der dem Projekt beigetragen hat, seinen Code patentieren und dann später die Verwendung verbieten kann. Das patent kann zwar weiterhin angemeldet werden, aber es wird durch Apache 2.0 automatisch eine Erlaubnis zur Verwendung gegeben während MIT maximale Freiheit für alle Verwender gibt.

---

## Weiter geht es

-> [Geschäftsmodelle](04-geschaeftsmodelle.md)

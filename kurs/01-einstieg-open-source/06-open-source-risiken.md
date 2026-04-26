# Risiken bei der Nutzung von Open Source

Open Source bietet enorme Vorteile: Transparenz, Kontrolle, keine [Vendor-Lock-in](../../wiki/glossar.md#lock-in--vendor-lock-in)-Fallen. Aber wer Open-Source-Software einsetzt, sollte auch die Risiken kennen. Diese sind real, und sie werden oft unterschätzt.

---

## 1. Die Illusion der Sicherheit durch offenen Code

Open Source bedeutet, dass der [Quellcode](../../wiki/glossar.md#quellcode-source-code) öffentlich einsehbar ist. Das klingt nach einem großen Sicherheitsvorteil und das ist es grundsätzlich auch da die Hoffnung besteht, dass die Community Fehler schnell findet. In der Praxis stimmt das allerdings nur für populäre Projekte. Mehr als 99% der Projekte sind nicht populär.

Weniger bekannte Bibliotheken können jahrelang schwerwiegende Sicherheitslücken enthalten, ohne dass jemand hinschaut. Selbst bei großen Projekten gibt es Ausnahmen:

| Schwachstelle                     | Projekt    | Zeit bis zur Entdeckung |
|-----------------------------------|------------|-------------------------|
| Heartbleed ([CVE](../../wiki/glossar.md#cve-common-vulnerabilities-and-exposures)-2014-0160)        | OpenSSL    | ~2 Jahre im Einsatz     |
| Log4Shell (CVE-2021-44228)        | Log4j      | ~8 Jahre im Einsatz     |
| XZ Utils Backdoor (CVE-2024-3094) | xz/liblzma | ~2 Jahre eingeschleust  |

Der Code ist öffentlich, aber das bedeutet nicht, dass jemand ihn wirklich liest.

---

## 2. Supply-Chain-Risiken: Das Dependency-Problem

Moderne Software besteht selten aus einer einzigen Komponente. Ein typisches
Node.js-Projekt hat hunderte direkte und tausende transitive Abhängigkeiten.

[![XKCD 2347: Dependency](https://imgs.xkcd.com/comics/dependency.png)](https://xkcd.com/2347/)

*Quelle: [XKCD 2347 - Dependency](https://xkcd.com/2347/) (CC BY-NC 2.5)*

Dieses Bild zeigt die Realität: Der gesamte Stapel moderner digitaler Infrastruktur ruht auf einem winzigen Projekt, das seit Jahren von einer einzigen Person (irgendwo in Nebraska) danklos gepflegt wird.

### Wie Supply-Chain-Angriffe funktionieren

Ein Angreifer muss nicht mehr die gut gesicherte Software eines großen Unternehmens angreifen. Stattdessen visiert er eine kleine, vertrauenswürdige Abhängigkeit an:

```
Deine App
└── beliebte-bibliothek (1M Downloads/Woche)
    └── kleine-hilfsbibliothek (1 Maintainer, kein Budget)
        └── ← Hier setzt der Angriff an
```

**Reale Beispiele:**

| Vorfall                  | Jahr | Was passierte                                                                           |
|--------------------------|------|-----------------------------------------------------------------------------------------|
| **event-stream**         | 2018 | Maintainer übergibt Kontrolle an Fremden; Backdoor für Kryptowallet-Diebstahl eingebaut |
| **colors.js / faker.js** | 2022 | Maintainer sabotiert bewusst eigene Pakete aus Protest                                  |
| **XZ Utils**             | 2024 | Langfristig eingepflanzter Social-Engineering-Angriff; Backdoor in SSH-Daemon           |
| **left-pad**             | 2016 | 11-Zeilen-Paket wird entfernt; halbes Internet bricht zusammen                          |

Das XZ-Utils-Beispiel ist besonders beunruhigend: Der Angreifer baute über Monate Vertrauen zur Community auf, half mit Patches und übernahm schrittweise Kontrolle, um dann eine Backdoor einzuschleusen, die nur durch Zufall entdeckt wurde.

Die Liste der Beispiele ist das bei Weitem nicht repräsentativ. Die Anzahl der jährlich neu veröffentlichten Open Source Pakete/Paketversionen übersteigt die Anzahl der Google-Suchen und sie wächst exponentiell. Es ist selbst für Maschinen nicht mehr einfach den Überblick zu bekommen. Es werden jeden Tag neue Schwachstellen in offener wie geschlossener Software entdeckt und durch KI dauert es teilweise nur noch Stunden nach der Veröffentlichung bis jemand das erste mal versucht die Schwachstelle auszunutzen.

---

## 3. "I am not a supplier": die falsche Erwartungshaltung

Wenn Unternehmen Open-Source-Pakete einsetzen, entstehen oft implizite Erwartungen, dass der Code gepflegt wird, Sicherheitslücken behoben werden, API-Kompatibilität erhalten bleibt. Das Problem: Diese Erwartungen sind einseitig.

Thomas Depierre hat das in seinem Essay ["I am not a supplier"](https://www.softwaremaxims.com/blog/not-a-supplier) treffend formuliert:

> "We are not suppliers. We do not have a business relationship with all these organisations.
> We are volunteers, writing code and putting it online under these Licenses. [...] The fact
> you made your product depend on it is your responsibility. Not mine."

Das ist kein Vorwurf an Maintainer. Es ist eine Beschreibung der Realität. Jede Open-Source-Lizenz enthält deshalb explizit:

```
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES, OR OTHER
LIABILITY [...].
```

**Was das bedeutet:**
- Es gibt keine SLA. Kein Anspruch auf Updates oder Bugfixes.
- Der Maintainer kann das Projekt jederzeit aufgeben.
- Du kannst keine Anforderungen stellen. Du kannst nur mitarbeiten oder forken.

Der Begriff "Software Supply Chain" suggeriert eine Lieferkette mit Lieferanten und Abnehmern. Aber wer Open-Source-Pakete nutzt, ohne sich zu beteiligen oder sie zu finanzieren, ist (in Depierres Worten) eher ein *Waschbär, der im Mülleimer nach kostenlosem Code wühlt*.

Das ist keine Wertung. Es ist ein Hinweis darauf, wer die Verantwortung trägt.

---

## 4. Projektabbrüche und Abandonware

Open-Source-Projekte können jederzeit eingestellt werden. Ohne Ankündigung, ohne Migration, ohne Support.

**Was "eingestellt" bedeuten kann:**
- Maintainer hört auf (Burnout, Jobwechsel, Lebensumstände)
- Projekt wird archiviert (nur noch lesbar, kein Support mehr)
- Projekt wird von einem Unternehmen aufgekauft und dann eingestellt
- Keine kritischen Sicherheitsupdates mehr

Für Self-Hoster bedeutet das: Eine Software, die heute perfekt funktioniert, kann in zwei Jahren ein Sicherheitsrisiko sein weil sie keine Updates mehr bekommt.

**Gegenmaßnahme:** Achte auf Vitalitätszeichen eines Projekts:

| Signal                    | Gut           | Kritisch               |
|---------------------------|---------------|------------------------|
| Letzte Commit-Aktivität   | < 3 Monate    | > 1 Jahr               |
| Anzahl aktiver Maintainer | 3+            | 1                      |
| Issue-Response-Zeit       | < 2 Wochen    | Monate oder nie        |
| Releases                  | Regelmäßig    | Selten / nie           |
| Community-Größe           | Aktiv, wächst | Schrumpfend, stagniert |

Das sind natürlich nur grobe Indikatoren.
---

## 5. Lizenz-Risiken

Lizenzen können sich ändern und das kann direkte Auswirkungen auf deine Nutzung haben.

### Lizenzwechsel

Wie im [Kapitel über Geschäftsmodelle](04-geschaeftsmodelle.md) beschrieben, haben mehrere Projekte ihre Lizenz nachträglich verschärft:

- **HashiCorp** (Terraform) wechselte von MPL 2.0 zu BSL 1.1
- **Elasticsearch** wechselte von Apache 2.0 zu SSPL / Elastic License
- **Redis-Module** wechselten von Apache 2.0 zu RSALv2

Das bedeutet: Software, die du heute frei nutzen kannst, könnte morgen nicht mehr unter denselben Bedingungen verfügbar sein.

### Copyleft-Falle

Wer [GPL](../../wiki/glossar.md#gpl-gnu-general-public-license)-Software in eigene Produkte integriert und diese weitergibt, muss das Produkt ebenfalls unter GPL veröffentlichen. Das ist von der Philosophie her gewollt, kann aber für Unternehmen oder Projekte, die proprietären Code schützen wollen, eine Falle sein.

---

## 6. Das Finanzierungsproblem

Viele kritische Open-Source-Projekte sind chronisch unterfinanziert. Ein Projekt, das millionenfach genutzt wird und kritische Infrastruktur unterstützt, kann von einer einzigen Person in der Freizeit ohne Bezahlung gewartet werden.

Die Konsequenzen:
- **Burnout** bei Maintainern ist verbreitet und führt zu Projektabbrüchen
- **Fehlende Ressourcen** für Sicherheitsaudits und Tests
- **Abhängigkeit von wenigen Personen** schafft [Single Points of Failure](../../wiki/glossar.md#single-point-of-failure-spof)

---

## Was du tun kannst

Open-Source-Risiken sind real, aber sie sind handhabbar. Wichtig ist eine bewusste Haltung beim Einsatz von Open-Source-Software:

### Als Self-Hoster

- **Abhängigkeiten prüfen:** Wie aktiv ist das Projekt? Wie viele Maintainer?
- **Updates zeitnah einspielen:** Sicherheitslücken werden oft schnell bekannt gemacht und schnell ausgenutzt.
- **Auf bekannte Projekte setzen:** Popularität ist kein Garant für Sicherheit, aber ein Indikator für mehr Aufmerksamkeit.
- **Backup-Strategie:** Wenn ein Dienst ausfällt oder eingestellt wird, was dann?

### Als Teil der Community

- **Finanzielle Unterstützung:** Viele Projekte haben Open Collective, GitHub Sponsors oder Patreon. Selbst kleine Beträge helfen.
- **Beiträge:** Issues melden, Dokumentation verbessern, Bugs fixen.
- **Bewusstsein schaffen:** In Unternehmen auf die Abhängigkeit von unbezahlter Open-Source-Arbeit hinweisen.

> "You are a raccoon digging through dumpsters for free code. So I would advise you to
> put these rules in the same dumpster."
>
> - Thomas Depierre, [I am not a supplier](https://www.softwaremaxims.com/blog/not-a-supplier)

Das klingt hart. Aber es beschreibt eine Asymmetrie, die viele Unternehmen und Einzelpersonen gern ignorieren: Open Source funktioniert, weil Menschen Arbeit investieren. Diese Arbeit verdient Anerkennung und Unterstützung.

---

## Zusammenfassung

| Risiko                | Ursache                              | Gegenmaßnahme                                        |
|-----------------------|--------------------------------------|------------------------------------------------------|
| Sicherheitslücken     | Code nicht geprüft, veraltete Pakete | Updates, Monitoring, bewusste Auswahl                |
| [Supply-Chain-Angriffe](../../wiki/glossar.md#supply-chain-angriff) | Transitive Abhängigkeiten            | SBOM, Dependency-Audits                              |
| Projektabbruch        | Unterfinanzierung, Burnout           | Projektgesundheit beobachten, aktive Projekte wählen |
| Falsche Erwartungen   | "Supply Chain"-Denken                | Open-Source-Lizenz ernst nehmen                      |
| Lizenzwechsel         | Kommerzielle Interessen              | Lizenzen beobachten, Alternativprojekte kennen       |
| [Copyleft](../../wiki/glossar.md#copyleft)-Falle        | GPL-Integration ohne Prüfung         | Lizenzen vor Integration prüfen                      |

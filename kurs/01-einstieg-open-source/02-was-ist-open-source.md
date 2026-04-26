# Was ist Open Source?

Open Source bedeutet, dass der **[Quellcode](../../wiki/glossar.md#quellcode-source-code) einer Software öffentlich einsehbar** ist. Jeder kann lesen, wie das Programm funktioniert. Doch Open Source ist mehr als nur "Code veröffentlichen". Es ist eine Philosophie, eine Bewegung und ein Entwicklungsmodell.

---

## Quellcode: was ist das überhaupt?

Jede Software wird von Menschen in einer Programmiersprache geschrieben. Dieser Text, der sogenannte **Quellcode**, ist für Menschen lesbar. Bevor Software ausgeführt werden kann, wird er meist in Maschinencode übersetzt (kompiliert).

```
Quellcode (lesbar)          Maschinencode (nicht lesbar)
─────────────────           ───────────────────────────
if x > 10:          →       01001101 10110001 ...
    print("groß")
```

Bei **[proprietärer Software](../../wiki/glossar.md#proprietäre-software)** (z.B. Microsoft Office, Adobe Photoshop) erhältst du nur den fertigen Maschinencode. Du kannst nicht reinschauen, wie das Programm wirklich funktioniert.

Bei **Open-Source-Software** bekommst du beides: Das fertige Programm *und* den vollständigen Quellcode.

---

## Die Open-Source-Definition

Nicht jede "freie Software" ist automatisch Open Source. Die gemeinnützige Organisation
**[Open Source Initiative (OSI)](../../wiki/glossar.md#osi-open-source-initiative)** hat 10 Kriterien definiert, die eine Software erfüllen muss:

- Freie Weitergabe: du darfst die Software verschenken oder verkaufen
- Quellcode muss enthalten oder öffentlich zugänglich sein
- Abgeleitete Werke müssen erlaubt sein
- Keine Diskriminierung von Personen, Gruppen oder Einsatzgebieten
- ... und weitere technische Kriterien

**Kurz gesagt:** Open-Source-Software darf jeder nutzen, studieren, verändern und weitergeben.

---

## Geschichte: Von Unix zu Linux zu heute

### Die Anfänge (1960er-1970er)

In der Frühzeit der Computerei war Software-Sharing selbstverständlich. Universitäten und Forschungslabore tauschten Code frei aus. Die Idee, Software zu "besitzen", existierte kaum.

### Die Gegenbewegung: GNU und die Free Software Foundation (1983)

Als Software zunehmend kommerziell und proprietär wurde, reagierte **Richard Stallman** 1983 mit dem **[GNU](../../wiki/glossar.md#gnu--gnulinux)-Projekt**: einem Versuch, ein vollständig freies Betriebssystem zu schaffen. 1985 gründete er die **[Free Software Foundation (FSF)](../../wiki/glossar.md#fsf-free-software-foundation)**.

Stallmans Philosophie: Software-Freiheit ist eine ethische Frage. Er unterscheidet vier fundamentale Freiheiten:

| Nr. | Freiheit                                  |
|-----|-------------------------------------------|
| 0   | Die Software für jeden Zweck ausführen    |
| 1   | Die Funktionsweise studieren und anpassen |
| 2   | Kopien weitergeben                        |
| 3   | Veränderte Versionen veröffentlichen      |

### Linux und der Durchbruch (1991)

1991 veröffentlichte Linus Torvalds den **[Linux-Kernel](../../wiki/glossar.md#kernel)**, zunächst als persönliches
Projekt. In Kombination mit den GNU-Werkzeugen entstand ein vollständiges,
freies Betriebssystem: **GNU/Linux**.

### Der Begriff "Open Source" (1998)

Der Begriff **"Free Software"** (freie Software) war problematisch: Im Englischen bedeutet "free" sowohl "frei" als auch "kostenlos". Viele Unternehmen schreckte das ab.

1998 prägten **Eric S. Raymond** und **Bruce Perens** den Begriff **"Open Source"** als neutralere, businessfreundlichere Alternative. Die Open Source Initiative (OSI) wurde gegründet.

### Heute

Open Source ist die Grundlage des modernen Internets:

- **Linux** läuft auf ~96% aller Server
- **Android** (basiert auf Linux) auf ~72% aller Smartphones
- **Firefox**, **Chromium**, **VS Code**, **Git** alles Open Source
- Cloudinfrastruktur bei Amazon, Google und Microsoft basiert auf Linux
- [Kubernetes](../../wiki/glossar.md#kubernetes), [Docker](../../wiki/glossar.md#docker), [PostgreSQL](../../wiki/glossar.md#postgresql), [nginx](../../wiki/glossar.md#nginx-engine-x) der Self-Hosting-Stack ist Open Source

---

## Free Software vs. Open Source

Diese beiden Begriffe werden oft verwechselt, haben aber unterschiedliche Schwerpunkte:

|                | Free Software (FSF)  | Open Source (OSI)                      |
|----------------|----------------------|----------------------------------------|
| Fokus          | Ethik und Freiheit   | Praktischer Nutzen, Entwicklungsmodell |
| Vertreter      | Richard Stallman     | Eric Raymond, Bruce Perens             |
| Slogan         | "Free as in freedom" | "Open development model"               |
| Überschneidung | Beide verwenden oft die gleichen Lizenzen                     |

In der Praxis überschneiden sich beide Welten stark. Die meisten Open-Source-Projekte
sind auch im Sinne der Free Software Foundation "frei" und umgekehrt.

---

## Weiter geht es

Jetzt weißt du, was Open Source ist und woher es kommt. Als nächstes schauen wir uns an, wie Lizenzen regeln, was du mit Open-Source-Software tun darfst:

→ [Lizenzen](03-lizenzen.md)

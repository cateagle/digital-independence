# Digitale Unabhängigkeit

Warum streben wir eigentlich **digitale Unabhängigkeit** an? Jeder hat ein Gefühl, was dieser Begriff bedeutet, aber dieser Begriff ist komplizierter, als er auf den ersten Blick wirkt. Er bedeutet vieles für viele verschiedene Menschen, und er hat Grenzen, die man kennen sollte.

---

## Was bedeutet "digitale Souveränität" und für wen?

Der Begriff **"digitale Souveränität"** ist in aller Munde. In Bundesministerien,EU-Kommissionen, Tech-Konferenzen. Das Problem: Politiker, Unternehmen und Zivilgesellschaft meinen damit oft völlig verschiedene, manchmal widersprüchliche Dinge.

Bevor man also "digitale Unabhängigkeit" fordert, muss man fragen: **Für wen? Von wem? Über was?**

**Was soll souverän werden? (Objekte)**

| Ebene    | Beispiele                                            |
|----------|------------------------------------------------------|
| Physisch | Chips, Hardware, Glasfaser, Rechenzentren            |
| Code     | Betriebssysteme, Anwendungen, KI-Modelle, Protokolle |
| Daten    | Datenflüsse, Datenschutz, Standards, Verschlüsselung |

**Wer soll souverän werden? (Akteure)**

| Ebene              | Beispiele                               |
|--------------------|-----------------------------------------|
| Individuen         | Du mit deinen Daten und deiner Software |
| Zivilgesellschaft  | NGOs, Universitäten, Gemeinden          |
| Staat / Verwaltung | Behörden, Schulen, Krankenhäuser        |
| Region / Nation    | Deutschland, die EU                     |

Diese Unterscheidung ist wichtig, denn was für den Staat Souveränität bedeutet, ist nicht dasselbe wie für dich als Einzelperson. Maßnahmen, die staatliche Kontrolle stärken, können individuelle Freiheit gleichzeitig einschränken.

---

## Die individuelle Dimension: deine Daten, dein Leben

Fangen wir dort an, wo dieser Kurs ansetzt: beim Individuum.

Stell dir vor, dein gesamtes digitales Leben läuft über einen einzigen Anbieter:

```
Alle deine Fotos     → Google Photos
Alle deine E-Mails   → Gmail
Alle deine Dokumente → Google Drive
Dein Kalender        → Google Calendar
Deine Kontakte       → Google Contacts
Dein Passwort-Safe   → LastPass / 1Password (Cloud)
```

Was passiert, wenn:
- Dein Konto wegen eines falschen Algorithmus-Urteils gesperrt wird?
- Der Anbieter den Dienst einstellt (Google hat das dutzende Male getan)?
- Deine Daten durch einen Hack abfließen (LastPass 2022)?
- Das Unternehmen sein Preismodell ändert (Heroku, Twitter/X-API)?
- Behörden aus einem anderen Land Zugriff auf deine Daten verlangen?

Der letzte Punkt ist kein theoretisches Szenario: Der US **Cloud Act** verpflichtet amerikanische Unternehmen, US-Behörden Zugriff auf ihre gespeicherten Daten zu gewähren, selbst wenn die Server in Deutschland oder der EU stehen. Wer seine E-Mails bei Google oder seine Dateien bei Microsoft lagert, unterliegt damit faktisch US-amerikanischem Recht.

### Kettenreaktionen

Die obige Liste zeigt Einzeldienste. Was sie verschweigt: Diese Dienste sind miteinander verknüpft und zwar so eng, dass der Ausfall eines einzigen den Zusammenbruch vieler
weiterer auslösen kann.

Das deutlichste Beispiel ist die **E-Mail-Adresse**:

```
E-Mail-Konto wird gesperrt oder gelöscht
│
├── Alle verbundenen Accounts sind abgeschnitten
│   (du kannst dich nirgends mehr einloggen, wo du "mit Google/E-Mail anmelden" genutzt hast)
│
├── Passwort-Rücksetzungen funktionieren nicht mehr
│   (alle "Passwort vergessen"-Links landen im gesperrten Postfach)
│
├── Wiederherstellungsadressen anderer E-Mail-Konten sind wertlos
│   (du verlierst auch Zugang zu deinen Backup-Postfächern)
│
├── Die gesamte Kommunikationshistorie ist weg
│   (Jahre von E-Mails, Verträge, Quittungen, Kontakte)
│
└── Kontakte verlieren dich
    (niemand kann dich mehr erreichen und weiß nicht warum)
```

Das ist keine Übertreibung. Google sperrt jährlich tausende Konten automatisch wegen Algorithmus-Fehlern, ungewöhnlichem Login-Verhalten oder Verstößen gegen Nutzungsbedingungen, ohne Vorwarnung und oft ohne Einspruchsmöglichkeit.
Wer kein Backup hat und keine Alternative, steht von einem Moment auf den nächsten ohne digitale Identität da.

E-Mail ist nur die bekannteste Variante dieses Musters. Das gleiche gilt für:

| Ausgefallener Dienst | Was mitbricht |
|---|---|
| **Cloud-Speicher** (Google Drive, Dropbox) | Alle geteilten Dokumente, Backups, Arbeitsgrundlagen |
| **Passwort-Manager** (LastPass-Hack 2022) | Zugang zu allen gespeicherten Accounts auf einen Schlag |
| **Social-Media-Account** | Persönliches Netzwerk, berufliche Kontakte, Follower-Beziehungen |
| **Smartphone-Konto** (Apple ID, Google Account) | Alle Apps, Käufe, 2FA-Codes, Fotos, Gerätezugriff |
| **Domain / Hosting** (für Organisationen) | Website, alle @domain.de-Mailadressen, öffentliche Erreichbarkeit |

Das Muster ist immer gleich: **Zentralisierung erzeugt [Single Points of Failure](../../wiki/glossar.md#single-point-of-failure-spof).**
Je mehr Dienste auf einem einzigen Konto oder Anbieter aufbauen, desto größer der
Schaden, wenn dieser eine Punkt wegbricht.

Für Organisationen (Vereine, kleine Unternehmen, NGOs) ist das existenzbedrohend:
Eine gesperrte Domain oder ein gehacktes Admin-Konto kann die gesamte digitale Infrastruktur über Nacht lahmlegen.

### Das Geschäftsmodell der Aufmerksamkeitsökonomie

Kostenlose Dienste sind selten wirklich kostenlos. Wenn du nicht für das Produkt zahlst,
bist du oft das Produkt:

- **Werbe-Tracking:** Gewohnheiten, Interessen, Aufenthaltsorte werden gesammelt und verkauft
- **Verhaltensmanipulation:** Algorithmen optimieren auf Verweildauer, nicht auf dein Wohlergehen
- **Datenweitergabe:** An Werbetreibende, Datenhändler, manchmal Behörden

Das ist kein Einzelfall oder Fehlverhalten einzelner Unternehmen. Es ist das Geschäftsmodell. Wer App-Rabatte gegen Nutzerdaten tauscht, tut das nicht aus Versehen.

---

## Die politische Dimension: Wer kontrolliert die Infrastruktur?

Digitale Unabhängigkeit ist nicht nur eine persönliche Frage. Sie ist eine politische.

### Geopolitische Abhängigkeiten

Europa verbraucht 20 Prozent der Mikrochips der Welt, produziert aber nur 7 Prozent selbst. 70 Prozent der Cloud-Leistungen beziehen europäische Unternehmen von US-Firmen. Bei komplexen Halbleitern sind 90 Prozent der Produktion in Taiwan ansässig.

Diese Abhängigkeiten sind solange unsichtbar, wie die Beziehungen stabil sind. Sie werden real, wenn:

- Satellitenbetreiber öffentlich damit drohen, kritische Kommunikationsinfrastruktur
  abzuschalten (wie 2022 im Fall Starlink/Ukraine geschehen)
- Handelsstreitigkeiten dazu genutzt werden, Druck auf die Regulierung digitaler Märkte
  auszuüben
- Visa-Sanktionen gegen Vertreter europäischer Zivilgesellschaft verhängt werden, weil sie an der Durchsetzung europäischer Digitalgesetze beteiligt waren. Das passierte z.B. 2024 gegen die Geschäftsführerinnen von HateAid und den Ex-EU-Kommissar Thierry Breton

Wer die Infrastruktur kontrolliert, kontrolliert auch, wer die Regeln macht.

### Privatisierung staatlicher Kernfunktionen

Ein weiteres Muster: Staaten lagern kritische Funktionen an private Technologieunternehmen aus und verlieren damit die Kontrolle über sie.

Ein Beispiel ist der 10-Milliarden-Dollar-Vertrag des US-Pentagons mit Palantir, einem Unternehmen, das Datenanalyse für Militär und Geheimdienste betreibt. Das ist keine neutrale IT-Dienstleistung: Es bedeutet, dass ein privates Unternehmen Zugang zu militärischen Entscheidungsprozessen erhält und diese mitgestaltet.Ähnliches gilt für die Nutzung kommerzieller Cloud-Dienste für Behörden- und Gesundheitsdaten.

Die Abhängigkeit entsteht nicht durch einen einzelnen schlechten Vertrag, sondern schleichend bis ein Wechsel technisch und finanziell kaum noch möglich ist.

---

## Antworten und ihre Grenzen

### Self-Hosting: individuelle Antwort mit klaren Grenzen

**[Self-Hosting](../../wiki/glossar.md#self-hosting)**: Das Betreiben eigener Dienste auf eigener oder selbst gewählter Hardware ist die direkteste Antwort auf individuelle Abhängigkeiten:

| Statt         | Selbst hosten           |
|---------------|-------------------------|
| Google Drive  | Nextcloud               |
| Gmail         | Mailcow                 |
| Google Photos | Immich / Photoprism     |
| LastPass      | Vaultwarden (Bitwarden) |
| Slack         | Mattermost / Matrix     |
| Notion        | Joplin / Outline        |

Self-Hosting gibt dir Kontrolle über deine Daten, schützt vor Kontosperrungen und Dienstabschaltungen und du lernst dabei, wie moderne Infrastruktur wirklich funktioniert.

**Aber:**
- Self-Hosting löst geopolitische Abhängigkeiten nicht. Wenn dein Server auf AWS-Hardware in einem US-Rechenzentrum läuft, greift der Cloud Act trotzdem.
- Individuelle Unabhängigkeit ändert nichts an der Marktmacht von Google oder Microsoft.
- Du trägst die gesamte Verantwortung für die betriebene Software und musst dich um den Betrieb kümmern. Die goßen Cloudunternehmen können viele Dinge automatisieren, die du selbst von Hand machen musst.
- Du kannst vermutlich 2-3 Dinge problemlos nebenbei selbst hosten. Was ist aber, wenn du wirklich alles selber machen würdest? Du hättests vermutlich über 100 einzelne Server, die du alle aktuell halten musst. Das ist ein Vollzeitjob. Daher ist es schon sinnvoll, gewisse Dinge von anderen hosten zu lassen. Die Frage ist eher **Was?** und **Von wem?**.

Self-Hosting ist ein wichtiger, sinnvoller Schritt, aber er ist nicht das Ende der Antwort.

### Dezentralisierung: das Fediverse-Modell

Eine andere Antwort kommt aus der Netzarchitektur: **[Dezentralisierung](../../wiki/glossar.md#dezentralisierung)**.

Das **[Fediverse](../../wiki/glossar.md#fediverse)** ist ein Verbund von Servern, die über offene Protokolle miteinander kommunizieren. Es zeigt, wie Infrastruktur ohne zentralen Kontrollpunkt funktionieren kann. Mastodon, PeerTube, Matrix/Element sind Fediverse-Dienste. Wer eine eigene Instanz betreibt, ist weder von einem Unternehmen noch von einem einzelnen Betreiber abhängig und kann trotzdem mit allen anderen kommunizieren.

Das dezentrale Modell hat einen weiteren Vorteil: Es schützt nicht nur vor Unternehmens-, sondern auch vor staatlichem Machtmissbrauch. Wer keine zentrale Plattform abschalten kann, kann auch keine zentrale Zensur durchsetzen.

### Open Source und Public Money, Public Code

Auf der Ebene von Software gibt es eine klare Grundregel: Software, die mit öffentlichen Mitteln finanziert wird, sollte auch öffentlich zugänglich sein, als Open Source.

Das ist der Kern der **"[Public Money, Public Code](../../wiki/glossar.md#public-money-public-code)"**-Forderung, die das Europäische Parlament 2025 in seinen Empfehlungen zur digitalen Souveränität ausdrücklich unterstützt hat.

Konkret arbeitet das Zentrum für Digitale Souveränität der Öffentlichen Verwaltung (**[ZenDiS](../../wiki/glossar.md#zendis-zentrum-für-digitale-souveränität-der-öffentlichen-verwaltung)**) an **openDesk**, einem Open-Source-Arbeitsplatz für Behörden, der proprietäre Office-Software ersetzen soll. Frankreich und die Niederlande kooperieren bereits, weitere Länder haben Interesse bekundet.

Offene Software allein reicht aber nicht. Es braucht auch die Kompetenz, sie eigenständig zu beurteilen, einzusetzen und weiterzuentwickeln. Und bei permissiven Lizenzen (MIT, Apache) besteht das Risiko, dass die Vorarbeit der öffentlichen Hand später von Unternehmen privatisiert wird. Das ist ein Argument dafür, bei öffentlich finanzierter Software [Copyleft](../../wiki/glossar.md#copyleft)-Lizenzen ([GPL](../../wiki/glossar.md#gpl-gnu-general-public-license), [AGPL](../../wiki/glossar.md#agpl-gnu-affero-general-public-license)) zu bevorzugen, die eine Privatisierung ausschließen.

---

## Die Grenzen: Was "digitale Souveränität" nicht löst

### Totale Kontrolle als Illusion

Es gibt eine verführerische Vorstellung von digitaler Unabhängigkeit: alles selbst betreiben, alle Abhängigkeiten eliminieren, nichts dem Zufall überlassen. Das klingt nach Sicherheit. In der Praxis endet es in Überforderung.

Wer jeden Dienst selbst betreibt, trägt auch die gesamte Verantwortung: Updates, Backups, Sicherheitslücken, Ausfälle. Der Aufwand wächst schneller als die gewonnene Unabhängigkeit. Die sinnvolle Frage ist nicht "wie werde ich von allem unabhängig", sondern: **"Von welchen Abhängigkeiten ist Unabhängigkeit wirklich wichtig und welche nehme ich bewusst in Kauf?"**

### Staatliche Kontrolle ist keine Lösung

"Digitale Souveränität" als staatliches Projekt kann in die entgegengesetzte Richtung kippen: Statt individuelle Freiheit zu stärken, schränkt staatliche Kontrolle über digitale Infrastruktur sie ein.

China hat das konsequent durchgezogen: ein nationales Intranet, abgeschottet vom globalen Netz, unter strikter staatlicher Kontrolle. Das nennt sich dort "Souveränes Internet". Es ist das Gegenteil von individueller digitaler Freiheit und zeigt, dass "Souveränität" ein Begriff ist, den autoritäre wie demokratische Akteure gleichermaßen für sich beanspruchen.

### Techno-Nationalismus löst keine Machtkonzentration

Die **EuroStack-Initiative** schlägt 300 Milliarden Euro Investitionen vor, um Europa in zentralen Technologiebereichen unabhängig von den USA zu machen von Chips über Cloud bis zu KI-Modellen.

Das Ziel ist verständlich. Die Gefahr: Wenn dabei nur amerikanische Gatekeeper durch europäische ersetzt werden, ist nichts gewonnen. Machtkonzentration und fehlende demokratische Kontrolle sind das Problem, nicht die Nationalität der beteiligten Unternehmen. Transparenz, Interoperabilität und offene Standards müssen von Anfang an mitgedacht werden, sonst reproduziert Europa dieselben Strukturen, von denen es sich gerade befreien will.

---

## Was bleibt: ein Spektrum bewusster Entscheidungen

Digitale Unabhängigkeit ist kein Zustand, den man erreicht. Es ist ein Prozess bewusster Entscheidungen auf individueller, gesellschaftlicher und politischer Ebene.

Jeder Schritt zählt:

| Ebene             | Beispiele                                                                           |
|-------------------|-------------------------------------------------------------------------------------|
| Individuum        | Open-Source-Software nutzen, eigene Dienste hosten, [Datensparsamkeit](../../wiki/glossar.md#datensparsamkeit) üben           |
| Community         | Dezentrale Plattformen (Fediverse) mitbetreiben und nutzen                          |
| Zivilgesellschaft | NGOs stärken, die für digitale Rechte kämpfen (GFF, [EFF](../../wiki/glossar.md#eff-electronic-frontier-foundation), HateAid)                   |
| Institutionen     | Öffentliche Verwaltungen auf Open Source umstellen, ZenDiS unterstützen             |
| Politik           | Public Money, Public Code durchsetzen, Cloud Act mit eigener Infrastruktur begegnen |

Das Ziel ist nicht vollständige Autarkie, sondern **bewusste Abhängigkeit**: Es geht darum zu wissen, von wem man abhängt, warum, und welche Risiken man damit eingeht.

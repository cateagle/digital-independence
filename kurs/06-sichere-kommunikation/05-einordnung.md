# Einordnung & Entscheidungshilfe: Welches Tool für welche Situation?

Wir haben jetzt Bedrohungsmodelle, E2EE, Matrix und Jitsi kennengelernt. Dieser Abschnitt fasst zusammen, wie man das in der Praxis anwendet.

## Die zentrale Frage ist nicht "sicher oder unsicher"

Sicherheit ist kein binärer Zustand. Die richtige Frage ist:

> **"Sicher genug für wen, gegen wen, in welchem Kontext?"**

Ein Geburtstagsplan muss nicht gegen Behörden geschützt werden. Journalisten, die mit Whistleblowern kommunizieren, haben andere Anforderungen als eine Schul-AG. Das ist keine Entschuldigung, schlecht abgesicherte Tools zu nutzen. Die Entscheidung ist immer ein Tradeoff.

## Wann reicht TLS?

TLS reicht, wenn:
- Du dem Betreiber des Dienstes vertraust (oder es dir egal ist)
- Der Inhalt der Kommunikation kein hohes Schutzinteresse hat
- Praktikabilität und Reichweite wichtiger sind als maximale Kontrolle

Beispiele: Nextcloud (eigener Server), Zoom für interne Firmenmeetings ohne sensible Inhalte, die meisten alltäglichen E-Mails.

## Wann brauche ich E2EE?

E2EE ist sinnvoll, wenn:
- Du dem Betreiber nicht vertrauen willst oder kannst
- Inhalte sensitiv sind (Gesundheit, politisch, beruflich vertraulich)
- Du eine langfristige Unabhängigkeit von einzelnen Plattformen willst

Beispiele: Kommunikation über politische Organisation, Anwalts- oder Arztkorrespondenz, persönliche Kommunikation über Drittanbieter-Server.

## Wann hilft auch E2EE nicht mehr?

E2EE hilft nicht, wenn:
- Das Endgerät kompromittiert ist
- Metadaten (wer mit wem, wann) ausreichend Information sind
- Der Gesprächspartner nicht vertrauenswürdig ist (er kann die Nachricht nach dem Entschlüsseln weitergeben)
- Du den Wiederherstellungsschlüssel verlierst

E2EE ist kein Allheilmittel und es gibt Anwendungsfälle für die es einfach nicht geeignet ist.

## Praktische Entscheidungshilfe

```
Ich will kommunizieren. Was nutze ich?
│
├── Mit wem? Ist die Person schon auf einer Plattform?
│   └── Ja -> Welche Plattform ist das? Ist sie akzeptabel?
│
├── Wie sensitiv ist der Inhalt?
│   ├── Niedrig -> TLS-verschlüsselter Dienst reicht (z.B. Signal, WhatsApp, Nextcloud Talk)
│   └── Hoch -> E2EE, verifizierten Kanal bevorzugen
│
├── Wem vertraue ich den Server-Betrieb an?
│   ├── Großem Konzern -> Risiko bewusst eingehen oder wechseln
│   ├── Eigenem Server -> Vertrauen liegt bei mir (und meinem VPS-Anbieter)
│   └── Föderiertem Netzwerk (Matrix) -> Vertrauen ist verteilt
│
└── Wie wichtig ist Erreichbarkeit?
    ├── Alle müssen erreichbar sein -> WhatsApp / Signal (Reichweite schlägt alles)
    └── Gezielter Kreis -> Matrix, Signal-Gruppe, eigene Instanz
```

## Werkzeugkasten: Was wir jetzt kennen

| Tool             | Typ          | E2EE            | Föderiert | Self-Hosting |
|------------------|--------------|-----------------|-----------|--------------|
| Matrix / Element | Chat         | ja (opt-in)     | ja        | ja           |
| Jitsi Meet       | Video        | experimentell   | nein      | ja           |
| Signal           | Chat + Video | ja              | nein      | nein         |
| WhatsApp         | Chat + Video | ja (Inhalt)     | nein      | nein         |
| Zoom             | Video        | nein (Standard) | nein      | nein         |
| Nextcloud Talk   | Chat + Video | nein (Standard) | nein      | ja           |

**Anmerkung zu WhatsApp:** E2EE ist technisch vorhanden (Signal-Protokoll). Aber Metadaten (wer mit wem wann) gehen an Meta. Backups in Google Drive / iCloud sind oft unverschlüsselt. Die Schlüssel liegen auf Geräten, aber das Ökosystem bleibt vollständig unter Metas Kontrolle.

## Was Self-Hosting konkret bringt

Wer Matrix oder Jitsi selbst hostet, gewinnt:
- Volle Kontrolle über die Daten (keine Drittanbieter-Logs)
- Keine Abhängigkeit von Geschäftsentscheidungen anderer
- Möglichkeit, Zugriffsregeln selbst zu definieren

Was es nicht bringt:
- Schutz vor einem kompromittierten Endgerät
- Automatisch bessere Verschlüsselung
- Schutz vor Metadatenanalyse, wenn der Server Teil eines größeren Netzes ist

## Zusammenfassung der Session

1. **Bedrohungsmodell zuerst**: Vor wem schütze ich mich? Netzwerk, Betreiber, Gerät. Das sind drei unterschiedliche Probleme.
2. **TLS schützt den Weg, E2EE schützt vor dem Betreiber**: Das ist ein fundamentaler Unterschied.
3. **Matrix** löst das Föderationsproblem: Kein Single Point of Control, dezentral, E2EE möglich.
4. **Jitsi** ist pragmatisch für Video, aber E2EE ist dort strukturell begrenzt.
5. **Kein Tool ist perfekt**: Jede Wahl ist ein Kompromiss zwischen Sicherheit, Komfort und Reichweite.

Die Fähigkeit, diese Kompromisse bewusst einzugehen, ist das eigentliche Ziel.

# Matrix & Element: Dezentraler, verschlüsselter Chat

Matrix ist das Kommunikationsprotokoll, das wir in diesem Kurs für den Kurs-Chat verwenden. Dieser Abschnitt erklärt, warum Matrix so aufgebaut ist wie es ist und was das in der Praxis bedeutet.

## Matrix ist ein Protokoll, kein Produkt

Der wichtigste Unterschied zu WhatsApp, Slack oder Telegram: **Matrix ist ein offenes Protokoll**, kein Dienst eines einzelnen Unternehmens.

Das Verhältnis zwischen Matrix und Element entspricht dem zwischen E-Mail und Thunderbird:
- **Matrix** ist das Protokoll (die Sprache, in der Server miteinander reden)
- **Element** ist ein Client (eine App, die das Protokoll spricht)
- Es gibt viele andere Clients: FluffyChat, Cinny, Nheko, ...

Welchen Client du nutzt, ist Geschmackssache. Das Ökosystem gehört keinem einzelnen Anbieter.

## Warum föderiert?

Matrix ist **föderiert**: Es gibt nicht einen zentralen Server, sondern viele unabhängige Homeserver, die miteinander kommunizieren.

```
Alice                               Bob
[auf matrix.org]                    [auf opencampus.sh]
      │                                    │
      └──────────── Matrix ────────────────┘
        (Server sprechen miteinander)
```

Bei einem zentralisierten Dienst wie WhatsApp ist Meta der Single Point of Failure: Sie können Accounts sperren, den Dienst abschalten, oder Daten herausgeben. Bei Matrix gibt es diesen einen Punkt nicht.

- Dein Homeserver kann von dir selbst betrieben werden
- Wenn `matrix.org` abgestürzt ist, kommunizieren alle anderen Homeserver weiter
- Kein einzelnes Unternehmen kann dich aus deiner Kommunikation ausschließen

Das ist die gleiche Logik wie beim [Fediverse](../../wiki/glossar.md#fediverse), das wir in der Session über Mastodon behandeln.

**Der Kompromiss:** Föderierung macht das System komplexer. Wer kontrolliert, welche Server miteinander kommunizieren dürfen? Was passiert, wenn ein Homeserver offline geht? Diese Fragen hat eine zentralisierte Architektur einfacher beantwortet.

## E2EE in Matrix: Wie es funktioniert

Matrix nutzt das **Megolm**-Protokoll für Gruppen-Verschlüsselung (basierend auf dem Signal-Protokoll). Die Schlüssel liegen auf den Endgeräten der Teilnehmer, nicht auf dem Homeserver.

E2EE ist in Matrix **opt-in**: Räume können mit oder ohne Verschlüsselung erstellt werden. Öffentliche Räume sind meist unverschlüsselt (weil jeder mitlesen kann sowieso), private Räume sollten verschlüsselt sein.

**Was der Homeserver sieht:**
- Wer kommuniziert mit wem (Metadaten)
- Wann und wie oft
- Die verschlüsselte Nachricht – aber nicht deren Inhalt

**Was der Homeserver nicht sieht:**
- Den Inhalt verschlüsselter Nachrichten

## Das Verifikationsproblem in Matrix

Matrix löst das Verifikationsproblem durch **Cross-Signing**: Du kannst deine eigenen Geräte gegenseitig als vertrauenswürdig markieren, und andere Nutzer können dein Gerät verifizieren. Das geht z.B. durch einen QR-Code-Scan oder durch Vergleich eines Sicherheits-Codes.

In der Praxis sieht man in Element häufig Warnmeldungen wie „Dieses Gerät wurde nicht verifiziert". Das ist ein Hinweis, dass du die Identität des Gegenübers noch nicht bestätigt hast.

Wer eine sehr hohe Sicherheitsanforderung hat, sollte verifizieren. Für den alltäglichen Einsatz reicht das Vertrauen in den Homeserver.

## Schlüssel-Backup: Was passiert bei Geräteverlust?

Wenn dein Gerät kaputtgeht oder du die App neu installierst, sind alle verschlüsselten Nachrichten ohne Backup verloren. Matrix löst das mit einem **verschlüsselten Key-Backup** auf dem Homeserver.

Der Backup ist mit einem Passwort oder einem Wiederherstellungsschlüssel gesichert. Der Homeserver speichert das Backup, kann es aber nicht lesen. Das ist der richtige Kompromiss zwischen Nutzbarkeit und Sicherheit.

**Wichtig:** Den Wiederherstellungsschlüssel sicher aufbewahren. Wer ihn verliert und das Gerät verliert, verliert den Zugang zu alten Nachrichten unwiederbringlich.

## Für wen ist Matrix die richtige Wahl?

**Gut geeignet für:**
- Gruppen, die keine zentralisierte Plattform wollen
- Organisationen, die einen eigenen Server betreiben wollen
- Alle, die ihre Kommunikation langfristig kontrollieren wollen

**Weniger geeignet für:**
- Jemanden, der schnell alle Kontakte erreichen will (Matrix hat keine kritische Masse wie WhatsApp)
- Sehr einfache Bedienung als oberste Priorität

**Was gegen Signal spricht** (als Alternative): 
- Signal ist zentralisiert, keine Föderierung
- Identität wird über Telefonnummer überprüft. Daten sind zwar sicher, aber wer die Telefonnummer kontrolliert kann sich auf Signal als die Person ausgeben (in der Standardeinstellung)
- Trotz starker E2EE können Metadaten durch Behördenanfragen abgefragt werden.

Signal ist technisch exzellent, aber architektonisch zentralisiert.

## Weiter geht es

→ [Jitsi Meet](04-jitsi.md)

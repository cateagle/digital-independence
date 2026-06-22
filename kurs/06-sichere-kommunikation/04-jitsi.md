# Jitsi Meet: Verschlüsselte Videokonferenzen und ihre Grenzen

Jitsi Meet ist eine Open-Source-Videokonferenzlösung, eine Alternative zu Zoom, Teams und Google Meet. Sie ist kostenlos, kann selbst gehostet werden, und funktioniert im Browser ohne Installation.

Die Sicherheitsarchitektur von Videokonferenzen ist strukturell anders als bei Chat. Darauf gehen wir hier ein.

## Was Jitsi ist

Jitsi Meet ist ein vollständiges Videokonferenzsystem. Auf [meet.jit.si](https://meet.jit.si) gibt es eine öffentlich zugängliche Instanz, die ohne Account nutzbar ist. Wer volle Kontrolle will, kann eine eigene Instanz hosten.

**Kernmerkmale:**
- Kein Account notwendig für Gäste
- Funktioniert im Browser (kein Download erforderlich)
- Open Source (Apache 2.0)
- Kann selbst gehostet werden
- Räume per Link teilen

## Warum E2EE bei Video strukturell schwieriger ist

Bei einem Zwei-Personen-Chat ist E2EE konzeptionell einfach: Alice verschlüsselt mit Bobs Key, Bob entschlüsselt mit seinem Private Key.

Bei einer Videokonferenz mit mehreren Teilnehmern gibt es ein fundamentales Problem: Wie überträgt man Audio/Video an zehn Personen gleichzeitig, ohne dass jeder Teilnehmer zehn separate verschlüsselte Streams sendet und empfängt?

Die Lösung, die Jitsi (und fast alle anderen Videokonferenzsysteme) nutzt, heißt **SFU** (Selective Forwarding Unit:

```
Alice ───► SFU ───► Bob
            └─────► Carol
            └─────► Dave
```

Der SFU-Server empfängt Alices Stream und verteilt ihn an alle anderen. Das ist effizient. Aber der Server muss dafür den Stream weiterleiten und klassischerweise muss er ihn dafür entschlüsseln können.

## E2EE in Jitsi: Wann es funktioniert, wann nicht

Jitsi hat seit 2020 experimentelle E2EE-Unterstützung über **Insertable Streams** (ein Web-API-Standard). Dabei wird der Stream im Browser verschlüsselt, bevor er zum SFU geht und der SFU leitet ihn weiter, ohne ihn lesen zu können.

**Einschränkungen:**
- Funktioniert nur in bestimmten Browsern (Chromium-basiert)
- Funktioniert nicht in der nativen Jitsi-App auf älteren Android-Versionen
- Muss explizit aktiviert werden. Es ist nicht der Standard
- Skaliert bei großen Gruppen schlechter

**Ohne E2EE:** Der Server sieht alle Audio- und Videostreams. Wer Jitsi auf einem vertrauenswürdigen eigenen Server betreibt, kann damit gut leben. Wer einem fremden Server nicht traut, sollte E2EE explizit aktivieren und prüfen, ob alle Teilnehmer Browser nutzen, die es unterstützen.

## Metadaten bei Videokonferenzen

Auch mit E2EE weiß der Server bei Jitsi:
- Wer hat wann an einer Konferenz teilgenommen
- Wie lange war jemand dabei
- Von welcher IP-Adresse aus

Wer ein sehr hohes Schutzbedürfnis hat, sollte diese Metadaten ebenfalls einrechnen.

## Die Frage der Instanz

Jitsi auf `meet.jit.si` wird von 8x8 betrieben, einem US-amerikanischen Unternehmen. Die Nutzungsbedingungen erlauben eine Nutzung kostenlos für kleine Meetings.

**Alternativen:**
- Viele europäische Bildungseinrichtungen und NGOs betreiben eigene Jitsi-Instanzen
- Selbst hosten (Exkurs)
- Netzbetreiber wie die [Bundeswehr](https://bwcloud.bwl.de), viele Universitäten und z.B. [digitalcourage.de](https://meet.digitalcourage.de) bieten Instanzen an

## Jitsi vs. Matrix Calls vs. BigBlueButton

Jitsi ist nicht die einzige Option:

|              | Jitsi Meet    | Matrix Calls         | BigBlueButton   |
|--------------|---------------|----------------------|-----------------|
| Fokus        | Konferenz     | In Matrix integriert | Lehre / Webinar |
| E2EE         | Experimentell | In Entwicklung       | Nein            |
| Self-Hosting | Einfach       | Komplex              | Komplex         |
| Skalierung   | Mittel        | Klein                | Groß            |

Für einfache Meetings ist Jitsi die pragmatische Wahl. Für enge Integration in eine Matrix-Kommunikationsinfrastruktur ist Matrix Calls interessant, aber noch nicht ausgereift.

## Weiter geht es
- [Einordnung & Entscheidungshilfe](05-einordnung.md)

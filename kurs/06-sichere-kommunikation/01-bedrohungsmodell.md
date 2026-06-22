# Das Bedrohungsmodell: Vor wem schütze ich mich eigentlich?

Bevor wir über Verschlüsselung reden, müssen wir eine Frage stellen, die häufig übersprungen wird:

> **Vor wem will ich mich eigentlich schützen?**

Die Antwort auf diese Frage bestimmt, welche Sicherheitsmaßnahmen sinnvoll sind und welche bloß Aufwand ohne Nutzen bedeuten.

## Drei Ebenen möglicher Angreifer

Nicht alle Bedrohungen sind gleich. Es hilft, sie in Ebenen zu denken:

### 1. Das Netzwerk

Jemand liest den Datenverkehr mit, während er von A nach B transportiert wird. Klassische Beispiele:
- Öffentliches WLAN im Café
- Der eigene Internetanbieter
- Ein staatlicher Akteur, der Internetknoten überwacht

**Schutz dagegen:** Transportverschlüsselung (TLS).

### 2. Der Betreiber

Der Server, über den deine Kommunikation läuft, gehört jemandem. Dieser jemand kann im Prinzip mitlesen. Entweder weil er es will, weil er gehackt wurde, oder weil er von Behörden dazu gezwungen wird.

Beispiele:
- WhatsApp-Server liegen bei Meta
- Signal betreibt eigene Server
- Ein selbst gehosteter Dienst liegt bei dir oder bei einem VPS-Anbieter

**TLS schützt hier nicht.** Die Verbindung zum Server ist verschlüsselt, aber der Server entschlüsselt die Nachricht, um sie weiterzuleiten. Wer den Server kontrolliert, kann lesen.

### 3. Das Gerät

Das Endgerät selbst ist kompromittiert. Das kann über Malware, physischer Zugriff, oder ein unsicheres Betriebssystem geschehen. Dagegen hilft keine Kommunikationsverschlüsselung der Welt. Die Nachricht wird vor dem Verschlüsseln oder nach dem Entschlüsseln abgegriffen.

**Kein Kommunikationstool der Welt schützt vor dieser Ebene.**

## Warum diese Unterscheidung wichtig ist

Viele Menschen denken bei "sicherer Kommunikation" an TLS und halten das für ausreichend. Das stimmt, wenn man nur dem Netzwerk nicht traut.

Wenn man aber dem Betreiber nicht traut (oder ihm nicht trauen möchte), braucht man Ende-zu-Ende-Verschlüsselung. Das ist eine fundamental andere Anforderung, die eine fundamental andere Architektur erfordert.

| Bedrohung                    | Schutz durch TLS | Schutz durch E2EE           |
|------------------------------|------------------|-----------------------------|
| Netzwerk / ISP               | ja               | ja                          |
| Betreiber des Servers        | nein             | ja                          |
| Behördenanfrage an Betreiber | nein             | ja (wenn korrekt umgesetzt) |
| Kompromittiertes Endgerät    | nein             | nein                        |

## Was Self-Hosting ändert und was nicht

Wenn du deinen eigenen Server betreibst, vertraust du dir selbst statt einem Drittanbieter. Das eliminiert einen relevanten Teil der Betreiber-Bedrohung.

Aber:
- Dein Server kann gehackt werden, wenn er schlecht abgesichert ist
- Dein VPS-Anbieter hat physischen Zugriff auf die Hardware
- Behörden können den VPS-Anbieter anfordern, nicht nur den Dienst selbst

Self-Hosting verlagert das Vertrauen. Es eliminiert es nicht.

## Das Bedrohungsmodell als Werkzeug

Das Bedrohungsmodell ist kein akademisches Konzept. Es ist eine praktische Frage:

> **"Wenn meine Kommunikation kompromittiert würde: Wer hätte das getan und wie?"**

Für ein Gespräch über den Geburtstagsplan reicht WhatsApp. Für organisierte politische Arbeit in einem autoritären Kontext nicht. Die meisten Menschen liegen irgendwo dazwischen.

Das Ziel dieses Kurses ist nicht, paranoid zu werden. Es geht darum, bewusst zu entscheiden und zu wissen, was man aufgibt, wenn man eine bestimmte Lösung wählt.

## Weiter geht es

- [Ende-zu-Ende-Verschlüsselung](02-e2ee.md)

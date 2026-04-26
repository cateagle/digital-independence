# Was ist Nextcloud?

## Einordnung: Was für eine Software ist das?

Nextcloud ist eine selbst hostbare Collaboration-Plattform. Nextcloud ist der Versuch, die wichtigsten Dienste, die du heute von Google oder Microsoft mietest, auf einem eigenen Server zu betreiben.

| Google-Dienst             | Microsoft-Äquivalent       | Nextcloud-Äquivalent          |
|---------------------------|----------------------------|-------------------------------|
| Google Drive              | OneDrive                   | Dateispeicher & Sync          |
| Google Docs/Sheets/Slides | Office 365 (Word/Excel/PPT)| Collabora Online / OnlyOffice |
| Google Kalender           | Outlook-Kalender           | Nextcloud Kalender            |
| Google Kontakte           | Outlook-Kontakte           | Nextcloud Kontakte            |
| Google Meet               | Microsoft Teams            | Nextcloud Talk                |
| Google Fotos              | OneDrive Fotos             | Nextcloud Photos              |
| Gmail                     | Outlook                    | Nextcloud Mail                |

Nextcloud ist kein Ersatz für jedes einzelne dieser Produkte in jeder Hinsicht, aber es ist ein funktionierender, datenschutzfreundlicher Einstiegspunkt.

Wenn man sich für eine einzige Sache zum selbst hosten entscheiden muss, dann ist Nextcloud vermutlich die Software, die die meisten Aufgaben auf einmal erfüllt.

---

## Lizenz

- **Lizenz:** AGPLv3 (Nextcloud Community Edition)
- **Sprache:** PHP (Backend), Vue.js (Frontend)
- **Quellcode:** [github.com/nextcloud/server](https://github.com/nextcloud/server)

Die [AGPL](../../wiki/glossar.md#agpl-gnu-affero-general-public-license) hat einen wichtigen Effekt: Wer Nextcloud als [SaaS](../../wiki/glossar.md#saas-software-as-a-service)-Dienst anbietet, muss seinen eigenen Code unter AGPL veröffentlichen. Das schützt die Community davor, dass jemand das Projekt einfach abzweigt und proprietär weiterbetreibt. Nextcloud lässt alle Contributors weiterhin Eigentümer an ihrem begetragenden Code bleiben, sodass die Nextcloud GmbH nicht in der Lage ist den Code zu privatisieren ohne die Zustimmung aller aktuell über 600 [Autoren](https://github.com/nextcloud/server/blob/master/AUTHORS) einzuholen.

---

## Geschichte: Woher kommt Nextcloud?

Nextcloud ist 2016 aus einem [Fork](../../wiki/glossar.md#fork) von [ownCloud](../../wiki/glossar.md#owncloud) entstanden.

Frank Karlitschek, einer der Gründer von ownCloud, verließ das Unternehmen im Streit über die Ausrichtung (zu viel Enterprise-Fokus, zu wenig Community) und gründete Nextcloud GmbH. Das ist ein klassisches Open-Source-Phänomen: Wenn die Community und das Unternehmen in verschiedene Richtungen wollen, forkt die Community.

ownCloud existiert noch, ist aber seitdem weit hinter Nextcloud zurückgefallen, sowohl in Aktivität als auch in Nutzerzahlen.

---

## Geschäftsmodell: Wie verdient Nextcloud Geld?

Nextcloud GmbH betreibt ein **[Open-Core](../../wiki/glossar.md#open-core)-Modell kombiniert mit Professional Services**:


Nextcloud Community (kostenlos, AGPLv3)
- Alle Basisfunktionen
- App Store (größtenteils kostenlos)
- Community-Support (Forum, GitHub)

Nextcloud Enterprise (kostenpflichtig)
- Nextcloud Subscription (Support-SLA)
- Compliance-Features
- Erweiterte Audit-Logs
- Dedizierter Support

Was das in der Praxis bedeutet: 

Als Privatperson oder kleines Team brauchst du kein Geld auszugeben. Die Community Edition ist vollständig nutzbar. Nextcloud verdient an Unternehmen, die Support-Garantien, Compliance-Zertifizierungen und SLAs brauchen.

---

## SaaS-Partner: Nextcloud ohne eigenen Server

Nextcloud fördert aktiv ein Ökosystem von [Hosting-Partnern](https://nextcloud.com/de/partner/). Wer Nextcloud nicht selbst betreiben will oder kann, kann es bei einem zertifizierten Anbieter buchen.

Nextcloud selbst verkauft kein Hosting, die zertifizierten Partner schon. Nextcloud verdient an Enterprise-Support und der Entwicklung von Anpassungen, nicht am Hosting. Das Hosting kleiner Nextcloud Server für wenige Euros pro Monat ist ein komplett anderes Geschäftsmodell mit anderer Vertriebs-/Servicestruktur. Nextcloud selbst kann sich so auf die Weiterentwicklung der Software und die Pflege der Community spezialisieren, während die Partnerunternehmen näher an den Endnutzern sind. Damit Nextcloud als Produkt für die Partnerunternehmen attraktiv bleibt, muss ständig darauf geachtet werden, dass es weiterhin leicht zu installieren bleibt.

Durch die große Community und den offenen Code haben die Partner den Vorteil, dass sie den eine gut Dokumentation haben und so gut wie jedes Problem schon einmal in einem [Forum](https://help.nextcloud.com/) angesprochen wurde. Wäre Nextcloud geschlossen, dann müssten sie sich komplett um alles selbst kümmern.

---

## Use Cases: Wer nutzt Nextcloud wofür?

### Privatpersonen
- Eigene "Google Drive"-Alternative: Fotos, Dokumente, Videos synchron auf allen Geräten
- Kontakte und Kalender ohne Google/Apple
- Passwörter (via Nextcloud Passwords App)

### Kleine Teams / Vereine
- Gemeinsame Dateiablage ohne Dropbox-Abo
- Kalender teilen ohne Microsoft/Google-Abhängigkeit
- Aufgabenverwaltung ([Nextcloud Deck](../../wiki/glossar.md#nextcloud-deck), ähnlich wie Trello)
- Videokonferenzen

### Unternehmen und Behörden
- [DSGVO](../../wiki/glossar.md#dsgvo-datenschutz-grundverordnung)-konforme Alternative zu Microsoft 365
- Dateiaustausch mit externen Partnern ohne Daten an US-Anbieter
- Viele Behörden in Deutschland und Frankreich nutzen Nextcloud aktiv

### Bekannte Nutzer
- Land Schleswig-Holstein
- Bundesministerien (Deutschland) setzen auf Nextcloud
- CERN
- TU Berlin und andere Hochschulen
- Deutsche Telekom (für Kundenangebote)

Aktuell suchen sehr viele Organisationen nach Alternativen zu den Amerikanischen Big Tech Unternehmen. Sehr viele entscheiden sich für Nextcloud. Wenn eine große Organisation erstmal auf Nextcloud gewechselt ist, sorgt die Trägheit solcher Organisationen dafür, dass die Finanzierung von Nextcloud langfristig gestärkt wird. Daher sind große Nutzer generell ein Zeichen für Stabilität bei Open Source Projekten.

---

## Was Nextcloud nicht ist

Nextcloud ist keine eierlegende Wollmilchsau. Kritisch betrachtet:

- **Kein E-Mail-Server**: die Mail-App ist ein Client, kein Server
- **Kein vollwertiger Office-Ersatz**: [Collabora Online](../../wiki/glossar.md#collabora-online)/[OnlyOffice](../../wiki/glossar.md#onlyoffice) sind eigenständige Projekte, die eingebunden werden
- **Wartungsaufwand**: Nextcloud braucht regelmäßige Updates und Pflege
- **Performance**: auf schwacher Hardware oder mit [SQLite](../../wiki/glossar.md#sqlite) als Datenbank langsam bei vielen Nutzern (privat eher irrelevant)
- **PHP**: Sehr Klassischer Technologie-Stack und nicht mehr ganz so modern, aber es gibt extrem viele Entwickler, die damit umgehen können.

Das sind keine Gründe, es nicht zu nutzen aber Gründe, realistische Erwartungen zu haben.

---

## Weiter geht es

→ [Nextcloud AIO Setup](02-nextcloud-aio-setup.md)
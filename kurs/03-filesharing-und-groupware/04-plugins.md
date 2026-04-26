# Exkurs: Nextcloud Apps & Plugins

## Der Nextcloud App Store

Nextcloud hat ein eigenes App-Ökosystem: [apps.nextcloud.com](https://apps.nextcloud.com/).
Dort gibt es über 300 Apps, von sinnvollen Erweiterungen bis zu schlecht gepflegten Experimenten.

Apps installieren: **Nextcloud Admin → Apps** (oben rechts, Nutzermenü → Apps).

---

## Apps, die standardmäßig dabei sind

Einige Apps kommen vorinstalliert und sind direkt nutzbar:

| App             | Funktion                                   |
|-----------------|--------------------------------------------|
| Files           | Dateimanager (Kern-App)                    |
| Photos          | Fotogalerie mit automatischer Organisation |
| Calendar        | [CalDAV](../../wiki/glossar.md#caldav)-Kalender                            |
| Contacts        | [CardDAV](../../wiki/glossar.md#carddav)-Adressbuch                         |
| Talk            | Chat und Videokonferenzen                  |
| Notes           | Einfache Markdown-Notizen                  |
| Deck            | [Kanban-Board](../../wiki/glossar.md#kanban-board) (ähnlich Trello)              |
| Mail            | E-Mail-Client (kein Mail-Server!)          |

---

## Empfehlenswerte Apps

### Collabora Online / OnlyOffice
Vollwertige Office-Editoren direkt im Browser. Wichtig: Diese Apps installieren nur den **Connector**. Der eigentliche Office-Server ist ein separater [Docker](../../wiki/glossar.md#docker)-Container.

AIO kann Collabora als optionale Komponente mitinstallieren.

**Wählen zwischen beiden:**
- **Collabora** (LibreOffice-basiert): Bessere Kompatibilität mit OpenDocument, [DSGVO](../../wiki/glossar.md#dsgvo-datenschutz-grundverordnung)-freundlich
- **[OnlyOffice](../../wiki/glossar.md#onlyoffice)**: Bessere Microsoft-Office-Kompatibilität, von vielen als "besser aussehend" wahrgenommen

### Nextcloud Passwords
Passwort-Manager mit Browser-Extension. Nutzt die Nextcloud-Instanz als Backend.
Alternative zu Bitwarden für alle, die alles auf einem Server konsolidieren wollen.

### External Storage Support
Erlaubt das Einbinden externer Speicher: [SFTP](../../wiki/glossar.md#sftp-ssh-file-transfer-protocol), [S3](../../wiki/glossar.md#s3-simple-storage-service), [SMB](../../wiki/glossar.md#smb-server-message-block)/CIFS, [FTP](../../wiki/glossar.md#ftp-file-transfer-protocol).
Nützlich, wenn Daten auf einem NAS liegen sollen und Nextcloud nur als Frontend dient.

### Two-Factor Authentication (TOTP)
Zweiter Faktor beim Login via Authenticator-App. Für jeden Server im offenen Internet empfohlen.

### Memories
Moderne Foto-App als Alternative zur eingebauten Photos-App. Besser für große Fotosammlungen.

---

## Apps mit Vorsicht

### Jede App aus einer unbekannten Quelle
Der Nextcloud App Store hat keine strenge Sicherheitsprüfung wie etwa der iOS App Store.
Apps können auf alle deine Nextcloud-Daten zugreifen. Vor der Installation:
- Wer entwickelt die App? Ist das Nextcloud GmbH oder eine Einzelperson?
- Wann wurde zuletzt ein Update veröffentlicht?
- Wie viele Installationen gibt es?
- Mit welcher Nextcloud-Version ist die App kompatibel?

### Apps ohne aktive Wartung
Eine App, deren letztes Update zwei Jahre alt ist, unterstützt möglicherweise die aktuelle Nextcloud-Version nicht richtig. Im besten Fall fehlen Features, im schlechtesten Fall verursacht sie Fehler oder Sicherheitsprobleme.

---

## Warum nicht einfach alle Apps installieren?

Jede installierte App:
- Vergrößert die **Angriffsfläche**: Mehr Code bedeutet tendenziell mehr potenzielle Schwachstellen.
- Erhöht den **Wartungsaufwand**: Apps müssen zusammen mit Nextcloud aktuell gehalten werden und sind nicht automatisch mit einer neuen Version kompatibel. Bei einem Hobbyprojekt kann das gerne ein paar Wochen bis Monate dauern, dass die App mit der neuesten Nextcloud Version kompatibel ist.
- Kann die **Performance** beeinflussen. Manche Apps führen bei jedem Seitenaufruf Datenbankabfragen aus.

Es ist nicht besonders schwierig eine einfache Nextcloud App zu entwickeln und mit KI-Tools geht das immer schneller. Allerdings ist auch hier die Gefahr. Jemand kann eine halbfertige App veröffentlichen und nicht vernünftig pflegen. Das kann privat für manche Sachen ok sein, wenn man Daten einer Organisation verwaltet, dann muss man deutlich gründlicher vorgehen.

**Faustregel:** Nur Apps installieren, die aktiv genutzt werden.

---

## App-Updates

Nextcloud-Apps werden **nicht automatisch** aktualisiert. Unter Admin → Apps → Updates siehst du verfügbare Updates. Diese sollten regelmäßig eingespielt werden,
besonders wenn sie Sicherheitsfixes enthalten.

AIO selbst pingt keine App-Updates automatisch durch. Das ist eine bekannte Lücke im AIO-Komfort-Versprechen.

---

## Weiter geht es

→ [KI-Features & KI-Backend](05-ki-features.md)
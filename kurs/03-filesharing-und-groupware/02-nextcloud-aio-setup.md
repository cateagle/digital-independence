# Nextcloud AIO Setup

## Warum AIO und nicht das "normale" Docker Image?

Es gibt zwei gängige Wege, Nextcloud mit [Docker](../../wiki/glossar.md#docker) zu betreiben:

### Option A: Nextcloud + einzelne Abhängigkeiten selbst zusammenstellen
```yaml
# docker-compose.yml (vereinfacht)
services:
  nextcloud:
    image: nextcloud:latest
  db:
    image: mariadb:latest
  redis:
    image: redis:latest
```
Das ist flexibel, aber du konfigurierst alles selbst: Datenbank, [Redis](../../wiki/glossar.md#redis), Cronjobs, Office-Integration, Backups. Damit kannst du tendenziell mehr User verwalten, aber du musst dich deutlich mehr mit der Technologie auseinandersetzen. Nichts davon funktioniert "out of the box" richtig.

### Option B: Nextcloud All-in-One (AIO)

[AIO](../../wiki/glossar.md#aio-all-in-one) ist ein offiziell von Nextcloud entwickeltes "Mastercontainer"-Image,
das alle notwendigen Komponenten selbst verwaltet und konfiguriert:
- Nextcloud (PHP-App)
- [PostgreSQL](../../wiki/glossar.md#postgresql) oder [MariaDB](../../wiki/glossar.md#mariadb) (je nach Konfiguration)
- Redis (Cache)
- [Collabora Online](../../wiki/glossar.md#collabora-online) (Office-Integration, optional)
- [Nextcloud Talk](../../wiki/glossar.md#nextcloud-talk) (Videokonferenzen, optional)
- Backup-Mechanismus (optional)
- Nginx als [Reverse Proxy](../../wiki/glossar.md#reverse-proxy)

Wenn du nicht 100+ User gleichzeitig darauf arbeiten lassen willst, solltest du immer zuerst AIO ausprobieren. Wenn es von der Leistung nicht ausreicht, dann kannst du immer noch die kompliziertere Variante ausprobieren. Es reduziert den operativen Aufwand drastisch und die Komponenten sind aufeinander abgestimmt sind. Wenn du einen Bug hast, dann bist du nicht der einzige. Tausende andere verwenden die exakt gleiche Konfiguration und irgendeiner davon wird das Problem schon gelöst haben.

Der Nachteil ist weniger Kontrolle über die einzelnen Teile. Das ist für die meisten aber ein akzeptabler Trade-off.

---

## Voraussetzungen

```bash
# Prüfen ob Docker läuft
docker --version
docker compose version

# Prüfen wie viel RAM verfügbar ist (AIO braucht mindestens 4 GB)
free -h

# Prüfen wie viel Speicher frei ist (empfohlen: 40+ GB)
df -h
```

Falls die VM zu wenig RAM hat: In VirtualBox die VM stoppen, dann unter
Einstellungen → System → Hauptspeicher auf mindestens 4096 MB erhöhen.

Der Dateispeicher ist mit 40 GB natürlich nichts für die echte nutzung. Bei einer echten Nextcloud installation willst du wahrscheinlich mehr Speicher haben und auch noch ein Laufwerk für Backups haben.

---

## Port Forwarding in VirtualBox anpassen

Für Nextcloud AIO brauchen wir zusätzliche Ports:

| Name       | Protokoll | Host-Port | Gast-Port | Zweck                          |
|------------|-----------|-----------|-----------|--------------------------------|
| SSH        | TCP       | 8022      | 22        | SSH-Zugang (bereits vorhanden) |
| HTTP       | TCP       | 8080      | 80        | HTTP (bereits vorhanden)       |
| HTTPS      | TCP       | 8443      | 443       | HTTPS für Nextcloud            |
| AIO-Admin  | TCP       | 8888      | 8080      | AIO Administrations-UI         |

> **Warum Port 8088 auf dem Host auf 8080 im Gast?**
> Der AIO-Mastercontainer lauscht intern auf Port 8080 (HTTP, nicht HTTPS).
> Wir leiten das auf Host-Port 8888 weiter, damit es nicht mit dem bereits
> gemappten Port 8080 (→ Gast 80) kollidiert.
> In der Praxis würden wir die Ports anders einrichten.

In VirtualBox: VM auswählen → Einstellungen → Netzwerk → Port-Weiterleitung.

---

## Schritt 1: Arbeitsverzeichnis und Compose-Datei anlegen

```bash
mkdir ~/nextcloud
cd ~/nextcloud
```

Jetzt eine `docker-compose.yml` mit folgendem Inhalt anlegen:

```yaml
services:
  nextcloud-aio-mastercontainer:
    image: ghcr.io/nextcloud-releases/all-in-one:latest
    restart: unless-stopped
    container_name: nextcloud-aio-mastercontainer
    ports:
      - "8080:8080"
    environment:
      APACHE_PORT: 11000
      APACHE_IP_BINDING: 127.0.0.1
    volumes:
      - nextcloud_aio_mastercontainer:/mnt/docker-aio-config
      - /var/run/docker.sock:/var/run/docker.sock:ro

volumes:
  nextcloud_aio_mastercontainer:
```

Was hier konfiguriert ist:

- `restart: unless-stopped` – Container startet nach VM-Neustart automatisch
- `ports: 8080:8080` – AIO-Admin-UI auf Port 8080 im Container erreichbar
- `APACHE_PORT: 11000` – Nextcloud selbst lauscht intern auf Port 11000 (nicht direkt exposed)
- `APACHE_IP_BINDING: 127.0.0.1` – Nextcloud nur lokal erreichbar; ein Reverse Proxy kümmert sich um [HTTPS](../../wiki/glossar.md#https-hypertext-transfer-protocol-secure)
- `/var/run/docker.sock` – AIO braucht Zugriff auf den Docker-Daemon, um selbst Container zu starten

> **Bind Mount vs. Docker Volume:**
> Bei Excalidraw haben wir nichts persistiert. Nach `docker compose down` war alles weg.
> Für Nextcloud wäre das katastrophal. Docker Volumes lösen das: Die Daten überleben
> Container-Neustarts und Updates. Nachteil: Sie liegen unter `/var/lib/docker/volumes/`
> und sind nicht so einfach direkt zugänglich.
>
> Das Volume `nextcloud_aio_mastercontainer` wird automatisch von Compose angelegt,
> wenn es noch nicht existiert – kein manuelles `docker volume create` nötig.

> **Risiko: Docker Socket Mount**
> Wer `/var/run/docker.sock` in einen Container einbindet, gibt diesem Container
> faktisch Root-Zugriff auf den Host. AIO braucht das, um die anderen Container
> (Datenbank, Redis, etc.) zu starten und zu verwalten. Das ist ein bekannter
> Trade-off beim AIO-Ansatz. Auf einem öffentlichen Server sollte AIO-Admin
> niemals ohne Authentifizierung aus dem Internet erreichbar sein.

---

## Schritt 2: AIO Mastercontainer starten

```bash
docker compose up -d
```

Fertig. Ab jetzt gilt für dieses Projekt immer dasselbe Muster:
- Starten: `docker compose up -d`
- Stoppen: `docker compose down`
- Logs anschauen: `docker compose logs -f`

---

## Schritt 3: AIO Admin Interface öffnen

Vom Host-Rechner im Browser:

```
http://localhost:8888
```

(Port 8888 auf dem Host wurde auf 8080 in der VM weitergeleitet)

Beim ersten Aufruf zeigt AIO ein **Passwort** an. Dieses Passwort einmalig notieren –
es ist der einzige Zugang zur AIO-Administrationsoberfläche.

---

## Schritt 4: Nextcloud-Domain konfigurieren

AIO fragt nach einer Domain. Da wir lokal in einer VM arbeiten und kein echtes [TLS](../../wiki/glossar.md#tls-transport-layer-security)-Zertifikat bekommen können, nutzen wir eine lokale IP `localhost`/`127.0.0.1` (localhost ist ein Alias für 127.0.0.1 und leitet zurück an das interne Netzwerk). 


> **Warum fragt AIO nach einer Domain?**
> Nextcloud ist für HTTPS-Betrieb mit einer echten Domain ausgelegt.
> Im Produktionsbetrieb würdest du hier `cloud.meinedomain.de` eintragen
> und [Let's Encrypt](../../wiki/glossar.md#lets-encrypt) übernimmt automatisch das TLS-Zertifikat.
> Für unsere VM akzeptieren wir die Unsicherheitswarnung des Browsers.

---

## Schritt 5: Container-Stack starten

In der AIO-Weboberfläche:

1. Optionale Komponenten auswählen (für den Anfang: alles abwählen, nur Nextcloud)
2. "Start containers" klicken
3. Warten – AIO lädt alle Images und startet die Container. Das dauert beim ersten Mal einige Minuten.

```bash
# Fortschritt beobachten
docker ps
docker logs nextcloud-aio-mastercontainer
```

---

## Schritt 6: Nextcloud aufrufen

Wenn AIO "Nextcloud is running" anzeigt:

```
https://localhost:8443
```

Der Browser warnt vor einem ungültigen Zertifikat (weil wir keine echte Domain haben).
Das ist in der VM-Umgebung akzeptabel – "Trotzdem fortfahren" klicken.

Der Admin-Account wird beim ersten Start von AIO automatisch angelegt.
Das Passwort wird in der AIO-Oberfläche angezeigt.

---

## Was jetzt läuft

```bash
docker ps
```

Du solltest mehrere Container sehen:
- `nextcloud-aio-mastercontainer`: die Schaltzentrale
- `nextcloud-aio-nextcloud`:  die eigentliche Nextcloud-App
- `nextcloud-aio-database`: PostgreSQL
- `nextcloud-aio-redis`: Cache
- `nextcloud-aio-apache`:  Reverse Proxy vor Nextcloud

AIO hat also das Setup übernommen, das wir sonst per Hand in einer `docker-compose.yml` definiert hätten, inklusive der richtigen Konfiguration dieser Komponenten untereinander.

---

## Updates

AIO hat einen eingebauten Update-Mechanismus. In der Admin-UI gibt es einen "Update available"-Button, sobald eine neue Nextcloud-Version verfügbar ist.

Bei Updates gilt wie immer *Kein Backup, kein Mitleid* -> Updates nie ohne vorheriges Backup einspielen!

---

## Weiter geht es

→ [Datenbanken](03-datenbanken.md)
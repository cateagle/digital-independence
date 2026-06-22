# Exkurs: Eigene Jitsi Meet Instanz aufsetzen

Dieser Exkurs zeigt, wie du eine eigene Jitsi Meet Instanz betreibst. Das offizielle Docker-Setup von Jitsi ist ungewöhnlich: Es gibt keine einzelne `docker-compose.yml` die du selbst schreibst, sondern ein Release-Paket mit vorkonfigurierter Compose-Datei und einer `.env`-Datei für alle Parameter.

## Was dafür nötig ist

- Ein Server mit öffentlicher IP (VPS empfohlen)
- Eine Domain, deren DNS du kontrollierst
- Docker und Docker Compose
- Offene Ports: 80/tcp, 443/tcp, **10000/udp**

**Warum UDP-Port 10000?**

Jitsi überträgt Audio und Video über UDP. Ohne diesen Port funktioniert Jitsi nicht. Der Aufbau von Verbindungen scheitert oder fällt auf schlechte Qualität zurück.

## Architektur: Vier Container

Anders als Synapse besteht Jitsi aus vier zusammenarbeitenden Diensten:

| Container | Aufgabe                                         |
|-----------|-------------------------------------------------|
| `web`     | Die Web-UI (Nginx)                              |
| `prosody` | XMPP-Server für Signalisierung                  |
| `jicofo`  | Konferenz-Koordination                          |
| `jvb`     | Jitsi Videobridge, der eigentliche Medienrouter |

Der JVB (Jitsi Videobridge) ist der rechenintensivste Teil. Er empfängt alle Videostreams und verteilt sie an die Teilnehmer.

## Schritt 1: Release-Paket herunterladen

> **Wichtig:** Nicht das Git-Repository klonen. Das Repository enthält Entwicklungs-Images. Für den Betrieb immer das Release-Paket verwenden.

```bash
wget $(wget -q -O - https://api.github.com/repos/jitsi/docker-jitsi-meet/releases/latest \
  | grep zip | cut -d\" -f4)
unzip *.zip
cd jitsi-docker-jitsi-meet-*
```

## Schritt 2: `.env` anlegen und konfigurieren

```bash
cp env.example .env
```

Mindestens diese Werte anpassen:

```bash
# Pflichtfeld: deine öffentliche Domain
PUBLIC_URL=https://meet.example.com

# Zeitzone
TZ=Europe/Berlin

# Ports (Standard, kann so bleiben)
HTTP_PORT=80
HTTPS_PORT=443
```

## Schritt 3: Interne Passwörter generieren

Jitsi nutzt interne XMPP-Accounts für die Kommunikation zwischen den Containern. Diese müssen gesetzt sein, sonst starten die Container nicht.

```bash
./gen-passwords.sh
```

Das Skript schreibt starke Zufallspasswörter in die `.env`-Datei. Die Passwörter müssen nicht gemerkt werden. Sie sind für die interne Kommunikation zwischen Containern, nicht für Benutzer.

> Niemals dieselben Passwörter für mehrere Jitsi-Instanzen verwenden.

## Schritt 4: Konfigurationsverzeichnisse anlegen

```bash
mkdir -p ~/.jitsi-meet-cfg/{web,transcripts,prosody/config,prosody/prosody-plugins-custom,jicofo,jvb,jigasi,jibri}
```

Diese Verzeichnisse nehmen die persistente Konfiguration der einzelnen Container auf. Ohne sie schlägt `docker compose up` fehl.

## Schritt 5: Let's Encrypt aktivieren (optional, empfohlen)

In der `.env` aktivieren:

```bash
ENABLE_LETSENCRYPT=1
LETSENCRYPT_DOMAIN=meet.example.com
LETSENCRYPT_EMAIL=admin@example.com
```

Jitsi bringt eine eigene Let's Encrypt-Integration mit. Port 80 muss dafür erreichbar sein.

**Wenn du einen eigenen Reverse Proxy verwendest**, stattdessen HTTPS deaktivieren und den Proxy vorschalten:

```bash
DISABLE_HTTPS=1
ENABLE_HTTP_REDIRECT=0
ENABLE_LETSENCRYPT=0
```

## Schritt 6: Firewall öffnen

```bash
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw allow 10000/udp
```

Ohne UDP 10000 funktionieren Konferenzen mit mehr als zwei Teilnehmern nicht. Bei zwei Teilnehmern schaltet Jitsi auf Peer-to-Peer um (kein Server nötig), ab drei Personen läuft alles über den JVB.

## Schritt 7: Starten

```bash
docker compose up -d
```

Logs beobachten, um zu sehen, dass alles läuft:

```bash
docker compose logs -f web
docker compose logs -f jvb
```

Jitsi ist erreichbar unter `https://meet.example.com`.

## Zugang absichern (empfohlen)

Standardmäßig kann jeder, der die URL kennt, Räume erstellen. Für einen privaten Server sollte Authentifizierung aktiviert werden:

In `.env`:
```bash
ENABLE_AUTH=1
ENABLE_GUESTS=1    # Gäste können beitreten, aber keine Räume eröffnen
JICOFO_AUTH_TYPE=internal
```

Benutzer anlegen:

```bash
docker exec -it jitsi-prosody-1 prosodyctl \
  --config /config/prosody.cfg.lua \
  adduser alice@meet.example.com
```

**Was `ENABLE_GUESTS=1` bedeutet:** Nur authentifizierte Benutzer können einen Raum *eröffnen*. Wer einen Link bekommt, kann beitreten ohne Account. Das ist ein sinnvoller Kompromiss für kleine Teams.

## Updates

Jitsi veröffentlicht regelmäßig neue Versionen. Das Update funktioniert wie die Erstinstallation: Das neue Release-Paket herunterladen, entpacken und die bestehende `.env` übernehmen.

```bash
# Aktuelle Version stoppen
docker compose down

# Neues Paket herunterladen (wie in Schritt 1)
wget $(wget -q -O - https://api.github.com/repos/jitsi/docker-jitsi-meet/releases/latest \
  | grep zip | cut -d\" -f4)
unzip *.zip

# .env aus dem alten Verzeichnis übernehmen
cp ../jitsi-alt/.env .env

# Wieder starten
docker compose up -d
```

> Die `.jitsi-meet-cfg`-Verzeichnisse und die `.env` sind die einzigen Dateien, die persistent erhalten werden müssen.

## Hardware-Richtwerte

| Teilnehmer | RAM  | CPU     |
|------------|------|---------|
| bis 10     | 2 GB | 2 Kerne |
| bis 25     | 4 GB | 4 Kerne |
| bis 50     | 8 GB | 8 Kerne |

Der JVB ist der Engpass: Er muss jeden eingehenden Videostream empfangen und an alle anderen weiterleiten. Bandbreite ist oft wichtiger als CPU.

## Weiterführende Dokumentation
- [Offizielles Jitsi Docker Handbuch](https://jitsi.github.io/handbook/docs/devops-guide/devops-guide-docker)
- [GitHub: jitsi/docker-jitsi-meet](https://github.com/jitsi/docker-jitsi-meet)

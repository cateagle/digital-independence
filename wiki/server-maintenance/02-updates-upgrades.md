# Updates & Upgrades - Spickzettel

## 1. Warum Updates wichtig sind

Updates schließen Sicherheitslücken, beheben Fehler und bringen neue Funktionen. Ein ungepatchter Server ist wie eine Haustür mit bekanntem Schlossdefekt – es ist nur eine Frage der Zeit.

**Windows-Analogie:** Windows Update ist automatisch. Auf einem Linux-Server liegst du in der Verantwortung. Du entscheidest wann und was aktualisiert wird – das gibt dir Kontrolle, aber auch Pflichten.

## 2. System-Updates mit apt

[`apt`](../glossar.md#apt-advanced-package-tool) ist der Paketmanager von Ubuntu – vergleichbar mit dem App Store, aber für Systempakete.

**Schritt 1: Paketlisten aktualisieren (Infos über verfügbare Updates holen):**
```bash
sudo apt update
```

**Schritt 2: Installierte Pakete aktualisieren:**
```bash
sudo apt upgrade
```

**Beide Schritte auf einmal:**
```bash
sudo apt update && sudo apt upgrade -y
```

**Vollständiges Upgrade inkl. neuer Abhängigkeiten:**
```bash
sudo apt update
sudo apt full-upgrade -y
```

**Was wurde aktualisiert?**
```bash
grep " upgrade " /var/log/dpkg.log | tail -20
```

**Nicht mehr benötigte Pakete entfernen:**
```bash
sudo apt autoremove
```

**Vollständiges Dist-Upgrade (vorsichtig anwenden):**
```bash
sudo apt full-upgrade
```

## 3. Unterschied: Patch / Minor / Major Update

Software-Versionsnummern folgen oft dem Schema `MAJOR.MINOR.PATCH`:

```
PostgreSQL 15.4.2
          │  │  └── Patch: Bugfixes, Sicherheitslücken – immer einspielen
          │  └───── Minor: Neue Features, abwärtskompatibel – gut testen
          └──────── Major: Breaking Changes möglich – sehr sorgfältig planen
```

**Beispiele:**
| Update-Typ | Beispiel | Risiko | Empfehlung |
|------------|----------|--------|------------|
| Patch | 15.4.1 → 15.4.2 | Gering | Sofort einspielen |
| Minor | 15.3 → 15.4 | Mittel | Nach Test einspielen |
| Major | 15 → 16 | Hoch | Sorgfältig planen, testen |

**Sicherheitsupdates (Patch-Level) immer zeitnah einspielen!**

## 4. Docker-Image-Updates

[Docker](../glossar.md#docker)-Container laufen auf Basis von Images. Wenn ein neues Image verfügbar ist (z.B. wegen einer Sicherheitslücke in der Anwendung), musst du das Image aktualisieren.

**Aktuelles Image herunterladen:**
```bash
docker pull nginx:latest
```

**Mit [Docker Compose](../glossar.md#docker-compose) aktualisieren (empfohlener Weg):**
```bash
# Im Verzeichnis mit der docker-compose.yml
docker compose pull          # Neue Images herunterladen
docker compose up -d         # Container mit neuen Images neu starten
```

**Veraltete Images aufräumen:**
```bash
docker image prune -f
```

**Welche Images sind installiert?**
```bash
docker images
```

**Wann wurde ein Image zuletzt aktualisiert?**
```bash
docker inspect nginx:latest | grep -i created
```

**Tipp:** Vermeide den `latest`-Tag in Produktion. Fixiere stattdessen eine konkrete Version:
```yaml
# docker-compose.yml – schlecht:
image: nginx:latest

# docker-compose.yml – besser:
image: nginx:1.25.3
```

So weißt du immer genau, welche Version läuft, und kannst bewusst upgraden.

## 5. Changelog lesen – was ändert sich?

Bevor du ein größeres Update einspielst, lies den Changelog der Software. Dort stehen Breaking Changes, neue Konfigurationsoptionen und bekannte Probleme.

**Wo findet man Changelogs?**
- Projektwebsite / GitHub Releases-Seite
- `apt changelog paketname` für Ubuntu-Pakete:
```bash
apt changelog nginx
```
- Docker Hub: Auf der Image-Seite unter "Tags" oder in der README

**Worauf achten:**
```
✓ "Breaking change" oder "Deprecated" – bedeutet Anpassungsbedarf
✓ Neue Konfigurationsoptionen
✓ Geänderte Standardwerte
✓ Entfernte Features
```

## 6. Rollback-Strategie

Manchmal geht ein Update schief. Dann muss man schnell zurückrollen.

**Vor jedem Update:**
```bash
# 1. Backup der Konfigurationsdateien
sudo cp -r /etc/nginx/ /root/backup-nginx-$(date +%Y-%m-%d)/

# 2. Backup der Datenbank (bei Datenbank-Updates besonders wichtig)
sudo -u postgres pg_dumpall > /root/backup-db-$(date +%Y-%m-%d).sql

# 3. Docker Compose: Aktuelle Image-Version notieren
docker compose images
```

**Paket auf eine ältere Version zurücksetzen:**
```bash
# Verfügbare Versionen anzeigen
apt list --all-versions nginx

# Ältere Version installieren
sudo apt install nginx=1.24.0-1ubuntu1
```

**Docker: Auf altes Image-Tag zurückrollen:**
```bash
# In docker-compose.yml die Versionsnummer ändern:
image: nginx:1.24.0

# Dann neu starten:
docker compose up -d
```

**Systemd-Snapshot vor kritischen Updates (optional, fortgeschritten):**
```bash
sudo systemctl snapshot pre-update.snapshot
```

## 7. Staging vs. Produktion

**Niemals kritische Updates direkt auf dem Produktionssystem testen.**

```
Staging-Server          →      Produktion
(Testumgebung)                 (Live-System)

1. Update einspielen           3. Update einspielen
2. Testen                         (wenn Staging OK)
```

**Minimaler Staging-Ansatz für kleine Self-Hoster:**

1. Lokale VM mit identischer Konfiguration (z.B. mit VirtualBox oder Multipass)
2. Update dort testen
3. Erst dann auf dem echten Server einspielen

**Multipass für schnelle lokale Ubuntu-VM:**
```bash
# VM erstellen
multipass launch --name staging

# Shell öffnen
multipass shell staging

# VM aufräumen
multipass delete staging && multipass purge
```

## 8. Automatische Sicherheitsupdates

Für Sicherheits-Patches kannst du automatische Updates aktivieren. Nur für Sicherheitsupdates empfohlen, nicht für alle Updates.

**Unattended-Upgrades installieren und konfigurieren:**
```bash
sudo apt install unattended-upgrades
sudo dpkg-reconfigure --priority=low unattended-upgrades
```

**Konfigurationsdatei:**
```bash
sudo nano /etc/apt/apt.conf.d/50unattended-upgrades
```

**Wichtige Einstellungen:**
```
// Nur Sicherheitsupdates automatisch einspielen
Unattended-Upgrade::Allowed-Origins {
    "${distro_id}:${distro_codename}-security";
};

// Per E-Mail benachrichtigen
Unattended-Upgrade::Mail "admin@example.com";

// Automatisch neu starten wenn nötig (Kernel-Updates)
Unattended-Upgrade::Automatic-Reboot "false";
```

**Status prüfen:**
```bash
sudo systemctl status unattended-upgrades
cat /var/log/unattended-upgrades/unattended-upgrades.log
```

## 9. Ubuntu-Version-Upgrade (LTS → LTS)

Ubuntu [LTS](../glossar.md#lts-long-term-support)-Versionen werden 5 Jahre unterstützt. Irgendwann musst du von einer LTS-Version zur nächsten wechseln (z.B. 22.04 → 24.04).

**Voraussetzungen:**
```bash
# 1. System vollständig aktuell halten
sudo apt update && sudo apt full-upgrade

# 2. update-manager-core installieren
sudo apt install update-manager-core

# 3. Backup aller Daten (!)
```

**Upgrade starten:**
```bash
sudo do-release-upgrade
```

**Wichtig:** Führe diesen Befehl nur über eine physische Konsole oder tmux/screen aus – eine unterbrochene SSH-Verbindung kann das System beschädigen.

```bash
# tmux installieren und starten
sudo apt install tmux
tmux new -s upgrade
sudo do-release-upgrade
# Wenn SSH-Verbindung abbricht: tmux attach -t upgrade
```

## 10. Nützliche Befehle auf einen Blick

| Befehl | Was es macht |
|--------|--------------|
| `sudo apt update` | Paketlisten aktualisieren |
| `sudo apt upgrade` | Pakete aktualisieren |
| `sudo apt full-upgrade` | Vollständiges Upgrade inkl. Abhängigkeiten |
| `sudo apt autoremove` | Nicht mehr benötigte Pakete entfernen |
| `apt list --upgradable` | Zeige verfügbare Updates |
| `docker compose pull` | Docker-Images aktualisieren |
| `docker compose up -d` | Container mit neuen Images starten |
| `docker image prune -f` | Alte Images aufräumen |
| `apt changelog paket` | Changelog eines Pakets anzeigen |

## 11. Troubleshooting

**apt update schlägt fehl ("404 Not Found"):**
```bash
# Veraltete Paketquellen bereinigen
sudo apt update 2>&1 | grep "404"
# Betroffene Repository-Datei in /etc/apt/sources.list.d/ entfernen
```

**Paket ist "on hold" und wird nicht aktualisiert:**
```bash
# Alle gehaltenen Pakete anzeigen
apt-mark showhold

# Paket freigeben
sudo apt-mark unhold paketname
```

**Docker-Container startet nach Update nicht:**
```bash
# Logs des Containers prüfen
docker compose logs app

# Auf alte Version zurückrollen (Image-Tag in compose.yml ändern)
docker compose up -d
```

**Neustart notwendig nach Kernel-Update:**
```bash
# Prüfen ob Neustart nötig
cat /var/run/reboot-required
# Welche Pakete erfordern Neustart?
cat /var/run/reboot-required.pkgs
```

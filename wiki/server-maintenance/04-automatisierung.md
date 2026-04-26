# Automatisierung - Spickzettel

## 1. Warum Automatisierung?

Aufgaben wie Backups, Updates und Log-Bereinigung müssen regelmäßig ausgeführt werden. Wenn du das manuell machst, vergisst du es früher oder später.

**Windows-Analogie:** Die Windows-Aufgabenplanung führt Programme zu bestimmten Zeiten aus. Auf Linux gibt es dafür [`cron`](../glossar.md#cron--crontab) (klassisch) und [`systemd`](../glossar.md#systemd)-Timer (moderner Ansatz).

**Grundprinzip:**
```
Wiederkehrende Aufgabe → Skript schreiben → Automatisch planen → Fertig
```

## 2. Cron – der klassische Job-Planer

`cron` ist ein Daemon, der Befehle zu festgelegten Zeiten ausführt. Jeder Benutzer hat seine eigene crontab (cron-Tabelle).

**Crontab bearbeiten:**
```bash
crontab -e
```

**Crontab anzeigen:**
```bash
crontab -l
```

**Root-Crontab bearbeiten (für System-Jobs):**
```bash
sudo crontab -e
```

### Cron-Syntax

```
┌───────────── Minute       (0–59)
│ ┌─────────── Stunde       (0–23)
│ │ ┌───────── Tag           (1–31)
│ │ │ ┌─────── Monat         (1–12)
│ │ │ │ ┌───── Wochentag     (0–7, 0 und 7 = Sonntag)
│ │ │ │ │
* * * * * /pfad/zum/befehl
```

**Praktische Beispiele:**
```
# Jeden Tag um 3:00 Uhr morgens
0 3 * * * /usr/local/bin/backup.sh

# Jede Stunde zur vollen Stunde
0 * * * * /usr/local/bin/check.sh

# Jeden Montag um 4:30 Uhr
30 4 * * 1 /usr/local/bin/weekly-cleanup.sh

# Jeden ersten des Monats um 2:00 Uhr
0 2 1 * * /usr/local/bin/monthly-report.sh

# Alle 15 Minuten
*/15 * * * * /usr/local/bin/ping-check.sh

# Montag bis Freitag um 8:00 Uhr
0 8 * * 1-5 /usr/local/bin/workday-task.sh
```

**Sonderwerte:**
| Wert | Bedeutung |
|------|-----------|
| `@reboot` | Beim Systemstart |
| `@daily` | Einmal täglich (= `0 0 * * *`) |
| `@weekly` | Einmal wöchentlich |
| `@monthly` | Einmal monatlich |
| `@hourly` | Einmal stündlich |

**Beispiel mit @reboot:**
```
@reboot /usr/local/bin/startup-cleanup.sh
```

**Tipp:** Cron-Syntax online testen: [crontab.guru](https://crontab.guru)

### Ausgabe von Cron-Jobs

Cron-Jobs laufen ohne Terminal. Standardmäßig wird die Ausgabe per E-Mail an den lokalen Benutzer geschickt (oft unerwünscht). Besser: Ausgabe in eine Log-Datei umleiten.

```bash
# Ausgabe in Log-Datei schreiben (stdout und stderr)
0 3 * * * /usr/local/bin/backup.sh >> /var/log/backup.log 2>&1

# Ausgabe komplett verwerfen (kein Logging)
0 3 * * * /usr/local/bin/backup.sh > /dev/null 2>&1
```

**Cron-Log prüfen:**
```bash
grep CRON /var/log/syslog | tail -20
```

## 3. Shell-Skripte für Wartungsaufgaben

Gute Skripte folgen einigen einfachen Grundprinzipien.

### Grundstruktur eines Wartungsskripts

```bash
#!/bin/bash
# Beschreibung: Was macht dieses Skript?
# Autor: Dein Name
# Datum: 2024-01-15

# Bei Fehler abbrechen, undefinierte Variablen als Fehler behandeln
set -euo pipefail

# Konfiguration
BACKUP_DIR="/mnt/backup"
LOG_FILE="/var/log/mein-skript.log"
DATUM=$(date '+%Y-%m-%d %H:%M:%S')

# Log-Funktion
log() {
    echo "[$DATUM] $1" | tee -a "$LOG_FILE"
}

log "Skript gestartet"

# Aufgaben ausführen
log "Schritt 1: Backup starten"
rsync -av /var/data/ "$BACKUP_DIR/data/" >> "$LOG_FILE" 2>&1

log "Schritt 2: Alte Logs aufräumen"
find /var/log/ -name "*.gz" -mtime +30 -delete

log "Skript abgeschlossen"
```

**`set -euo pipefail` erklärt:**
| Option | Bedeutung |
|--------|-----------|
| `-e` | Skript bricht bei Fehler ab |
| `-u` | Fehler bei undefinierten Variablen |
| `-o pipefail` | Fehler in Pipes werden erkannt |

### Beispiel: Backup-Skript mit Fehlerbehandlung

```bash
#!/bin/bash
set -euo pipefail

BACKUP_DIR="/mnt/backup"
DB_NAME="meine_datenbank"
LOG_FILE="/var/log/backup.log"
MAX_ALTER_TAGE=30

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

fehler_behandlung() {
    log "FEHLER: Skript in Zeile $1 fehlgeschlagen"
    # Hier könnte eine Benachrichtigung gesendet werden
    exit 1
}

trap 'fehler_behandlung $LINENO' ERR

log "=== Backup gestartet ==="

# Verzeichnis erstellen
HEUTE=$(date +%Y-%m-%d)
mkdir -p "$BACKUP_DIR/$HEUTE"

# Dateien sichern
log "Sichere /var/www..."
rsync -av /var/www/ "$BACKUP_DIR/$HEUTE/www/" >> "$LOG_FILE" 2>&1

# Datenbank sichern
log "Sichere PostgreSQL-Datenbank '$DB_NAME'..."
sudo -u postgres pg_dump "$DB_NAME" | gzip > "$BACKUP_DIR/$HEUTE/db.sql.gz"

# Alte Backups löschen
log "Lösche Backups älter als $MAX_ALTER_TAGE Tage..."
find "$BACKUP_DIR" -maxdepth 1 -type d -mtime +"$MAX_ALTER_TAGE" -exec rm -rf {} + || true

log "=== Backup erfolgreich abgeschlossen ==="
```

**Skript ausführbar machen:**
```bash
chmod +x /usr/local/bin/backup.sh
```

**Skript testen:**
```bash
sudo bash -x /usr/local/bin/backup.sh
```

(`-x` zeigt jeden ausgeführten Befehl – hilfreich beim Debugging)

## 4. systemd-Timer als Alternative zu cron

systemd-Timer sind die modernere Alternative zu cron unter Ubuntu. Sie haben einige Vorteile:
- Ausgabe landet automatisch in [journald](../glossar.md#journald--journalctl) (kein separates Logging nötig)
- Timing ist robuster (z.B. "5 Minuten nach dem Systemstart")
- Einfacher zu überwachen und zu debuggen

**Ein systemd-Timer besteht aus zwei Dateien:**

### Schritt 1: Service-Datei erstellen

```bash
sudo nano /etc/systemd/system/backup.service
```

```ini
[Unit]
Description=Tägliches Backup
After=network.target

[Service]
Type=oneshot
ExecStart=/usr/local/bin/backup.sh
User=root

[Install]
WantedBy=multi-user.target
```

### Schritt 2: Timer-Datei erstellen

```bash
sudo nano /etc/systemd/system/backup.timer
```

```ini
[Unit]
Description=Backup täglich um 3:00 Uhr
Requires=backup.service

[Timer]
OnCalendar=*-*-* 03:00:00
Persistent=true

[Install]
WantedBy=timers.target
```

**`Persistent=true`** bedeutet: Wenn der Server um 3 Uhr aus war und um 4 Uhr hochfährt, wird der Job nachgeholt.

### Schritt 3: Timer aktivieren

```bash
sudo systemctl daemon-reload
sudo systemctl enable backup.timer
sudo systemctl start backup.timer
```

**Timer-Status prüfen:**
```bash
sudo systemctl status backup.timer
sudo systemctl list-timers
```

**Service manuell auslösen (zum Testen):**
```bash
sudo systemctl start backup.service
sudo journalctl -u backup.service -f
```

### Timer-Zeitangaben (OnCalendar)

```
*-*-* 03:00:00       # Täglich um 3:00 Uhr
Mon *-*-* 04:00:00   # Montags um 4:00 Uhr
*-*-1 02:00:00       # Ersten des Monats um 2:00 Uhr
hourly               # Jede Stunde
daily                # Täglich
weekly               # Wöchentlich
monthly              # Monatlich
```

**Zeitangabe validieren:**
```bash
systemd-analyze calendar "Mon *-*-* 04:00:00"
```

## 5. Benachrichtigungen bei Fehlern

Ein Skript, das nachts fehlschlägt, muss trotzdem auffallen. Es gibt verschiedene Möglichkeiten, Benachrichtigungen zu versenden.

### Option A: E-Mail mit msmtp (einfach)

`msmtp` ist ein einfacher SMTP-Client zum Versenden von E-Mails aus Skripten.

**Installation:**
```bash
sudo apt install msmtp msmtp-mta
```

**Konfiguration (`/etc/msmtprc`):**
```
defaults
auth           on
tls            on
tls_trust_file /etc/ssl/certs/ca-certificates.crt
logfile        /var/log/msmtp.log

account        mein-smtp
host           smtp.example.com
port           587
from           server@example.com
user           server@example.com
password       mein-passwort

account default : mein-smtp
```

**Berechtigungen setzen:**
```bash
sudo chmod 600 /etc/msmtprc
```

**Benachrichtigung im Skript versenden:**
```bash
#!/bin/bash
set -euo pipefail

LOG_FILE="/var/log/backup.log"
EMPFAENGER="admin@example.com"

benachrichtige_fehler() {
    local zeile=$1
    local nachricht="FEHLER: Backup-Skript auf $(hostname) in Zeile $zeile fehlgeschlagen."
    echo "$nachricht" | msmtp "$EMPFAENGER"
    echo "[$(date)] $nachricht" >> "$LOG_FILE"
    exit 1
}

trap 'benachrichtige_fehler $LINENO' ERR

# ... Rest des Skripts
```

### Option B: Healthchecks.io (empfohlen für Anfänger)

[Healthchecks.io](https://healthchecks.io) ist ein Dienst der überwacht, ob dein Cron-Job regelmäßig läuft. Wenn er ausbleibt, bekommst du eine E-Mail – ohne eigene E-Mail-Konfiguration.

**Prinzip:**
```
Cron-Job läuft → ping an healthchecks.io → "Alles gut"
Cron-Job bleibt aus → healthchecks.io → E-Mail-Benachrichtigung
```

**Integration im Skript:**
```bash
#!/bin/bash
set -euo pipefail

HEALTHCHECK_URL="https://hc-ping.com/deine-uuid-hier"

# Bei Fehler: Healthcheck mit Fehler-Signal anpingen
trap 'curl -fsS "${HEALTHCHECK_URL}/fail" > /dev/null' ERR

# Aufgaben ausführen
rsync -av /var/data/ /mnt/backup/data/

# Erfolgreich: Healthcheck anpingen
curl -fsS "$HEALTHCHECK_URL" > /dev/null
```

### Option C: systemd-Dienst bei Fehler benachrichtigen

```ini
# /etc/systemd/system/backup.service

[Unit]
Description=Tägliches Backup
OnFailure=backup-fehler@%n.service

[Service]
Type=oneshot
ExecStart=/usr/local/bin/backup.sh
```

```ini
# /etc/systemd/system/backup-fehler@.service

[Unit]
Description=Backup-Fehlerbenachrichtigung

[Service]
Type=oneshot
ExecStart=/bin/bash -c 'echo "Backup fehlgeschlagen auf %H" | msmtp admin@example.com'
```

## 6. Nützliche Automatisierungs-Patterns

### Log-Rotation verwalten

`logrotate` rotiert, komprimiert und löscht alte Log-Dateien automatisch.

**Konfiguration für eigene Logs:**
```bash
sudo nano /etc/logrotate.d/meine-app
```

```
/var/log/backup.log {
    daily
    rotate 30
    compress
    missingok
    notifempty
    create 0640 root root
}
```

**Manuell testen:**
```bash
sudo logrotate --debug /etc/logrotate.d/meine-app
```

### Docker-Housekeeping

```bash
#!/bin/bash
# /usr/local/bin/docker-cleanup.sh
# Entfernt ungenutzte Docker-Ressourcen

echo "[$(date)] Docker-Cleanup gestartet"

# Gestoppte Container entfernen
docker container prune -f

# Ungetaggte Images entfernen
docker image prune -f

# Ungenutzte Volumes entfernen (Vorsicht: nur wirklich ungenutzte!)
docker volume prune -f

# Netzwerke aufräumen
docker network prune -f

echo "[$(date)] Docker-Cleanup abgeschlossen"
echo "Freier Speicher: $(df -h / | awk 'NR==2 {print $4}')"
```

**Wöchentlich per Cron ausführen:**
```
0 4 * * 0 /usr/local/bin/docker-cleanup.sh >> /var/log/docker-cleanup.log 2>&1
```

## 7. Nützliche Befehle auf einen Blick

| Befehl | Was es macht |
|--------|--------------|
| `crontab -e` | Cron-Jobs des aktuellen Benutzers bearbeiten |
| `crontab -l` | Cron-Jobs anzeigen |
| `sudo crontab -e` | Cron-Jobs von root bearbeiten |
| `grep CRON /var/log/syslog` | Cron-Aktivität im Log prüfen |
| `systemctl list-timers` | Alle systemd-Timer anzeigen |
| `systemctl status backup.timer` | Status eines Timers |
| `systemctl start backup.service` | Service manuell ausführen |
| `journalctl -u backup.service` | Logs eines systemd-Services |
| `bash -x skript.sh` | Skript mit Debug-Ausgabe ausführen |

## 8. Troubleshooting

**Cron-Job läuft nicht:**
```bash
# 1. Cron-Log prüfen
grep CRON /var/log/syslog | grep -i "CMD\|error" | tail -20

# 2. Ist der Cron-Daemon aktiv?
sudo systemctl status cron

# 3. Skript manuell als root testen
sudo /usr/local/bin/mein-skript.sh

# 4. Cron-Job mit absolutem Pfad angeben (cron hat eingeschränkte PATH-Variable)
# Falsch:
0 3 * * * backup.sh
# Richtig:
0 3 * * * /usr/local/bin/backup.sh
```

**systemd-Timer läuft nicht:**
```bash
# Timer-Status prüfen
sudo systemctl status backup.timer

# Wann wird er das nächste Mal ausgeführt?
sudo systemctl list-timers backup.timer

# Service-Log prüfen
sudo journalctl -u backup.service --since "24 hours ago"

# Service-Datei auf Fehler prüfen
sudo systemd-analyze verify backup.service
```

**Skript bricht unerwartet ab:**
```bash
# Mit set -x debuggen (zeigt jeden Befehl)
bash -x /usr/local/bin/mein-skript.sh 2>&1 | head -50

# Exitcode des letzten Befehls prüfen
echo $?   # 0 = Erfolg, alles andere = Fehler
```

**PATH-Probleme in Cron-Jobs:**

Cron hat eine sehr eingeschränkte PATH-Variable. Wenn ein Skript im Terminal läuft, aber nicht in Cron, liegt es oft daran.

```bash
# Am Anfang der crontab eintragen:
PATH=/usr/local/sbin:/usr/local/bin:/sbin:/bin:/usr/sbin:/usr/bin

# Oder im Skript absolute Pfade verwenden:
/usr/bin/rsync statt rsync
/usr/bin/docker statt docker
```

# Immich: Installation, Backup und Restore

Diese Anleitung geht davon aus, dass der Server/die VM beriets Docker Compose und wget installiert hat.

## 1. Installation

### Verzeichnis anlegen

```bash
mkdir -p /opt/immich && cd /opt/immich
```

### docker-compose.yml und .env herunterladen

```bash
wget -O docker-compose.yml https://github.com/immich-app/immich/releases/latest/download/docker-compose.yml
wget -O .env https://github.com/immich-app/immich/releases/latest/download/example.env
```

### .env anpassen

```bash
nano .env
```

Mindestens diese Werte setzen:

```env
UPLOAD_LOCATION=/opt/immich/data
DB_DATA_LOCATION=/opt/immich/postgres
DB_PASSWORD=<sicheres-passwort>
```

`UPLOAD_LOCATION` auf das grösste Laufwerk legen. Keine Docker-managed Volumes verwenden, direkte Pfade sind einfacher zu sichern.

### Starten

```bash
docker compose up -d
docker compose ps
```

Immich ist erreichbar unter `http://<server-ip>:2283`.

## 2. Automatische Datenbank-Backups konfigurieren

Immich erstellt täglich Datenbank-Dumps als `.sql.gz` nach `UPLOAD_LOCATION/backups/`. Manuelles `pg_dump` ist nicht nötig.

Einstellung prüfen oder anpassen: **Administration / Settings / Backup**

Standard: täglich 02:00 Uhr, letzte 14 Dumps aufbewahren. Die Dumps enthalten nur Metadaten, keine Fotos oder Videos.

## 3. Verschlüsselte Backups mit rclone

### Warum Verschlüsselung auf dem Zielserver?

rsync über SSH verschlüsselt nur den Transport. Auf dem Zielserver liegen die Dateien im Klartext. Günstiger Speicher bedeutet in der Regel weniger Sicherheit auf Serverseite.

rclone crypt verschlüsselt die Dateien lokal, bevor sie übertragen werden. Der Zielserver speichert nur Chiffretext, auch die Dateinamen werden verschlüsselt. Ohne das Passwort sind die Daten wertlos, selbst mit root-Zugriff auf dem Backup-Server.

rclone crypt wurde hier gewählt weil borg und restic proprietäre Formate verwenden, die ein passendes Tool auf dem Zielserver erfordern. rclone legt reguläre Dateien ab, benötigt serverseitig nichts ausser SFTP, und der Delta-Sync funktioniert weiterhin.

### rclone installieren

```bash
apt install rclone
```

### rclone konfigurieren

```bash
rclone config
```

Zwei Remotes anlegen.

**Remote 1: SFTP-Verbindung zum Backup-Server**

```
n
Name: backup-sftp
Type: sftp
Host: backup-server
User: user
Key file: /root/.ssh/immich_backup
```

**Remote 2: Verschlüsselungs-Layer darüber**

```
n
Name: backup-crypt
Type: crypt
Remote: backup-sftp:/backups/immich
Filename encryption: standard
Password: <langes-zufälliges-passwort>
Password2 (salt): <zweites-zufälliges-passwort>
```

Die Passwörter separat aufbewahren, zum Beispiel in einem Passwort-Manager oder ausgedruckt. Wer die Passwörter verliert, verliert den Zugang zu allen Backups.

Die rclone-Konfiguration liegt unter `/root/.config/rclone/rclone.conf`. Diese Datei enthält die Passwörter in obfuskierter Form und muss ebenfalls gesichert werden.

```bash
chmod 600 /root/.config/rclone/rclone.conf
```

### SSH-Key einrichten

```bash
ssh-keygen -t ed25519 -C "immich-backup" -f /root/.ssh/immich_backup
ssh-copy-id -i /root/.ssh/immich_backup.pub user@backup-server
```

Kein Passwort auf dem Key setzen, Cron-Jobs können keine Passwörter eingeben. Einmalig manuell verbinden, um den Host-Fingerprint zu bestätigen:

```bash
ssh -i /root/.ssh/immich_backup user@backup-server
```

## 4. Backup-Strategie: täglich und monatlich

Täliche Backups werden mit `--delete` synchronisiert. Wenn lokal etwas gelöscht wird, verschwindet es auch im Backup. Das monatliche Backup ist ein unveränderlicher Snapshot für den Fall, dass eine versehentliche Löschung erst Wochen später auffällt.

```
backup-crypt:daily/      täglich aktualisiert, rollierend
backup-crypt:monthly/    Snapshot vom 1. jeden Monats
```

### Docker Images und ML Model Cache sichern

Versionstags sind nicht stabil. Es gibt keine Garantie, dass ein bestimmtes Image zu einem späteren Zeitpunkt noch bit-für-bit identisch aus der Registry geladen werden kann. Für eine saubere Wiederherstellung ohne Kompatibilitätsprobleme müssen die Images lokal gespeichert werden.

Das gilt auch für den ML Model Cache. Die Modelle werden von externen Quellen geladen und können sich ändern oder nicht mehr verfügbar sein.

```bash
nano /opt/immich/export-images.sh
chmod 700 /opt/immich/export-images.sh
```

```bash
#!/bin/bash
# Docker Images und ML Model Cache exportieren
# Wird vor dem eigentlichen Backup ausgeführt

set -euo pipefail

EXPORT_DIR="/opt/immich/docker-export"
mkdir -p "$EXPORT_DIR"

# Alle laufenden Immich-Images ermitteln und exportieren
docker images --format '{{.Repository}}:{{.Tag}}' \
  | grep -E 'immich|valkey|postgres' \
  | sort -u \
  | xargs docker save \
  | gzip > "$EXPORT_DIR/immich-images.tar.gz"

# ML Model Cache aus dem Docker Volume exportieren
docker run --rm \
  -v immich_model-cache:/model-cache:ro \
  -v "$EXPORT_DIR":/export \
  alpine sh -c "tar czf /export/model-cache.tar.gz -C /model-cache ."
```

### Backup-Skript

```bash
nano /opt/immich/backup.sh
chmod 700 /opt/immich/backup.sh
```

```bash
#!/bin/bash
# Immich Backup
#
# Immich erstellt täglich um 02:00 einen DB-Dump nach UPLOAD_LOCATION/backups/.
# Dieses Skript läuft um 02:05. Die DB wird zuerst gesichert, Assets danach.
# Im schlechtesten Fall sind mehr Assets vorhanden als die DB kennt, das ist harmlos.
# Umgekehrt würde die DB auf fehlende Dateien zeigen.
#
# Am 1. jeden Monats wird zusätzlich ein monatlicher Snapshot erstellt.

set -euo pipefail

UPLOAD_LOCATION="/opt/immich/data"
EXPORT_DIR="/opt/immich/docker-export"
LOG="/var/log/immich-backup.log"
MONTHLY_MARKER=$(date +%d)

exec >> "$LOG" 2>&1
echo "=== Backup gestartet: $(date) ==="

# Docker Images und Model Cache exportieren
/opt/immich/export-images.sh

backup_path() {
  local src="$1"
  local dest_name="$2"

  rclone sync "$src" "backup-crypt:daily/$dest_name" \
    --transfers=4 \
    --checkers=8 \
    --log-level INFO

  if [ "$MONTHLY_MARKER" = "01" ]; then
    echo "Monatlicher Snapshot: $dest_name"
    rclone copy "$src" "backup-crypt:monthly/$(date +%Y-%m)/$dest_name" \
      --transfers=4 \
      --checkers=8 \
      --log-level INFO
  fi
}

# 1. Datenbank-Dumps (zuerst)
backup_path "$UPLOAD_LOCATION/backups" "backups"

# 2. Assets
backup_path "$UPLOAD_LOCATION/library" "library"
backup_path "$UPLOAD_LOCATION/upload"  "upload"
backup_path "$UPLOAD_LOCATION/profile" "profile"

# 3. Docker Images und Model Cache
backup_path "$EXPORT_DIR" "docker"

echo "=== Backup abgeschlossen: $(date) ==="
```

`rclone sync` macht die Zielseite identisch mit der Quelle. `rclone copy` überträgt nur, löscht nichts, deshalb für monatliche Snapshots. In der Praxis kann man das gerne anders machen, aber ich wollte einmal beides beispielhaft zeigen.

### Cron einrichten

```bash
crontab -e
```

```cron
5 2 * * * /opt/immich/backup.sh
0 3 2 * * /opt/immich/cleanup-monthly.sh
```

### Monatliche Backups aufräumen

```bash
nano /opt/immich/cleanup-monthly.sh
chmod 700 /opt/immich/cleanup-monthly.sh
```

```bash
#!/bin/bash
# Monatliche Snapshots älter als 12 Monate löschen

set -euo pipefail
KEEP=12

rclone lsf backup-crypt:monthly/ --dirs-only \
  | sort \
  | head -n -$KEEP \
  | while read -r dir; do
      echo "Lösche alten Snapshot: $dir"
      rclone purge "backup-crypt:monthly/$dir"
    done
```

## 5. Restore

Ein Restore besteht aus dem DB-Dump und den Asset-Dateien vom selben Zeitpunkt. Da die Dateien kurz nach dem Dump gesichert werden, können minimal mehr Assets vorhanden sein als die DB kennt. Das ist harmlos, Immich ignoriert unbekannte Dateien.

### Schritt 1: Immich stoppen

```bash
cd /opt/immich && docker compose down
```

### Schritt 2: Quelle wählen

Vom täglichen Backup:

```bash
RESTORE_SRC="backup-crypt:daily"
```

Vom monatlichen Backup, zum Beispiel Januar 2025:

```bash
RESTORE_SRC="backup-crypt:monthly/2025-01"
```

### Schritt 3: Dateien zurückspielen

rclone entschlüsselt beim Lesen automatisch. Auf dem lokalen Server landen die Dateien wieder im Klartext.

```bash
rclone sync "$RESTORE_SRC/backups"  /opt/immich/data/backups/
rclone sync "$RESTORE_SRC/library"  /opt/immich/data/library/
rclone sync "$RESTORE_SRC/upload"   /opt/immich/data/upload/
rclone sync "$RESTORE_SRC/profile"  /opt/immich/data/profile/
```

### Schritt 4: Docker Images wiederherstellen (falls nötig)

```bash
rclone copy "$RESTORE_SRC/docker" /opt/immich/docker-export/
docker load < /opt/immich/docker-export/immich-images.tar.gz
```

ML Model Cache wiederherstellen:

```bash
docker run --rm \
  -v immich_model-cache:/model-cache \
  -v /opt/immich/docker-export:/import:ro \
  alpine sh -c "tar xzf /import/model-cache.tar.gz -C /model-cache"
```

### Schritt 5: DB-Dump einspielen über Immich Web-UI

```bash
docker compose up -d
```

Im Browser: **Administration / Maintenance / Restore database backup**

Den passenden Dump auswählen und auf Restore klicken. Immich erstellt automatisch einen Restore-Punkt vor der Operation.

### Schritt 6: Thumbnails und Videos neu generieren

**Administration / Job Queues**

- Generate Thumbnails: Run all
- Transcode Videos: Run all (optional)

## Backup-Umfang

| Inhalt                                | Gesichert | Über    | Grund                                          |
|---------------------------------------|-----------|---------|------------------------------------------------|
| DB-Dumps (.sql.gz)                    | ja        | rclone  | Essenziell, enthält alle Metadaten             |
| Originalfotos und -videos             | ja        | rclone  | Nicht reproduzierbar                           |
| Profilbilder                          | ja        | rclone  | Nicht reproduzierbar                           |
| Docker Images                         | ja        | rclone  | Versionstags sind nicht stabil                 |
| ML Model Cache                        | ja        | rclone  | Externe Quelle, keine Stabilitätsgarantie      |
| Thumbnails (thumbs/)                  | nein      |         | Werden automatisch neu generiert               |
| Transcodierte Videos (encoded-video/) | nein      |         | Werden automatisch neu generiert               |
| Konfigdateien                         | ja        | git     | Git ermöglicht Versionierung. Achtung: Secrets |
| Secrets                               | ja        | separat | z.B. in Passwortmanager. nicht in git/rclone   |

# SSH - Spickzettel

## 1. Was ist SSH?

[SSH](../glossar.md#ssh-secure-shell) (Secure Shell) ist ein Protokoll, das eine sichere, verschlüsselte Verbindung zu einem anderen Computer über das Netzwerk herstellt. Mit SSH kannst du auf deinem Server arbeiten, als ob du direkt davor sitzen würdest.

**Windows-Analogie:** Wie Remote Desktop (RDP), aber nur mit Textkonsole – und deutlich sicherer und leichter automatisierbar. Moderne Windows-Versionen haben ebenfalls einen SSH-Client eingebaut.

SSH läuft standardmäßig auf **Port 22**.

**Grundlegende Verbindung:**
```bash
ssh benutzername@server-ip-oder-domain
# Beispiel:
ssh alice@93.184.216.34
ssh alice@meinserver.de
```

## 2. SSH-Schlüsselpaar erstellen (ssh-keygen)

SSH kann auf zwei Arten authentifizieren: Passwort (unsicher) oder Schlüsselpaar (sicher). Ein Schlüsselpaar besteht aus:
- **Privater Schlüssel** (`~/.ssh/id_ed25519`): Bleibt immer auf deinem Rechner, niemals teilen!
- **Öffentlicher Schlüssel** (`~/.ssh/id_ed25519.pub`): Wird auf dem Server hinterlegt.

**Analogie:** Der öffentliche Schlüssel ist das Schloss, der private Schlüssel ist der Schlüssel dazu. Das Schloss kannst du überall verteilen – aber den Schlüssel behältst du bei dir.

```bash
# Modernes ed25519-Schlüsselpaar erstellen (empfohlen)
ssh-keygen -t ed25519 -C "alice@meinrechner"

# Du wirst gefragt:
# - Wo speichern? (Standard: ~/.ssh/id_ed25519 – einfach Enter drücken)
# - Passphrase? (empfohlen: Passphrase eingeben für extra Schutz)
```

**Schlüsseldateien anzeigen:**
```bash
ls -la ~/.ssh/
# id_ed25519      ← privater Schlüssel (chmod 600, nur du darfst lesen!)
# id_ed25519.pub  ← öffentlicher Schlüssel (kann geteilt werden)
```

**Öffentlichen Schlüssel anzeigen:**
```bash
cat ~/.ssh/id_ed25519.pub
# Ausgabe: ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAA... alice@meinrechner
```

## 3. Öffentlichen Schlüssel auf Server kopieren (ssh-copy-id)

```bash
# Schlüssel auf Server kopieren (einmalig, mit Passwort)
ssh-copy-id -i ~/.ssh/id_ed25519.pub alice@meinserver.de

# Danach: Anmeldung ohne Passwort möglich
ssh alice@meinserver.de
```

**Manuell (falls ssh-copy-id nicht verfügbar):**
```bash
# Auf dem Server:
mkdir -p ~/.ssh
chmod 700 ~/.ssh
nano ~/.ssh/authorized_keys
# Inhalt von id_ed25519.pub hineinkopieren
chmod 600 ~/.ssh/authorized_keys
```

## 4. SSH-Konfigurationsdatei (~/.ssh/config)

Statt immer `ssh alice@93.184.216.34 -p 2222` zu tippen, kannst du Abkürzungen definieren:

```bash
nano ~/.ssh/config
```

```
# Eintrag für einen Server
Host meinserver
    HostName 93.184.216.34
    User alice
    Port 22
    IdentityFile ~/.ssh/id_ed25519

# Eintrag für einen zweiten Server
Host produktiv
    HostName server2.example.com
    User deploy
    Port 2222
    IdentityFile ~/.ssh/id_ed25519_produktiv
```

**Danach einfach:**
```bash
ssh meinserver          # Statt: ssh alice@93.184.216.34
ssh produktiv           # Statt: ssh deploy@server2.example.com -p 2222
```

**Berechtigungen der Config-Datei setzen:**
```bash
chmod 600 ~/.ssh/config
```

## 5. Dateien übertragen mit scp und rsync

### scp (Secure Copy) – einfach, für einzelne Dateien

```bash
# Datei vom lokalen Rechner auf Server kopieren
scp lokale-datei.txt alice@meinserver.de:/home/alice/

# Datei vom Server auf lokalen Rechner kopieren
scp alice@meinserver.de:/home/alice/logfile.txt ./

# Verzeichnis kopieren (rekursiv mit -r)
scp -r ./mein-ordner alice@meinserver.de:/home/alice/

# Mit SSH-Config-Alias
scp datei.txt meinserver:/home/alice/
```

### rsync – mächtig, für Synchronisation und Backups

rsync überträgt nur Änderungen (Deltas) – ideal für Backups und große Verzeichnisse.

```bash
# Verzeichnis auf Server synchronisieren
rsync -avz ./mein-projekt/ alice@meinserver.de:/var/www/mein-projekt/

# Vom Server auf lokalen Rechner synchronisieren
rsync -avz alice@meinserver.de:/var/www/backup/ ./lokales-backup/

# Dry-run: erst prüfen, was passieren würde
rsync -avz --dry-run ./ordner/ alice@meinserver.de:/ziel/
```

**Flags bei rsync:**
| Flag | Bedeutung |
|------|-----------|
| `-a` | Archivmodus (Berechtigungen, Zeiten erhalten) |
| `-v` | Ausführliche Ausgabe (verbose) |
| `-z` | Komprimierung während der Übertragung |
| `--delete` | Auf Ziel löschen, was in Quelle nicht mehr vorhanden ist |
| `--dry-run` | Simulation ohne tatsächliche Änderungen |

## 6. Port-Weiterleitung (SSH Tunneling)

SSH kann Netzwerkverbindungen durch den verschlüsselten Tunnel leiten.

### Lokale Port-Weiterleitung

"Ich möchte einen Dienst auf dem Server lokal auf meinem Rechner nutzen."

```bash
# Beispiel: PostgreSQL auf dem Server (Port 5432) lokal als Port 15432 erreichbar
ssh -L 15432:localhost:5432 alice@meinserver.de

# Danach lokal verbinden:
psql -h localhost -p 15432 -U dbuser mydatabase
```

```
Dein Rechner:15432  →  SSH-Tunnel  →  meinserver:5432
```

### Remote Port-Weiterleitung

"Ich möchte meinen lokalen Dienst vorübergehend über den Server zugänglich machen."

```bash
# Lokalen Port 3000 auf dem Server als Port 8080 erreichbar machen
ssh -R 8080:localhost:3000 alice@meinserver.de
```

## 7. SSH-Härtung (Sicherheitstipps)

Diese Einstellungen machst du in `/etc/ssh/sshd_config` auf dem Server:

```bash
sudo nano /etc/ssh/sshd_config
```

**Wichtige Einstellungen:**
```
# Passwort-Login deaktivieren (nur Schlüssel erlauben)
PasswordAuthentication no

# Root-Login deaktivieren
PermitRootLogin no

# Nur bestimmte Benutzer erlauben
AllowUsers alice deploy

# Maximale Login-Versuche begrenzen
MaxAuthTries 3

# Inaktive Sitzungen trennen
ClientAliveInterval 300
ClientAliveCountMax 2
```

**Nach Änderungen SSH neu starten:**
```bash
sudo systemctl restart sshd
```

**Wichtig:** Immer erst in einer zweiten SSH-Sitzung testen, bevor du die aktuelle schließt!

### SSH-Port ändern (optional)

Den Standard-Port 22 zu ändern reduziert automatisierte Angriffe (Security through Obscurity – kein vollständiger Schutz, aber weniger Lärm in den Logs):

```
# In /etc/ssh/sshd_config:
Port 2222
```

```bash
# Verbindung dann mit Port-Angabe:
ssh -p 2222 alice@meinserver.de

# Oder in ~/.ssh/config:
Host meinserver
    Port 2222
```

### Fail2ban installieren (Brute-Force-Schutz)

```bash
sudo apt install fail2ban
sudo systemctl enable --now fail2ban
```

[Fail2Ban](../glossar.md#fail2ban) sperrt IP-Adressen automatisch nach mehreren fehlgeschlagenen Login-Versuchen.

## 8. SSH-Agent (Passphrase nur einmal eingeben)

Wenn dein privater Schlüssel mit einer [Passphrase](../glossar.md#passphrase) geschützt ist, musst du sie nicht bei jeder Verbindung eingeben:

```bash
# SSH-Agent starten und Schlüssel hinzufügen
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519
# Passphrase einmal eingeben – gilt für diese Sitzung
```

## 9. Troubleshooting

**Verbindung schlägt fehl:**
```bash
# Verbose-Modus: zeigt genaue Fehlerursache
ssh -v alice@meinserver.de
ssh -vvv alice@meinserver.de   # Noch mehr Details
```

**"Permission denied (publickey)":**
```bash
# Berechtigungen auf dem Server prüfen
ls -la ~/.ssh/
# ~/.ssh muss 700 sein, authorized_keys muss 600 sein
chmod 700 ~/.ssh
chmod 600 ~/.ssh/authorized_keys
```

**SSH-Dienst auf Server prüfen:**
```bash
sudo systemctl status sshd
sudo journalctl -u sshd -n 50
```

## 10. Referenztabelle

| Befehl | Was es macht |
|--------|-------------|
| `ssh user@host` | Auf Server verbinden |
| `ssh-keygen -t ed25519` | Schlüsselpaar erstellen |
| `ssh-copy-id user@host` | Öffentlichen Schlüssel auf Server kopieren |
| `scp datei user@host:/pfad/` | Datei auf Server kopieren |
| `rsync -avz quelle/ user@host:/ziel/` | Verzeichnis synchronisieren |
| `ssh -L lokal:remote:port user@host` | Lokale Port-Weiterleitung |
| `ssh -v user@host` | Verbindung mit Debug-Ausgabe |
| `sudo systemctl restart sshd` | SSH-Dienst neu starten |

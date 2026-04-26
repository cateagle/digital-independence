# Root-Login per SSH verhindern - Spickzettel

## 1. Warum ist Root-Login per SSH gefährlich?

Der `root`-Benutzer existiert auf **jedem** Linux-System und hat **uneingeschränkte** Macht über den Server. Angreifer wissen das und versuchen sich genau mit diesem Benutzernamen einzuloggen.

Wenn [Root-Login](../glossar.md#root-login) per [SSH](../glossar.md#ssh-secure-shell) erlaubt ist, haben Angreifer es leichter:
- Der Benutzername ist bekannt: `root`
- Sie müssen nur noch das Passwort erraten
- Bei Erfolg haben sie sofort vollen Zugriff – kein weiterer Schritt nötig

**Windows-Analogie:** Das entspricht dem eingebauten `Administrator`-Konto unter Windows, das standardmäßig deaktiviert ist – aus genau diesem Grund.

Die Lösung: Root-Login per SSH komplett deaktivieren und stattdessen mit einem normalen Benutzer und `sudo` arbeiten.

## 2. Einen sudo-fähigen Benutzer einrichten

Bevor du Root-Login deaktivierst, stelle sicher, dass du einen normalen Benutzer mit `sudo`-Rechten hast.

Benutzer erstellen:
```bash
sudo adduser alice
```

Benutzer zur sudo-Gruppe hinzufügen:
```bash
sudo usermod -aG sudo alice
```

Testen – als neuer Benutzer einloggen und sudo prüfen:
```bash
su - alice
sudo whoami
```

Ausgabe sollte sein: `root`

Wenn das funktioniert, kannst du Root-Login sicher deaktivieren.

## 3. Root-Login in sshd_config deaktivieren

Die SSH-Konfigurationsdatei bearbeiten:

```bash
sudo nano /etc/ssh/sshd_config
```

Suche nach der Zeile mit `PermitRootLogin` und ändere sie:

```
# Vorher:
PermitRootLogin yes

# Nachher:
PermitRootLogin no
```

Falls die Zeile auskommentiert ist (beginnt mit `#`), entferne das `#` und setze sie auf `no`.

SSH-Dienst neu starten, damit die Änderung wirksam wird:
```bash
sudo systemctl restart ssh
```

> Wichtig: Teste die neue Konfiguration in einem **neuen** Terminal-Fenster, bevor du das aktuelle schließt. So kannst du zurück, falls etwas schiefläuft.

## 4. Passwort-Authentifizierung deaktivieren (nur SSH-Keys)

Passwörter können erraten oder durch Brute-Force geknackt werden. [SSH-Keys](../glossar.md#ssh-key-ssh-schlüsselpaar) sind deutlich sicherer: Sie bestehen aus einem mathematischen Schlüsselpaar, das praktisch nicht zu erraten ist.

**Windows-Analogie:** Ein SSH-Key ist wie ein physischer Türschlüssel – man braucht das Original, ein Passwort ist wie ein PIN, den man theoretisch erraten kann.

### Schritt 1: SSH-Key auf deinem lokalen Rechner erstellen

Führe diesen Befehl auf deinem **lokalen Computer** aus (nicht auf dem Server):

```bash
ssh-keygen -t ed25519 -C "mein-server-key"
```

Du wirst nach einem Speicherort gefragt (Standard ist `~/.ssh/id_ed25519`) und optional nach einer [Passphrase](../glossar.md#passphrase) (empfohlen!).

Zwei Dateien werden erstellt:
- `~/.ssh/id_ed25519` – der **private** Schlüssel (niemals teilen!)
- `~/.ssh/id_ed25519.pub` – der **öffentliche** Schlüssel (auf den Server kopieren)

### Schritt 2: Öffentlichen Key auf den Server kopieren

```bash
ssh-copy-id -i ~/.ssh/id_ed25519.pub alice@DEINE-SERVER-IP
```

Alternativ manuell:
```bash
# Auf dem Server als alice:
mkdir -p ~/.ssh
chmod 700 ~/.ssh
nano ~/.ssh/authorized_keys
# Den Inhalt der .pub-Datei einfügen
chmod 600 ~/.ssh/authorized_keys
```

### Schritt 3: Testen, ob Key-Login funktioniert

```bash
ssh alice@DEINE-SERVER-IP
```

Wenn du ohne Passwort (oder nur mit der Key-Passphrase) reinkommst, funktioniert es.

### Schritt 4: Passwort-Authentifizierung deaktivieren

Erst wenn der Key-Login funktioniert, Passwort-Login deaktivieren:

```bash
sudo nano /etc/ssh/sshd_config
```

Folgende Zeilen suchen und anpassen:

```
PasswordAuthentication no
PubkeyAuthentication yes
```

SSH neu starten:
```bash
sudo systemctl restart ssh
```

> Warnung: Wenn du deinen privaten Key verlierst und kein anderer Zugang besteht, bist du dauerhaft ausgesperrt. Bewahre eine Backup-Kopie sicher auf!

## 5. Alle sicherheitsrelevanten sshd_config-Einstellungen im Überblick

```
# Root-Login verbieten
PermitRootLogin no

# Nur Key-Authentifizierung erlauben
PasswordAuthentication no
PubkeyAuthentication yes

# Leere Passwörter verbieten
PermitEmptyPasswords no

# Maximale Anmeldeversuche begrenzen
MaxAuthTries 3
```

Nach jeder Änderung:
```bash
sudo systemctl restart ssh
```

Wenn das nicht funktioniert, dann einfach `systemctl restart sshd` ausprobieren. Das ist je nach System manchmal etwas unterschiedlich.

**Tipp:** Du kannst dich mehrfach gleichzeitig per ssh verbinden. Wenn du änderungen machst, dann schließe deine ssh Verbindung nicht bevor du in einen zweiten Terminal alles ausprobiert hast.

## 6. Konfiguration auf Syntaxfehler prüfen

Bevor du SSH neu startest, kannst du die Konfiguration testen:

```bash
sudo sshd -t
```

Wenn keine Ausgabe erscheint, ist die Konfiguration fehlerfrei.

## 7. Nützliche Befehle – Übersicht

| Befehl | Was er macht |
|--------|--------------|
| `sudo nano /etc/ssh/sshd_config` | SSH-Konfiguration bearbeiten |
| `sudo sshd -t` | Konfiguration auf Fehler prüfen |
| `sudo systemctl restart ssh` | SSH-Dienst neu starten |
| `sudo systemctl status ssh` | SSH-Status anzeigen |
| `ssh-keygen -t ed25519` | Neues SSH-Schlüsselpaar erstellen |
| `ssh-copy-id user@server` | Public Key auf Server kopieren |

## 8. Häufige Fehler

**Problem:** Du hast dich durch eine falsche sshd_config ausgesperrt.
**Lösung:** Nutze die Notfallkonsole deines Cloud-Anbieters. Bearbeite die Datei dort und starte SSH neu.

**Problem:** Key-Login funktioniert nicht.
**Lösung:** Prüfe Berechtigungen:
```bash
chmod 700 ~/.ssh
chmod 600 ~/.ssh/authorized_keys
```
Die Berechtigungen müssen exakt stimmen, sonst ignoriert SSH den Key.

**Problem:** Du hast den privaten Key verloren.
**Lösung:** Nur über Notfallkonsole möglich. Dort einen neuen Public Key in `~/.ssh/authorized_keys` eintragen.

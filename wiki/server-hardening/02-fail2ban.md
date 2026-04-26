# Fail2ban - Spickzettel

## 1. Was ist Fail2ban?

Wenn dein Server im Internet erreichbar ist, versuchen automatisierte Bots ständig, sich einzuloggen – oft Tausende Versuche pro Tag. Sie probieren einfach viele Passwörter durch (sogenannte [Brute-Force-Angriffe](../glossar.md#brute-force-angriff)).

Fail2ban beobachtet deine Log-Dateien und sperrt IP-Adressen automatisch, wenn sie zu oft falsche Passwörter eingeben.

**Windows-Analogie:** Windows sperrt ein Benutzerkonto nach zu vielen Fehlversuchen (Account Lockout Policy). Fail2ban macht dasselbe, aber sperrt die angreifende IP-Adresse in der [Firewall](../glossar.md#firewall) – nicht nur das Konto.

Fail2ban arbeitet dabei mit [UFW](../glossar.md#ufw-uncomplicated-firewall) zusammen: Es fügt automatisch Firewall-Regeln hinzu, um eine IP zu sperren, und entfernt sie wieder nach einer festgelegten Zeit.

## 2. Installation

```bash
sudo apt update
sudo apt install fail2ban
```

Dienst starten und dauerhaft aktivieren:
```bash
sudo systemctl start fail2ban
sudo systemctl enable fail2ban
```

Status prüfen:
```bash
sudo systemctl status fail2ban
```

## 3. Konfiguration: jail.local erstellen

Fail2ban wird über "Jails" konfiguriert – ein Jail überwacht einen bestimmten Dienst (z.B. SSH).

Die Hauptkonfiguration liegt in `/etc/fail2ban/jail.conf`, aber diese Datei solltest du **nicht direkt bearbeiten**. Sie wird bei Updates überschrieben. Erstelle stattdessen eine eigene Datei:

```bash
sudo cp /etc/fail2ban/jail.conf /etc/fail2ban/jail.local
sudo nano /etc/fail2ban/jail.local
```

Oder erstelle eine neue, leere Konfiguration (empfohlen für Anfänger):

```bash
sudo nano /etc/fail2ban/jail.local
```

## 4. SSH-Schutz konfigurieren

Füge folgendes in `/etc/fail2ban/jail.local` ein:

```ini
[DEFAULT]
# Wie lange eine IP gesperrt wird (in Sekunden)
# 3600 = 1 Stunde
bantime  = 3600

# Zeitfenster, in dem Fehlversuche gezählt werden (in Sekunden)
# 600 = 10 Minuten
findtime  = 600

# Wie viele Fehlversuche erlaubt sind, bevor die IP gesperrt wird
maxretry = 5

[sshd]
enabled = true
port    = ssh
logpath = %(sshd_log)s
backend = %(sshd_backend)s
```

Nach jeder Änderung Fail2ban neu starten:
```bash
sudo systemctl restart fail2ban
```

## 5. Aktive Jails und gesperrte IPs anzeigen

Übersicht aller aktiven Jails:
```bash
sudo fail2ban-client status
```

Ausgabe:
```
Status
|- Number of jail:	1
`- Jail list:	sshd
```

Details für den SSH-Jail (zeigt gesperrte IPs):
```bash
sudo fail2ban-client status sshd
```

Ausgabe:
```
Status for the jail: sshd
|- Filter
|  |- Currently failed:	2
|  |- Total failed:	47
|  `- File list:	/var/log/auth.log
`- Actions
   |- Currently banned:	1
   |- Total banned:	3
   `- Banned IP list:	192.168.1.100
```

## 6. Logs prüfen

Fail2ban schreibt eigene Log-Dateien. Hier siehst du, welche IPs gesperrt wurden:

```bash
sudo tail -f /var/log/fail2ban.log
```

Typische Ausgabe:
```
2024-03-01 12:34:56,789 fail2ban.actions [1234]: NOTICE  [sshd] Ban 192.168.1.100
2024-03-01 13:34:56,789 fail2ban.actions [1234]: NOTICE  [sshd] Unban 192.168.1.100
```

Fehlgeschlagene SSH-Logins anzeigen (die Fail2ban überwacht):
```bash
sudo tail -f /var/log/auth.log
```

## 7. IPs manuell sperren

Du kannst eine IP auch manuell sperren, ohne auf Fehlversuche zu warten:

```bash
sudo fail2ban-client set sshd banip 192.168.1.100
```

## 8. IPs manuell freischalten (entbannen)

Wenn du aus Versehen deine eigene IP oder die IP eines legitimen Nutzers gesperrt hast:

```bash
sudo fail2ban-client set sshd unbanip 192.168.1.100
```

Deine eigene aktuelle IP herausfinden:
```bash
curl ifconfig.me
```

## 9. Eigene IP dauerhaft von der Sperre ausschließen

Damit du dich nicht selbst aussperrst, kannst du deine IP auf die [Whitelist](../glossar.md#whitelist) setzen. In `/etc/fail2ban/jail.local` unter `[DEFAULT]`:

```ini
[DEFAULT]
ignoreip = 127.0.0.1/8 ::1 DEINE.IP.ADRESSE.HIER
```

Beispiel:
```ini
ignoreip = 127.0.0.1/8 ::1 203.0.113.42
```

## 10. Nützliche Befehle – Übersicht

| Befehl | Was er macht |
|--------|--------------|
| `sudo fail2ban-client status` | Alle aktiven Jails anzeigen |
| `sudo fail2ban-client status sshd` | Status des SSH-Jails anzeigen |
| `sudo fail2ban-client set sshd banip <IP>` | IP manuell sperren |
| `sudo fail2ban-client set sshd unbanip <IP>` | IP manuell freischalten |
| `sudo systemctl restart fail2ban` | Fail2ban neu starten (nach Konfigurationsänderung) |
| `sudo tail -f /var/log/fail2ban.log` | Logs in Echtzeit verfolgen |

## 11. Häufige Fehler

**Problem:** Fail2ban startet nicht nach Konfigurationsänderung.
**Lösung:** Konfiguration auf Syntaxfehler prüfen:
```bash
sudo fail2ban-client -t
```

**Problem:** Du hast dich selbst gesperrt und kommst nicht mehr per SSH rein.
**Lösung:** Nutze die Notfallkonsole deines Cloud-Anbieters (z.B. VNC-Konsole bei Hetzner, "Emergency Console" bei DigitalOcean). Dort kannst du die IP entbannen:
```bash
sudo fail2ban-client set sshd unbanip DEINE.IP
```

**Problem:** Fail2ban sperrt niemanden, obwohl es Angriffe gibt.
**Lösung:** Prüfe, ob der Log-Pfad in der Jail-Konfiguration korrekt ist:
```bash
sudo fail2ban-client get sshd logpath
```

# Firewall mit UFW - Spickzettel

## 1. Was ist eine Firewall?

Eine [Firewall](../glossar.md#firewall) ist wie ein Türsteher für deinen Server. Sie entscheidet, welche Verbindungen rein- und rausdürfen.

Ohne Firewall kann jeder auf der Welt versuchen, sich mit jedem offenen [Port](../glossar.md#port) deines Servers zu verbinden. Das ist so, als würdest du dein Haus mit weit offenen Türen und Fenstern stehen lassen.

**Windows-Analogie:** Windows hat eine eingebaute Firewall (Windows Defender Firewall), die du in der Systemsteuerung findest. [UFW](../glossar.md#ufw-uncomplicated-firewall) ist das Linux-Äquivalent, aber über die Kommandozeile bedient.

UFW steht für **Uncomplicated Firewall** – sie ist absichtlich einfach gehalten und eignet sich gut für Anfänger.

## 2. UFW installieren und aktivieren

UFW ist auf Ubuntu 24.04 bereits vorinstalliert. Prüfe zuerst den Status:

```bash
sudo ufw status
```

Ausgabe, wenn deaktiviert:
```
Status: inactive
```

Firewall aktivieren:
```bash
sudo ufw enable
```

Firewall deaktivieren (Vorsicht – nur zum Testen!):
```bash
sudo ufw disable
```

## 3. Die Standard-Policy: Alles sperren

Der erste und wichtigste Schritt: Alles blockieren, was nicht explizit erlaubt ist.

```bash
sudo ufw default deny incoming   # Alle eingehenden Verbindungen blockieren
sudo ufw default allow outgoing  # Alle ausgehenden Verbindungen erlauben
```

Das Prinzip dahinter: [**Allowlist**](../glossar.md#allowlist) **statt** [**Blocklist**](../glossar.md#blocklist). Statt zu versuchen, alles Schlechte zu blockieren, erlaubst du nur das Nötige.

> Wichtig: Richte Regeln ein, bevor du UFW aktivierst – sonst sperrst du dich möglicherweise aus!

## 4. Regeln hinzufügen: Ports erlauben

### SSH erlauben (Port 22)

[SSH](../glossar.md#ssh-secure-shell) ist die Verbindung, über die du überhaupt auf deinen Server zugreifst. Ohne diese Regel sperrst du dich aus!

```bash
sudo ufw allow ssh
```

Das ist eine Abkürzung für:
```bash
sudo ufw allow 22/tcp
```

### HTTP und HTTPS erlauben (Web-Traffic)

```bash
sudo ufw allow http    # Port 80
sudo ufw allow https   # Port 443
```

Oder mit Portnummern:
```bash
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
```

### Einen benutzerdefinierten Port erlauben

```bash
sudo ufw allow 8080/tcp
```

## 5. Regeln hinzufügen: Ports sperren

```bash
sudo ufw deny 3306/tcp   # MySQL-Port sperren
sudo ufw deny 5432/tcp   # PostgreSQL-Port sperren
```

Datenbank-Ports sollten niemals direkt aus dem Internet erreichbar sein!

## 6. Regeln entfernen

Wenn du eine Regel nicht mehr brauchst:

```bash
sudo ufw delete allow 8080/tcp
```

Oder zuerst alle Regeln auflisten, dann per Nummer löschen:
```bash
sudo ufw status numbered
```
Ausgabe:
```
     To                         Action      From
     --                         ------      ----
[ 1] 22/tcp                     ALLOW IN    Anywhere
[ 2] 80/tcp                     ALLOW IN    Anywhere
[ 3] 443/tcp                    ALLOW IN    Anywhere
```

```bash
sudo ufw delete 3   # Löscht die dritte Regel (443/tcp)
```

## 7. Status und Regeln anzeigen

```bash
sudo ufw status          # Kurzübersicht
sudo ufw status verbose  # Detaillierte Ansicht mit Policies
sudo ufw status numbered # Mit Nummern (zum Löschen)
```

Beispielausgabe von `sudo ufw status verbose`:
```
Status: active
Logging: on (low)
Default: deny (incoming), allow (outgoing), disabled (routed)
New profiles: skip

To                         Action      From
--                         ------      ----
22/tcp                     ALLOW IN    Anywhere
80/tcp                     ALLOW IN    Anywhere
443/tcp                    ALLOW IN    Anywhere
```

## 8. Empfohlene Grundkonfiguration für einen Webserver

Die folgende Reihenfolge ist wichtig – erst SSH erlauben, dann aktivieren!

```bash
# Standard-Policy setzen
sudo ufw default deny incoming
sudo ufw default allow outgoing

# Notwendige Ports öffnen
sudo ufw allow ssh      # SSH (Port 22) – ZUERST!
sudo ufw allow http     # HTTP (Port 80)
sudo ufw allow https    # HTTPS (Port 443)

# Firewall aktivieren
sudo ufw enable

# Status prüfen
sudo ufw status verbose
```

## 9. Logging aktivieren

UFW kann blockierte Verbindungsversuche protokollieren:

```bash
sudo ufw logging on
```

Logs ansehen:
```bash
sudo tail -f /var/log/ufw.log
```

## 10. Nützliche Befehle – Übersicht

| Befehl | Was er macht |
|--------|--------------|
| `sudo ufw enable` | Firewall aktivieren |
| `sudo ufw disable` | Firewall deaktivieren |
| `sudo ufw status` | Status und Regeln anzeigen |
| `sudo ufw status verbose` | Detaillierten Status anzeigen |
| `sudo ufw allow ssh` | SSH erlauben |
| `sudo ufw allow 8080/tcp` | Port 8080 erlauben |
| `sudo ufw deny 3306/tcp` | Port 3306 sperren |
| `sudo ufw delete allow 8080/tcp` | Regel entfernen |
| `sudo ufw reset` | Alle Regeln zurücksetzen |

## 11. Häufige Fehler

**Problem:** Du sperrst dich per SSH aus.
**Lösung:** Immer zuerst SSH erlauben, bevor du UFW aktivierst. Bei manchen Cloud-Anbietern gibt es eine Notfallkonsole, über die du dich trotzdem einloggen kannst.

**Problem:** Eine Anwendung ist nicht erreichbar, obwohl sie läuft.
**Lösung:** Prüfe, ob der Port in UFW erlaubt ist:
```bash
sudo ufw status
```

**Problem:** Du willst alle Regeln zurücksetzen und neu anfangen.
**Lösung:**
```bash
sudo ufw reset   # Setzt alle Regeln zurück, deaktiviert UFW
```
Danach musst du UFW neu konfigurieren und aktivieren.

# Hosts, Ports und Protokolle - Spickzettel

## 1. Was ist ein Host?

Ein Host ist jedes Gerät im Netzwerk, das eine IP-Adresse hat und Daten senden oder empfangen kann.

Beispiele für Hosts:
- Dein Laptop (`192.168.1.10`)
- Ein Server im Rechenzentrum (`93.184.216.34`)
- Ein Raspberry Pi im Heimnetzwerk (`192.168.1.50`)
- Sogar dein Router ist ein Host

**Windows-Analogie:** Ein Host ist wie ein Haus. Jedes Haus (Host) hat eine Adresse (IP-Adresse). Aber ein Haus hat viele Türen – das sind die Ports.

## 2. Was ist ein Port?

Ein [Port](../glossar.md#port) ist eine nummerierte "Tür" an einem Host. Über Ports können auf einem einzigen Server viele verschiedene Dienste gleichzeitig laufen.

```
Eingehende Verbindung:
  93.184.216.34 : 443
  └─────────────┘ └───┘
      Host (IP)   Port (HTTPS)
```

Ports sind Zahlen von 0 bis 65535:
- **0–1023:** Bekannte Ports (Well-Known Ports) – reserviert für Standarddienste, erfordern Root
- **1024–49151:** Registrierte Ports – von Anwendungen genutzt
- **49152–65535:** Dynamische/Ephemere Ports – temporäre Verbindungen

**Analogie:** Der Host ist das Gebäude, die IP-Adresse die Straße + Hausnummer, der Port die Zimmernummer.

## 3. Bekannte Ports und ihre Dienste

| Port | Protokoll | Dienst | Beschreibung |
|------|-----------|--------|-------------|
| 22 | TCP | SSH | Sichere Fernverwaltung |
| 25 | TCP | SMTP | E-Mail versenden |
| 53 | TCP/UDP | DNS | Namensauflösung |
| 80 | TCP | HTTP | Unverschlüsseltes Web |
| 443 | TCP | HTTPS | Verschlüsseltes Web |
| 3306 | TCP | MySQL | MySQL-Datenbank |
| 5432 | TCP | PostgreSQL | PostgreSQL-Datenbank |
| 6379 | TCP | Redis | Redis Cache/Datenbank |
| 8080 | TCP | HTTP-Alt | Oft für Entwicklung/Proxy |
| 27017 | TCP | MongoDB | MongoDB-Datenbank |

**Merkhilfe:** Port 80 = [HTTP](../glossar.md#http-hypertext-transfer-protocol) (unverschlüsselt), Port 443 = [HTTPS](../glossar.md#https-hypertext-transfer-protocol-secure) (verschlüsselt). Im Browser tippst du normalerweise keine Ports – der Browser ergänzt sie automatisch.

## 4. TCP vs. UDP

Es gibt zwei grundlegende Transportprotokolle:

### [TCP](../glossar.md#tcp-transmission-control-protocol) (Transmission Control Protocol)
- **Zuverlässig:** Datenpakete werden in richtiger Reihenfolge zugestellt
- **Verbindungsorientiert:** Es wird eine Verbindung aufgebaut (Handshake), bevor Daten fließen
- **Langsamer:** Bestätigung jedes Pakets kostet Zeit
- **Verwendung:** HTTP/HTTPS, SSH, E-Mail, Datenbanken – überall wo Korrektheit wichtig ist

### [UDP](../glossar.md#udp-user-datagram-protocol) (User Datagram Protocol)
- **Unzuverlässig:** Pakete können verloren gehen oder in falscher Reihenfolge ankommen
- **Verbindungslos:** Pakete werden einfach losgeschickt, kein Handshake
- **Schneller:** Kein Overhead durch Bestätigungen
- **Verwendung:** DNS, Video-Streaming, Online-Spiele, VoIP – überall wo Geschwindigkeit wichtiger als Vollständigkeit ist

**Analogie:**
- TCP = Einschreiben: Du weißt, dass der Brief angekommen ist.
- UDP = normale Postkarte: Du schickst sie ab und hoffst das Beste.

## 5. Offene Ports anzeigen mit ss

`ss` (Socket Statistics) zeigt, welche Ports auf deinem Server gerade lauschen (offen sind).

```bash
# Alle lauschenden TCP- und UDP-Ports anzeigen
ss -tulnp
```

**Bedeutung der Flags:**
| Flag | Bedeutung |
|------|-----------|
| `-t` | TCP-Verbindungen |
| `-u` | UDP-Verbindungen |
| `-l` | Nur lauschende (listening) Sockets |
| `-n` | Nummern statt Namen anzeigen (kein DNS) |
| `-p` | Prozess anzeigen, der den Port nutzt |

**Beispielausgabe:**
```
Netid  State   Recv-Q  Send-Q  Local Address:Port  Peer Address:Port  Process
tcp    LISTEN  0       128     0.0.0.0:22           0.0.0.0:*          users:(("sshd",pid=1234,fd=3))
tcp    LISTEN  0       511     0.0.0.0:80           0.0.0.0:*          users:(("nginx",pid=5678,fd=6))
tcp    LISTEN  0       511     0.0.0.0:443          0.0.0.0:*          users:(("nginx",pid=5678,fd=7))
```

- `0.0.0.0:22` = Port 22 lauscht auf allen Netzwerkinterfaces
- `127.0.0.1:5432` = Port 5432 (PostgreSQL) nur auf localhost erreichbar (gut so!)

**Alternativ mit netstat (älter, aber bekannt):**
```bash
sudo netstat -tulnp
```

## 6. Ports mit der Firewall verwalten (ufw)

Ubuntu verwendet `ufw` ([UFW](../glossar.md#ufw-uncomplicated-firewall) – Uncomplicated Firewall) als einfache [Firewall](../glossar.md#firewall)-Verwaltung.

**Status anzeigen:**
```bash
sudo ufw status
sudo ufw status verbose
```

**Firewall einschalten (Vorsicht: erst SSH erlauben!):**
```bash
sudo ufw allow 22/tcp   # SSH erst erlauben!
sudo ufw enable         # Erst dann einschalten
```

**Ports erlauben:**
```bash
# Einzelnen Port erlauben
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp

# Nach Dienstname (wenn bekannt)
sudo ufw allow ssh
sudo ufw allow http
sudo ufw allow https

# Port-Bereich erlauben
sudo ufw allow 8000:8080/tcp
```

**Ports sperren:**
```bash
sudo ufw deny 3306/tcp    # MySQL nicht von außen erreichbar
sudo ufw deny 5432/tcp    # PostgreSQL nicht von außen erreichbar
```

**Regel entfernen:**
```bash
sudo ufw delete allow 8080/tcp
```

**Firewall-Regeln anzeigen (nummeriert):**
```bash
sudo ufw status numbered
```

**Wichtige Sicherheitsregel:** Datenbanken (MySQL, PostgreSQL, Redis) sollten niemals öffentlich erreichbar sein. Nur Ports öffnen, die tatsächlich gebraucht werden.

## 7. Verbindungen von außen testen

Vom eigenen Rechner aus testen, ob ein Port erreichbar ist:

```bash
# Mit nc (netcat)
nc -zv meinserver.de 22
nc -zv meinserver.de 443

# Mit curl (HTTP/HTTPS)
curl -I http://meinserver.de
curl -I https://meinserver.de
```

Auf dem Server selbst prüfen, ob ein Dienst lokal antwortet:
```bash
curl http://localhost:8080
curl http://127.0.0.1:3000
```

## 8. Localhost und 127.0.0.1

`localhost` und `127.0.0.1` sind immer der eigene Rechner (Loopback-Interface). Dienste, die nur auf `127.0.0.1` lauschen, sind nicht von außen erreichbar.

```bash
# Diese Dienste sind nur lokal erreichbar
ss -tulnp | grep 127.0.0.1

# Diese Dienste sind von außen erreichbar
ss -tulnp | grep '0.0.0.0'
```

**Best Practice:** Datenbanken nur auf `127.0.0.1` binden lassen – der Webserver auf demselben Server kann sie trotzdem erreichen, aber niemand von außen.

## 9. Troubleshooting

**Port ist belegt:**
```bash
# Welcher Prozess nutzt Port 8080?
sudo ss -tulnp | grep :8080
sudo lsof -i :8080
```

**Verbindung schlägt fehl:**
```bash
# Ist der Dienst überhaupt am Laufen?
sudo systemctl status nginx

# Lauscht er auf dem richtigen Port?
ss -tulnp | grep nginx

# Lässt die Firewall es durch?
sudo ufw status
```

## 10. Referenztabelle

| Befehl | Was es macht |
|--------|-------------|
| `ss -tulnp` | Alle offenen Ports anzeigen |
| `sudo ufw status` | Firewall-Regeln anzeigen |
| `sudo ufw allow 443/tcp` | Port 443 öffnen |
| `sudo ufw deny 3306/tcp` | Port 3306 sperren |
| `nc -zv host port` | Verbindung zu Port testen |
| `sudo lsof -i :PORT` | Prozess auf Port anzeigen |
| `curl http://localhost:PORT` | Lokalen Dienst testen |

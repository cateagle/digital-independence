# Reverse Proxy - Spickzettel

## 1. Was ist ein Reverse Proxy?

Ein [Reverse Proxy](../glossar.md#reverse-proxy) ist ein Server, der zwischen dem Internet und deinen eigentlichen Diensten sitzt. Anfragen kommen beim Reverse Proxy an, der sie dann an den richtigen internen Dienst weiterleitet.

```
Internet
   │
   ▼
[ Reverse Proxy ]   ← einziger öffentlich erreichbarer Punkt
   │
   ├──► Dienst A (localhost:3000)  – z.B. eine Node.js-App
   ├──► Dienst B (localhost:8080)  – z.B. ein Wiki
   └──► Dienst C (localhost:5000)  – z.B. eine API
```

**Windows-Analogie:** Wie die Empfangsdame in einem Bürogebäude. Alle Besucher sprechen zuerst mit ihr. Sie weiß, wen man besuchen möchte, und leitet einen ans richtige Büro weiter. Die internen Büros (Dienste) sind von außen nicht direkt erreichbar.

## 2. Warum braucht man einen Reverse Proxy?

**Mehrere Dienste auf einem Server unter Port 80/443:**
Ohne Reverse Proxy könnte auf Port 443 nur ein einziger Dienst lauschen. Mit einem Reverse Proxy kann derselbe Port unterschiedliche Domains auf unterschiedliche Dienste weiterleiten:

```
https://wiki.example.com   →  Reverse Proxy  →  localhost:8080
https://app.example.com    →  Reverse Proxy  →  localhost:3000
https://api.example.com    →  Reverse Proxy  →  localhost:5000
```

**Weitere Vorteile:**
- **TLS-Terminierung:** [TLS](../glossar.md#tls-transport-layer-security) wird einmalig am Proxy beendet – interne Dienste brauchen kein [HTTPS](../glossar.md#https-hypertext-transfer-protocol-secure)
- **Sicherheit:** Interne Dienste sind nicht direkt aus dem Internet erreichbar
- **Load Balancing:** Anfragen können auf mehrere Server verteilt werden
- **Caching:** Statische Inhalte können gecacht werden
- **Komprimierung:** Antworten können komprimiert werden (gzip)
- **Logging:** Zentrales Zugriffslogging an einer Stelle

## 3. Nginx als Reverse Proxy

[Nginx](../glossar.md#nginx-engine-x) ist ein leistungsstarker Webserver, der auch als Reverse Proxy eingesetzt wird. Er ist auf Ubuntu sehr verbreitet.

**Installation:**
```bash
sudo apt update
sudo apt install nginx
sudo systemctl enable --now nginx
```

### Grundlegende Reverse-Proxy-Konfiguration

Jede Domain bekommt eine eigene Konfigurationsdatei in `/etc/nginx/sites-available/`:

```bash
sudo nano /etc/nginx/sites-available/meine-app
```

```nginx
server {
    listen 80;
    server_name app.example.com;

    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

**Konfiguration aktivieren:**
```bash
# Symlink in sites-enabled erstellen
sudo ln -s /etc/nginx/sites-available/meine-app /etc/nginx/sites-enabled/

# Konfiguration testen (immer zuerst!)
sudo nginx -t

# Nginx neu laden (ohne Verbindungsunterbrechung)
sudo systemctl reload nginx
```

### Erklärung der proxy_set_header Zeilen

| Header | Bedeutung |
|--------|-----------|
| `Host` | Teilt dem Backend-Dienst mit, welche Domain angefragt wurde |
| `X-Real-IP` | Echte IP-Adresse des Besuchers (nicht die des Proxys) |
| `X-Forwarded-For` | Kette aller IP-Adressen auf dem Weg zum Server |
| `X-Forwarded-Proto` | Ob der Besucher http oder https verwendet hat |

Ohne diese Header würde der Backend-Dienst denken, alle Anfragen kämen von `127.0.0.1`.

### Komplettes Beispiel mit HTTPS und HTTP-Redirect

```nginx
# HTTP → HTTPS Weiterleitung
server {
    listen 80;
    server_name app.example.com;
    return 301 https://$host$request_uri;
}

# HTTPS mit Reverse Proxy
server {
    listen 443 ssl;
    server_name app.example.com;

    ssl_certificate /etc/letsencrypt/live/app.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/app.example.com/privkey.pem;
    include /etc/letsencrypt/options-ssl-nginx.conf;
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;

    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Für WebSocket-Unterstützung (z.B. bei Next.js, Socket.io):
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

### Mehrere Dienste auf einem Server

```nginx
# Datei: /etc/nginx/sites-available/wiki
server {
    listen 443 ssl;
    server_name wiki.example.com;
    # ... ssl-Einstellungen ...

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

```nginx
# Datei: /etc/nginx/sites-available/api
server {
    listen 443 ssl;
    server_name api.example.com;
    # ... ssl-Einstellungen ...

    location / {
        proxy_pass http://localhost:5000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### Statische Dateien direkt ausliefern

Nginx kann statische Dateien effizienter ausliefern als die meisten Anwendungen:

```nginx
server {
    listen 443 ssl;
    server_name example.com;
    # ... ssl-Einstellungen ...

    # Statische Dateien direkt ausliefern
    location /static/ {
        alias /var/www/meine-app/static/;
        expires 30d;
        add_header Cache-Control "public, immutable";
    }

    # Alles andere an die App weiterleiten
    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## 4. Nginx-Konfiguration verwalten

```bash
# Konfiguration auf Fehler prüfen (vor jedem Reload!)
sudo nginx -t

# Nginx neu laden (laufende Verbindungen bleiben offen)
sudo systemctl reload nginx

# Nginx neu starten (alle Verbindungen werden getrennt)
sudo systemctl restart nginx

# Nginx-Status anzeigen
sudo systemctl status nginx

# Access-Log live beobachten
sudo tail -f /var/log/nginx/access.log

# Error-Log live beobachten
sudo tail -f /var/log/nginx/error.log
```

**Verzeichnisstruktur von Nginx:**
```
/etc/nginx/
├── nginx.conf                 ← Haupt-Konfiguration (selten ändern)
├── sites-available/           ← Konfigurationen (aktiv oder nicht)
│   ├── default
│   └── meine-app
└── sites-enabled/             ← Symlinks auf aktive Konfigurationen
    └── meine-app → ../sites-available/meine-app
```

## 5. Traefik als Alternative

Traefik ist ein moderner Reverse Proxy, der besonders gut mit Docker zusammenarbeitet. Er konfiguriert sich automatisch, wenn neue Docker-Container starten.

**Vorteile gegenüber Nginx:**
- Automatische TLS-Zertifikate ohne manuellen [Certbot](../glossar.md#certbot)-Aufruf
- Konfiguration über Docker-Labels direkt am Container
- Eingebautes Dashboard für Übersicht

**Beispiel mit [Docker Compose](../glossar.md#docker-compose):**
```yaml
# docker-compose.yml
services:
  traefik:
    image: traefik:v3.0
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
      - ./traefik.yml:/traefik.yml
      - ./letsencrypt:/letsencrypt

  meine-app:
    image: meine-app:latest
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.meine-app.rule=Host(`app.example.com`)"
      - "traefik.http.routers.meine-app.entrypoints=websecure"
      - "traefik.http.routers.meine-app.tls.certresolver=letsencrypt"
```

Traefik erkennt den neuen Container automatisch, richtet das Routing ein und stellt das TLS-Zertifikat aus – ohne Nginx-Config-Dateien schreiben zu müssen.

**Empfehlung:** Für den Einstieg ist Nginx besser geeignet (einfacher zu verstehen, mehr Dokumentation). Traefik lohnt sich, wenn du viele Docker-basierte Dienste verwaltest.

## 6. Troubleshooting

**Nginx startet nicht / reload schlägt fehl:**
```bash
# Konfigurationsfehler finden
sudo nginx -t
# Gibt genau an, in welcher Zeile der Fehler liegt
```

**"502 Bad Gateway":**
- Der Backend-Dienst (z.B. localhost:3000) ist nicht erreichbar
```bash
# Ist der Dienst gestartet?
sudo systemctl status meine-app
# Lauscht er auf dem richtigen Port?
ss -tulnp | grep :3000
```

**"404 Not Found" obwohl der Dienst läuft:**
- `server_name` in der Nginx-Konfiguration prüfen
- Symlink in `sites-enabled` prüfen
```bash
ls -la /etc/nginx/sites-enabled/
```

**Besucher-IP ist immer 127.0.0.1 in den App-Logs:**
- `proxy_set_header X-Real-IP` und `X-Forwarded-For` fehlen oder die App liest sie nicht aus

## 7. Referenztabelle

| Befehl | Was es macht |
|--------|-------------|
| `sudo nginx -t` | Konfiguration auf Fehler prüfen |
| `sudo systemctl reload nginx` | Nginx neu laden (ohne Ausfall) |
| `sudo systemctl restart nginx` | Nginx neu starten |
| `sudo tail -f /var/log/nginx/error.log` | Fehlerlog live beobachten |
| `sudo ln -s /etc/nginx/sites-available/app /etc/nginx/sites-enabled/` | Konfiguration aktivieren |
| `sudo certbot --nginx -d app.example.com` | TLS-Zertifikat für Domain ausstellen |

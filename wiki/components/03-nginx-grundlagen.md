# Nginx Grundlagen - Spickzettel

## 1. Was ist Nginx?

Nginx (ausgesprochen: "Engine-X") ist ein [Webserver](../glossar.md#webserver) und [Reverse Proxy](../glossar.md#reverse-proxy). Beim [Self-Hosting](../glossar.md#self-hosting) übernimmt Nginx meist zwei Aufgaben:

1. **Statische Dateien ausliefern** - HTML, CSS, JS, Bilder direkt an den Browser schicken
2. **Reverse Proxy** - Anfragen von außen entgegennehmen und an einen anderen [Container](../glossar.md#container) (z.B. auf [Port](../glossar.md#port) 3000) weiterleiten

Nginx ist der erste Anlaufpunkt für alle Anfragen von außen. Es kümmert sich auch um [HTTPS](../glossar.md#https-hypertext-transfer-protocol-secure)/SSL.

## 2. Nginx als Docker Container starten

Das offizielle Nginx-Image von Docker Hub:

```bash
docker run -d \
  --name nginx \
  -p 80:80 \
  nginx:alpine
```

Die Standard-Testseite ist jetzt unter `http://deine-server-ip` erreichbar.

Mit Docker Compose (empfohlen):

```yaml
services:
  nginx:
    image: nginx:alpine
    container_name: nginx
    ports:
      - "80:80"
      - "443:443"
    restart: unless-stopped
```

```bash
docker compose up -d
docker compose ps  # Status prüfen
```

## 3. Konfiguration per Volume einbinden

Nginx-Konfigurationen werden als Dateien in den Container gemountet. Die wichtigsten Pfade im Container:

```
/etc/nginx/
├── nginx.conf              # Hauptkonfiguration
└── conf.d/                 # Einzelne Site-Konfigurationen (werden automatisch geladen)
    └── default.conf        # Standard-Konfiguration
```

**Projektstruktur auf dem Host:**
```
./nginx/
├── nginx.conf              # Optional: eigene Hauptkonfiguration
└── conf.d/
    └── meine-seite.conf    # Deine Site-Konfigurationen
```

**Docker Compose mit Volumes:**
```yaml
services:
  nginx:
    image: nginx:alpine
    container_name: nginx
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx/conf.d:/etc/nginx/conf.d:ro
      - ./nginx/nginx.conf:/etc/nginx/nginx.conf:ro  # Optional
    restart: unless-stopped
```

## 4. nginx.conf - Die Hauptkonfiguration

Die Datei legt globale Einstellungen fest. Das Alpine-Image bringt eine sinnvolle Standardkonfiguration mit - du wirst sie selten anpassen müssen.

**Ausschnitt der wichtigsten Bereiche:**
```nginx
# Globaler Block - gilt für den gesamten Nginx-Prozess
worker_processes auto;

events {
    worker_connections 1024;  # Maximale gleichzeitige Verbindungen
}

http {
    # HTTP-Block - gilt für alle Webserver-Konfigurationen
    include /etc/nginx/conf.d/*.conf;  # Lädt deine Site-Konfigurationen
}
```

## 5. Statische Seite ausliefern

Statische Dateien werden ebenfalls per Volume in den Container gemountet.

**Projektstruktur:**
```
./
├── docker-compose.yml
├── nginx/
│   └── conf.d/
│       └── meineseite.conf
└── html/
    └── index.html
```

**`nginx/conf.d/meineseite.conf`:**
```nginx
server {
    listen 80;
    server_name meineseite.example.com;

    root /var/www/meineseite;
    index index.html;

    location / {
        try_files $uri $uri/ =404;
    }
}
```

**Docker Compose:**
```yaml
services:
  nginx:
    image: nginx:alpine
    container_name: nginx
    ports:
      - "80:80"
    volumes:
      - ./nginx/conf.d:/etc/nginx/conf.d:ro
      - ./html:/var/www/meineseite:ro
    restart: unless-stopped
```

```bash
docker compose up -d
```

## 6. Reverse Proxy zu einem anderen Container

Das häufigste Muster beim Self-Hosting: Eine Anwendung läuft in einem eigenen Container, und Nginx leitet Anfragen von außen dorthin weiter.

**Wichtig:** Im Docker-Netzwerk werden Container über ihren Service-Namen angesprochen, nicht über `localhost`.

**`nginx/conf.d/meine-app.conf`:**
```nginx
server {
    listen 80;
    server_name app.example.com;

    location / {
        proxy_pass http://meine-app:3000;  # Service-Name aus docker-compose.yml

        # Wichtige Header weiterleiten
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

**Was die Header bedeuten:**
- `Host` - Die ursprüngliche Domain, damit die App weiß, unter welcher Domain sie läuft
- `X-Real-IP` - Die echte IP-Adresse des Besuchers (ohne Proxy wäre es die Nginx-IP)
- `X-Forwarded-Proto` - Ob die ursprüngliche Anfrage HTTP oder HTTPS war

**Docker Compose mit App und Nginx im selben Netzwerk:**
```yaml
services:
  nginx:
    image: nginx:alpine
    container_name: nginx
    ports:
      - "80:80"
    volumes:
      - ./nginx/conf.d:/etc/nginx/conf.d:ro
    depends_on:
      - meine-app
    restart: unless-stopped

  meine-app:
    image: meine-app:latest
    container_name: meine-app
    expose:
      - "3000"     # Nur intern erreichbar, kein Port-Mapping nach außen
    restart: unless-stopped
```

Beide Services sind automatisch im selben Standard-Netzwerk und können sich gegenseitig über den Service-Namen erreichen.

## 7. SSL mit Let's Encrypt (certbot)

[Let's Encrypt](../glossar.md#lets-encrypt) bietet kostenlose SSL-Zertifikate. Mit Docker wird [Certbot](../glossar.md#certbot) als separater Container ausgeführt.

**Docker Compose mit certbot:**
```yaml
services:
  nginx:
    image: nginx:alpine
    container_name: nginx
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx/conf.d:/etc/nginx/conf.d:ro
      - ./certbot/conf:/etc/letsencrypt:ro
      - ./certbot/www:/var/www/certbot:ro
    restart: unless-stopped

  certbot:
    image: certbot/certbot
    volumes:
      - ./certbot/conf:/etc/letsencrypt
      - ./certbot/www:/var/www/certbot
```

**Nginx-Konfiguration für die certbot-Challenge:**
```nginx
server {
    listen 80;
    server_name app.example.com;

    location /.well-known/acme-challenge/ {
        root /var/www/certbot;
    }

    location / {
        return 301 https://$host$request_uri;
    }
}
```

**Zertifikat beantragen:**
```bash
docker compose run --rm certbot certonly \
  --webroot \
  --webroot-path=/var/www/certbot \
  -d app.example.com \
  --email deine@email.com \
  --agree-tos
```

**Nginx nach Erhalt des Zertifikats - HTTPS-Konfiguration:**
```nginx
server {
    listen 80;
    server_name app.example.com;
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl;
    server_name app.example.com;

    ssl_certificate /etc/letsencrypt/live/app.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/app.example.com/privkey.pem;

    location / {
        proxy_pass http://meine-app:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

**Zertifikat erneuern (manuell oder per Cronjob):**
```bash
docker compose run --rm certbot renew
docker compose exec nginx nginx -s reload
```

## 8. Konfiguration testen

Bevor du Nginx neu lädst, immer die Konfiguration prüfen. Ein Syntaxfehler kann dazu führen, dass Nginx nicht startet.

```bash
docker compose exec nginx nginx -t
```

Ausgabe bei korrekter Konfiguration:
```
nginx: the configuration file /etc/nginx/nginx.conf syntax is ok
nginx: configuration file /etc/nginx/nginx.conf test is successful
```

Ausgabe bei einem Fehler:
```
nginx: [emerg] unexpected "}" in /etc/nginx/conf.d/meine-app.conf:12
nginx: configuration file /etc/nginx/nginx.conf test failed
```

Der Fehler zeigt Datei und Zeilennummer - dort nachschauen und korrigieren.

## 9. reload vs. restart

| Befehl | Was passiert | Wann verwenden |
|---|---|---|
| `docker compose exec nginx nginx -s reload` | Konfiguration neu laden, laufende Verbindungen bleiben offen | Nach Konfigurations-Änderungen |
| `docker compose restart nginx` | Container neu starten, alle Verbindungen unterbrochen | Nach Updates oder wenn reload nicht hilft |

**Faustregel:** Immer zuerst `nginx -t`, dann `reload`. Nur wenn nötig `restart`.

```bash
# Standard-Workflow nach Konfigurationsänderungen:
docker compose exec nginx nginx -t && docker compose exec nginx nginx -s reload
```

## 10. Häufige Befehle im Überblick

| Befehl | Bedeutung |
|---|---|
| `docker compose up -d nginx` | Nginx-Container starten |
| `docker compose stop nginx` | Nginx-Container stoppen |
| `docker compose restart nginx` | Nginx-Container neu starten |
| `docker compose exec nginx nginx -t` | Konfiguration auf Syntaxfehler prüfen |
| `docker compose exec nginx nginx -T` | Konfiguration anzeigen (alle includes aufgelöst) |
| `docker compose exec nginx nginx -s reload` | Konfiguration neu laden |
| `docker compose logs -f nginx` | Alle Logs in Echtzeit verfolgen |
| `docker compose exec nginx tail -f /var/log/nginx/error.log` | Nur Fehlerlog verfolgen |

## 11. Troubleshooting

**Nginx startet nicht:**
```bash
docker compose exec nginx nginx -t    # Syntaxfehler finden
docker compose logs nginx             # Container-Logs anzeigen
```

**502 Bad Gateway:**
- Der Ziel-Container läuft nicht oder ist nicht erreichbar
- Service-Namen in der Konfiguration prüfen: `proxy_pass http://service-name:port`
- Netzwerk prüfen: `docker compose exec nginx ping meine-app`

**403 Forbidden:**
- Berechtigungsproblem auf dem gemounteten Verzeichnis
- Volume-Pfad im Container prüfen: `docker compose exec nginx ls -la /var/www/meineseite`

**Zertifikat lässt sich nicht beantragen:**
- Domain muss auf den Server zeigen (DNS-Eintrag prüfen)
- Port 80 muss von außen erreichbar sein (Firewall prüfen)
- certbot-Challenge-Pfad in der Nginx-Konfiguration prüfen

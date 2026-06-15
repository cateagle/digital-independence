# Nextcloud AIO - Lokale VM: Komplette Troubleshooting-Doku

**Kontext:** Nextcloud AIO auf Ubuntu 24.04 LTS in VirtualBox (NAT), Zugriff vom Windows-Host.
**Ergebnis:** Funktionierendes Nextcloud unter `https://nextcloud.local:8443`

---

## Finale Architektur

```
Firefox (Windows)
  → https://nextcloud.local:8443
  → Windows hosts-Datei: nextcloud.local = 127.0.0.1
  → VirtualBox NAT: Host:8443 → Gast:443
  → nginx (selbst-signiertes Zertifikat, Port 443)
  → http://localhost:11000 (intern)
  → Nextcloud AIO Apache (Caddy, Reverse-Proxy-Modus)
  → Nextcloud PHP-FPM
  → PostgreSQL-Datenbank
```

---

## VirtualBox Port-Weiterleitung

Einstellungen → Netzwerk → Adapter 1 → Erweitert → Port-Weiterleitung:

| Host-Port | Gast-Port | Zweck |
|-----------|-----------|-------|
| 8022 | 22 | SSH |
| 8080 | 80 | HTTP |
| 8443 | 443 | HTTPS (nginx) |
| 8888 | 8080 | AIO Admin |
| 11000 | 11000 | AIO Apache intern |

---

## docker-compose.yml (funktionierend)

```yaml
services:
  nextcloud-aio-mastercontainer:
    image: ghcr.io/nextcloud-releases/all-in-one:latest
    restart: unless-stopped
    container_name: nextcloud-aio-mastercontainer
    ports:
      - "8080:8080"
    environment:
      APACHE_PORT: 11000
      APACHE_IP_BINDING: 0.0.0.0
      SKIP_DOMAIN_VALIDATION: true
    volumes:
      - nextcloud_aio_mastercontainer:/mnt/docker-aio-config
      - /var/run/docker.sock:/var/run/docker.sock:ro

volumes:
  nextcloud_aio_mastercontainer:
    name: nextcloud_aio_mastercontainer
```

**Warum `name:` explizit?** Ohne das fuegt Docker Compose den Ordnernamen als Prefix hinzu (`nextcloud_nextcloud_aio_mastercontainer`) - AIO findet dann seinen eigenen Speicher nicht.

**Warum `APACHE_PORT: 11000`?** Reverse-Proxy-Modus. AIO versucht im Standard-Modus ein Let's Encrypt-Zertifikat fuer `nextcloud.local` zu holen - das schlaegt fuer lokale Domains fehl. Im Reverse-Proxy-Modus uebernimmt nginx die SSL-Verschluesselung.

---

## Docker DNS reparieren

Falls Docker keine externen Images downloaden kann:

```bash
sudo nano /etc/docker/daemon.json
```

Inhalt:
```json
{
  "dns": ["8.8.8.8", "8.8.4.4"]
}
```

```bash
sudo systemctl restart docker
```

**Warum?** Ubuntu nutzt intern `127.0.0.53` (systemd-resolved) als DNS. Docker-Container koennen diesen nicht erreichen.

---

## Windows hosts-Datei

Datei oeffnen als Administrator:
```
C:\Windows\System32\drivers\etc\hosts
```

Eintrag hinzufuegen (falls nicht vorhanden):
```
127.0.0.1 nextcloud.local
```

**Wichtig:** Keine `#` davor - das macht die Zeile zum Kommentar und Windows ignoriert sie.

---

## nginx als Reverse Proxy

### Installation
```bash
sudo apt install nginx -y
```

### Selbst-signiertes Zertifikat
```bash
sudo openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout /etc/ssl/private/nextcloud.key \
  -out /etc/ssl/certs/nextcloud.crt \
  -subj "/CN=nextcloud.local"
```

### Konfiguration
```bash
sudo nano /etc/nginx/sites-available/nextcloud
```

Inhalt:
```nginx
server {
    listen 443 ssl;
    server_name nextcloud.local;

    ssl_certificate /etc/ssl/certs/nextcloud.crt;
    ssl_certificate_key /etc/ssl/private/nextcloud.key;

    client_max_body_size 512M;

    location / {
        proxy_pass http://localhost:11000;
        proxy_set_header Host $host;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto https;
        proxy_read_timeout 86400;
    }
}
```

```bash
sudo ln -s /etc/nginx/sites-available/nextcloud /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
```

---

## Firewall oeffnen

```bash
sudo ufw allow 443
sudo ufw allow 80
```

**Warum?** Ubuntu's ufw-Firewall blockiert standardmaessig alle Ports ausser SSH. nginx auf Port 443 ist ohne diese Regel von aussen nicht erreichbar.

---

## Nextcloud: Override-Config fuer richtigen Port

Nextcloud generiert interne Links basierend auf seiner Domain-Konfiguration. Ohne Override wuerde es auf `https://nextcloud.local/login` (Port 443) weiterleiten statt auf Port 8443.

```bash
docker exec -it nextcloud-aio-nextcloud bash
```

```bash
cat > /var/www/html/config/z_override.config.php << 'EOF'
<?php
$CONFIG = [
  'overwritehost' => 'nextcloud.local:8443',
  'overwrite.cli.url' => 'https://nextcloud.local:8443',
];
EOF
exit
```

**Warum `z_` im Dateinamen?** Nextcloud laedt alle `*.config.php` Dateien alphabetisch. AIO hat eine `reverse-proxy.config.php` die eigene Werte setzt. `z_override.config.php` kommt alphabetisch danach und gewinnt.

---

## Zugriff

Browser: `https://nextcloud.local:8443`

Firefox zeigt Zertifikatswarnung (selbst-signiertes Zertifikat ist normal fuer lokale VMs):
→ "Erweitert" → "Weiter zu nextcloud.local:8443 (riskant)"

---

## Alle Probleme und Loesungen

| Problem | Ursache | Loesung |
|---------|---------|---------|
| Docker downloaded nichts | Kein DNS in Docker-Containern | `/etc/docker/daemon.json` mit 8.8.8.8 |
| AIO findet Volume nicht | Compose-Prefix automatisch hinzugefuegt | `name:` explizit in YAML setzen |
| `nextcloud.local` nicht aufloesbar | hosts-Datei-Eintrag auskommentiert | `#` entfernen |
| `SSL_ERROR_INTERNAL_ERROR_ALERT` | Let's Encrypt schlaegt fuer lokale Domains fehl | Reverse-Proxy-Modus + `SKIP_DOMAIN_VALIDATION` |
| Nextcloud-Login schlaegt fehl | Alter DB-Container, neue Passwoerter | AIO-Volumes loeschen, neu aufsetzen |
| Browser: "kann nicht verbinden" | ufw-Firewall blockiert Port 443 | `sudo ufw allow 443` |
| Redirect auf Port 443 statt 8443 | Nextcloud kannte externen Port nicht | `z_override.config.php` mit `overwritehost` |
| `occ`-Aenderungen ignoriert | `reverse-proxy.config.php` ueberschreibt sie | Datei mit `z_`-Prefix anlegen |

---

## Neustart-Anleitung (naechstes Mal)

```bash
# SSH verbinden
ssh -p 8022  <dein-benutzername>@127.0.0.1

# Nextcloud starten
cd ~/nextcloud
docker compose up -d

# AIO Admin pruefen
# https://localhost:8888
# Login: <dein-aio-passphrase>
# "Start and update containers" klicken

# Nextcloud oeffnen
# https://nextcloud.local:8443
# admin / <dein-passphrase>
```

---

*Erstellt: 2026-05-11 | Kurs: Digital Independence - OpenCampus Kiel*
